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

/// Create a new epic
pub async fn create_epic(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateEpicRequest>,
) -> Result<(StatusCode, Json<GuidanceEpic>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let epic = service::create_epic(&pool, auth.user_id, &body).await?;
    Ok((StatusCode::CREATED, Json(epic)))
}

/// List all epics for the authenticated user
pub async fn list_epics(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<GuidanceEpic>>, AppError> {
    let epics = service::list_epics(&pool, auth.user_id).await?;
    Ok(Json(epics))
}

/// Get a single epic by ID
pub async fn get_epic(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<GuidanceEpic>, AppError> {
    let epic = service::get_epic(&pool, id, auth.user_id).await?;
    Ok(Json(epic))
}

/// Update an epic by ID
pub async fn update_epic(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateEpicRequest>,
) -> Result<Json<GuidanceEpic>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let epic = service::update_epic(&pool, id, auth.user_id, &body).await?;
    Ok(Json(epic))
}

/// Delete an epic by ID
pub async fn delete_epic(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::delete_epic(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
