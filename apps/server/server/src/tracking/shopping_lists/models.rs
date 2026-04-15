use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Raw database row for `tracking_shopping_lists`. All columns included.
#[derive(Debug, sqlx::FromRow)]
pub struct TrackingShoppingListRow {
    pub id: Uuid,
    pub name: String,
    pub household_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Public response type. Excludes `household_id` and `deleted_at`.
#[derive(Debug, Serialize)]
pub struct TrackingShoppingList {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TrackingShoppingListRow> for TrackingShoppingList {
    fn from(row: TrackingShoppingListRow) -> Self {
        TrackingShoppingList {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateShoppingListRequest {
    pub name: String,
    pub household_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateShoppingListRequest {
    pub name: Option<String>,
}

/// Query parameter for handlers that scope by household.
#[derive(Debug, Deserialize)]
pub struct HouseholdQuery {
    pub household_id: Uuid,
}
