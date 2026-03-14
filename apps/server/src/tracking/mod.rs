//! Tracking domain module.
//!
//! Handles items, categories, locations, stock levels, and shopping lists.

use axum::{Router, routing::get};
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/tracking/*` in the main router.
pub fn router() -> Router<PgPool> {
	Router::new().route("/items", get(list_items))
}

/// Placeholder: List all tracked items.
///
/// **NOT IMPLEMENTED** - This is a placeholder endpoint for future implementation.
///
/// Planned routes for tracking module:
/// - GET /tracking/items - List items (placeholder)
/// - POST /tracking/items - Create item
/// - GET /tracking/items/:id - Get item details
/// - PATCH /tracking/items/:id - Update item
/// - DELETE /tracking/items/:id - Delete item
/// - GET /tracking/categories - List categories
/// - POST /tracking/categories - Create category
/// - GET /tracking/locations - List locations
/// - POST /tracking/locations - Create location
#[utoipa::path(
	get,
	path = "/tracking/items",
	tag = "Not Implemented",
	responses(
		(status = 501, description = "Not Implemented - This endpoint is a placeholder for future implementation")
	),
	description = "Placeholder endpoint - not yet implemented. This will list all tracked items in the future."
)]
pub async fn list_items() -> &'static str {
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
