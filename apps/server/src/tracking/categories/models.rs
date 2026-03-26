use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Database-backed tracking category record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct TrackingCategory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub household_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_category_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request payload for creating a new tracking category
#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    pub household_id: Uuid,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
    pub parent_category_id: Option<Uuid>,
}

/// Request payload for updating an existing tracking category.
///
/// The `description` field uses `Option<Option<String>>` to distinguish three
/// JSON states: field absent (don't touch), field is `null` (set to NULL),
/// field has a value (set to that value).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "crate::serde_util::double_option")]
    pub description: Option<Option<String>>,
    #[serde(default, deserialize_with = "crate::serde_util::double_option")]
    pub parent_category_id: Option<Option<Uuid>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_request_valid_name_passes() {
        let req = CreateCategoryRequest {
            household_id: Uuid::new_v4(),
            name: "Electronics".to_string(),
            description: None,
            parent_category_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_empty_name_fails() {
        let req = CreateCategoryRequest {
            household_id: Uuid::new_v4(),
            name: "".to_string(),
            description: None,
            parent_category_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_over_200_chars_fails() {
        let req = CreateCategoryRequest {
            household_id: Uuid::new_v4(),
            name: "a".repeat(201),
            description: None,
            parent_category_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_exactly_200_chars_passes() {
        let req = CreateCategoryRequest {
            household_id: Uuid::new_v4(),
            name: "a".repeat(200),
            description: None,
            parent_category_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_valid_name_passes() {
        let req = UpdateCategoryRequest {
            name: Some("Updated name".to_string()),
            description: None,
            parent_category_id: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_empty_name_fails() {
        let req = UpdateCategoryRequest {
            name: Some("".to_string()),
            description: None,
            parent_category_id: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_all_none_passes() {
        let req = UpdateCategoryRequest {
            name: None,
            description: None,
            parent_category_id: None,
        };
        assert!(req.validate().is_ok());
    }

    // -- Double-option deserialization for description -------------------------

    #[test]
    fn update_request_description_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateCategoryRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, None);
    }

    #[test]
    fn update_request_description_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "description": null}"#;
        let req: UpdateCategoryRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(None));
    }

    #[test]
    fn update_request_description_with_value_is_some_some() {
        let json = r#"{"name": "test", "description": "hello"}"#;
        let req: UpdateCategoryRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(Some("hello".to_string())));
    }
}
