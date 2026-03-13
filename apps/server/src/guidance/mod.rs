//! Guidance domain module.
//!
//! Handles epics, quests, routines, checkpoints, and focus sessions.

use axum::Router;
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/guidance/*` in the main router.
pub fn router() -> Router<PgPool> {
	Router::new()
	// Future routes will be added here:
	// .route("/quests", get(list_quests))
	// .route("/routines", get(list_routines))
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
