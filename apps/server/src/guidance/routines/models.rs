use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GuidanceRoutine {
    pub id: Uuid,
    pub user_id: Uuid,
    pub household_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub frequency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRoutineRequest {
    pub household_id: Option<Uuid>,
    #[validate(length(min = 1, max = 500))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 1))]
    pub frequency: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRoutineRequest {
    #[validate(length(min = 1, max = 500))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub frequency: Option<String>,
    pub status: Option<String>,
}
