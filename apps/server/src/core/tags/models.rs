use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Database-backed tag record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Tag {
    pub id: Uuid,
    pub user_id: Uuid,
    pub household_id: Option<Uuid>,
    pub name: String,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Request payload for creating a new tag
#[derive(Debug, Deserialize, Validate)]
pub struct CreateTagRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: String,
    pub color: Option<String>,
    pub household_id: Option<Uuid>,
}

/// Request payload for updating an existing tag.
///
/// The `color` field uses `Option<Option<String>>` to distinguish three
/// JSON states: field absent (don't touch), field is `null` (set to NULL),
/// field has a value (set to that value).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTagRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::serde_util::double_option"
    )]
    pub color: Option<Option<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_request_valid_name_passes() {
        let req = CreateTagRequest {
            name: "Important".to_string(),
            color: None,
            household_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_empty_name_fails() {
        let req = CreateTagRequest {
            name: "".to_string(),
            color: None,
            household_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_over_50_chars_fails() {
        let req = CreateTagRequest {
            name: "a".repeat(51),
            color: None,
            household_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_exactly_50_chars_passes() {
        let req = CreateTagRequest {
            name: "a".repeat(50),
            color: None,
            household_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_with_color_passes() {
        let req = CreateTagRequest {
            name: "Urgent".to_string(),
            color: Some("#ff0000".to_string()),
            household_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_valid_name_passes() {
        let req = UpdateTagRequest {
            name: Some("Updated tag".to_string()),
            color: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_empty_name_fails() {
        let req = UpdateTagRequest {
            name: Some("".to_string()),
            color: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_name_over_50_chars_fails() {
        let req = UpdateTagRequest {
            name: Some("a".repeat(51)),
            color: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_all_none_passes() {
        let req = UpdateTagRequest {
            name: None,
            color: None,
        };
        assert!(req.validate().is_ok());
    }

    // -- Double-option deserialization for color --------------------------------

    #[test]
    fn update_request_color_absent_is_none() {
        let json = r#"{"name": "tag"}"#;
        let req: UpdateTagRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.color, None); // field absent -> None (don't touch)
    }

    #[test]
    fn update_request_color_explicit_null_is_some_none() {
        let json = r#"{"name": "tag", "color": null}"#;
        let req: UpdateTagRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.color, Some(None)); // field null -> Some(None) (set to NULL)
    }

    #[test]
    fn update_request_color_with_value_is_some_some() {
        let json = r##"{"name": "tag", "color": "#ff0000"}"##;
        let req: UpdateTagRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.color, Some(Some("#ff0000".to_string()))); // value -> Some(Some(val))
    }

    #[test]
    fn update_request_color_empty_string_is_some_some_empty() {
        let json = r#"{"color": ""}"#;
        let req: UpdateTagRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.color, Some(Some(String::new())));
    }
}
