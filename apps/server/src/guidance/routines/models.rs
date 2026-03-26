use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::contracts::{RoutineFrequency, RoutineStatus};

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GuidanceRoutine {
    pub id: Uuid,
    pub user_id: Uuid,
    pub household_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub frequency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoutineRequest {
    pub household_id: Option<Uuid>,
    #[validate(length(min = 1, max = 500))]
    pub name: String,
    pub description: Option<String>,
    pub frequency: RoutineFrequency,
}

/// Request payload for updating an existing routine.
///
/// The `description` field uses `Option<Option<String>>` to distinguish three
/// JSON states: field absent (don't touch), field is `null` (set to NULL),
/// field has a value (set to that value).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRoutineRequest {
    #[validate(length(min = 1, max = 500))]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub description: Option<Option<String>>,
    pub frequency: Option<RoutineFrequency>,
    pub status: Option<RoutineStatus>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    // -- CreateRoutineRequest validation ----------------------------------------

    #[test]
    fn create_request_valid_name_passes() {
        let req = CreateRoutineRequest {
            household_id: None,
            name: "Morning workout".to_string(),
            description: None,
            frequency: RoutineFrequency::Daily,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_empty_name_fails() {
        let req = CreateRoutineRequest {
            household_id: None,
            name: "".to_string(),
            description: None,
            frequency: RoutineFrequency::Daily,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_at_500_chars_passes() {
        let req = CreateRoutineRequest {
            household_id: None,
            name: "a".repeat(500),
            description: None,
            frequency: RoutineFrequency::Weekly,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_name_over_500_chars_fails() {
        let req = CreateRoutineRequest {
            household_id: None,
            name: "a".repeat(501),
            description: None,
            frequency: RoutineFrequency::Weekly,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_valid_frequency_daily() {
        let req = CreateRoutineRequest {
            household_id: None,
            name: "Daily routine".to_string(),
            description: None,
            frequency: RoutineFrequency::Daily,
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.frequency.as_str(), "daily");
    }

    #[test]
    fn create_request_valid_frequency_monthly() {
        let req = CreateRoutineRequest {
            household_id: None,
            name: "Monthly routine".to_string(),
            description: None,
            frequency: RoutineFrequency::Monthly,
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.frequency.as_str(), "monthly");
    }

    // -- UpdateRoutineRequest validation ----------------------------------------

    #[test]
    fn update_request_valid_name_passes() {
        let req = UpdateRoutineRequest {
            name: Some("Updated routine".to_string()),
            description: None,
            frequency: None,
            status: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_empty_name_fails() {
        let req = UpdateRoutineRequest {
            name: Some("".to_string()),
            description: None,
            frequency: None,
            status: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_name_over_500_chars_fails() {
        let req = UpdateRoutineRequest {
            name: Some("a".repeat(501)),
            description: None,
            frequency: None,
            status: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_all_none_passes() {
        let req = UpdateRoutineRequest {
            name: None,
            description: None,
            frequency: None,
            status: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_with_enum_frequency() {
        let req = UpdateRoutineRequest {
            name: None,
            description: None,
            frequency: Some(RoutineFrequency::Biweekly),
            status: None,
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.frequency.unwrap().as_str(), "biweekly");
    }

    #[test]
    fn update_request_with_enum_status() {
        let req = UpdateRoutineRequest {
            name: None,
            description: None,
            frequency: None,
            status: Some(RoutineStatus::Paused),
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.status.unwrap().as_str(), "paused");
    }

    // -- Double-option deserialization for description --------------------------

    #[test]
    fn update_request_description_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateRoutineRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, None);
    }

    #[test]
    fn update_request_description_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "description": null}"#;
        let req: UpdateRoutineRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(None));
    }

    #[test]
    fn update_request_description_with_value_is_some_some() {
        let json = r#"{"name": "test", "description": "new desc"}"#;
        let req: UpdateRoutineRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(Some("new desc".to_string())));
    }

    #[test]
    fn update_request_description_empty_string_is_some_some_empty() {
        let json = r#"{"description": ""}"#;
        let req: UpdateRoutineRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(Some(String::new())));
    }
}
