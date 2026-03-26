use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Household {
    pub id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct HouseholdMembership {
    pub id: Uuid,
    pub household_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateHouseholdRequest {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct InviteMemberRequest {
    pub user_email: String,
    pub role: Option<String>,
}
