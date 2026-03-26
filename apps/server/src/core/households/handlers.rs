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
    models::{CreateHouseholdRequest, Household, HouseholdMembership, HouseholdRole, InviteMemberRequest},
    service,
};

/// Create a new household and assign the caller as its owner
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

/// List all households the authenticated user belongs to
pub async fn list_households(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Household>>, AppError> {
    let households = service::list_user_households(&pool, auth.user_id).await?;
    Ok(Json(households))
}

/// Invite an existing user to a household by email (owner/admin only)
pub async fn invite_member(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(household_id): Path<Uuid>,
    Json(body): Json<InviteMemberRequest>,
) -> Result<Json<HouseholdMembership>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Verify caller has owner/admin role in the household
    let caller_role = service::get_member_role(&pool, household_id, auth.user_id).await?;
    match caller_role.as_deref() {
        Some("owner") | Some("admin") => {}
        Some(_) => {
            return Err(AppError::Forbidden(
                "Only owners can invite members".to_string(),
            ));
        }
        None => {
            return Err(AppError::Forbidden(
                "You are not a member of this household".to_string(),
            ));
        }
    }

    let role = body.role.as_ref().unwrap_or(&HouseholdRole::Member).as_str();
    let membership =
        service::invite_member_by_email(&pool, household_id, &body.user_email, role).await?;
    Ok(Json(membership))
}
