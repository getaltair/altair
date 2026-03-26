use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::contracts::{Priority, QuestStatus, RoutineStatus};
use crate::error::AppError;
use crate::guidance::quests::models::GuidanceQuest;
use super::models::*;

/// Create a new routine for the given user.
///
/// Status defaults to `Active` (via the database default).
pub async fn create_routine(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateRoutineRequest,
) -> Result<GuidanceRoutine, AppError> {
    let routine = sqlx::query_as::<_, GuidanceRoutine>(
        r#"INSERT INTO guidance_routines (user_id, household_id, name, description, frequency)
           VALUES ($1, $2, $3, $4, $5)
           RETURNING id, user_id, household_id, name, description, frequency, status, created_at, updated_at"#,
    )
    .bind(user_id)
    .bind(req.household_id)
    .bind(&req.name)
    .bind(&req.description)
    .bind(req.frequency.as_str())
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Routine already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
            AppError::BadRequest("Invalid field value".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::BadRequest("Referenced resource does not exist".to_string())
        }
        _ => AppError::Database(e),
    })?;

    tracing::info!(routine_id = %routine.id, user_id = %user_id, "Created guidance routine");
    Ok(routine)
}

/// List routines visible to the user (owned or belonging to their households)
pub async fn list_routines(
    pool: &PgPool,
    user_id: Uuid,
    household_ids: &[Uuid],
) -> Result<Vec<GuidanceRoutine>, AppError> {
    sqlx::query_as::<_, GuidanceRoutine>(
        r#"SELECT id, user_id, household_id, name, description, frequency, status, created_at, updated_at
           FROM guidance_routines
           WHERE user_id = $1 OR household_id = ANY($2)
           ORDER BY created_at DESC"#,
    )
    .bind(user_id)
    .bind(household_ids)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Get a single routine by ID, accessible if user owns it or is in its household
pub async fn get_routine(
    pool: &PgPool,
    routine_id: Uuid,
    user_id: Uuid,
    household_ids: &[Uuid],
) -> Result<GuidanceRoutine, AppError> {
    sqlx::query_as::<_, GuidanceRoutine>(
        r#"SELECT id, user_id, household_id, name, description, frequency, status, created_at, updated_at
           FROM guidance_routines
           WHERE id = $1 AND (user_id = $2 OR household_id = ANY($3))"#,
    )
    .bind(routine_id)
    .bind(user_id)
    .bind(household_ids)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Routine not found".to_string()))
}

/// Update a routine's fields. Only the routine owner can update it.
///
/// The `description` field uses double-option semantics: `None` = don't touch,
/// `Some(None)` = set to NULL, `Some(Some(val))` = set to value.
pub async fn update_routine(
    pool: &PgPool,
    routine_id: Uuid,
    user_id: Uuid,
    req: &UpdateRoutineRequest,
) -> Result<GuidanceRoutine, AppError> {
    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE guidance_routines SET updated_at = now()");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }

    match &req.description {
        Some(None) => {
            qb.push(", description = NULL");
        }
        Some(Some(val)) => {
            qb.push(", description = ");
            qb.push_bind(val.clone());
        }
        None => {}
    }

    if let Some(ref frequency) = req.frequency {
        qb.push(", frequency = ");
        qb.push_bind(frequency.as_str());
    }

    if let Some(ref status) = req.status {
        qb.push(", status = ");
        qb.push_bind(status.as_str());
    }

    qb.push(" WHERE id = ");
    qb.push_bind(routine_id);
    qb.push(" AND user_id = ");
    qb.push_bind(user_id);
    qb.push(" RETURNING id, user_id, household_id, name, description, frequency, status, created_at, updated_at");

    let routine = qb
        .build_query_as::<GuidanceRoutine>()
        .fetch_optional(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Routine already exists".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
                AppError::BadRequest("Invalid field value".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::BadRequest("Referenced resource does not exist".to_string())
            }
            _ => AppError::Database(e),
        })?
        .ok_or_else(|| AppError::NotFound("Routine not found".to_string()))?;

    tracing::info!(routine_id = %routine_id, user_id = %user_id, "Updated guidance routine");
    Ok(routine)
}

