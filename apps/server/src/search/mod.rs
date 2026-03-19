//! Search domain module.
//!
//! Handles keyword indexing, semantic embeddings, and hybrid ranking.
//! Placeholder for search/indexing functionality.

use crate::state::AppState;
use axum::{Router, routing::get};

/// Create the router for this module.
///
/// Routes are mounted at `/search/*` in the main router.
pub fn router() -> Router<AppState> {
	Router::new().route("/", get(search))
}

/// Placeholder: Search across all entities.
///
/// **NOT IMPLEMENTED** - This is a placeholder endpoint for future implementation.
///
/// Planned routes for search module:
/// - GET /search - Search across entities (placeholder)
/// - POST /search/index - Index document for search
/// - DELETE /search/index/:id - Remove document from index
#[utoipa::path(
	get,
	path = "/search",
	tag = "Not Implemented",
	responses(
		(status = 501, description = "Not Implemented - This endpoint is a placeholder for future implementation")
	),
	description = "Placeholder endpoint - not yet implemented. This will provide keyword and semantic search in the future."
)]
pub async fn search() -> &'static str {
	"Not Implemented"
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn router_is_mountable() {
		let _router: Router<AppState> = router();
	}
}
