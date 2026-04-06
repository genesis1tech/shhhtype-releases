use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::config::keychain;

const ACTIVATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/activate";
const VALIDATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/validate";
const DEACTIVATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/deactivate";
const KEYCHAIN_LICENSE_KEY: &str = "license_key";
const KEYCHAIN_TRIAL_START: &str = "trial_start";
const TRIAL_DAYS: i64 = 7;
/// Re-validate license with LemonSqueezy every 24 hours
const VALIDATION_INTERVAL_SECS: i64 = 86400;

/// Admin license keys — full access, never expire, no LemonSqueezy call.
const ADMIN_KEY_PREFIX: &str = "ADMIN-";

/// LemonSqueezy product ID for the beta product.
const BETA_PRODUCT_ID: u64 = 928696;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseStatus {
    Trial,
    TrialExpired,
    Licensed,
    Beta,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialInfo {
    pub days_remaining: i64,
    pub expired: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub license_key: String,
    pub instance_id: String,
    pub activated_at: String,
    pub machine_id: String,
    /// Last time the license was validated online (RFC 3339).
    #[serde(default)]
    pub last_validated: String,
    /// LemonSqueezy product ID — used to distinguish beta vs paid licenses.
    #[serde(default)]
    pub product_id: u64,
}

#[derive(Deserialize)]
struct LemonSqueezyMeta {
    product_id: u64,
}

#[derive(Deserialize)]
struct LemonSqueezyActivateResponse {
    activated: bool,
    instance: Option<LemonSqueezyInstance>,
    meta: Option<LemonSqueezyMeta>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct LemonSqueezyInstance {
    id: String,
}

#[derive(Deserialize)]
struct LemonSqueezyValidateResponse {
    valid: bool,
    meta: Option<LemonSqueezyMeta>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct LemonSqueezyDeactivateResponse {
    deactivated: bool,
    error: Option<String>,
}

/// Get a stable machine identifier by hashing the macOS hardware UUID.
pub fn get_machine_id() -> String {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("ioreg")
            .args(["-rd1", "-c", "IOPlatformExpertDevice"])
            .output()
            .ok();

        if let Some(output) = output {
            let text = String::from_utf8_lossy(&output.stdout);
            for line in text.lines() {
                if line.contains("IOPlatformUUID") {
                    if let Some(uuid) = line.split('"').nth(3) {
                        // Hash the UUID for privacy
                        use std::collections::hash_map::DefaultHasher;
                        use std::hash::{Hash, Hasher};
                        let mut hasher = DefaultHasher::new();
                        uuid.hash(&mut hasher);
                        return format!("{:x}", hasher.finish());
                    }
                }
            }
        }
        // Fallback: use hostname
        hostname_fallback()
    }
    #[cfg(not(target_os = "macos"))]
    {
        hostname_fallback()
    }
}

fn hostname_fallback() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let hostname = gethostname::gethostname().to_string_lossy().to_string();
    let mut hasher = DefaultHasher::new();
    hostname.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Check if a license key is a valid admin key.
/// Admin keys: ADMIN-<32 hex chars> — full access, never expire.
fn is_valid_admin_key(key: &str) -> bool {
    if !key.starts_with(ADMIN_KEY_PREFIX) {
        return false;
    }
    let suffix = &key[ADMIN_KEY_PREFIX.len()..];
    suffix.len() == 32 && suffix.chars().all(|c| c.is_ascii_hexdigit())
}

/// Check if a key is admin (local-only, no LemonSqueezy).
fn is_local_key(key: &str) -> bool {
    is_valid_admin_key(key)
}

/// Returns the license status for a given product_id.
pub fn status_for_product(product_id: u64) -> LicenseStatus {
    if product_id == BETA_PRODUCT_ID {
        LicenseStatus::Beta
    } else {
        LicenseStatus::Licensed
    }
}

fn license_path(data_dir: &Path) -> std::path::PathBuf {
    data_dir.join("license.json")
}

/// Activate a license key — admin keys validated locally, all others via LemonSqueezy.
pub fn activate_license(key: &str, data_dir: &Path) -> Result<LicenseInfo> {
    // Admin keys: validate format locally, store in keychain, skip LemonSqueezy
    if is_local_key(key) {
        let machine_id = get_machine_id();
        let now = chrono::Utc::now().to_rfc3339();
        let info = LicenseInfo {
            license_key: key.to_string(),
            instance_id: format!("admin-{}", &machine_id[..8]),
            activated_at: now.clone(),
            machine_id,
            last_validated: now,
            product_id: 0,
        };
        if let Err(e) = keychain::set_secret(KEYCHAIN_LICENSE_KEY, key) {
            return Err(anyhow!("Failed to store admin key: {}", e));
        }
        save_license_metadata(data_dir, &info);
        log::info!("admin license activated");
        return Ok(info);
    }

    let machine_id = get_machine_id();

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let resp = client
        .post(ACTIVATE_URL)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "license_key": key,
            "instance_name": machine_id,
        }))
        .send()?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().unwrap_or_default();
        return Err(anyhow!("Activation failed ({}): {}", status, body));
    }

    let response: LemonSqueezyActivateResponse = resp.json()?;

    if !response.activated {
        return Err(anyhow!(
            "Activation failed: {}",
            response.error.unwrap_or_else(|| "Unknown error".into())
        ));
    }

    let instance = response.instance.ok_or_else(|| anyhow!("No instance in response"))?;
    let meta = response.meta.ok_or_else(|| anyhow!("No product metadata in activation response"))?;
    let product_id = meta.product_id;

    let now = chrono::Utc::now().to_rfc3339();
    let info = LicenseInfo {
        license_key: key.to_string(),
        instance_id: instance.id,
        activated_at: now.clone(),
        machine_id,
        last_validated: now,
        product_id,
    };

    // Store license key securely in keychain
    if let Err(e) = keychain::set_secret(KEYCHAIN_LICENSE_KEY, key) {
        log::error!("Failed to store license key in keychain: {}", e);
    }

    // Save non-secret metadata to disk (license_key omitted)
    save_license_metadata(data_dir, &info);

    Ok(info)
}

