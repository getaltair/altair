use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateNoteRequest {
    pub id: Option<Uuid>,
    pub title: String,
    pub content: Option<String>,
    pub initiative_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNoteRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct NoteResponse {
    pub id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub initiative_id: Option<Uuid>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NoteListQuery {
    #[serde(default)]
    pub initiative_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSnapshotRequest {
    pub captured_at: DateTime<Utc>,
}

/// Snapshots are immutable — no `updated_at` field (invariant E-6).
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SnapshotResponse {
    pub id: Uuid,
    pub note_id: Uuid,
    pub content: Option<String>,
    pub captured_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
