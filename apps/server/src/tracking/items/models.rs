use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::contracts::{ItemEventType, TrackingItemStatus};

/// Database-backed tracking item record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct TrackingItem {
    pub id: Uuid,
    pub user_id: Uuid,
    pub household_id: Uuid,
    pub category_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub unit: Option<String>,
    pub min_quantity: Option<i32>,
    pub barcode: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Database-backed tracking item event record (append-only log)
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct TrackingItemEvent {
    pub id: Uuid,
    pub item_id: Uuid,
    pub user_id: Uuid,
    pub event_type: String,
    pub quantity_change: i32,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Request payload for creating a new tracking item
#[derive(Debug, Deserialize, Validate)]
pub struct CreateItemRequest {
    pub household_id: Uuid,
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub quantity: Option<i32>,
    pub unit: Option<String>,
    pub min_quantity: Option<i32>,
    pub barcode: Option<String>,
}

/// Request payload for updating an existing tracking item.
///
/// Nullable fields use `Option<Option<T>>` to distinguish three JSON states:
/// field absent (don't touch), field is `null` (set to NULL), field has a
/// value (set to that value).
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateItemRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "crate::serde_util::double_option")]
    pub description: Option<Option<String>>,
    #[serde(default, deserialize_with = "crate::serde_util::double_option")]
    pub category_id: Option<Option<Uuid>>,
    #[serde(default, deserialize_with = "crate::serde_util::double_option")]
    pub location_id: Option<Option<Uuid>>,
    pub quantity: Option<i32>,
    #[serde(default, deserialize_with = "crate::serde_util::double_option")]
    pub unit: Option<Option<String>>,
    #[serde(default, deserialize_with = "crate::serde_util::double_option")]
    pub min_quantity: Option<Option<i32>>,
    #[serde(default, deserialize_with = "crate::serde_util::double_option")]
    pub barcode: Option<Option<String>>,
    pub status: Option<TrackingItemStatus>,
}

/// Request payload for creating a new item event
#[derive(Debug, Deserialize, Validate)]
pub struct CreateItemEventRequest {
    pub event_type: ItemEventType,
    pub quantity_change: i32,
    pub notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- CreateItemRequest validation ----------------------------------------

    #[test]
    fn create_request_valid_name_passes() {
        let req = CreateItemRequest {
            household_id: Uuid::new_v4(),
            name: "Paper towels".to_string(),
            description: None,
            category_id: None,
            location_id: None,
            quantity: None,
            unit: None,
            min_quantity: None,
            barcode: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn create_request_empty_name_fails() {
        let req = CreateItemRequest {
            household_id: Uuid::new_v4(),
            name: "".to_string(),
            description: None,
            category_id: None,
            location_id: None,
            quantity: None,
            unit: None,
            min_quantity: None,
            barcode: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_over_200_chars_fails() {
        let req = CreateItemRequest {
            household_id: Uuid::new_v4(),
            name: "a".repeat(201),
            description: None,
            category_id: None,
            location_id: None,
            quantity: None,
            unit: None,
            min_quantity: None,
            barcode: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn create_request_name_exactly_200_chars_passes() {
        let req = CreateItemRequest {
            household_id: Uuid::new_v4(),
            name: "a".repeat(200),
            description: None,
            category_id: None,
            location_id: None,
            quantity: None,
            unit: None,
            min_quantity: None,
            barcode: None,
        };
        assert!(req.validate().is_ok());
    }

    // -- UpdateItemRequest validation ----------------------------------------

    #[test]
    fn update_request_valid_name_passes() {
        let req = UpdateItemRequest {
            name: Some("Updated name".to_string()),
            description: None,
            category_id: None,
            location_id: None,
            quantity: None,
            unit: None,
            min_quantity: None,
            barcode: None,
            status: None,
        };
        assert!(req.validate().is_ok());
    }

    #[test]
    fn update_request_empty_name_fails() {
        let req = UpdateItemRequest {
            name: Some("".to_string()),
            description: None,
            category_id: None,
            location_id: None,
            quantity: None,
            unit: None,
            min_quantity: None,
            barcode: None,
            status: None,
        };
        assert!(req.validate().is_err());
    }

    #[test]
    fn update_request_all_none_passes() {
        let req = UpdateItemRequest {
            name: None,
            description: None,
            category_id: None,
            location_id: None,
            quantity: None,
            unit: None,
            min_quantity: None,
            barcode: None,
            status: None,
        };
        assert!(req.validate().is_ok());
    }

    // -- Double-option deserialization for description -----------------------

    #[test]
    fn update_request_description_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateItemRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, None);
    }

    #[test]
    fn update_request_description_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "description": null}"#;
        let req: UpdateItemRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(None));
    }

    #[test]
    fn update_request_description_with_value_is_some_some() {
        let json = r#"{"name": "test", "description": "hello"}"#;
        let req: UpdateItemRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.description, Some(Some("hello".to_string())));
    }

    // -- Double-option deserialization for category_id -----------------------

    #[test]
    fn update_request_category_id_absent_is_none() {
        let json = r#"{"name": "test"}"#;
        let req: UpdateItemRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.category_id, None);
    }

    #[test]
    fn update_request_category_id_explicit_null_is_some_none() {
        let json = r#"{"name": "test", "category_id": null}"#;
        let req: UpdateItemRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.category_id, Some(None));
    }

    #[test]
    fn update_request_category_id_with_value_is_some_some() {
        let id = Uuid::new_v4();
        let json = serde_json::json!({"name": "test", "category_id": id});
        let req: UpdateItemRequest = serde_json::from_value(json).unwrap();
        assert_eq!(req.category_id, Some(Some(id)));
    }

    // -- CreateItemEventRequest serde ---------------------------------------

    #[test]
    fn create_event_request_valid_event_type_deserializes() {
        let json = serde_json::json!({
            "event_type": "consumed",
            "quantity_change": -1,
            "notes": null
        });
        let req: CreateItemEventRequest = serde_json::from_value(json).unwrap();
        assert_eq!(req.event_type, ItemEventType::Consumed);
        assert_eq!(req.quantity_change, -1);
    }

    #[test]
    fn create_event_request_invalid_event_type_fails() {
        let json = serde_json::json!({
            "event_type": "purchased",
            "quantity_change": 1
        });
        let result = serde_json::from_value::<CreateItemEventRequest>(json);
        assert!(result.is_err());
    }

    // -- UpdateItemRequest status serde -------------------------------------

    #[test]
    fn update_request_valid_status_deserializes() {
        let json = r#"{"status": "archived"}"#;
        let req: UpdateItemRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.status, Some(TrackingItemStatus::Archived));
    }

    #[test]
    fn update_request_invalid_status_fails() {
        let json = r#"{"status": "deleted"}"#;
        let result = serde_json::from_str::<UpdateItemRequest>(json);
        assert!(result.is_err());
    }
}
