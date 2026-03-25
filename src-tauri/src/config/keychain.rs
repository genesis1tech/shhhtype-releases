/// Secure credential storage via macOS Keychain.
/// Falls back to plaintext on non-macOS platforms (not currently supported).

const SERVICE_NAME: &str = "com.g1tech.shhhtype";

/// Store a secret in the OS keychain.
pub fn set_secret(key: &str, value: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use security_framework::passwords::{set_generic_password, delete_generic_password};
        // Delete first to avoid "duplicate item" errors on update
        let _ = delete_generic_password(SERVICE_NAME, key);
        set_generic_password(SERVICE_NAME, key, value.as_bytes())
            .map_err(|e| format!("Keychain set failed for '{}': {}", key, e))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (key, value);
        Err("Keychain not available on this platform".into())
    }
}

/// Retrieve a secret from the OS keychain. Returns None if not found.
pub fn get_secret(key: &str) -> Option<String> {
    #[cfg(target_os = "macos")]
    {
        use security_framework::passwords::get_generic_password;
        get_generic_password(SERVICE_NAME, key)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = key;
        None
    }
}

/// Delete a secret from the OS keychain.
pub fn delete_secret(key: &str) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        use security_framework::passwords::delete_generic_password;
        delete_generic_password(SERVICE_NAME, key)
            .map_err(|e| format!("Keychain delete failed for '{}': {}", key, e))
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = key;
        Ok(())
    }
}