/// Save license metadata to disk without the license key.
fn save_license_metadata(data_dir: &Path, info: &LicenseInfo) {
    let disk_info = LicenseInfo {
        license_key: String::new(),
        instance_id: info.instance_id.clone(),
        activated_at: info.activated_at.clone(),
        machine_id: info.machine_id.clone(),
        last_validated: info.last_validated.clone(),
        product_id: info.product_id,
    };
    if let Ok(content) = serde_json::to_string_pretty(&disk_info) {
        let _ = std::fs::write(license_path(data_dir), content);
    }
}

/// Check license status from local file + keychain.
/// A license.json without a matching keychain key is treated as invalid
/// (prevents users from creating a fake local file).
/// Returns Trial/TrialExpired if no license is activated.
pub fn check_license(data_dir: &Path) -> LicenseStatus {
    // Admin keys stored in keychain bypass LemonSqueezy validation
    if let Some(key) = keychain::get_secret(KEYCHAIN_LICENSE_KEY) {
        if is_valid_admin_key(&key) {
            return LicenseStatus::Licensed;
        }
        // Legacy local-only BETA-* keys are no longer valid — force re-activation
        if key.starts_with("BETA-") {
            log::warn!("Legacy BETA key found in keychain — requires re-activation via LemonSqueezy");
            let _ = keychain::delete_secret(KEYCHAIN_LICENSE_KEY);
            let _ = std::fs::remove_file(license_path(data_dir));
            return trial_status(data_dir);
        }
    }

    let path = license_path(data_dir);
    if !path.exists() {
        return trial_status(data_dir);
    }

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return trial_status(data_dir),
    };

    let mut info: LicenseInfo = match serde_json::from_str(&content) {
        Ok(i) => i,
        Err(_) => return LicenseStatus::Invalid,
    };

    // Verify machine fingerprint matches
    if info.machine_id != get_machine_id() {
        return LicenseStatus::Invalid;
    }

    // Migrate: if license_key is still in the JSON file, move it to keychain
    if !info.license_key.is_empty() {
        log::info!("Migrating license key from plaintext to keychain");
        let _ = keychain::set_secret(KEYCHAIN_LICENSE_KEY, &info.license_key);
        info.license_key = String::new();
        save_license_metadata(data_dir, &info);
    }

    // SECURITY: license.json alone is not sufficient — must have key in keychain.
    // This prevents users from crafting a fake license.json to bypass licensing.
    if keychain::get_secret(KEYCHAIN_LICENSE_KEY).is_none() {
        log::warn!("license.json exists but no license key in keychain — invalid");
        return LicenseStatus::Invalid;
    }

    // product_id = 0 means activated before this change — treat as Licensed
    // (the next 24h revalidation will refresh product_id from LemonSqueezy)
    if info.product_id == 0 {
        return LicenseStatus::Licensed;
    }

    // Distinguish beta vs paid by the product ID stored at activation
    status_for_product(info.product_id)
}

