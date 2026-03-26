use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GuidanceFocusSession {
    pub id: Uuid,
    pub quest_id: Uuid,
    pub user_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFocusSessionRequest {
    pub quest_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    #[validate(range(min = 1))]
    pub duration_minutes: Option<i32>,
    #[validate(length(max = 2000))]
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateFocusSessionRequest {
    pub ended_at: Option<DateTime<Utc>>,
    #[validate(range(min = 1))]
    pub duration_minutes: Option<i32>,
    #[validate(length(max = 2000))]
    pub notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    // -- CreateFocusSessionRequest validation -----------------------------------

    #[test]
    fn create_request_valid_passes() {
        let req = CreateFocusSessionRequest {
            quest_id: Uuid::new_v4(),
            started_at: now(),
            ended_at: None,
            duration_minutes: None,
            notes: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_duration_minutes_one_passes() {
        let req = CreateFocusSessionRequest {
            quest_id: Uuid::new_v4(),
            started_at: now(),
            ended_at: None,
            duration_minutes: Some(1),
            notes: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_duration_minutes_zero_fails() {
        let req = CreateFocusSessionRequest {
            quest_id: Uuid::new_v4(),
            started_at: now(),
            ended_at: None,
            duration_minutes: Some(0),
            notes: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_duration_minutes_negative_fails() {
        let req = CreateFocusSessionRequest {
            quest_id: Uuid::new_v4(),
            started_at: now(),
            ended_at: None,
            duration_minutes: Some(-1),
            notes: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_notes_at_2000_chars_passes() {
        let req = CreateFocusSessionRequest {
            quest_id: Uuid::new_v4(),
            started_at: now(),
            ended_at: None,
            duration_minutes: None,
            notes: Some("a".repeat(2000)),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_notes_over_2000_chars_fails() {
        let req = CreateFocusSessionRequest {
            quest_id: Uuid::new_v4(),
            started_at: now(),
            ended_at: None,
            duration_minutes: None,
            notes: Some("a".repeat(2001)),
        };
        assert!(req.validate().is_err());
    }

    // -- UpdateFocusSessionRequest validation -----------------------------------

    #[test]
    fn update_request_valid_passes() {
        let req = UpdateFocusSessionRequest {
            ended_at: Some(now()),
            duration_minutes: Some(30),
            notes: Some("Good session".to_string()),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_all_none_passes() {
        let req = UpdateFocusSessionRequest {
            ended_at: None,
            duration_minutes: None,
            notes: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_duration_minutes_one_passes() {
        let req = UpdateFocusSessionRequest {
            ended_at: None,
            duration_minutes: Some(1),
            notes: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_duration_minutes_zero_fails() {
        let req = UpdateFocusSessionRequest {
            ended_at: None,
            duration_minutes: Some(0),
            notes: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_notes_over_2000_chars_fails() {
        let req = UpdateFocusSessionRequest {
            ended_at: None,
            duration_minutes: None,
            notes: Some("a".repeat(2001)),
        };
        assert!(req.validate().is_err());
    }
}
