//! Guidance domain module.
//!
//! Handles epics, quests, routines, checkpoints, and focus sessions.

use crate::state::AppState;
use axum::{Router, routing::get};

/// Create the router for this module.
///
/// Routes are mounted at `/guidance/*` in the main router.
pub fn router() -> Router<AppState> {
	Router::new()
		.route("/quests", get(list_quests))
		.route("/routines", get(list_routines))
}

/// Placeholder: List all quests.
///
/// **NOT IMPLEMENTED** - This is a placeholder endpoint for future implementation.
///
/// Planned routes for guidance module:
/// - GET /guidance/quests - List quests (placeholder)
/// - GET /guidance/routines - List routines (placeholder)
/// - POST /guidance/quests - Create quest
/// - GET /guidance/quests/:id - Get quest details
/// - PATCH /guidance/quests/:id - Update quest
/// - DELETE /guidance/quests/:id - Delete quest
/// - POST /guidance/routines - Create routine
/// - GET /guidance/routines/:id - Get routine details
/// - PATCH /guidance/routines/:id - Update routine
/// - DELETE /guidance/routines/:id - Delete routine
#[utoipa::path(
	get,
	path = "/guidance/quests",
	tag = "Not Implemented",
	responses(
		(status = 501, description = "Not Implemented - This endpoint is a placeholder for future implementation")
	),
	description = "Placeholder endpoint - not yet implemented. This will list all quests in the future."
)]
pub async fn list_quests() -> &'static str {
	"Not Implemented"
}

/// Placeholder: List all routines.
///
/// **NOT IMPLEMENTED** - This is a placeholder endpoint for future implementation.
///
/// Planned routes for guidance module:
/// - GET /guidance/quests - List quests (placeholder)
/// - GET /guidance/routines - List routines (placeholder)
/// - POST /guidance/quests - Create quest
/// - GET /guidance/quests/:id - Get quest details
/// - PATCH /guidance/quests/:id - Update quest
/// - DELETE /guidance/quests/:id - Delete quest
/// - POST /guidance/routines - Create routine
/// - GET /guidance/routines/:id - Get routine details
/// - PATCH /guidance/routines/:id - Update routine
/// - DELETE /guidance/routines/:id - Delete routine
#[utoipa::path(
	get,
	path = "/guidance/routines",
	tag = "Not Implemented",
	responses(
		(status = 501, description = "Not Implemented - This endpoint is a placeholder for future implementation")
	),
	description = "Placeholder endpoint - not yet implemented. This will list all routines in the future."
)]
pub async fn list_routines() -> &'static str {
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
