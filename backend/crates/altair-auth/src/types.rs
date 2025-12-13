//! Authentication types and errors
//!
//! This module defines the core types used in authentication operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User credential stored in database
///
/// Contains the password hash for a user. One-to-one relationship with User.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCredential {
    /// User ID this credential belongs to
    pub user_id: String,
    /// Argon2id password hash in PHC format
    pub password_hash: String,
    /// Timestamp of last password update
    pub updated_at: DateTime<Utc>,
}

impl UserCredential {
    /// Create a new user credential
    pub fn new(user_id: String, password_hash: String) -> Self {
        Self {
            user_id,
            password_hash,
            updated_at: Utc::now(),
        }
    }
}

/// Authentication response returned after successful login/register
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Authenticated user profile
    pub user: super::User,
    /// Session token for subsequent requests
    pub session_token: String,
    /// Session expiration timestamp
    pub expires_at: DateTime<Utc>,
}

impl AuthResponse {
    /// Create a new auth response
    pub fn new(user: super::User, session_token: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            user,
            session_token,
            expires_at,
        }
    }
}

/// Authentication errors
#[derive(Debug, Clone, Serialize, Deserialize, thiserror::Error)]
pub enum AuthError {
    /// Invalid email/password combination
    #[error("Invalid credentials")]
    InvalidCredentials,

    /// Session token has expired
    #[error("Session expired")]
    SessionExpired,

    /// OS keychain is not available on this platform
    #[error("Keychain unavailable: {0}")]
    KeychainUnavailable(String),

    /// User not found in database
    #[error("User not found")]
    UserNotFound,

    /// User already exists (registration conflict)
    #[error("User already exists")]
    UserAlreadyExists,

    /// Generic authentication error
    #[error("Authentication error: {0}")]
    Other(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_credential_creation() {
        let user_id = "user-123".to_string();
        let password_hash = "$argon2id$v=19$m=19456,t=2,p=1$...".to_string();

        let credential = UserCredential::new(user_id.clone(), password_hash.clone());

        assert_eq!(credential.user_id, user_id);
        assert_eq!(credential.password_hash, password_hash);
        assert!(credential.updated_at <= Utc::now());
    }

    #[test]
    fn test_user_credential_serialization() {
        let credential = UserCredential::new(
            "user-123".to_string(),
            "$argon2id$v=19$m=19456,t=2,p=1$...".to_string(),
        );

        let json = serde_json::to_string(&credential).unwrap();
        assert!(json.contains("user-123"));
        assert!(json.contains("argon2id"));

        let deserialized: UserCredential = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.user_id, credential.user_id);
        assert_eq!(deserialized.password_hash, credential.password_hash);
    }

    #[test]
    fn test_auth_response_creation() {
        let user = super::super::User {
            id: "user-123".to_string(),
            email: "test@example.com".to_string(),
            name: Some("Test User".to_string()),
        };
        let token = "session-token-xyz".to_string();
        let expires_at = Utc::now();

        let response = AuthResponse::new(user.clone(), token.clone(), expires_at);

        assert_eq!(response.user.id, user.id);
        assert_eq!(response.session_token, token);
        assert_eq!(response.expires_at, expires_at);
    }

    #[test]
    fn test_auth_response_serialization() {
        let user = super::super::User {
            id: "user-123".to_string(),
            email: "test@example.com".to_string(),
            name: Some("Test User".to_string()),
        };
        let response = AuthResponse::new(user, "token-123".to_string(), Utc::now());

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("user-123"));
        assert!(json.contains("token-123"));

        let deserialized: AuthResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.user.id, response.user.id);
        assert_eq!(deserialized.session_token, response.session_token);
    }

    #[test]
    fn test_auth_error_display() {
        let errors = vec![
            AuthError::InvalidCredentials,
            AuthError::SessionExpired,
            AuthError::KeychainUnavailable("test error".to_string()),
            AuthError::UserNotFound,
            AuthError::UserAlreadyExists,
            AuthError::Other("custom error".to_string()),
        ];

        for error in errors {
            let display = format!("{}", error);
            assert!(!display.is_empty());
        }
    }

    #[test]
    fn test_auth_error_serialization() {
        let error = AuthError::InvalidCredentials;
        let json = serde_json::to_string(&error).unwrap();
        let deserialized: AuthError = serde_json::from_str(&json).unwrap();

        match deserialized {
            AuthError::InvalidCredentials => {}
            _ => panic!("Expected InvalidCredentials variant"),
        }
    }
}
