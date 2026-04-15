use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a shopping list item.
///
/// Stored as VARCHAR(20) in Postgres. Decoded from String in Row; serialized
/// as snake_case strings in API responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShoppingListItemStatus {
    Pending,
    Purchased,
    Removed,
}

impl ShoppingListItemStatus {
    /// Returns true when transitioning from `self` to `next` is permitted.
    ///
    /// Valid transitions:
    /// - Pending → Purchased  (check off an item)
    /// - Pending → Removed    (remove without purchasing)
    /// - Purchased → Pending  (un-check / reverse a purchase)
    ///
    /// `Removed` is a terminal state — no transitions out are allowed.
    pub fn can_transition_to(&self, next: &Self) -> bool {
        matches!(
            (self, next),
            (
                ShoppingListItemStatus::Pending,
                ShoppingListItemStatus::Purchased
            ) | (
                ShoppingListItemStatus::Pending,
                ShoppingListItemStatus::Removed
            ) | (
                ShoppingListItemStatus::Purchased,
                ShoppingListItemStatus::Pending
            )
        )
    }
}

/// Raw database row for `tracking_shopping_list_items`. Maps 1:1 to all table columns.
///
/// `quantity` is cast to INTEGER in queries so sqlx maps it to `i32` (the column
/// is NUMERIC in Postgres). `status` is stored as a String; parse via
/// `str_to_status` when the enum is needed.
#[derive(Debug, sqlx::FromRow)]
pub struct TrackingShoppingListItemRow {
    pub id: Uuid,
    pub shopping_list_id: Uuid,
    pub item_id: Option<Uuid>,
    pub name: String,
    pub quantity: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Public API response type for a shopping list item. Excludes `deleted_at`.
#[derive(Debug, Serialize)]
pub struct TrackingShoppingListItem {
    pub id: Uuid,
    pub shopping_list_id: Uuid,
    pub item_id: Option<Uuid>,
    pub name: String,
    pub quantity: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TrackingShoppingListItemRow> for TrackingShoppingListItem {
    fn from(row: TrackingShoppingListItemRow) -> Self {
        TrackingShoppingListItem {
            id: row.id,
            shopping_list_id: row.shopping_list_id,
            item_id: row.item_id,
            name: row.name,
            quantity: row.quantity,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Request body for adding an item to a shopping list.
#[derive(Debug, Deserialize)]
pub struct CreateShoppingListItemRequest {
    pub name: String,
    pub quantity: Option<i32>,
    /// Optional link to an inventory item. Must belong to the same household (invariant E-9).
    pub item_id: Option<Uuid>,
}

/// Request body for updating a shopping list item's status.
#[derive(Debug, Deserialize)]
pub struct UpdateShoppingListItemRequest {
    pub status: ShoppingListItemStatus,
}

/// Query parameter for handlers that scope by household.
#[derive(Debug, Deserialize)]
pub struct HouseholdQuery {
    pub household_id: Uuid,
}