/// Delete a routine by ID. Only the routine owner can delete it.
pub async fn delete_routine(
    pool: &PgPool,
    routine_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM guidance_routines WHERE id = $1 AND user_id = $2")
        .bind(routine_id)
        .bind(user_id)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Resource already exists".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
                AppError::BadRequest("Invalid field value".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::BadRequest("Referenced resource does not exist".to_string())
            }
            _ => AppError::Database(e),
        })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Routine not found".to_string()));
    }

    tracing::info!(routine_id = %routine_id, user_id = %user_id, "Deleted guidance routine");
    Ok(())
}

/// Trigger a routine by creating a new quest from it.
///
/// The routine must be accessible to the user (owned or household member) and
/// must have an `active` status. Creates a new quest with the routine's name,
/// description, and household, setting the caller as the quest owner.
pub async fn trigger_routine(
    pool: &PgPool,
    routine_id: Uuid,
    user_id: Uuid,
    household_ids: &[Uuid],
) -> Result<GuidanceQuest, AppError> {
    let routine = sqlx::query_as::<_, GuidanceRoutine>(
        r#"SELECT id, user_id, household_id, name, description, frequency, status, created_at, updated_at
           FROM guidance_routines
           WHERE id = $1 AND (user_id = $2 OR household_id = ANY($3))"#,
    )
    .bind(routine_id)
    .bind(user_id)
    .bind(household_ids)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Routine not found".to_string()))?;

    if routine.status != RoutineStatus::Active.as_str() {
        return Err(AppError::BadRequest(format!(
            "Cannot trigger routine with status '{}'",
            routine.status
        )));
    }

    let quest = sqlx::query_as::<_, GuidanceQuest>(
        r#"INSERT INTO guidance_quests (user_id, household_id, name, description, status, priority)
           VALUES ($1, $2, $3, $4, $5, $6)
           RETURNING id, epic_id, initiative_id, user_id, household_id, name, description,
                     status, priority, due_date, estimated_minutes, completed_at, created_at, updated_at"#,
    )
    .bind(user_id)
    .bind(routine.household_id)
    .bind(&routine.name)
    .bind(&routine.description)
    .bind(QuestStatus::Pending.as_str())
    .bind(Priority::Medium.as_str())
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Resource already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
            AppError::BadRequest("Invalid field value".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::BadRequest("Referenced resource does not exist".to_string())
        }
        _ => AppError::Database(e),
    })?;

    tracing::info!(
        routine_id = %routine_id,
        quest_id = %quest.id,
        user_id = %user_id,
        "Triggered routine, created quest"
    );
    Ok(quest)
}

/// Add a tag to a routine. Verifies routine ownership before adding.
pub async fn add_routine_tag(
    pool: &PgPool,
    routine_id: Uuid,
    tag_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let exists: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM guidance_routines WHERE id = $1 AND user_id = $2",
    )
    .bind(routine_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    if exists.is_none() {
        return Err(AppError::NotFound("Routine not found".to_string()));
    }

    sqlx::query("INSERT INTO routine_tags (routine_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
        .bind(routine_id)
        .bind(tag_id)
        .execute(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Resource already exists".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
                AppError::BadRequest("Invalid field value".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::BadRequest("Referenced resource does not exist".to_string())
            }
            _ => AppError::Database(e),
        })?;

    tracing::info!(routine_id = %routine_id, tag_id = %tag_id, "Added tag to routine");
    Ok(())
}

/// Remove a tag from a routine.
///
/// Returns 404 if the routine does not belong to the user or the tag is not
/// associated with the routine.
pub async fn remove_routine_tag(
    pool: &PgPool,
    routine_id: Uuid,
    tag_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let exists: Option<(Uuid,)> = sqlx::query_as(
        "SELECT id FROM guidance_routines WHERE id = $1 AND user_id = $2",
    )
    .bind(routine_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?;

    if exists.is_none() {
        return Err(AppError::NotFound("Routine not found".to_string()));
    }

    let result = sqlx::query("DELETE FROM routine_tags WHERE routine_id = $1 AND tag_id = $2")
        .bind(routine_id)
        .bind(tag_id)
        .execute(pool)
        .await
        .map_err(AppError::Database)?;

    if result.rows_affected() == 0 {
        tracing::warn!(routine_id = %routine_id, tag_id = %tag_id, "Tag not found on routine");
        return Err(AppError::NotFound(
            "Tag not found on this routine".to_string(),
        ));
    }

    tracing::info!(routine_id = %routine_id, tag_id = %tag_id, "Removed tag from routine");
    Ok(())
}
