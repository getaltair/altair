use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct GuidanceDailyCheckin {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: NaiveDate,
    pub energy_level: Option<i32>,
    pub mood: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrUpdateCheckinRequest {
    pub date: Option<NaiveDate>,
    pub energy_level: Option<i32>,
    pub mood: Option<String>,
    pub notes: Option<String>,
}
