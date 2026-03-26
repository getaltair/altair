use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use super::models::{Household, HouseholdMembership};

pub async fn create_household(
    pool: &PgPool,
    name: &str,
    created_by: Uuid,
) -> Result<Household, AppError> {
    // INSERT into households
    let household = sqlx::query_as::<_, Household>(
        "INSERT INTO households (name, created_by) VALUES ($1, $2) RETURNING id, name, created_by, created_at",
    )
    .bind(name)
    .bind(created_by)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to create household: {e}")))?;

    // INSERT the creator as owner in household_memberships
    sqlx::query(
        "INSERT INTO household_memberships (household_id, user_id, role) VALUES ($1, $2, $3)",
    )
    .bind(household.id)
    .bind(created_by)
    .bind("owner")
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to create membership: {e}")))?;

    Ok(household)
}

pub async fn list_user_households(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Household>, AppError> {
    sqlx::query_as::<_, Household>(
        r#"SELECT h.id, h.name, h.created_by, h.created_at
           FROM households h
           INNER JOIN household_memberships hm ON hm.household_id = h.id
           WHERE hm.user_id = $1"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to list households: {e}")))
}

pub async fn add_member(
    pool: &PgPool,
    household_id: Uuid,
    user_id: Uuid,
    role: &str,
) -> Result<HouseholdMembership, AppError> {
    sqlx::query_as::<_, HouseholdMembership>(
        "INSERT INTO household_memberships (household_id, user_id, role) VALUES ($1, $2, $3) RETURNING id, household_id, user_id, role, joined_at",
    )
    .bind(household_id)
    .bind(user_id)
    .bind(role)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if e.to_string().contains("unique") || e.to_string().contains("duplicate") {
            AppError::Conflict("User is already a member of this household".to_string())
        } else {
            AppError::Internal(format!("Failed to add member: {e}"))
        }
    })
}

pub async fn invite_member_by_email(
    pool: &PgPool,
    household_id: Uuid,
    email: &str,
    role: &str,
) -> Result<HouseholdMembership, AppError> {
    // Look up user by email
    let user: (Uuid,) = sqlx::query_as("SELECT id FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to lookup user: {e}")))?
        .ok_or_else(|| AppError::NotFound(format!("No user found with email: {email}")))?;

    add_member(pool, household_id, user.0, role).await
}

pub async fn is_member(
    pool: &PgPool,
    household_id: Uuid,
    user_id: Uuid,
) -> Result<bool, AppError> {
    let row: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM household_memberships WHERE household_id = $1 AND user_id = $2",
    )
    .bind(household_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to check membership: {e}")))?;

    Ok(row.is_some())
}
