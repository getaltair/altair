use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use validator::Validate;

use crate::{config::Config, error::AppError};
use super::{
    middleware::AuthenticatedUser,
    models::{AuthResponse, LoginRequest, RegisterRequest, UpdateProfileRequest, UserProfile},
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

    let profile = UserProfile {
        id: user.id,
        email: user.email,
        display_name: user.display_name,
        created_at: user.created_at,
    };

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
    let user = service::authenticate(&pool, &body.email, &body.password).await?;

    let raw_token =
        service::create_session(&pool, user.id, config.session_ttl_hours(), None).await?;

    let profile = UserProfile {
        id: user.id,
        email: user.email,
        display_name: user.display_name,
        created_at: user.created_at,
    };

    Ok(Json(AuthResponse {
        token: raw_token,
        user: profile,
    }))
}

/// Invalidate the current session (logout)
pub async fn logout(
    _auth: AuthenticatedUser,
    State(pool): State<PgPool>,
    headers: HeaderMap,
) -> Result<StatusCode, AppError> {
    let raw_token = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| AppError::Unauthorized("Missing token".to_string()))?;

    // Hash the raw token to look up the session in the database
    let bytes = hex::decode(raw_token)
        .map_err(|_| AppError::Unauthorized("Invalid token format".to_string()))?;
    let hash = Sha256::digest(&bytes);
    let token_hash = hex::encode(hash);

    service::invalidate_session(&pool, &token_hash).await?;

    Ok(StatusCode::NO_CONTENT)
}

/// Get the current authenticated user's profile
pub async fn get_me(
    auth: AuthenticatedUser,
    State(pool): State<PgPool>,
) -> Result<Json<UserProfile>, AppError> {
    let user = service::get_user_by_id(&pool, auth.user_id).await?;

    Ok(Json(UserProfile {
        id: user.id,
        email: user.email,
        display_name: user.display_name,
        created_at: user.created_at,
    }))
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

    Ok(Json(UserProfile {
        id: user.id,
        email: user.email,
        display_name: user.display_name,
        created_at: user.created_at,
    }))
}
