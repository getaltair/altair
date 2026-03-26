use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Database-backed tag record
#[derive(Debug, Serialize, sqlx::FromRow)]
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

/// Request payload for updating an existing tag
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTagRequest {
    #[validate(length(min = 1, max = 50))]
    pub name: Option<String>,
    pub color: Option<String>,
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
}
