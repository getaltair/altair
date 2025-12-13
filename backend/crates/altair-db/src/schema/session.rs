//! Session type - Authentication session management

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// Session - Represents an active authentication session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Option<Thing>,
    pub token: String, // 256-bit random hex string
    pub user: Thing,   // Reference to user record
    pub expires_at: DateTime<Utc>,
    pub device_id: String,
    pub created_at: DateTime<Utc>,
}

impl Session {
    /// Check if the session has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    /// Check if the session should be refreshed (within 1 day of expiration)
    pub fn should_refresh(&self) -> bool {
        let one_day_before_expiry = self.expires_at - chrono::Duration::days(1);
        Utc::now() > one_day_before_expiry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_is_expired() {
        let expired_session = Session {
            id: None,
            token: "test_token".to_string(),
            user: Thing::from(("user", "test")),
            expires_at: Utc::now() - Duration::hours(1),
            device_id: "test_device".to_string(),
            created_at: Utc::now() - Duration::days(1),
        };

        assert!(expired_session.is_expired());

        let active_session = Session {
            id: None,
            token: "test_token".to_string(),
            user: Thing::from(("user", "test")),
            expires_at: Utc::now() + Duration::hours(1),
            device_id: "test_device".to_string(),
            created_at: Utc::now(),
        };

        assert!(!active_session.is_expired());
    }

    #[test]
    fn test_should_refresh() {
        let should_refresh_session = Session {
            id: None,
            token: "test_token".to_string(),
            user: Thing::from(("user", "test")),
            expires_at: Utc::now() + Duration::hours(12), // Less than 1 day remaining
            device_id: "test_device".to_string(),
            created_at: Utc::now() - Duration::days(6),
        };

        assert!(should_refresh_session.should_refresh());

        let no_refresh_session = Session {
            id: None,
            token: "test_token".to_string(),
            user: Thing::from(("user", "test")),
            expires_at: Utc::now() + Duration::days(5), // More than 1 day remaining
            device_id: "test_device".to_string(),
            created_at: Utc::now(),
        };

        assert!(!no_refresh_session.should_refresh());
    }
}
