use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use super::service;

/// Authenticated user context injected by the auth middleware extractor.
///
/// Any handler that includes `AuthenticatedUser` in its parameter list will
/// automatically require a valid `Authorization: Bearer <token>` header.
/// If the token is missing, malformed, expired, or invalid, the request
/// is rejected with a 401 Unauthorized response.
#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    #[allow(dead_code)]
    pub household_ids: Vec<Uuid>,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    PgPool: axum::extract::FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract bearer token from Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".to_string()))?;

        let raw_token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| {
                AppError::Unauthorized("Invalid Authorization header format".to_string())
            })?;

        // Get pool from state
        let pool = PgPool::from_ref(state);

        // Validate session
        let (user, _session) = service::validate_session(&pool, raw_token).await?;

        // Get household IDs
        let household_ids = service::get_user_household_ids(&pool, user.id).await?;

        Ok(AuthenticatedUser {
            user_id: user.id,
            household_ids,
        })
    }
}
