use anyhow::Result;
use std::sync::LazyLock;

const GITHUB_RELEASES_URL: &str =
    "https://api.github.com/repos/genesis1tech/vox2txt/releases/latest";

/// Reuse a single HTTP client for update checks.
static HTTP_CLIENT: LazyLock<reqwest::blocking::Client> = LazyLock::new(|| {
    reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent(format!("ShhhType/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .expect("Failed to build HTTP client for update checker")
});

/// Information about the latest GitHub release.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct LatestRelease {
    pub tag_name: String,
    pub html_url: String,
    pub name: String,
}

/// Check GitHub for a newer release than the current app version.
/// Returns `Some(LatestRelease)` if a newer version exists, `None` otherwise.
pub fn check_for_update() -> Result<Option<LatestRelease>> {
    let resp = HTTP_CLIENT.get(GITHUB_RELEASES_URL).send()?;

    if !resp.status().is_success() {
        anyhow::bail!("GitHub API returned status {}", resp.status());
    }

    let release: LatestRelease = resp.json()?;

    let remote_version = release.tag_name.strip_prefix('v').unwrap_or(&release.tag_name);
    let current_version = env!("CARGO_PKG_VERSION");

    if is_newer(remote_version, current_version) {
        log::info!(
            "Update available: {} (current: {})",
            remote_version,
            current_version
        );
        Ok(Some(release))
    } else {
        log::info!(
            "App is up to date (current: {}, latest: {})",
            current_version,
            remote_version
        );
        Ok(None)
    }
}

/// Simple semver comparison: returns true if `remote` is newer than `current`.
fn is_newer(remote: &str, current: &str) -> bool {
    let parse = |v: &str| -> Vec<u32> {
        v.split('.')
            .filter_map(|s| s.parse::<u32>().ok())
            .collect()
    };
    let r = parse(remote);
    let c = parse(current);
    // Compare component by component
    for i in 0..r.len().max(c.len()) {
        let rv = r.get(i).copied().unwrap_or(0);
        let cv = c.get(i).copied().unwrap_or(0);
        if rv > cv {
            return true;
        }
        if rv < cv {
            return false;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_newer() {
        assert!(is_newer("0.2.0", "0.1.0"));
        assert!(is_newer("1.0.0", "0.9.9"));
        assert!(!is_newer("0.1.0", "0.1.0"));
        assert!(!is_newer("0.0.9", "0.1.0"));
        assert!(is_newer("0.1.1", "0.1.0"));
    }
}