/// Validate license online with LemonSqueezy.
/// Called on app startup. Checks if 24h have passed since last validation.
/// On failure (revoked/expired), removes local license and demotes to trial state.
/// On network error, trusts the cached status (grace period).
pub fn validate_license_online(data_dir: &Path) {
    // Admin keys don't need online validation
    if let Some(key) = keychain::get_secret(KEYCHAIN_LICENSE_KEY) {
        if is_local_key(&key) {
            return;
        }
    }

    let path = license_path(data_dir);
    if !path.exists() {
        return;
    }

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return,
    };

    let mut info: LicenseInfo = match serde_json::from_str(&content) {
        Ok(i) => i,
        Err(_) => return,
    };

    // Check if validation is needed (every 24h)
    if !info.last_validated.is_empty() {
        if let Ok(last) = chrono::DateTime::parse_from_rfc3339(&info.last_validated) {
            let elapsed = chrono::Utc::now().signed_duration_since(last.with_timezone(&chrono::Utc));
            if elapsed.num_seconds() < VALIDATION_INTERVAL_SECS {
                log::info!("License validated {}s ago, skipping online check", elapsed.num_seconds());
                return;
            }
        }
    }

    // Get the license key from keychain
    let license_key = match keychain::get_secret(KEYCHAIN_LICENSE_KEY) {
        Some(k) => k,
        None => {
            log::warn!("No license key in keychain, removing invalid license file");
            let _ = std::fs::remove_file(&path);
            return;
        }
    };

    log::info!("Validating license with LemonSqueezy...");
    let client = match reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            log::warn!("Failed to create HTTP client for validation: {}", e);
            return; // Grace period — trust cached status
        }
    };

    let resp = match client
        .post(VALIDATE_URL)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "license_key": license_key,
            "instance_id": info.instance_id,
        }))
        .send()
    {
        Ok(r) => r,
        Err(e) => {
            log::warn!("License validation network error (grace period): {}", e);
            return; // Grace period — trust cached status on network failure
        }
    };

    if resp.status().is_success() {
        match resp.json::<LemonSqueezyValidateResponse>() {
            Ok(validation) => {
                if validation.valid {
                    log::info!("License validated successfully");
                    info.last_validated = chrono::Utc::now().to_rfc3339();
                    // Refresh product_id from server (handles product migrations)
                    if let Some(meta) = validation.meta {
                        if info.product_id != meta.product_id {
                            log::info!("Updating product_id: {} → {}", info.product_id, meta.product_id);
                        }
                        info.product_id = meta.product_id;
                    } else {
                        log::debug!("Validation response missing meta — preserving existing product_id");
                    }
                    save_license_metadata(data_dir, &info);
                } else {
                    log::warn!(
                        "License is no longer valid: {}",
                        validation.error.unwrap_or_else(|| "revoked or expired".into())
                    );
                    // License revoked/expired — remove local license
                    let _ = std::fs::remove_file(&path);
                    let _ = keychain::delete_secret(KEYCHAIN_LICENSE_KEY);
                }
            }
            Err(e) => {
                log::warn!("Failed to parse validation response: {}", e);
                // Grace period — don't revoke on parse error
            }
        }
    } else {
        log::warn!("License validation returned status {}", resp.status());
        // Grace period for server errors
    }
}

// ── Trial management ──────────────────────────────────────────────────

