use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GuidanceEpic {
    pub id: Uuid,
    pub initiative_id: Option<Uuid>,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEpicRequest {
    pub initiative_id: Option<Uuid>,
    #[validate(length(min = 1, max = 500))]
    pub name: String,
    pub description: Option<String>,
    pub priority: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEpicRequest {
    #[validate(length(min = 1, max = 500))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub initiative_id: Option<Uuid>,
}
