use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::contracts::{InitiativeStatus, Priority};

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GuidanceEpic {
    pub id: Uuid,
    pub initiative_id: Option<Uuid>,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEpicRequest {
    pub initiative_id: Option<Uuid>,
    #[validate(length(min = 1, max = 500))]
    pub name: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
}

/// Request payload for updating an existing epic.
///
/// Nullable fields (`description`, `initiative_id`) use `Option<Option<T>>`
/// to distinguish three JSON states: field absent (don't touch), field is
/// `null` (set to NULL), field has a value (set to that value).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEpicRequest {
    #[validate(length(min = 1, max = 500))]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub description: Option<Option<String>>,
    pub status: Option<InitiativeStatus>,
    pub priority: Option<Priority>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub initiative_id: Option<Option<Uuid>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    // -- CreateEpicRequest validation -------------------------------------------

    #[test]
    fn create_request_valid_name_passes() {
        let req = CreateEpicRequest {
            initiative_id: None,
            name: "Valid Epic".to_string(),
            description: None,
            priority: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_empty_name_fails() {
        let req = CreateEpicRequest {
            initiative_id: None,
            name: "".to_string(),
            description: None,
            priority: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_at_500_chars_passes() {
        let req = CreateEpicRequest {
            initiative_id: None,
            name: "a".repeat(500),
            description: None,
            priority: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_name_over_500_chars_fails() {
        let req = CreateEpicRequest {
            initiative_id: None,
            name: "a".repeat(501),
            description: None,
            priority: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_valid_priority_passes() {
        let req = CreateEpicRequest {
            initiative_id: None,
            name: "Epic with priority".to_string(),
            description: None,
            priority: Some(Priority::High),
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.priority.unwrap().as_str(), "high");
    }

    // -- UpdateEpicRequest validation -------------------------------------------

    #[test]
    fn update_request_valid_name_passes() {
        let req = UpdateEpicRequest {
            name: Some("Updated epic".to_string()),
            description: None,
            status: None,
            priority: None,
            initiative_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_empty_name_fails() {
        let req = UpdateEpicRequest {
            name: Some("".to_string()),
            description: None,
            status: None,
            priority: None,
            initiative_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_name_over_500_chars_fails() {
        let req = UpdateEpicRequest {
            name: Some("a".repeat(501)),
            description: None,
            status: None,
            priority: None,
            initiative_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_all_none_passes() {
        let req = UpdateEpicRequest {
            name: None,
            description: None,
            status: None,
            priority: None,
            initiative_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_with_enum_status() {
        let req = UpdateEpicRequest {
            name: None,
            description: None,
            status: Some(InitiativeStatus::Active),
            priority: None,
            initiative_id: None,
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.status.unwrap().as_str(), "active");
    }

    #[test]
    fn update_request_with_enum_priority() {
        let req = UpdateEpicRequest {
            name: None,
            description: None,
            status: None,
            priority: Some(Priority::Critical),
            initiative_id: None,
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.priority.unwrap().as_str(), "critical");
    }

    // -- Double-option deserialization for description --------------------------

    #[test]
    fn update_request_description_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateEpicRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, None);
    }

    #[test]
    fn update_request_description_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "description": null}"#;
        let req: UpdateEpicRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(None));
    }

    #[test]
    fn update_request_description_with_value_is_some_some() {
        let json = r#"{"name": "test", "description": "new desc"}"#;
        let req: UpdateEpicRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(Some("new desc".to_string())));
    }

    #[test]
    fn update_request_description_empty_string_is_some_some_empty() {
        let json = r#"{"description": ""}"#;
        let req: UpdateEpicRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(Some(String::new())));
    }

    // -- Double-option deserialization for initiative_id -------------------------

    #[test]
    fn update_request_initiative_id_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateEpicRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.initiative_id, None);
    }

    #[test]
    fn update_request_initiative_id_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "initiative_id": null}"#;
        let req: UpdateEpicRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.initiative_id, Some(None));
    }

    #[test]
    fn update_request_initiative_id_with_value_is_some_some() {
        let id = Uuid::new_v4();
        let json = format!(r#"{{"name": "test", "initiative_id": "{}"}}"#, id);
        let req: UpdateEpicRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req.initiative_id, Some(Some(id)));
    }
}
