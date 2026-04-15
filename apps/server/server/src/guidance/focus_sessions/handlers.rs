use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use super::models::{CreateFocusSessionRequest, UpdateFocusSessionRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;

#[derive(Debug, Deserialize)]
pub struct SessionListParams {
    pub quest_id: Option<Uuid>,
}

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<SessionListParams>,
) -> Result<impl IntoResponse, AppError> {
    let sessions = service::list_sessions(&state.db, auth.user_id, params.quest_id).await?;
    Ok(Json(sessions))
}

pub async fn get_one(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let session = service::get_session(&state.db, id, auth.user_id).await?;
    Ok(Json(session))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateFocusSessionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let session = service::create_session(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(session)))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateFocusSessionRequest>,
) -> Result<impl IntoResponse, AppError> {
    let session = service::update_session(&state.db, id, auth.user_id, req).await?;
    Ok(Json(session))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_session(&state.db, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
