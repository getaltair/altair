use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use super::models::*;

/// Create or update a daily check-in for the user.
///
/// Uses an upsert (INSERT ... ON CONFLICT) to handle the case where a check-in
/// for the same user and date already exists. If no date is provided, defaults
/// to today (UTC). On conflict, uses COALESCE to preserve existing values when
/// the incoming value is NULL, preventing accidental overwrites.
pub async fn create_or_update_checkin(
    pool: &PgPool,
    user_id: Uuid,
    req: &CreateOrUpdateCheckinRequest,
) -> Result<GuidanceDailyCheckin, AppError> {
    let date = req.date.unwrap_or_else(|| Utc::now().date_naive());

    let checkin = sqlx::query_as::<_, GuidanceDailyCheckin>(
        r#"INSERT INTO guidance_daily_checkins (user_id, date, energy_level, mood, notes)
           VALUES ($1, $2, $3, $4, $5)
           ON CONFLICT (user_id, date) DO UPDATE
           SET energy_level = COALESCE(EXCLUDED.energy_level, guidance_daily_checkins.energy_level),
               mood = COALESCE(EXCLUDED.mood, guidance_daily_checkins.mood),
               notes = COALESCE(EXCLUDED.notes, guidance_daily_checkins.notes),
               created_at = guidance_daily_checkins.created_at
           RETURNING id, user_id, date, energy_level, mood, notes, created_at"#,
    )
    .bind(user_id)
    .bind(date)
    .bind(req.energy_level)
    .bind(&req.mood)
    .bind(&req.notes)
    .fetch_one(pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict("Daily check-in already exists".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
            AppError::BadRequest("Invalid field value".to_string())
        }
        sqlx::Error::Database(db_err) if db_err.is_foreign_key_violation() => {
            AppError::BadRequest("Referenced resource does not exist".to_string())
        }
        _ => AppError::Database(e),
    })?;

    tracing::info!(checkin_id = %checkin.id, user_id = %user_id, date = %date, "Upserted daily check-in");
    Ok(checkin)
}

/// List all daily check-ins for the user, ordered by date descending
pub async fn list_checkins(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<GuidanceDailyCheckin>, AppError> {
    sqlx::query_as::<_, GuidanceDailyCheckin>(
        r#"SELECT id, user_id, date, energy_level, mood, notes, created_at
           FROM guidance_daily_checkins
           WHERE user_id = $1
           ORDER BY date DESC"#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
    .map_err(AppError::Database)
}

/// Get a single daily check-in by ID, scoped to the user
pub async fn get_checkin(
    pool: &PgPool,
    checkin_id: Uuid,
    user_id: Uuid,
) -> Result<GuidanceDailyCheckin, AppError> {
    sqlx::query_as::<_, GuidanceDailyCheckin>(
        r#"SELECT id, user_id, date, energy_level, mood, notes, created_at
           FROM guidance_daily_checkins
           WHERE id = $1 AND user_id = $2"#,
    )
    .bind(checkin_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(AppError::Database)?
    .ok_or_else(|| AppError::NotFound("Daily check-in not found".to_string()))
}
