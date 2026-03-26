use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Role a user can hold within a household
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HouseholdRole {
    Owner,
    Member,
}

impl fmt::Display for HouseholdRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HouseholdRole::Owner => write!(f, "owner"),
            HouseholdRole::Member => write!(f, "member"),
        }
    }
}

impl HouseholdRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            HouseholdRole::Owner => "owner",
            HouseholdRole::Member => "member",
        }
    }
}

/// Database-backed household record
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Household {
    pub id: Uuid,
    pub name: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

/// Database-backed household membership record linking a user to a household
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct HouseholdMembership {
    pub id: Uuid,
    pub household_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub joined_at: DateTime<Utc>,
}

/// Request payload for creating a new household
#[derive(Debug, Deserialize, Validate)]
pub struct CreateHouseholdRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
}

/// Request payload for inviting a user to a household
#[derive(Debug, Deserialize, Validate)]
pub struct InviteMemberRequest {
    #[validate(email)]
    pub user_email: String,
    pub role: Option<HouseholdRole>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn household_role_serialization() {
        let owner_json = serde_json::to_string(&HouseholdRole::Owner)
            .expect("serialization should succeed");
        assert_eq!(owner_json, "\"owner\"");

        let member_json = serde_json::to_string(&HouseholdRole::Member)
            .expect("serialization should succeed");
        assert_eq!(member_json, "\"member\"");
    }

    #[test]
    fn household_role_as_str() {
        assert_eq!(HouseholdRole::Owner.as_str(), "owner");
        assert_eq!(HouseholdRole::Member.as_str(), "member");
    }
}
