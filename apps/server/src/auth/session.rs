//! Session validation logic for Better-Auth sessions.
//!
//! This module provides functions to validate session tokens and retrieve
//! associated user data from the database.

use super::models::{Session, User};
use sqlx::{PgPool, Row};
use thiserror::Error;

/// Errors that can occur during session validation.
#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum AuthError {
	/// The provided session token does not exist in the database.
	#[error("Invalid session token")]
	InvalidToken,

	/// The session exists but has expired.
	#[error("Session expired")]
	ExpiredToken,

	/// The session's associated user could not be found.
	#[error("User not found")]
	UserNotFound,

	/// A database error occurred during validation.
	#[error("Database error: {0}")]
	DatabaseError(#[from] sqlx::Error),
}

/// Validates a session token and returns the associated user and session.
///
/// This function:
/// 1. Queries the Better-Auth `session` table by token
/// 2. JOINs with the `user` table to get user data
/// 3. Checks that the session has not expired (`expires_at > NOW()`)
///
/// # Arguments
///
/// * `pool` - PostgreSQL connection pool
/// * `token` - Session token to validate
///
/// # Returns
///
/// Returns `Ok((User, Session))` if the token is valid and not expired.
/// Returns `Err(AuthError)` if the token is invalid, expired, or a database error occurs.
///
/// # Errors
///
/// - `AuthError::InvalidToken` - No session found with the provided token
/// - `AuthError::ExpiredToken` - Session exists but expires_at <= NOW()
/// - `AuthError::UserNotFound` - Session exists but associated user not found
/// - `AuthError::DatabaseError` - SQL query failed
#[allow(dead_code)]
pub async fn validate_session_token(
	pool: &PgPool,
	token: &str,
) -> Result<(User, Session), AuthError> {
	let row = sqlx::query(
		r#"
		SELECT
			session.id,
			session.expires_at,
			session.token,
			session.created_at,
			session.updated_at,
			session.ip_address,
			session.user_agent,
			session.user_id as session_user_id,
			"user".id as user_id,
			"user".name as user_name,
			"user".email as user_email,
			"user".email_verified as user_email_verified,
			"user".image as user_image,
			"user".created_at as user_created_at,
			"user".updated_at as user_updated_at
		FROM session
		JOIN "user" ON session.user_id = "user".id
		WHERE session.token = $1
		"#,
	)
	.bind(token)
	.fetch_optional(pool)
	.await?
	.ok_or(AuthError::InvalidToken)?;

	let user_id: uuid::Uuid = row.try_get("user_id")?;
	let session_id: uuid::Uuid = row.try_get("id")?;
	let expires_at: chrono::DateTime<chrono::Utc> = row.try_get("expires_at")?;

	if expires_at <= chrono::Utc::now() {
		return Err(AuthError::ExpiredToken);
	}

	let user = User {
		id: user_id,
		name: row.try_get("user_name")?,
		email: row.try_get("user_email")?,
		email_verified: row.try_get("user_email_verified")?,
		image: row.try_get("user_image")?,
		created_at: row.try_get("user_created_at")?,
		updated_at: row.try_get("user_updated_at")?,
	};

	let session = Session {
		id: session_id,
		expires_at,
		token: row.try_get("token")?,
		created_at: row.try_get("created_at")?,
		updated_at: row.try_get("updated_at")?,
		ip_address: row.try_get("ip_address")?,
		user_agent: row.try_get("user_agent")?,
		user_id: row.try_get("session_user_id")?,
	};

	Ok((user, session))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_auth_error_display() {
		assert_eq!(
			format!("{}", AuthError::InvalidToken),
			"Invalid session token"
		);
		assert_eq!(format!("{}", AuthError::ExpiredToken), "Session expired");
		assert_eq!(format!("{}", AuthError::UserNotFound), "User not found");
	}
}
