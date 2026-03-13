//! Attachments domain module.
//!
//! Handles file metadata, upload handling, and attachment references.

use axum::Router;
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/attachments/*` in the main router.
pub fn router() -> Router<PgPool> {
	Router::new()
	// Future routes will be added here:
	// .route("/", post(upload_attachment))
	// .route("/:id", get(get_attachment))
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
