//! Handlers submodule for core domain.
//!
//! Organizes handler functions by resource type:
//! - households: Household management
//! - initiatives: Initiative and goal tracking
//! - tags: Tag categorization
//! - relations: Cross-entity relationships

pub mod households;
pub mod initiatives;
pub mod relations;
pub mod tags;

/// Combine all core domain handlers into a single router.
///
/// This function is called by `core::router()` to mount
/// all core domain routes at `/core/*`.
#[allow(dead_code)] // Wired in Task 20
pub fn router() -> axum::Router<sqlx::PgPool> {
	axum::Router::new()
		.nest("/households", households::routes())
		.nest("/initiatives", initiatives::routes())
		.nest("/relations", relations::routes())
		.nest("/tags", tags::routes())
}
