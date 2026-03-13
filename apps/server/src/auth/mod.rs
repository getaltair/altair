//! Auth domain module.
//!
//! Handles authentication, identity, sessions, and authorization.

use axum::Router;
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/auth/*` in the main router.
pub fn router() -> Router<PgPool> {
	Router::new()
	// Future routes will be added here:
	// .route("/login", post(login))
	// .route("/logout", post(logout))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn router_is_mountable() {
		// Verify the router can be created and has correct type
		let _router: Router<PgPool> = router();
	}
}
