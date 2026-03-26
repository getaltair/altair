use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::middleware::AuthenticatedUser;
use crate::error::AppError;
use super::{models::*, service};

/// Create or update a daily check-in
pub async fn create_or_update_checkin(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateOrUpdateCheckinRequest>,
) -> Result<(StatusCode, Json<GuidanceDailyCheckin>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let checkin = service::create_or_update_checkin(&pool, auth.user_id, &body).await?;
    Ok((StatusCode::CREATED, Json(checkin)))
}

/// List all daily check-ins for the authenticated user
pub async fn list_checkins(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<GuidanceDailyCheckin>>, AppError> {
    let checkins = service::list_checkins(&pool, auth.user_id).await?;
    Ok(Json(checkins))
}

/// Get a single daily check-in by ID
pub async fn get_checkin(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<GuidanceDailyCheckin>, AppError> {
    let checkin = service::get_checkin(&pool, id, auth.user_id).await?;
    Ok(Json(checkin))
}
