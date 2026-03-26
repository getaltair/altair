use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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

/// Request payload for updating an existing shopping list
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateShoppingListRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: Option<String>,
    pub status: Option<String>,
}

/// Request payload for adding an item to a shopping list
#[derive(Debug, Deserialize, Validate)]
pub struct CreateShoppingListItemRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub item_id: Option<Uuid>,
    pub quantity: Option<i32>,
    pub unit: Option<String>,
}

/// Request payload for updating a shopping list item
#[derive(Debug, Deserialize, Validate)]
pub struct UpdateShoppingListItemRequest {
    pub name: Option<String>,
    pub quantity: Option<i32>,
    pub unit: Option<String>,
}
