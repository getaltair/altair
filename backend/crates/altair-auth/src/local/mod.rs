//! Local authentication provider
//!
//! Provides local authentication using OS keychain for token storage
//! and password hashing for credential verification.

pub mod keychain;
pub mod password;

// Re-export commonly used types and functions
pub use keychain::KeychainStorage;
pub use password::{generate_token, hash_password, verify_password};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Session information for local authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    /// User ID associated with this session
    pub user_id: String,
    /// Session token
    pub token: Option<String>,
    /// Session expiration
    pub expires_at: Option<DateTime<Utc>>,
}

impl Session {
    /// Create a new session with auto-generated token
    ///
    /// When a device ID is provided (non-None), a secure random token
    /// is generated and expiration is set to 24 hours from now.
    pub fn new(user_id: String, _device_id: Option<String>) -> Self {
        // Generate a secure random token if we have a device
        let (token, expires_at) = if _device_id.is_some() {
            let token = password::generate_token();
            let expires = Utc::now() + chrono::Duration::hours(24);
            (Some(token), Some(expires))
        } else {
            (None, None)
        };

        Self {
            user_id,
            token,
            expires_at,
        }
    }

    /// Check if the session has expired
    pub fn is_expired(&self) -> bool {
        match self.expires_at {
            Some(expires) => Utc::now() > expires,
            None => true, // No expiration = invalid session
        }
    }
}

/// Placeholder for local auth provider
pub struct LocalAuthProvider;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation_with_device() {
        let session = Session::new("user123".to_string(), Some("device-abc".to_string()));
        assert_eq!(session.user_id, "user123");
        assert!(session.token.is_some());
        assert!(session.expires_at.is_some());
        // Token should be 64 hex chars (32 bytes)
        assert_eq!(session.token.as_ref().unwrap().len(), 64);
    }

    #[test]
    fn test_session_creation_without_device() {
        let session = Session::new("user123".to_string(), None);
        assert_eq!(session.user_id, "user123");
        assert!(session.token.is_none());
        assert!(session.expires_at.is_none());
    }

    #[test]
    fn test_session_is_expired() {
        let session = Session {
            user_id: "user123".to_string(),
            token: Some("token".to_string()),
            expires_at: Some(Utc::now() - chrono::Duration::hours(1)),
        };
        assert!(session.is_expired());
    }

    #[test]
    fn test_session_not_expired() {
        let session = Session::new("user123".to_string(), Some("device".to_string()));
        assert!(!session.is_expired());
    }
}
