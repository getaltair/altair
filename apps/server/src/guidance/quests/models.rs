use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GuidanceQuest {
    pub id: Uuid,
    pub epic_id: Option<Uuid>,
    pub initiative_id: Option<Uuid>,
    pub user_id: Uuid,
    pub household_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<NaiveDate>,
    pub estimated_minutes: Option<i32>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuestRequest {
    pub epic_id: Option<Uuid>,
    pub initiative_id: Option<Uuid>,
    pub household_id: Option<Uuid>,
    #[validate(length(min = 1, max = 500))]
    pub name: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub estimated_minutes: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateQuestRequest {
    #[validate(length(min = 1, max = 500))]
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub estimated_minutes: Option<i32>,
    pub epic_id: Option<Uuid>,
}
