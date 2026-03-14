//! HTTP handlers for user endpoints.
//!
//! This module provides route handlers for user-related operations.

use axum::{extract::State, response::Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{PgPool, Row};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::auth::{AuthenticatedUser, ErrorResponse};

/// User model representing the custom `users` table.
///
/// Contains application-specific user data:
/// - id: UUID primary key (matches Better-Auth user.id)
/// - email: Email address
/// - display_name: User's display name
/// - timezone: User's timezone preference
/// - is_active: Whether the user account is active
/// - created_at: Account creation timestamp
/// - updated_at: Last update timestamp
#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AppUser {
	pub id: Uuid,
	pub email: String,
	pub display_name: String,
	pub timezone: String,
	pub is_active: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Get the currently authenticated user's profile.
///
/// This endpoint returns the application-specific user profile for the authenticated user.
/// Authentication is enforced by the `AuthenticatedUser` extractor.
///
/// # Returns
///
/// Returns a JSON response with the user data:
/// - `id`: User's UUID
/// - `email`: Email address
/// - `display_name`: Display name
/// - `timezone`: User's timezone preference
/// - `is_active`: Whether the user account is active
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
/// Returns 404 Not Found if:
/// - User profile does not exist in the custom users table
///
/// These errors are handled automatically by the `AuthenticatedUser` extractor
/// and database query.
#[utoipa::path(
	get,
	path = "/users/me",
	responses(
		(status = 200, description = "User profile", body = AppUser),
		(status = 401, description = "Unauthorized", body = ErrorResponse),
		(status = 404, description = "User not found", body = ErrorResponse)
	),
	security(("better_auth_session" = []))
)]
#[allow(dead_code)] // Wired in Task 20
#[axum::debug_handler]
pub async fn me(
	State(pool): State<PgPool>,
	user: AuthenticatedUser,
) -> Result<Json<AppUser>, crate::error::AppError> {
	let user_id = user.0.id;

	let row = sqlx::query(
		r#"
		SELECT id, email, display_name, timezone, is_active, created_at, updated_at
		FROM users
		WHERE id = $1
		"#,
	)
	.bind(user_id)
	.fetch_one(&pool)
	.await?;

	let app_user = AppUser {
		id: row.get("id"),
		email: row.get("email"),
		display_name: row.get("display_name"),
		timezone: row.get("timezone"),
		is_active: row.get("is_active"),
		created_at: row.get("created_at"),
		updated_at: row.get("updated_at"),
	};

	Ok(Json(app_user))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn app_user_serialization() {
		let id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
		let now = Utc::now();

		let user = AppUser {
			id,
			email: "test@example.com".to_string(),
			display_name: "Test User".to_string(),
			timezone: "America/Chicago".to_string(),
			is_active: true,
			created_at: now,
			updated_at: now,
		};

		let json = serde_json::to_string(&user).unwrap();
		assert!(json.contains("test@example.com"));
		assert!(json.contains("Test User"));
		assert!(json.contains("America/Chicago"));
		assert!(json.contains("\"is_active\":true"));
	}

	#[test]
	fn app_user_debug() {
		let user = AppUser {
			id: Uuid::new_v4(),
			email: "debug@example.com".to_string(),
			display_name: "Debug User".to_string(),
			timezone: "UTC".to_string(),
			is_active: true,
			created_at: Utc::now(),
			updated_at: Utc::now(),
		};

		let debug_str = format!("{user:?}");
		assert!(debug_str.contains("Debug User"));
		assert!(debug_str.contains("debug@example.com"));
	}
}
