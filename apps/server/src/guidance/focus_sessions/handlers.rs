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

/// Create a new focus session
pub async fn create_focus_session(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateFocusSessionRequest>,
) -> Result<(StatusCode, Json<GuidanceFocusSession>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let session = service::create_focus_session(&pool, auth.user_id, &body).await?;
    Ok((StatusCode::CREATED, Json(session)))
}

/// List all focus sessions for the authenticated user
pub async fn list_focus_sessions(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<GuidanceFocusSession>>, AppError> {
    let sessions = service::list_focus_sessions(&pool, auth.user_id).await?;
    Ok(Json(sessions))
}

/// Get a single focus session by ID
pub async fn get_focus_session(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<GuidanceFocusSession>, AppError> {
    let session = service::get_focus_session(&pool, id, auth.user_id).await?;
    Ok(Json(session))
}

/// Update a focus session by ID
pub async fn update_focus_session(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateFocusSessionRequest>,
) -> Result<Json<GuidanceFocusSession>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let session = service::update_focus_session(&pool, id, auth.user_id, &body).await?;
    Ok(Json(session))
}

/// Delete a focus session by ID
pub async fn delete_focus_session(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::delete_focus_session(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
