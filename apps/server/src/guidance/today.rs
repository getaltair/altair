use axum::{extract::State, Json};
use chrono::Utc;
use sqlx::PgPool;
use serde::Serialize;

use crate::auth::middleware::AuthenticatedUser;
use crate::auth::service as auth_service;
use crate::error::AppError;
use crate::guidance::daily_checkins::models::GuidanceDailyCheckin;
use crate::guidance::quests::models::GuidanceQuest;
use crate::guidance::routines::models::GuidanceRoutine;

#[derive(Debug, Serialize)]
pub struct TodayResponse {
    pub quests: Vec<GuidanceQuest>,
    pub routines: Vec<GuidanceRoutine>,
    pub checkin: Option<GuidanceDailyCheckin>,
}

/// Get the "today" dashboard: pending/in-progress quests due today or without
/// a due date, active routines, and today's check-in.
pub async fn handler(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<TodayResponse>, AppError> {
    let today = Utc::now().date_naive();
    let household_ids = auth_service::get_user_household_ids(&pool, auth.user_id).await?;

    // Quests: pending or in_progress, due today or no due date, user-owned or household-visible
    let quests = sqlx::query_as::<_, GuidanceQuest>(
        r#"SELECT id, epic_id, initiative_id, user_id, household_id, name, description,
                  status, priority, due_date, estimated_minutes, completed_at, created_at, updated_at
           FROM guidance_quests
           WHERE (user_id = $1 OR household_id = ANY($2))
             AND status IN ('pending', 'in_progress')
             AND (due_date IS NULL OR due_date <= $3)
           ORDER BY created_at DESC"#,
    )
    .bind(auth.user_id)
    .bind(&household_ids)
    .bind(today)
    .fetch_all(&pool)
    .await
    .map_err(AppError::Database)?;

    // Active routines
    let routines = sqlx::query_as::<_, GuidanceRoutine>(
        r#"SELECT id, user_id, household_id, name, description, frequency, status, created_at, updated_at
           FROM guidance_routines
           WHERE (user_id = $1 OR household_id = ANY($2))
             AND status = 'active'
           ORDER BY name ASC"#,
    )
    .bind(auth.user_id)
    .bind(&household_ids)
    .fetch_all(&pool)
    .await
    .map_err(AppError::Database)?;

    // Today's check-in
    let checkin = sqlx::query_as::<_, GuidanceDailyCheckin>(
        r#"SELECT id, user_id, date, energy_level, mood, notes, created_at
           FROM guidance_daily_checkins
           WHERE user_id = $1 AND date = $2"#,
    )
    .bind(auth.user_id)
    .bind(today)
    .fetch_optional(&pool)
    .await
    .map_err(AppError::Database)?;

    Ok(Json(TodayResponse { quests, routines, checkin }))
}
