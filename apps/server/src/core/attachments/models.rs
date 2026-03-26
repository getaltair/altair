use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Attachment {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub storage_key: String,
    pub size_bytes: i64,
    pub processing_state: String,
    pub created_at: DateTime<Utc>,
}
