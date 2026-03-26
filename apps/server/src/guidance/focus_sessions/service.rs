use sqlx::{PgPool, QueryBuilder};
use uuid::Uuid;

use crate::error::AppError;
use super::models::*;

/// Create a new focus session for the given user
pub async fn create_focus_session(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateFocusSessionRequest,
) -> Result<GuidanceFocusSession, AppError> {
    let session = sqlx::query_as::<_, GuidanceFocusSession>(
        r#"INSERT INTO guidance_focus_sessions (quest_id, user_id, started_at, ended_at, duration_minutes, notes)
           VALUES ($1, $2, $3, $4, $5, $6)
           RETURNING id, quest_id, user_id, started_at, ended_at, duration_minutes, notes, created_at"#,
    )
    .bind(req.quest_id)
    .bind(user_id)
    .bind(req.started_at)
    .bind(req.ended_at)
    .bind(req.duration_minutes)
    .bind(&req.notes)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Focus session already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
            AppError::BadRequest("Invalid field value".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::BadRequest("Referenced resource does not exist".to_string())
        }
        _ => AppError::Database(e),
    })?;

    tracing::info!(session_id = %session.id, user_id = %user_id, "Created focus session");
    Ok(session)
}

/// List all focus sessions for the authenticated user
pub async fn list_focus_sessions(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<GuidanceFocusSession>, AppError> {
    sqlx::query_as::<_, GuidanceFocusSession>(
        r#"SELECT id, quest_id, user_id, started_at, ended_at, duration_minutes, notes, created_at
           FROM guidance_focus_sessions
           WHERE user_id = $1
           ORDER BY created_at DESC"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Get a single focus session by ID, scoped to the user
pub async fn get_focus_session(
    pool: &PgPool,
    session_id: Uuid,
    user_id: Uuid,
) -> Result<GuidanceFocusSession, AppError> {
    sqlx::query_as::<_, GuidanceFocusSession>(
        r#"SELECT id, quest_id, user_id, started_at, ended_at, duration_minutes, notes, created_at
           FROM guidance_focus_sessions
           WHERE id = $1 AND user_id = $2"#,
    )
    .bind(session_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Focus session not found".to_string()))
}

/// Update a focus session's fields. Only the session owner can update it.
pub async fn update_focus_session(
    pool: &PgPool,
    session_id: Uuid,
    user_id: Uuid,
    req: &UpdateFocusSessionRequest,
) -> Result<GuidanceFocusSession, AppError> {
    let mut qb: QueryBuilder<sqlx::Postgres> =
        QueryBuilder::new("UPDATE guidance_focus_sessions SET id = id");

    if let Some(ref ended_at) = req.ended_at {
        qb.push(", ended_at = ");
        qb.push_bind(*ended_at);
    }

    if let Some(ref duration_minutes) = req.duration_minutes {
        qb.push(", duration_minutes = ");
        qb.push_bind(*duration_minutes);
    }

    if let Some(ref notes) = req.notes {
        qb.push(", notes = ");
        qb.push_bind(notes.clone());
    }

    qb.push(" WHERE id = ");
    qb.push_bind(session_id);
    qb.push(" AND user_id = ");
    qb.push_bind(user_id);
    qb.push(" RETURNING id, quest_id, user_id, started_at, ended_at, duration_minutes, notes, created_at");

    let session = qb
        .build_query_as::<GuidanceFocusSession>()
        .fetch_optional(pool)
        .await
        .map_err(|e| match &e {
            sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
                AppError::Conflict("Focus session already exists".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
                AppError::BadRequest("Invalid field value".to_string())
            }
            sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
                AppError::BadRequest("Referenced resource does not exist".to_string())
            }
            _ => AppError::Database(e),
        })?
        .ok_or_else(|| AppError::NotFound("Focus session not found".to_string()))?;

    tracing::info!(session_id = %session_id, user_id = %user_id, "Updated focus session");
    Ok(session)
}

/// Delete a focus session by ID. Only the session owner can delete it.
pub async fn delete_focus_session(
    pool: &PgPool,
    session_id: Uuid,
    user_id: Uuid,
) -> Result<(), AppError> {
    let result = sqlx::query(
        "DELETE FROM guidance_focus_sessions WHERE id = $1 AND user_id = $2",
    )
    .bind(session_id)
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
        return Err(AppError::NotFound("Focus session not found".to_string()));
    }

    tracing::info!(session_id = %session_id, user_id = %user_id, "Deleted focus session");
    Ok(())
}
