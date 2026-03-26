use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::middleware::AuthenticatedUser;
use crate::error::AppError;
use super::{
    models::{CreateInitiativeRequest, Initiative, UpdateInitiativeRequest},
    service,
};

/// Query parameters for listing initiatives
#[derive(Debug, Deserialize)]
pub struct ListInitiativesQuery {
    pub household_id: Option<Uuid>,
}

/// Create a new initiative owned by the authenticated user
pub async fn create_initiative(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateInitiativeRequest>,
) -> Result<(StatusCode, Json<Initiative>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let initiative = service::create_initiative(&pool, auth.user_id, body).await?;
    Ok((StatusCode::CREATED, Json(initiative)))
}

/// List initiatives visible to the authenticated user, optionally filtered by household
pub async fn list_initiatives(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Query(query): Query<ListInitiativesQuery>,
) -> Result<Json<Vec<Initiative>>, AppError> {
    let initiatives =
        service::list_initiatives(&pool, auth.user_id, query.household_id).await?;
    Ok(Json(initiatives))
}

/// Get a single initiative by ID
pub async fn get_initiative(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Initiative>, AppError> {
    let initiative = service::get_initiative(&pool, id, auth.user_id).await?;
    Ok(Json(initiative))
}

/// Update an existing initiative (partial update)
pub async fn update_initiative(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateInitiativeRequest>,
) -> Result<Json<Initiative>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let initiative = service::update_initiative(&pool, id, auth.user_id, body).await?;
    Ok(Json(initiative))
}

/// Soft-delete (archive) an initiative
pub async fn delete_initiative(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service::soft_delete_initiative(&pool, id, auth.user_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
