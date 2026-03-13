//! Tag handler module.
//!
//! Handles CRUD operations for tag resources.
//!
//! Tags provide flexible categorization for initiatives, tasks,
//! and other domain entities for better organization and filtering.

use axum::Router;
use sqlx::PgPool;

/// Create routes for tag operations.
///
/// Returns a router with all tag endpoints.
#[allow(dead_code)] // Wired in Task 20
pub fn routes() -> Router<PgPool> {
	Router::new()
	// TODO: Add tag routes in P3-005 Tasks 15-19
	// .route("/", post(create_tag))
	// .route("/", get(list_tags))
	// .route("/:id", get(get_tag))
	// .route("/:id", patch(update_tag))
	// .route("/:id", delete(delete_tag))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn routes_is_mountable() {
		let _router: Router<PgPool> = routes();
	}
}
