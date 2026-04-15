use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use super::models::{CreateEpicRequest, UpdateEpicRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub initiative_id: Option<Uuid>,
}

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<ListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let epics = service::list_epics(&state.db, auth.user_id, params.initiative_id).await?;
    Ok(Json(epics))
}

pub async fn get_one(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let epic = service::get_epic(&state.db, id, auth.user_id).await?;
    Ok(Json(epic))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateEpicRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.title.trim().is_empty() {
        return Err(AppError::BadRequest("title must not be empty".to_string()));
    }
    let epic = service::create_epic(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(epic)))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateEpicRequest>,
) -> Result<impl IntoResponse, AppError> {
    let epic = service::update_epic(&state.db, id, auth.user_id, req).await?;
    Ok(Json(epic))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_epic(&state.db, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
