//! Auth domain module.
//!
//! Handles authentication, identity, sessions, and authorization.

mod authorization;
mod extractor;
mod handlers;
mod models;
mod session;

#[allow(unused_imports)]
pub use authorization::{
	AuthorizationError, HouseholdRole, UserOwnableTable, can_access_attachment,
	can_access_household, can_access_initiative, can_access_tag, can_access_user_owned,
	require_attachment_access, require_household_role, require_initiative_access,
	require_user_owned,
};
#[allow(unused_imports)]
pub use extractor::AuthenticatedUser;
#[allow(unused_imports)]
pub use models::{Session, User};
#[allow(unused_imports)]
pub use session::{AuthError, validate_session_token};

use axum::Router;
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/auth/*` in the main router.
pub fn router() -> Router<PgPool> {
	handlers::router()
	// Future routes will be added here:
	// .route("/login", post(login))
	// .route("/logout", post(logout))
}

#[cfg(test)]
mod tests {
	use super::*;
	use axum::{
		http::StatusCode,
		response::{IntoResponse, Response},
	};
	use chrono::Utc;
	use sqlx::Error as SqlxError;
	use uuid::Uuid;

	#[test]
	fn router_is_mountable() {
		// Verify the router can be created and has correct type
		let _router: Router<PgPool> = router();
	}

	// ========================================
	// AuthError Tests (No DB Required)
	// ========================================

	#[test]
	fn auth_error_invalid_token_display() {
		let err = AuthError::InvalidToken;
		assert_eq!(format!("{err}"), "Invalid session token");
	}

	#[test]
	fn auth_error_expired_token_display() {
		let err = AuthError::ExpiredToken;
		assert_eq!(format!("{err}"), "Session expired");
	}

	#[test]
	fn auth_error_user_not_found_display() {
		let err = AuthError::UserNotFound;
		assert_eq!(format!("{err}"), "User not found");
	}

	#[test]
	fn auth_error_database_error_display() {
		let err = AuthError::DatabaseError(SqlxError::RowNotFound);
		assert!(format!("{err}").contains("Database error"));
	}

	#[test]
	fn auth_error_debug_impl() {
		let err = AuthError::InvalidToken;
		assert!(format!("{err:?}").contains("InvalidToken"));

		let err = AuthError::ExpiredToken;
		assert!(format!("{err:?}").contains("ExpiredToken"));

		let err = AuthError::UserNotFound;
		assert!(format!("{err:?}").contains("UserNotFound"));
	}

	// ========================================
	// IntoResponse Tests for AuthError (No DB Required)
	// ========================================

	#[test]
	fn auth_error_invalid_token_into_response() {
		let err = AuthError::InvalidToken;
		let response: Response = err.into_response();
		assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

		// Note: Body consumption requires async runtime, so we only check status
	}

	#[test]
	fn auth_error_expired_token_into_response() {
		let err = AuthError::ExpiredToken;
		let response: Response = err.into_response();
		assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
	}

	#[test]
	fn auth_error_user_not_found_into_response() {
		let err = AuthError::UserNotFound;
		let response: Response = err.into_response();
		assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
	}

	#[test]
	fn auth_error_database_error_into_response() {
		let err = AuthError::DatabaseError(SqlxError::RowNotFound);
		let response: Response = err.into_response();
		assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
	}

	// ========================================
	// AuthenticatedUser Tests (No DB Required)
	// ========================================

	#[test]
	fn authenticated_user_creation() {
		let user = User {
			id: Uuid::new_v4(),
			name: "Test User".to_string(),
			email: "test@example.com".to_string(),
			email_verified: true,
			image: None,
			created_at: Utc::now(),
			updated_at: Utc::now(),
		};

		let auth_user = AuthenticatedUser(user.clone());
		assert_eq!(auth_user.0.id, user.id);
		assert_eq!(auth_user.0.name, user.name);
		assert_eq!(auth_user.0.email, user.email);
	}

	#[test]
	fn authenticated_user_clone() {
		let user = User {
			id: Uuid::new_v4(),
			name: "Clone Test".to_string(),
			email: "clone@example.com".to_string(),
			email_verified: false,
			image: Some("https://example.com/avatar.png".to_string()),
			created_at: Utc::now(),
			updated_at: Utc::now(),
		};

		let auth_user = AuthenticatedUser(user);
		let cloned = auth_user.clone();

		assert_eq!(auth_user.0.id, cloned.0.id);
		assert_eq!(auth_user.0.email, cloned.0.email);
	}

	#[test]
	fn authenticated_user_debug() {
		let user = User {
			id: Uuid::new_v4(),
			name: "Debug Test".to_string(),
			email: "debug@example.com".to_string(),
			email_verified: true,
			image: None,
			created_at: Utc::now(),
			updated_at: Utc::now(),
		};

		let auth_user = AuthenticatedUser(user);
		let debug_str = format!("{auth_user:?}");

		assert!(debug_str.contains("AuthenticatedUser"));
		assert!(debug_str.contains("Debug Test"));
	}

	// ========================================
	// User Model Tests (No DB Required)
	// ========================================

	#[test]
	fn user_creation_all_fields() {
		let id = Uuid::new_v4();
		let now = Utc::now();

		let user = User {
			id,
			name: "Full User".to_string(),
			email: "full@example.com".to_string(),
			email_verified: true,
			image: Some("https://example.com/img.png".to_string()),
			created_at: now,
			updated_at: now,
		};

		assert_eq!(user.id, id);
		assert_eq!(user.name, "Full User");
		assert_eq!(user.email, "full@example.com");
		assert!(user.email_verified);
		assert!(user.image.is_some());
	}

	#[test]
	fn user_with_null_image() {
		let user = User {
			id: Uuid::new_v4(),
			name: "No Image".to_string(),
			email: "noimage@example.com".to_string(),
			email_verified: false,
			image: None,
			created_at: Utc::now(),
			updated_at: Utc::now(),
		};

		assert!(user.image.is_none());
	}

	#[test]
	fn user_clone() {
		let user = User {
			id: Uuid::new_v4(),
			name: "Clone User".to_string(),
			email: "clone@example.com".to_string(),
			email_verified: false,
			image: None,
			created_at: Utc::now(),
			updated_at: Utc::now(),
		};

		let cloned = user.clone();
		assert_eq!(user.id, cloned.id);
		assert_eq!(user.name, cloned.name);
	}

	#[test]
	fn user_serialization() {
		let user = User {
			id: Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap(),
			name: "Serialize Test".to_string(),
			email: "serialize@example.com".to_string(),
			email_verified: true,
			image: None,
			created_at: Utc::now(),
			updated_at: Utc::now(),
		};

		let json = serde_json::to_string(&user).unwrap();
		assert!(json.contains("Serialize Test"));
		assert!(json.contains("serialize@example.com"));
	}

	// ========================================
	// Session Model Tests (No DB Required)
	// ========================================

	#[test]
	fn session_creation_all_fields() {
		let session_id = Uuid::new_v4();
		let user_id = Uuid::new_v4();
		let now = Utc::now();
		let expires = now + chrono::Duration::hours(24);

		let session = Session {
			id: session_id,
			expires_at: expires,
			token: "test-token-123".to_string(),
			created_at: now,
			updated_at: now,
			ip_address: Some("192.168.1.1".to_string()),
			user_agent: Some("Mozilla/5.0".to_string()),
			user_id,
		};

		assert_eq!(session.id, session_id);
		assert_eq!(session.token, "test-token-123");
		assert_eq!(session.user_id, user_id);
		assert!(session.ip_address.is_some());
		assert!(session.user_agent.is_some());
	}

	#[test]
	fn session_with_null_optional_fields() {
		let session = Session {
			id: Uuid::new_v4(),
			expires_at: Utc::now() + chrono::Duration::hours(24),
			token: "minimal-token".to_string(),
			created_at: Utc::now(),
			updated_at: Utc::now(),
			ip_address: None,
			user_agent: None,
			user_id: Uuid::new_v4(),
		};

		assert!(session.ip_address.is_none());
		assert!(session.user_agent.is_none());
	}

	#[test]
	fn session_clone() {
		let session = Session {
			id: Uuid::new_v4(),
			expires_at: Utc::now() + chrono::Duration::hours(24),
			token: "clone-token".to_string(),
			created_at: Utc::now(),
			updated_at: Utc::now(),
			ip_address: None,
			user_agent: None,
			user_id: Uuid::new_v4(),
		};

		let cloned = session.clone();
		assert_eq!(session.id, cloned.id);
		assert_eq!(session.token, cloned.token);
	}

	#[test]
	fn session_debug_impl() {
		let session = Session {
			id: Uuid::new_v4(),
			expires_at: Utc::now() + chrono::Duration::hours(24),
			token: "debug-token".to_string(),
			created_at: Utc::now(),
			updated_at: Utc::now(),
			ip_address: Some("10.0.0.1".to_string()),
			user_agent: None,
			user_id: Uuid::new_v4(),
		};

		let debug_str = format!("{session:?}");
		assert!(debug_str.contains("Session"));
		assert!(debug_str.contains("debug-token"));
	}

	// ========================================
	// Integration Tests (Require DB - Marked as #[ignore])
	// ========================================

	/// Validates a real session token against the database.
	///
	/// **Database setup required:**
	/// - PostgreSQL with Better-Auth schema migrated
	/// - Test user inserted via Better-Auth or directly
	/// - Valid session token in `session` table
	///
	/// Run with: `cargo test -p server -- --ignored`
	///
	/// TODO: Implement in P3-007 when we add test database infrastructure
	#[test]
	#[ignore = "requires running database with Better-Auth schema - see P3-007"]
	fn valid_session_returns_user() {
		// See P3-007 for test database infrastructure
		unimplemented!("Requires test database - see P3-007")
	}

	/// Validates that an invalid session token returns InvalidToken error.
	///
	/// **Database setup required:**
	/// - PostgreSQL with Better-Auth schema
	///
	/// TODO: Implement in P3-007 when we add test database infrastructure
	#[test]
	#[ignore = "requires running database - see P3-007"]
	fn invalid_session_returns_error() {
		// See P3-007 for test database infrastructure
		unimplemented!("Requires test database - see P3-007")
	}

	/// Validates that a missing cookie returns appropriate error.
	///
	/// **Database setup required:**
	/// - PostgreSQL with Better-Auth schema
	/// - HTTP client or tower::Service for request mocking
	///
	/// TODO: Implement in P3-007 when we add test database infrastructure
	#[test]
	#[ignore = "requires running database and HTTP context - see P3-007"]
	fn missing_cookie_returns_error() {
		// See P3-007 for test database infrastructure
		unimplemented!("Requires test database and HTTP mocking - see P3-007")
	}

	/// Validates that an expired session returns ExpiredToken error.
	///
	/// **Database setup required:**
	/// - PostgreSQL with Better-Auth schema
	/// - Test user and session with `expires_at < NOW()`
	///
	/// TODO: Implement in P3-007 when we add test database infrastructure
	#[test]
	#[ignore = "requires running database - see P3-007"]
	fn expired_session_returns_error() {
		// See P3-007 for test database infrastructure
		unimplemented!("Requires test database - see P3-007")
	}
}
