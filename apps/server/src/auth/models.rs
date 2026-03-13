//! Database models for auth domain.
//!
//! These structs map to Better-Auth's PostgreSQL tables using `sqlx::FromRow`.
//! Field names use snake_case to match database column names exactly.

use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// User model representing the `user` table from Better-Auth.
///
/// Matches the Better-Auth `user` schema:
/// - id: UUID primary key
/// - name: Display name (required)
/// - email: Email address (required, unique)
/// - email_verified: Whether email has been verified (default false)
/// - image: Profile image URL (optional)
/// - created_at: Account creation timestamp
/// - updated_at: Last update timestamp
#[derive(Debug, Clone, sqlx::FromRow, Serialize)]
#[allow(dead_code)]
pub struct User {
	pub id: Uuid,
	pub name: String,
	pub email: String,
	pub email_verified: bool,
	pub image: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

/// Session model representing the `session` table from Better-Auth.
///
/// Matches the Better-Auth `session` schema:
/// - id: UUID primary key
/// - expires_at: When the session expires
/// - token: Unique session token
/// - created_at: Session creation timestamp
/// - updated_at: Last update timestamp
/// - ip_address: Client IP address (optional)
/// - user_agent: Client user agent string (optional)
/// - user_id: Foreign key to user table
#[derive(Debug, Clone, sqlx::FromRow)]
#[allow(dead_code)]
pub struct Session {
	pub id: Uuid,
	pub expires_at: DateTime<Utc>,
	pub token: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub ip_address: Option<String>,
	pub user_agent: Option<String>,
	pub user_id: Uuid,
}
