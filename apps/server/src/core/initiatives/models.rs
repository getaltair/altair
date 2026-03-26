use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Database-backed initiative record
#[derive(Debug, Serialize, sqlx::FromRow)]
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
    pub status: Option<String>,
}

/// Request payload for updating an existing initiative
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateInitiativeRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
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
}
