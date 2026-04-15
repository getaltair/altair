use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Raw DB row for `tracking_items` — maps 1:1 to all table columns.
///
/// `quantity` is stored as NUMERIC in Postgres; queries cast it to DOUBLE PRECISION
/// so it maps to `f64` without requiring an external decimal crate.
#[derive(Debug, sqlx::FromRow)]
pub struct TrackingItemRow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub quantity: f64,
    pub barcode: Option<String>,
    pub location_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub user_id: Uuid,
    pub household_id: Option<Uuid>,
    pub initiative_id: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Public API response type — excludes `household_id` and `deleted_at`.
#[derive(Debug, Serialize)]
pub struct TrackingItem {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub quantity: f64,
    pub barcode: Option<String>,
    pub location_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub user_id: Uuid,
    pub initiative_id: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TrackingItemRow> for TrackingItem {
    fn from(row: TrackingItemRow) -> Self {
        TrackingItem {
            id: row.id,
            name: row.name,
            description: row.description,
            quantity: row.quantity,
            barcode: row.barcode,
            location_id: row.location_id,
            category_id: row.category_id,
            user_id: row.user_id,
            initiative_id: row.initiative_id,
            expires_at: row.expires_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

/// Request body for creating a tracking item.
///
/// `id` is optional — the client may supply a UUID (invariant E-2); if absent,
/// the server generates one via `Uuid::new_v4()`.
#[derive(Debug, Deserialize)]
pub struct CreateItemRequest {
    pub id: Option<Uuid>,
    pub name: String,
    pub household_id: Uuid,
    pub description: Option<String>,
    pub barcode: Option<String>,
    pub location_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Request body for updating a tracking item — all fields optional.
#[derive(Debug, Deserialize)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub barcode: Option<String>,
    pub location_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
}
