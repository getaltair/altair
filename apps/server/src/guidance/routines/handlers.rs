use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::middleware::AuthenticatedUser;
use crate::auth::service as auth_service;
use crate::error::AppError;
use crate::guidance::quests::models::GuidanceQuest;
use super::{models::*, service};

/// Create a new routine
pub async fn create_routine(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateRoutineRequest>,
) -> Result<(StatusCode, Json<GuidanceRoutine>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let routine = service::create_routine(&pool, auth.user_id, &body).await?;
    Ok((StatusCode::CREATED, Json(routine)))
}

/// List all routines visible to the authenticated user
pub async fn list_routines(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<GuidanceRoutine>>, AppError> {
    let household_ids = auth_service::get_user_household_ids(&pool, auth.user_id).await?;
    let routines = service::list_routines(&pool, auth.user_id, &household_ids).await?;
    Ok(Json(routines))
}

/// Get a single routine by ID
pub async fn get_routine(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<GuidanceRoutine>, AppError> {
    let household_ids = auth_service::get_user_household_ids(&pool, auth.user_id).await?;
    let routine = service::get_routine(&pool, id, auth.user_id, &household_ids).await?;
    Ok(Json(routine))
}

/// Update a routine by ID
pub async fn update_routine(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateRoutineRequest>,
) -> Result<Json<GuidanceRoutine>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let routine = service::update_routine(&pool, id, auth.user_id, &body).await?;
    Ok(Json(routine))
}

/// Delete a routine by ID
pub async fn delete_routine(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::delete_routine(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Trigger a routine, creating a new quest from it
pub async fn trigger_routine(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<GuidanceQuest>), AppError> {
    let household_ids = auth_service::get_user_household_ids(&pool, auth.user_id).await?;
    let quest = service::trigger_routine(&pool, id, auth.user_id, &household_ids).await?;
    Ok((StatusCode::CREATED, Json(quest)))
}

/// Add a tag to a routine
pub async fn add_routine_tag(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path((id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    service::add_routine_tag(&pool, id, tag_id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Remove a tag from a routine
pub async fn remove_routine_tag(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path((id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    service::remove_routine_tag(&pool, id, tag_id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
