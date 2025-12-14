//! OS Keychain integration for secure credential storage
//!
//! Provides cross-platform secure storage for session tokens and credentials
//! using the operating system's native keychain (macOS Keychain, Windows
//! Credential Manager, Linux Secret Service).

use altair_core::Result;

const SERVICE_NAME: &str = "altair";
const TOKEN_KEY: &str = "session_token";

/// Keychain storage for secure credential management
pub struct KeychainStorage {
    service: String,
}

impl KeychainStorage {
    /// Create a new keychain storage instance
    pub fn new() -> Self {
        Self {
            service: SERVICE_NAME.to_string(),
        }
    }

    /// Create a keychain storage with custom service name
    pub fn with_service(service: impl Into<String>) -> Self {
        Self {
            service: service.into(),
        }
    }

    /// Store a session token in the keychain
    pub async fn store_token(&self, token: &str) -> Result<()> {
        if token.is_empty() {
            return Err(altair_core::Error::Auth(
                "Cannot store empty token".to_string(),
            ));
        }

        let entry = keyring::Entry::new(&self.service, TOKEN_KEY)
            .map_err(|e| altair_core::Error::Auth(format!("Keychain error: {}", e)))?;

        entry
            .set_password(token)
            .map_err(|e| altair_core::Error::Auth(format!("Failed to store token: {}", e)))?;

        Ok(())
    }

    /// Retrieve the session token from the keychain
    pub async fn get_token(&self) -> Result<Option<String>> {
        let entry = keyring::Entry::new(&self.service, TOKEN_KEY)
            .map_err(|e| altair_core::Error::Auth(format!("Keychain error: {}", e)))?;

        match entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(altair_core::Error::Auth(format!(
                "Failed to get token: {}",
                e
            ))),
        }
    }

    /// Delete the session token from the keychain
    pub async fn delete_token(&self) -> Result<()> {
        let entry = keyring::Entry::new(&self.service, TOKEN_KEY)
            .map_err(|e| altair_core::Error::Auth(format!("Keychain error: {}", e)))?;

        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
            Err(e) => Err(altair_core::Error::Auth(format!(
                "Failed to delete token: {}",
                e
            ))),
        }
    }

    /// Store a generic credential
    pub async fn store_credential(&self, key: &str, value: &str) -> Result<()> {
        let entry = keyring::Entry::new(&self.service, key)
            .map_err(|e| altair_core::Error::Auth(format!("Keychain error: {}", e)))?;

        entry
            .set_password(value)
            .map_err(|e| altair_core::Error::Auth(format!("Failed to store credential: {}", e)))?;

        Ok(())
    }

    /// Retrieve a generic credential
    pub async fn get_credential(&self, key: &str) -> Result<Option<String>> {
        let entry = keyring::Entry::new(&self.service, key)
            .map_err(|e| altair_core::Error::Auth(format!("Keychain error: {}", e)))?;

        match entry.get_password() {
            Ok(value) => Ok(Some(value)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(altair_core::Error::Auth(format!(
                "Failed to get credential: {}",
                e
            ))),
        }
    }

    /// Delete a generic credential
    pub async fn delete_credential(&self, key: &str) -> Result<()> {
        let entry = keyring::Entry::new(&self.service, key)
            .map_err(|e| altair_core::Error::Auth(format!("Keychain error: {}", e)))?;

        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already deleted
            Err(e) => Err(altair_core::Error::Auth(format!(
                "Failed to delete credential: {}",
                e
            ))),
        }
    }
}

impl Default for KeychainStorage {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keychain_storage_creation() {
        let storage = KeychainStorage::new();
        assert_eq!(storage.service, SERVICE_NAME);
    }

    #[test]
    fn test_keychain_storage_custom_service() {
        let storage = KeychainStorage::with_service("custom-service");
        assert_eq!(storage.service, "custom-service");
    }
}
