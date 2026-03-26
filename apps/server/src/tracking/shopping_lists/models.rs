use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::contracts::ShoppingListStatus;

/// Database-backed shopping list record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct TrackingShoppingList {
    pub id: Uuid,
    pub user_id: Uuid,
    pub household_id: Uuid,
    pub name: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database-backed shopping list item record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct TrackingShoppingListItem {
    pub id: Uuid,
    pub shopping_list_id: Uuid,
    pub item_id: Option<Uuid>,
    pub name: String,
    pub quantity: i32,
    pub unit: Option<String>,
    pub is_checked: bool,
    pub created_at: DateTime<Utc>,
}

/// Request payload for creating a new shopping list
#[derive(Debug, Deserialize, Validate)]
pub struct CreateShoppingListRequest {
    pub household_id: Uuid,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
}

/// Request payload for updating an existing shopping list.
///
/// All fields are optional for true partial-update semantics: only fields
/// present in the JSON body are modified, absent fields are left unchanged.
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateShoppingListRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub status: Option<ShoppingListStatus>,
}

/// Request payload for adding an item to a shopping list
#[derive(Debug, Deserialize, Validate)]
pub struct CreateShoppingListItemRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub item_id: Option<Uuid>,
    #[validate(range(min = 1))]
    pub quantity: Option<i32>,
    pub unit: Option<String>,
}

/// Request payload for updating a shopping list item.
///
/// All fields are optional for true partial-update semantics. The `unit`
/// field uses `Option<Option<String>>` to distinguish three JSON states:
/// field absent (don't touch), field is `null` (set to NULL), field has a
/// value (set to that value).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateShoppingListItemRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub quantity: Option<i32>,
    #[serde(default, deserialize_with = "crate::serde_util::double_option")]
    pub unit: Option<Option<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- CreateShoppingListRequest -------------------------------------------

    #[test]
    fn create_list_request_valid_name_passes() {
        let req = CreateShoppingListRequest {
            household_id: Uuid::new_v4(),
            name: "Weekly groceries".to_string(),
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_list_request_empty_name_fails() {
        let req = CreateShoppingListRequest {
            household_id: Uuid::new_v4(),
            name: "".to_string(),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_list_request_name_over_200_chars_fails() {
        let req = CreateShoppingListRequest {
            household_id: Uuid::new_v4(),
            name: "a".repeat(201),
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_list_request_name_exactly_200_chars_passes() {
        let req = CreateShoppingListRequest {
            household_id: Uuid::new_v4(),
            name: "a".repeat(200),
        };
        assert!(req.validate().is_ok());
    }

    // -- CreateShoppingListItemRequest ---------------------------------------

    #[test]
    fn create_item_request_valid_name_passes() {
        let req = CreateShoppingListItemRequest {
            name: "Milk".to_string(),
            item_id: None,
            quantity: None,
            unit: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_item_request_empty_name_fails() {
        let req = CreateShoppingListItemRequest {
            name: "".to_string(),
            item_id: None,
            quantity: None,
            unit: None,
        };
        assert!(req.validate().is_err());
    }

    // -- UpdateShoppingListRequest -------------------------------------------

    #[test]
    fn update_list_request_all_none_passes() {
        let req = UpdateShoppingListRequest {
            name: None,
            status: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_list_request_valid_status_passes() {
        let json = r#"{"status": "completed"}"#;
        let req: UpdateShoppingListRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.status, Some(ShoppingListStatus::Completed));
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_list_request_invalid_status_rejected_by_serde() {
        let json = r#"{"status": "cancelled"}"#;
        let result = serde_json::from_str::<UpdateShoppingListRequest>(json);
        assert!(result.is_err());
    }

    // -- UpdateShoppingListItemRequest ---------------------------------------

    #[test]
    fn update_item_request_all_none_passes() {
        let req = UpdateShoppingListItemRequest {
            name: None,
            quantity: None,
            unit: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_item_request_empty_name_fails() {
        let req = UpdateShoppingListItemRequest {
            name: Some("".to_string()),
            quantity: None,
            unit: None,
        };
        assert!(req.validate().is_err());
    }

    // -- Double-option deserialization for unit -------------------------------

    #[test]
    fn update_item_request_unit_absent_is_none() {
        let json = r#"{"name": "Eggs"}"#;
        let req: UpdateShoppingListItemRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.unit, None);
    }

    #[test]
    fn update_item_request_unit_explicit_null_is_some_none() {
        let json = r#"{"name": "Eggs", "unit": null}"#;
        let req: UpdateShoppingListItemRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.unit, Some(None));
    }

    #[test]
    fn update_item_request_unit_with_value_is_some_some() {
        let json = r#"{"name": "Eggs", "unit": "dozen"}"#;
        let req: UpdateShoppingListItemRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.unit, Some(Some("dozen".to_string())));
    }
}
