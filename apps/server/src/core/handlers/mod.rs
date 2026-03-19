//! Handlers submodule for core domain.
//!
//! Organizes handler functions by resource type:
//! - households: Household management
//! - initiatives: Initiative and goal tracking
//! - tags: Tag categorization
//! - relations: Cross-domain entity relationships
//!
pub mod households;
pub mod initiatives;
pub mod relations;
pub mod tags;

use crate::state::AppState;
use axum::Router;

/// Combine all core domain handlers into a single router.
///
/// This function is called by `core::router()` to mount
/// all core domain routes at `/core/*`.
#[allow(dead_code)] // Wired in Task 20
pub fn router() -> Router<AppState> {
	axum::Router::new()
		.nest("/households", households::routes())
		.nest("/initiatives", initiatives::routes())
		.nest("/relations", relations::routes())
		.nest("/tags", tags::routes())
}
