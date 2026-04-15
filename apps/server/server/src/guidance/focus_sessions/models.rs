use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FocusSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub quest_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Client-provided fields when starting a focus session.
/// `duration_minutes` and `ended_at` are intentionally absent — both are server-managed.
#[derive(Debug, Deserialize)]
pub struct CreateFocusSessionRequest {
    pub quest_id: Uuid,
    /// If omitted, the server defaults to NOW().
    pub started_at: Option<DateTime<Utc>>,
}

/// Client-provided fields when updating a focus session.
/// `duration_minutes` is intentionally absent — it is always server-computed from `ended_at`.
#[derive(Debug, Deserialize)]
pub struct UpdateFocusSessionRequest {
    pub ended_at: Option<DateTime<Utc>>,
}
