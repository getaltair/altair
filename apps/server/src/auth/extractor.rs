//! Axum extractors for authenticated requests.
//!
//! This module provides custom extractors that validate session tokens
//! from cookies and return the authenticated user.

use super::models::User;
use super::session::{AuthError, validate_session_token};
use axum::{
	extract::FromRequestParts,
	http::{StatusCode, request::Parts},
	response::{IntoResponse, Json, Response},
};
use axum_extra::extract::CookieJar;
use serde::Serialize;
use sqlx::PgPool;

/// Newtype wrapper for authenticated users.
///
/// This struct is used as an Axum extractor to enforce authentication
/// on route handlers. When extracted, it validates the session token
/// from the `better-auth.session_token` cookie and returns the
/// authenticated user.
///
/// # Example
///
/// ```ignore
/// use axum::routing::get;
/// use crate::auth::extractor::AuthenticatedUser;
///
/// async fn protected_route(user: AuthenticatedUser) -> &'static str {
///     "Hello, authenticated user!"
/// }
///
/// let app = Router::new().route("/protected", get(protected_route));
/// ```
#[allow(dead_code)] // TODO(P3-004): Remove when protected routes use this extractor
#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub User);

/// Error response format for authentication failures.
#[derive(Serialize)]
struct ErrorResponse {
	error: String,
	message: String,
}

impl IntoResponse for AuthError {
	fn into_response(self) -> Response {
		let (status, error, message) = match self {
			AuthError::InvalidToken => (
				StatusCode::UNAUTHORIZED,
				"UNAUTHORIZED".to_string(),
				"Invalid or missing session token".to_string(),
			),
			AuthError::ExpiredToken => (
				StatusCode::UNAUTHORIZED,
				"UNAUTHORIZED".to_string(),
				"Session has expired".to_string(),
			),
			AuthError::UserNotFound => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"INTERNAL_ERROR".to_string(),
				"User not found".to_string(),
			),
			AuthError::DatabaseError(err) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"INTERNAL_ERROR".to_string(),
				format!("Database error: {err}"),
			),
		};

		let body = Json(ErrorResponse { error, message });
		(status, body).into_response()
	}
}

impl FromRequestParts<PgPool> for AuthenticatedUser {
	type Rejection = Response;

	async fn from_request_parts(
		parts: &mut Parts,
		state: &PgPool,
	) -> Result<Self, Self::Rejection> {
		let jar = CookieJar::from_request_parts(parts, state)
			.await
			.map_err(|err| {
				(
					StatusCode::BAD_REQUEST,
					Json(ErrorResponse {
						error: "BAD_REQUEST".to_string(),
						message: format!("Failed to parse cookies: {err}"),
					}),
				)
					.into_response()
			})?;

		let token = jar
			.get("better-auth.session_token")
			.ok_or_else(|| {
				(
					StatusCode::UNAUTHORIZED,
					Json(ErrorResponse {
						error: "UNAUTHORIZED".to_string(),
						message: "Session token cookie not found".to_string(),
					}),
				)
					.into_response()
			})?
			.value();

		let pool = state;
		let (user, _session) = validate_session_token(pool, token)
			.await
			.map_err(|err| err.into_response())?;

		Ok(AuthenticatedUser(user))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_authenticated_user_debug() {
		let user = User {
			id: uuid::Uuid::new_v4(),
			name: "Test User".to_string(),
			email: "test@example.com".to_string(),
			email_verified: true,
			image: None,
			created_at: chrono::Utc::now(),
			updated_at: chrono::Utc::now(),
		};
		let auth_user = AuthenticatedUser(user);
		assert!(format!("{:?}", auth_user).contains("AuthenticatedUser"));
	}
}
