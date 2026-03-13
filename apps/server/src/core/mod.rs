//! Core domain module.
//!
//! Handles users, households, initiatives, tags, and relations.

pub mod handlers;

use axum::Router;
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/core/*` in the main router.
#[allow(dead_code)]
pub fn router() -> Router<PgPool> {
	Router::new()
	// Future routes will be added here:
	// .route("/users/me", get(get_current_user))
	// .route("/households", get(list_households))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn router_is_mountable() {
		let _router: Router<PgPool> = router();
	}
}
