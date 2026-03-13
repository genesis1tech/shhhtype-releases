use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

const ACTIVATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/activate";
const DEACTIVATE_URL: &str = "https://api.lemonsqueezy.com/v1/licenses/deactivate";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LicenseStatus {
    Free,
    Licensed,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseInfo {
    pub license_key: String,
    pub instance_id: String,
    pub activated_at: String,
    pub machine_id: String,
}

#[derive(Deserialize)]
struct LemonSqueezyActivateResponse {
    activated: bool,
    instance: Option<LemonSqueezyInstance>,
    error: Option<String>,
}

#[derive(Deserialize)]
struct LemonSqueezyInstance {
    id: String,
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

fn license_path(data_dir: &Path) -> std::path::PathBuf {
    data_dir.join("license.json")
}

/// Activate a license key with LemonSqueezy.
pub fn activate_license(key: &str, data_dir: &Path) -> Result<LicenseInfo> {
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

    let info = LicenseInfo {
        license_key: key.to_string(),
        instance_id: instance.id,
        activated_at: chrono::Utc::now().to_rfc3339(),
        machine_id,
    };

    // Save to disk
    let content = serde_json::to_string_pretty(&info)?;
    std::fs::write(license_path(data_dir), content)?;

    Ok(info)
}

/// Check license status from local file (no network call).
pub fn check_license(data_dir: &Path) -> LicenseStatus {
    let path = license_path(data_dir);
    if !path.exists() {
        return LicenseStatus::Free;
    }

    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return LicenseStatus::Free,
    };

    let info: LicenseInfo = match serde_json::from_str(&content) {
        Ok(i) => i,
        Err(_) => return LicenseStatus::Invalid,
    };

    // Verify machine fingerprint matches
    if info.machine_id != get_machine_id() {
        return LicenseStatus::Invalid;
    }

    LicenseStatus::Licensed
}

/// Deactivate the license (network call + delete local file).
pub fn deactivate_license(data_dir: &Path) -> Result<()> {
    let path = license_path(data_dir);
    if !path.exists() {
        return Ok(());
    }

    let content = std::fs::read_to_string(&path)?;
    let info: LicenseInfo = serde_json::from_str(&content)?;

    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let resp = client
        .post(DEACTIVATE_URL)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "license_key": info.license_key,
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

    // Always delete local file even if network call fails
    let _ = std::fs::remove_file(&path);
    Ok(())
}