/// Ensure a trial start timestamp exists in the Keychain.
/// The trial start is stored in the OS Keychain (not a file) to prevent
/// users from deleting/modifying it to reset the trial period.
pub fn ensure_trial_start(data_dir: &Path) {
    if keychain::get_secret(KEYCHAIN_TRIAL_START).is_some() {
        return; // Already recorded
    }

    // Anti-tamper: if the app was already installed (onboarding done) but
    // the keychain entry is missing, someone may have cleared the keychain
    // to reset their trial. Treat as expired by setting start to 30 days ago.
    let onboarding_flag = data_dir.join(".onboarding_complete");
    if onboarding_flag.exists() {
        // Also check if there's an old .trial_start file we can migrate from
        let old_file = data_dir.join(".trial_start");
        if old_file.exists() {
            if let Ok(content) = std::fs::read_to_string(&old_file) {
                let trimmed = content.trim();
                if chrono::DateTime::parse_from_rfc3339(trimmed).is_ok() {
                    log::info!("Migrating trial start from file to keychain");
                    let _ = keychain::set_secret(KEYCHAIN_TRIAL_START, trimmed);
                    let _ = std::fs::remove_file(&old_file);
                    return;
                }
            }
        }

        // No valid file either — keychain was cleared after install.
        // Set trial start to 30 days ago (well past expiry).
        let expired_start = (chrono::Utc::now() - chrono::Duration::days(30)).to_rfc3339();
        log::warn!("Trial keychain entry missing on existing install — marking trial as expired");
        let _ = keychain::set_secret(KEYCHAIN_TRIAL_START, &expired_start);
        return;
    }

    // Fresh install — start trial now
    let now = chrono::Utc::now().to_rfc3339();
    let _ = keychain::set_secret(KEYCHAIN_TRIAL_START, &now);
    log::info!("Trial started: {}", now);
    crate::analytics::track("trial_started", serde_json::json!({}));
}

/// Get the number of days remaining in the trial (can be negative).
fn trial_days_remaining(_data_dir: &Path) -> i64 {
    let start_str = match keychain::get_secret(KEYCHAIN_TRIAL_START) {
        Some(s) => s,
        None => return 0, // No keychain entry = expired
    };
    let start = match chrono::DateTime::parse_from_rfc3339(start_str.trim()) {
        Ok(dt) => dt.with_timezone(&chrono::Utc),
        Err(_) => return 0,
    };
    let elapsed = chrono::Utc::now().signed_duration_since(start);
    TRIAL_DAYS - elapsed.num_days()
}

/// Return Trial or TrialExpired based on install date.
fn trial_status(data_dir: &Path) -> LicenseStatus {
    ensure_trial_start(data_dir);
    let remaining = trial_days_remaining(data_dir);
    if remaining > 0 {
        LicenseStatus::Trial
    } else {
        LicenseStatus::TrialExpired
    }
}

/// Get detailed trial information for the frontend.
pub fn get_trial_info(data_dir: &Path) -> TrialInfo {
    ensure_trial_start(data_dir);
    let days_remaining = trial_days_remaining(data_dir);
    let expired = days_remaining <= 0;

    let message = if expired {
        "Your trial has expired. Subscribe to continue using ShhhType.".to_string()
    } else if days_remaining == 1 {
        "1 day left. Subscribe to retain access!".to_string()
    } else if days_remaining == 2 {
        format!("{} days left. Don't lose access — subscribe now!", days_remaining)
    } else if days_remaining <= 3 {
        format!("{} days left in your trial. Subscribe soon!", days_remaining)
    } else {
        format!("{} days left in your free trial.", days_remaining)
    };

    TrialInfo {
        days_remaining,
        expired,
        message,
    }
}

/// Returns true if the app is usable (licensed or in active trial).
pub fn is_app_usable(data_dir: &Path) -> bool {
    matches!(
        check_license(data_dir),
        LicenseStatus::Licensed | LicenseStatus::Beta | LicenseStatus::Trial
    )
}

/// Deactivate the license (network call + delete local file + remove keychain entry).
pub fn deactivate_license(data_dir: &Path) -> Result<()> {
    let path = license_path(data_dir);
    if !path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&path)?;
    let info: LicenseInfo = serde_json::from_str(&content)?;

    // Retrieve the license key from keychain
    let license_key = keychain::get_secret(KEYCHAIN_LICENSE_KEY).unwrap_or_default();

    if !license_key.is_empty() {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()?;

        let resp = client
            .post(DEACTIVATE_URL)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "license_key": license_key,
                "instance_id": info.instance_id,
            }))
            .send()?;

        if resp.status().is_success() {
            let response: LemonSqueezyDeactivateResponse = resp.json()?;
            if !response.deactivated {
                log::warn!(
                    "Deactivation response: {}",
                    response.error.unwrap_or_else(|| "Unknown".into())
                );
            }
        }
    }

    // Always delete local file and keychain entry even if network call fails
    let _ = std::fs::remove_file(&path);
    let _ = keychain::delete_secret(KEYCHAIN_LICENSE_KEY);
    Ok(())
}
