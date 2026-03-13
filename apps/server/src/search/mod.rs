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
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn router_is_mountable() {
		let _router: Router<PgPool> = router();
	}
}
