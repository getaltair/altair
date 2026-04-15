use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct DailyCheckin {
    pub id: Uuid,
    pub user_id: Uuid,
    pub checkin_date: NaiveDate,
    pub energy_level: i32,
    pub mood: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCheckinRequest {
    pub checkin_date: NaiveDate,
    pub energy_level: i32,
    pub mood: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCheckinRequest {
    pub checkin_date: Option<NaiveDate>,
    pub energy_level: Option<i32>,
    pub mood: Option<String>,
    pub notes: Option<String>,
}
