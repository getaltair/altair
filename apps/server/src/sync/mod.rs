//! Sync domain module.
//!
//! Handles mutation ingestion, conflict detection, and device checkpoints.
//! Placeholder for Phase 4 PowerSync integration.

use axum::Router;
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/sync/*` in the main router.
pub fn router() -> Router<PgPool> {
	Router::new()
	// Future routes will be added here for Phase 4:
	// .route("/mutations", post(submit_mutation))
	// .route("/checkpoint", get(get_checkpoint))
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
