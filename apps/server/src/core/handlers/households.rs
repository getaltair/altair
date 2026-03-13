//! Household handler module.
//!
//! Handles CRUD operations for household resources.
//!
//! Households represent shared spaces where family members collaborate
//! on goals, tasks, and shared resources.

use axum::Router;
use sqlx::PgPool;

/// Create routes for household operations.
///
/// Returns a router with all household endpoints.
#[allow(dead_code)] // Wired in Task 20
pub fn routes() -> Router<PgPool> {
	Router::new()
	// TODO: Add household routes in P3-005 Tasks 7-9
	// .route("/", post(create_household))
	// .route("/", get(list_households))
	// .route("/:id", get(get_household))
	// .route("/:id", patch(update_household))
	// .route("/:id", delete(delete_household))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn routes_is_mountable() {
		let _router: Router<PgPool> = routes();
	}
}
