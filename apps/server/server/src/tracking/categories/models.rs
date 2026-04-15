use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Raw DB row for `tracking_categories` — maps 1:1 to all table columns.
#[derive(Debug, sqlx::FromRow)]
pub struct TrackingCategoryRow {
    pub id: Uuid,
    pub name: String,
    pub household_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Public API response type — excludes `household_id` and `deleted_at`.
#[derive(Debug, Serialize)]
pub struct TrackingCategory {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<TrackingCategoryRow> for TrackingCategory {
    fn from(row: TrackingCategoryRow) -> Self {
        TrackingCategory {
            id: row.id,
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCategoryRequest {
    pub name: String,
    pub household_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
}
