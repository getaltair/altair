use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Supported event types for a tracking item.
///
/// Stored as VARCHAR(20) in Postgres; serialized/deserialized as snake_case strings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum ItemEventType {
    Restock,
    Consume,
    Purchase,
    PurchaseReversed,
    Adjustment,
    Move,
    Expire,
    Loss,
}

/// Raw DB row for `tracking_item_events` — maps 1:1 to all table columns.
///
/// `quantity_change` is cast to DOUBLE PRECISION in queries so sqlx maps it to `f64`.
#[derive(Debug, sqlx::FromRow)]
pub struct TrackingItemEventRow {
    pub id: Uuid,
    pub item_id: Uuid,
    pub event_type: String,
    pub quantity_change: f64,
    pub from_location_id: Option<Uuid>,
    pub to_location_id: Option<Uuid>,
    pub notes: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Public API response type for an item event.
///
/// Item events have no `household_id` column (derived from the parent item) and
/// no `deleted_at` (events are append-only per invariant D-5).
#[derive(Debug, Serialize)]
pub struct TrackingItemEvent {
    pub id: Uuid,
    pub item_id: Uuid,
    pub event_type: String,
    pub quantity_change: f64,
    pub from_location_id: Option<Uuid>,
    pub to_location_id: Option<Uuid>,
    pub notes: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<TrackingItemEventRow> for TrackingItemEvent {
    fn from(row: TrackingItemEventRow) -> Self {
        TrackingItemEvent {
            id: row.id,
            item_id: row.item_id,
            event_type: row.event_type,
            quantity_change: row.quantity_change,
            from_location_id: row.from_location_id,
            to_location_id: row.to_location_id,
            notes: row.notes,
            occurred_at: row.occurred_at,
            created_at: row.created_at,
        }
    }
}

/// Request body for creating an item event.
///
/// `id` is optional — the client may supply a UUID (invariant E-2); if absent,
/// the server generates one via `Uuid::new_v4()`.
///
/// `quantity_delta` is the signed quantity change: positive = stock increase,
/// negative = stock decrease. The server validates that applying this delta
/// will not reduce the derived quantity below zero (invariant E-7).
///
/// `occurred_at` defaults to NOW() if None.
///
/// `from_location_id` and `to_location_id` are required for `Move` events;
/// at least one must be non-None. They are ignored for other event types.
#[derive(Debug, Deserialize)]
pub struct CreateItemEventRequest {
    pub id: Option<Uuid>,
    pub item_id: Uuid,
    pub event_type: ItemEventType,
    pub quantity_delta: f64,
    pub occurred_at: Option<DateTime<Utc>>,
    pub from_location_id: Option<Uuid>,
    pub to_location_id: Option<Uuid>,
    pub notes: Option<String>,
}
