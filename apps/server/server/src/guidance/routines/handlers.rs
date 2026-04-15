use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::{CreateRoutineRequest, SpawnRequest, UpdateRoutineRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let routines = service::list_routines(&state.db, auth.user_id).await?;
    Ok(Json(routines))
}

pub async fn get_one(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let routine = service::get_routine(&state.db, id, auth.user_id).await?;
    Ok(Json(routine))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateRoutineRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.title.trim().is_empty() {
        return Err(AppError::BadRequest("title must not be empty".to_string()));
    }
    let routine = service::create_routine(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(routine)))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateRoutineRequest>,
) -> Result<impl IntoResponse, AppError> {
    let routine = service::update_routine(&state.db, id, auth.user_id, req).await?;
    Ok(Json(routine))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_routine(&state.db, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn spawn(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<SpawnRequest>,
) -> Result<impl IntoResponse, AppError> {
    let quest = service::spawn_routine_quest(&state.db, id, auth.user_id, req.due_date).await?;
    Ok((StatusCode::CREATED, Json(quest)))
}
