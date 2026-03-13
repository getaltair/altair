//! Knowledge domain module.
//!
//! Handles notes, backlinks, and graph relationships.

use axum::Router;
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/knowledge/*` in the main router.
pub fn router() -> Router<PgPool> {
	Router::new()
	// Future routes will be added here:
	// .route("/notes", get(list_notes))
	// .route("/notes/:id", get(get_note))
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
