use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Raw database row from `tracking_locations` — all columns.
#[derive(Debug, sqlx::FromRow)]
pub struct TrackingLocationRow {
    pub id: Uuid,
    pub household_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Public response type — excludes `household_id` and `deleted_at`.
#[derive(Debug, Serialize)]
pub struct TrackingLocation {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TrackingLocationRow> for TrackingLocation {
    fn from(row: TrackingLocationRow) -> Self {
        TrackingLocation {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateLocationRequest {
    pub name: String,
    pub household_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLocationRequest {
    pub name: Option<String>,
}
