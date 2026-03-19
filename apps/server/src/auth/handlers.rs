//! HTTP handlers for auth endpoints.
//!
//! This module provides route handlers for authentication-related operations.

use super::extractor::AuthenticatedUser;
use super::models::User;
use crate::state::AppState;
use axum::{Json, Router, extract::State, routing::get};
use utoipa::ToSchema;

/// Error response format for authentication failures.
#[derive(ToSchema)]
#[allow(dead_code)]
pub struct ErrorResponse {
	/// Error type code
	error: String,
	/// Human-readable error message
	message: String,
}

/// Get the currently authenticated user.
///
/// This endpoint returns the user profile for the authenticated user.
/// Authentication is enforced by the `AuthenticatedUser` extractor.
///
/// # Returns
///
/// Returns a JSON response with the user data:
/// - `id`: User's UUID
/// - `name`: Display name
/// - `email`: Email address
/// - `email_verified`: Whether email is verified
/// - `image`: Profile image URL (optional)
/// - `created_at`: Account creation timestamp
/// - `updated_at`: Last update timestamp
///
/// # Errors
///
/// Returns 401 Unauthorized if:
/// - Session token cookie is missing
/// - Session token is invalid
/// - Session has expired
///
/// These errors are handled automatically by the `AuthenticatedUser` extractor.
#[utoipa::path(
	get,
	path = "/auth/me",
	responses(
		(status = 200, description = "Current user profile", body = User),
		(status = 401, description = "Unauthorized - missing or invalid session", body = ErrorResponse)
	),
	security(
		("better_auth_session" = [])
	),
	tag = "Auth"
)]
#[axum::debug_handler]
pub async fn me(State(_state): State<AppState>, user: AuthenticatedUser) -> Json<User> {
	Json(user.0)
}

/// Create the auth handlers router.
///
/// Routes are mounted at `/auth/*` in the main router.
pub fn router() -> Router<AppState> {
	Router::new().route("/me", get(me))
}
