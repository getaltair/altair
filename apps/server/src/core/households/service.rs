use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use super::models::{Household, HouseholdMembership};

/// Create a new household and add the creator as its owner within a transaction
pub async fn create_household(
    pool: &PgPool,
    name: &str,
    created_by: Uuid,
) -> Result<Household, AppError> {
    let mut tx = pool.begin().await.map_err(AppError::Database)?;

    let household = sqlx::query_as::<_, Household>(
        "INSERT INTO households (name, created_by) VALUES ($1, $2) RETURNING id, name, created_by, created_at",
    )
    .bind(name)
    .bind(created_by)
    .fetch_one(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    // Assign the creator as household owner
    sqlx::query(
        "INSERT INTO household_memberships (household_id, user_id, role) VALUES ($1, $2, $3)",
    )
    .bind(household.id)
    .bind(created_by)
    .bind("owner")
    .execute(&mut *tx)
    .await
    .map_err(AppError::Database)?;

    tx.commit().await.map_err(AppError::Database)?;

    Ok(household)
}

/// List all households a user belongs to
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
    .map_err(AppError::Database)
}

/// Add a user as a member of a household with the given role
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
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("User is already a member of this household".to_string())
        }
        _ => AppError::Database(e),
    })
}

/// Invite a user to a household by their email address
pub async fn invite_member_by_email(
    pool: &PgPool,
    household_id: Uuid,
    email: &str,
    role: &str,
) -> Result<HouseholdMembership, AppError> {
    let user: (Uuid,) = sqlx::query_as("SELECT id FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Could not invite user. Verify the email address is correct.".to_string()))?;

    add_member(pool, household_id, user.0, role).await
}

/// Check whether a user is a member of a household
#[allow(dead_code)]
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
    .map_err(AppError::Database)?;

    Ok(row.is_some())
}

/// Get a user's role within a household, or None if they are not a member
pub async fn get_member_role(
    pool: &PgPool,
    household_id: Uuid,
    user_id: Uuid,
) -> Result<Option<String>, AppError> {
    let row: Option<(String,)> = sqlx::query_as(
        "SELECT role FROM household_memberships WHERE household_id = $1 AND user_id = $2",
    )
    .bind(household_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    Ok(row.map(|(role,)| role))
}
