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
use super::{models::*, service};

/// Create a new quest
pub async fn create_quest(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateQuestRequest>,
) -> Result<(StatusCode, Json<GuidanceQuest>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let quest = service::create_quest(&pool, auth.user_id, &body).await?;
    Ok((StatusCode::CREATED, Json(quest)))
}

/// List all quests visible to the authenticated user
pub async fn list_quests(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<GuidanceQuest>>, AppError> {
    let household_ids = auth_service::get_user_household_ids(&pool, auth.user_id).await?;
    let quests = service::list_quests(&pool, auth.user_id, &household_ids).await?;
    Ok(Json(quests))
}

/// Get a single quest by ID
pub async fn get_quest(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<GuidanceQuest>, AppError> {
    let household_ids = auth_service::get_user_household_ids(&pool, auth.user_id).await?;
    let quest = service::get_quest(&pool, id, auth.user_id, &household_ids).await?;
    Ok(Json(quest))
}

/// Update a quest by ID
pub async fn update_quest(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateQuestRequest>,
) -> Result<Json<GuidanceQuest>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let quest = service::update_quest(&pool, id, auth.user_id, &body).await?;
    Ok(Json(quest))
}

/// Delete a quest by ID
pub async fn delete_quest(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::delete_quest(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Mark a quest as completed
pub async fn complete_quest(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<GuidanceQuest>, AppError> {
    let household_ids = auth_service::get_user_household_ids(&pool, auth.user_id).await?;
    let quest = service::complete_quest(&pool, id, auth.user_id, &household_ids).await?;
    Ok(Json(quest))
}

/// Add a tag to a quest
pub async fn add_quest_tag(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path((id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    service::add_quest_tag(&pool, id, tag_id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Remove a tag from a quest
pub async fn remove_quest_tag(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path((id, tag_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, AppError> {
    service::remove_quest_tag(&pool, id, tag_id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
