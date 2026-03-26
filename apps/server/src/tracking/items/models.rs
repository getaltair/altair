use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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

/// Request payload for updating an existing tracking item
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateItemRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub quantity: Option<i32>,
    pub unit: Option<String>,
    pub min_quantity: Option<i32>,
    pub barcode: Option<String>,
    pub status: Option<String>,
}

/// Request payload for creating a new item event
#[derive(Debug, Deserialize, Validate)]
pub struct CreateItemEventRequest {
    pub event_type: String,
    pub quantity_change: i32,
    pub notes: Option<String>,
}
