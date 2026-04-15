use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::{CreateQuestRequest, QuestListParams, UpdateQuestRequest};
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
    Query(params): Query<QuestListParams>,
) -> Result<impl IntoResponse, AppError> {
    let quests = service::list_quests(&state.db, auth.user_id, params).await?;
    Ok(Json(quests))
}

pub async fn get_one(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let quest = service::get_quest(&state.db, id, auth.user_id).await?;
    Ok(Json(quest))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateQuestRequest>,
) -> Result<impl IntoResponse, AppError> {
    if req.title.trim().is_empty() {
        return Err(AppError::BadRequest("title must not be empty".to_string()));
    }
    let quest = service::create_quest(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(quest)))
}

pub async fn update(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateQuestRequest>,
) -> Result<impl IntoResponse, AppError> {
    let quest = service::update_quest(&state.db, id, auth.user_id, req).await?;
    Ok(Json(quest))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_quest(&state.db, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
