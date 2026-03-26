use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::auth::middleware::AuthenticatedUser;
use crate::error::AppError;
use super::{
    models::{CreateHouseholdRequest, Household, HouseholdMembership, InviteMemberRequest},
    service,
};

pub async fn create_household(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<CreateHouseholdRequest>,
) -> Result<(StatusCode, Json<Household>), AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let household = service::create_household(&pool, &body.name, auth.user_id).await?;
    Ok((StatusCode::CREATED, Json(household)))
}

pub async fn list_households(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Household>>, AppError> {
    let households = service::list_user_households(&pool, auth.user_id).await?;
    Ok(Json(households))
}

pub async fn invite_member(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(household_id): Path<Uuid>,
    Json(body): Json<InviteMemberRequest>,
) -> Result<Json<HouseholdMembership>, AppError> {
    // Verify caller is a member of the household
    let is_member = service::is_member(&pool, household_id, auth.user_id).await?;
    if !is_member {
        return Err(AppError::Forbidden(
            "You are not a member of this household".to_string(),
        ));
    }
    let role = body.role.as_deref().unwrap_or("member");
    let membership =
        service::invite_member_by_email(&pool, household_id, &body.user_email, role).await?;
    Ok(Json(membership))
}
