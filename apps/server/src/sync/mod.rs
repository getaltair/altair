//! Sync domain module.
//!
//! Handles mutation ingestion, conflict detection, and device checkpoints.
//! Placeholder for Phase 4 PowerSync integration.

use crate::state::AppState;
use axum::{Router, routing::get};

/// Create the router for this module.
///
/// Routes are mounted at `/sync/*` in the main router.
pub fn router() -> Router<AppState> {
	Router::new().route("/checkpoint", get(get_checkpoint))
}

/// Placeholder: Get sync checkpoint.
///
/// **NOT IMPLEMENTED** - This is a placeholder endpoint for future implementation.
///
/// Planned routes for sync module (Phase 4 PowerSync integration):
/// - GET /sync/checkpoint - Get sync checkpoint (placeholder)
/// - POST /sync/mutations - Submit mutations
/// - GET /sync/changes - Get changes since checkpoint
#[utoipa::path(
	get,
	path = "/sync/checkpoint",
	tag = "Not Implemented",
	responses(
		(status = 501, description = "Not Implemented - This endpoint is a placeholder for future implementation")
	),
	description = "Placeholder endpoint - not yet implemented. This will return PowerSync checkpoint data in the future."
)]
pub async fn get_checkpoint() -> &'static str {
	"Not Implemented"
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn router_is_mountable() {
		// Verify the router can be created and has correct type
		let _router: Router<AppState> = router();
	}
}
