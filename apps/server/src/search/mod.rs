//! Search domain module.
//!
//! Handles keyword indexing, semantic embeddings, and hybrid ranking.
//! Placeholder for search/indexing functionality.

use axum::Router;
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/search/*` in the main router.
pub fn router() -> Router<PgPool> {
	Router::new()
	// Future routes will be added here:
	// .route("/", get(search))
	// .route("/index", post(index_document))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn router_is_mountable() {
		let _router: Router<PgPool> = router();
	}
}
