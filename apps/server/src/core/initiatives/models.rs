use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::contracts::InitiativeStatus;

/// Database-backed initiative record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Initiative {
    pub id: Uuid,
    pub user_id: Uuid,
    pub household_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request payload for creating a new initiative
#[derive(Debug, Deserialize, Validate)]
pub struct CreateInitiativeRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
    pub household_id: Option<Uuid>,
    pub status: Option<InitiativeStatus>,
}

/// Request payload for updating an existing initiative.
///
/// The `description` field uses `Option<Option<String>>` to distinguish three
/// JSON states: field absent (don't touch), field is `null` (set to NULL),
/// field has a value (set to that value).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateInitiativeRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub description: Option<Option<String>>,
    pub status: Option<InitiativeStatus>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_request_valid_name_passes() {
        let req = CreateInitiativeRequest {
            name: "My initiative".to_string(),
            description: None,
            household_id: None,
            status: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_empty_name_fails() {
        let req = CreateInitiativeRequest {
            name: "".to_string(),
            description: None,
            household_id: None,
            status: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_over_200_chars_fails() {
        let req = CreateInitiativeRequest {
            name: "a".repeat(201),
            description: None,
            household_id: None,
            status: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_exactly_200_chars_passes() {
        let req = CreateInitiativeRequest {
            name: "a".repeat(200),
            description: None,
            household_id: None,
            status: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_with_enum_status() {
        let req = CreateInitiativeRequest {
            name: "Test".to_string(),
            description: None,
            household_id: None,
            status: Some(InitiativeStatus::Active),
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.status.unwrap().as_str(), "active");
    }

    #[test]
    fn update_request_valid_name_passes() {
        let req = UpdateInitiativeRequest {
            name: Some("Updated name".to_string()),
            description: None,
            status: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_empty_name_fails() {
        let req = UpdateInitiativeRequest {
            name: Some("".to_string()),
            description: None,
            status: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_name_over_200_chars_fails() {
        let req = UpdateInitiativeRequest {
            name: Some("a".repeat(201)),
            description: None,
            status: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_all_none_passes() {
        let req = UpdateInitiativeRequest {
            name: None,
            description: None,
            status: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_with_enum_status() {
        let req = UpdateInitiativeRequest {
            name: None,
            description: None,
            status: Some(InitiativeStatus::Paused),
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.status.unwrap().as_str(), "paused");
    }

    // -- Double-option deserialization for description -------------------------

    #[test]
    fn update_request_description_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateInitiativeRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, None); // field absent -> None (don't touch)
    }

    #[test]
    fn update_request_description_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "description": null}"#;
        let req: UpdateInitiativeRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(None)); // field null -> Some(None) (set to NULL)
    }

    #[test]
    fn update_request_description_with_value_is_some_some() {
        let json = r#"{"name": "test", "description": "hello"}"#;
        let req: UpdateInitiativeRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(Some("hello".to_string()))); // value -> Some(Some(val))
    }

    #[test]
    fn update_request_description_empty_string_is_some_some_empty() {
        let json = r#"{"description": ""}"#;
        let req: UpdateInitiativeRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(Some(String::new())));
    }
}
