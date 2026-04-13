use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

use super::models::CreateRelationRequest;
use super::service;
use crate::AppState;
use crate::auth::models::AuthUser;
use crate::error::AppError;

pub async fn list(
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let relations = service::list_relations(&state.db, auth.user_id).await?;
    Ok(Json(relations))
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthUser,
    Json(req): Json<CreateRelationRequest>,
) -> Result<impl IntoResponse, AppError> {
    let relation = service::create_relation(&state.db, auth.user_id, req).await?;
    Ok((StatusCode::CREATED, Json(relation)))
}

pub async fn delete(
    State(state): State<AppState>,
    auth: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    service::delete_relation(&state.db, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
