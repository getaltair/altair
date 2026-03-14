//! Attachments domain module.
//!
//! Handles file metadata, upload handling, and attachment references.

use axum::{Router, routing::get};
use sqlx::PgPool;

/// Create the router for this module.
///
/// Routes are mounted at `/attachments/*` in the main router.
pub fn router() -> Router<PgPool> {
	Router::new().route("/", get(list_attachments))
}

/// Placeholder: List all attachments.
///
/// **NOT IMPLEMENTED** - This is a placeholder endpoint for future implementation.
///
/// Planned routes for attachments module:
/// - GET /attachments - List attachments (placeholder)
/// - POST /attachments - Upload attachment
/// - GET /attachments/:id - Get attachment metadata
/// - GET /attachments/:id/download - Download attachment file
/// - DELETE /attachments/:id - Delete attachment
#[utoipa::path(
	get,
	path = "/attachments",
	tag = "Not Implemented",
	responses(
		(status = 501, description = "Not Implemented - This endpoint is a placeholder for future implementation")
	),
	description = "Placeholder endpoint - not yet implemented. This will list all attachments in the future."
)]
pub async fn list_attachments() -> &'static str {
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
