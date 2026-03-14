//! Knowledge domain module.
//!
//! Handles notes, backlinks, and graph relationships.

use axum::{Router, routing::get};
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/knowledge/*` in the main router.
pub fn router() -> Router<PgPool> {
	Router::new().route("/notes", get(list_notes))
}

/// Placeholder: List all notes.
///
/// **NOT IMPLEMENTED** - This is a placeholder endpoint for future implementation.
///
/// Planned routes for knowledge module:
/// - GET /knowledge/notes - List notes (placeholder)
/// - POST /knowledge/notes - Create note
/// - GET /knowledge/notes/:id - Get note details
/// - PATCH /knowledge/notes/:id - Update note
/// - DELETE /knowledge/notes/:id - Delete note
/// - GET /knowledge/notes/:id/backlinks - Get backlinks to note
/// - POST /knowledge/notes/:id/backlinks - Create backlink
#[utoipa::path(
	get,
	path = "/knowledge/notes",
	tag = "Not Implemented",
	responses(
		(status = 501, description = "Not Implemented - This endpoint is a placeholder for future implementation")
	),
	description = "Placeholder endpoint - not yet implemented. This will list all notes in the future."
)]
pub async fn list_notes() -> &'static str {
	"Not Implemented"
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
