//! Tracking domain module.
//!
//! Handles items, categories, locations, stock levels, and shopping lists.

use axum::Router;
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/tracking/*` in the main router.
pub fn router() -> Router<PgPool> {
	Router::new()
	// Future routes will be added here:
	// .route("/items", get(list_items))
	// .route("/locations", get(list_locations))
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
