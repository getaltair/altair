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

/// Public API response for a note.
///
/// `deleted_at` is intentionally absent — every service query filters `WHERE deleted_at IS NULL`,
/// so the field would always serialize as `null`. Exposing it leaks the soft-delete
/// implementation to clients. The DB mapping layer (`NoteRow` in `service.rs`) carries it
/// internally.
#[derive(Debug, Serialize, Deserialize)]
pub struct NoteResponse {
    pub id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub initiative_id: Option<Uuid>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
///
/// Timestamp semantics:
/// - `captured_at`: client-reported capture time; may precede server ingestion for
///   offline-first clients.
/// - `created_at`: server insertion timestamp (when the row was physically written).
///
/// `content` is `String` (not `Option`) because the INSERT uses `COALESCE(n.content, '')`
/// to guarantee a non-NULL value even when the source note has no content.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SnapshotResponse {
    pub id: Uuid,
    pub note_id: Uuid,
    pub content: String,
    pub captured_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
