use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GuidanceDailyCheckin {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub energy_level: Option<i32>,
    pub mood: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrUpdateCheckinRequest {
    pub date: Option<NaiveDate>,
    #[validate(range(min = 1, max = 5))]
    pub energy_level: Option<i32>,
    #[validate(length(max = 500))]
    pub mood: Option<String>,
    #[validate(length(max = 2000))]
    pub notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    // -- CreateOrUpdateCheckinRequest validation --------------------------------

    #[test]
    fn checkin_request_valid_passes() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: Some(3),
            mood: Some("happy".to_string()),
            notes: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn checkin_request_all_none_passes() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: None,
            mood: None,
            notes: None,
        };
        assert!(req.validate().is_ok());
    }

    // -- energy_level range validation ------------------------------------------

    #[test]
    fn energy_level_one_passes() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: Some(1),
            mood: None,
            notes: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn energy_level_five_passes() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: Some(5),
            mood: None,
            notes: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn energy_level_zero_fails() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: Some(0),
            mood: None,
            notes: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn energy_level_six_fails() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: Some(6),
            mood: None,
            notes: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn energy_level_negative_fails() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: Some(-1),
            mood: None,
            notes: None,
        };
        assert!(req.validate().is_err());
    }

    // -- mood length validation -------------------------------------------------

    #[test]
    fn mood_at_500_chars_passes() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: None,
            mood: Some("a".repeat(500)),
            notes: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn mood_over_500_chars_fails() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: None,
            mood: Some("a".repeat(501)),
            notes: None,
        };
        assert!(req.validate().is_err());
    }

    // -- notes length validation ------------------------------------------------

    #[test]
    fn notes_at_2000_chars_passes() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: None,
            mood: None,
            notes: Some("a".repeat(2000)),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn notes_over_2000_chars_fails() {
        let req = CreateOrUpdateCheckinRequest {
            date: None,
            energy_level: None,
            mood: None,
            notes: Some("a".repeat(2001)),
        };
        assert!(req.validate().is_err());
    }
}
