use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::contracts::Priority;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GuidanceQuest {
    pub id: Uuid,
    pub epic_id: Option<Uuid>,
    pub initiative_id: Option<Uuid>,
    pub user_id: Uuid,
    pub household_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<NaiveDate>,
    pub estimated_minutes: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuestRequest {
    pub epic_id: Option<Uuid>,
    pub initiative_id: Option<Uuid>,
    pub household_id: Option<Uuid>,
    #[validate(length(min = 1, max = 500))]
    pub name: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub due_date: Option<NaiveDate>,
    pub estimated_minutes: Option<i32>,
}

/// Request payload for updating an existing quest.
///
/// Status changes are intentionally excluded; status transitions are handled
/// exclusively through the `complete_quest` endpoint.
///
/// Nullable fields (`description`, `due_date`, `estimated_minutes`, `epic_id`)
/// use `Option<Option<T>>` to distinguish three JSON states: field absent
/// (don't touch), field is `null` (set to NULL), field has a value (set to
/// that value).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateQuestRequest {
    #[validate(length(min = 1, max = 500))]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub description: Option<Option<String>>,
    pub priority: Option<Priority>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub due_date: Option<Option<NaiveDate>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub estimated_minutes: Option<Option<i32>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub epic_id: Option<Option<Uuid>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    // -- CreateQuestRequest validation ------------------------------------------

    #[test]
    fn create_request_valid_name_passes() {
        let req = CreateQuestRequest {
            epic_id: None,
            initiative_id: None,
            household_id: None,
            name: "Valid Quest".to_string(),
            description: None,
            priority: None,
            due_date: None,
            estimated_minutes: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_empty_name_fails() {
        let req = CreateQuestRequest {
            epic_id: None,
            initiative_id: None,
            household_id: None,
            name: "".to_string(),
            description: None,
            priority: None,
            due_date: None,
            estimated_minutes: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_at_500_chars_passes() {
        let req = CreateQuestRequest {
            epic_id: None,
            initiative_id: None,
            household_id: None,
            name: "a".repeat(500),
            description: None,
            priority: None,
            due_date: None,
            estimated_minutes: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_name_over_500_chars_fails() {
        let req = CreateQuestRequest {
            epic_id: None,
            initiative_id: None,
            household_id: None,
            name: "a".repeat(501),
            description: None,
            priority: None,
            due_date: None,
            estimated_minutes: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_valid_priority_passes() {
        let req = CreateQuestRequest {
            epic_id: None,
            initiative_id: None,
            household_id: None,
            name: "Quest with priority".to_string(),
            description: None,
            priority: Some(Priority::Medium),
            due_date: None,
            estimated_minutes: None,
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.priority.unwrap().as_str(), "medium");
    }

    // -- UpdateQuestRequest validation ------------------------------------------

    #[test]
    fn update_request_valid_name_passes() {
        let req = UpdateQuestRequest {
            name: Some("Updated quest".to_string()),
            description: None,
            priority: None,
            due_date: None,
            estimated_minutes: None,
            epic_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_empty_name_fails() {
        let req = UpdateQuestRequest {
            name: Some("".to_string()),
            description: None,
            priority: None,
            due_date: None,
            estimated_minutes: None,
            epic_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_name_over_500_chars_fails() {
        let req = UpdateQuestRequest {
            name: Some("a".repeat(501)),
            description: None,
            priority: None,
            due_date: None,
            estimated_minutes: None,
            epic_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_all_none_passes() {
        let req = UpdateQuestRequest {
            name: None,
            description: None,
            priority: None,
            due_date: None,
            estimated_minutes: None,
            epic_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_with_enum_priority() {
        let req = UpdateQuestRequest {
            name: None,
            description: None,
            priority: Some(Priority::Low),
            due_date: None,
            estimated_minutes: None,
            epic_id: None,
        };
        assert!(req.validate().is_ok());
        assert_eq!(req.priority.unwrap().as_str(), "low");
    }

    // -- Double-option deserialization for description --------------------------

    #[test]
    fn update_request_description_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, None);
    }

    #[test]
    fn update_request_description_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "description": null}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(None));
    }

    #[test]
    fn update_request_description_with_value_is_some_some() {
        let json = r#"{"name": "test", "description": "new desc"}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(Some("new desc".to_string())));
    }

    #[test]
    fn update_request_description_empty_string_is_some_some_empty() {
        let json = r#"{"description": ""}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(Some(String::new())));
    }

    // -- Double-option deserialization for due_date ------------------------------

    #[test]
    fn update_request_due_date_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.due_date, None);
    }

    #[test]
    fn update_request_due_date_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "due_date": null}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.due_date, Some(None));
    }

    #[test]
    fn update_request_due_date_with_value_is_some_some() {
        let json = r#"{"name": "test", "due_date": "2025-06-15"}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        let expected = NaiveDate::from_ymd_opt(2025, 6, 15).unwrap();
        assert_eq!(req.due_date, Some(Some(expected)));
    }

    // -- Double-option deserialization for estimated_minutes ---------------------

    #[test]
    fn update_request_estimated_minutes_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.estimated_minutes, None);
    }

    #[test]
    fn update_request_estimated_minutes_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "estimated_minutes": null}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.estimated_minutes, Some(None));
    }

    #[test]
    fn update_request_estimated_minutes_with_value_is_some_some() {
        let json = r#"{"name": "test", "estimated_minutes": 60}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.estimated_minutes, Some(Some(60)));
    }

    // -- Double-option deserialization for epic_id ------------------------------

    #[test]
    fn update_request_epic_id_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.epic_id, None);
    }

    #[test]
    fn update_request_epic_id_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "epic_id": null}"#;
        let req: UpdateQuestRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.epic_id, Some(None));
    }

    #[test]
    fn update_request_epic_id_with_value_is_some_some() {
        let id = Uuid::new_v4();
        let json = format!(r#"{{"name": "test", "epic_id": "{}"}}"#, id);
        let req: UpdateQuestRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req.epic_id, Some(Some(id)));
    }
}
