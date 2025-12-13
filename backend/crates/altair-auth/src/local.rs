//! Local authentication provider (placeholder)
//!
//! TODO: Implement local authentication strategy

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
    /// Create a new session
    pub fn new(user_id: String, token: Option<String>) -> Self {
        let expires_at = token.as_ref().map(|_| {
            Utc::now() + chrono::Duration::hours(24) // 24-hour expiration
        });

        Self {
            user_id,
            token,
            expires_at,
        }
    }
}

/// Placeholder for local auth provider
pub struct LocalAuthProvider;
