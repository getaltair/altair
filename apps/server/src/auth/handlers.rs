use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use sqlx::PgPool;
use validator::Validate;

use crate::{config::Config, error::AppError};
use super::{
    jwt,
    middleware::AuthenticatedUser,
    models::{
        AuthResponse, LoginRequest, PowerSyncTokenResponse, RegisterRequest, UpdateProfileRequest,
        UserProfile,
    },
    service,
};

/// Register a new user and return a session token
pub async fn register(
    State(pool): State<PgPool>,
    State(config): State<Config>,
    Json(body): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let user =
        service::create_user(&pool, &body.email, &body.display_name, &body.password).await?;

    let raw_token =
        service::create_session(&pool, user.id, config.session_ttl_hours(), None).await?;

    let profile: UserProfile = user.into();

    Ok(Json(AuthResponse {
        token: raw_token,
        user: profile,
    }))
}

/// Authenticate a user and return a session token
pub async fn login(
    State(pool): State<PgPool>,
    State(config): State<Config>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    body.validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let user = service::authenticate(&pool, &body.email, &body.password).await?;

    let raw_token =
        service::create_session(&pool, user.id, config.session_ttl_hours(), None).await?;

    let profile: UserProfile = user.into();

    Ok(Json(AuthResponse {
        token: raw_token,
        user: profile,
    }))
}

/// Invalidate the current session (logout)
pub async fn logout(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<StatusCode, AppError> {
    service::invalidate_session_by_id(&pool, auth.session_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// Get the current authenticated user's profile
pub async fn get_me(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<UserProfile>, AppError> {
    let user = service::get_user_by_id(&pool, auth.user_id).await?;

    Ok(Json(user.into()))
}

/// Update the current authenticated user's profile
pub async fn update_me(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    Json(body): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfile>, AppError> {
    let user = if let Some(display_name) = body.display_name {
        service::update_user_display_name(&pool, auth.user_id, &display_name).await?
    } else {
        service::get_user_by_id(&pool, auth.user_id).await?
    };

    Ok(Json(user.into()))
}

/// Generate a short-lived PowerSync JWT for the authenticated user.
///
/// The token includes the user's household IDs so that PowerSync
/// sync rules can filter data to the correct buckets.
pub async fn powersync_token(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    State(config): State<Config>,
) -> Result<Json<PowerSyncTokenResponse>, AppError> {
    let household_ids = service::get_user_household_ids(&pool, auth.user_id).await?;

    let token = jwt::generate_powersync_token(&config, auth.user_id, household_ids)?;

    Ok(Json(PowerSyncTokenResponse {
        token,
        powersync_url: config.powersync_url().to_string(),
    }))
}
