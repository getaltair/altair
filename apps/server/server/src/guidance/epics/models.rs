use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Status of a guidance epic, stored as a varchar in the database.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum EpicStatus {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Epic {
    pub id: Uuid,
    pub user_id: Uuid,
    pub initiative_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: EpicStatus,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEpicRequest {
    pub initiative_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: Option<EpicStatus>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEpicRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<EpicStatus>,
    pub sort_order: Option<i32>,
}
