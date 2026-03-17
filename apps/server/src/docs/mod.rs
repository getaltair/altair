//! OpenAPI documentation module.
//!
//! This module defines the OpenAPI specification for the Altair API,
//! which is auto-generated using the utoipa crate and its axum integration.

use axum::Router;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

#[allow(dead_code)]
struct AuthSecurityAddon;

impl Modify for AuthSecurityAddon {
	fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
		let mut components = openapi.components.take().unwrap_or_default();
		components.add_security_scheme(
			"better_auth_session",
			SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new(
				"better-auth.session_token",
			))),
		);
		openapi.components = Some(components);
	}
}

/// OpenAPI specification for the Altair API.
///
/// This struct implements `OpenApi` trait and contains
/// complete API documentation including paths, schemas, and metadata.
/// The paths and schemas sections will be populated by handlers
/// annotated with `#[utoipa::path]` and `#[utoipa::ToSchema]`.
#[derive(OpenApi)]
#[openapi(
	info(
		title = "Altair API",
		version = "0.1.0",
		description = "Personal OS API for managing knowledge, goals, and resources",
		contact(name = "Altair Project", email = "noreply@altair.dev")
	),
	paths(
		crate::handlers::health::health_check,
		crate::auth::handlers::me,
		crate::handlers::users::me,
		crate::core::handlers::households::list,
		crate::core::handlers::households::get_household,
		crate::core::handlers::households::create,
		crate::core::handlers::households::update,
		crate::core::handlers::households::delete_household,
		crate::core::handlers::households::list_memberships,
		crate::core::handlers::initiatives::list,
		crate::core::handlers::initiatives::get_initiative,
		crate::core::handlers::initiatives::create,
		crate::core::handlers::initiatives::update,
		crate::core::handlers::initiatives::delete_initiative,
		crate::core::handlers::tags::list,
		crate::core::handlers::tags::get_tag,
		crate::core::handlers::tags::create,
		crate::core::handlers::tags::update,
		crate::core::handlers::tags::delete_tag,
		crate::core::handlers::relations::list,
		crate::core::handlers::relations::get_single,
		crate::core::handlers::relations::create,
		crate::core::handlers::relations::delete_relation,
		crate::core::handlers::relations::update_status,
		crate::guidance::list_quests,
		crate::guidance::list_routines,
		crate::knowledge::list_notes,
		crate::tracking::list_items,
		crate::attachments::list_attachments,
		crate::sync::get_checkpoint,
		crate::search::search,
	),
		components(schemas(
		crate::handlers::users::AppUser,
		crate::auth::ErrorResponse,
		crate::auth::User,
		crate::core::handlers::households::Household,
		crate::core::handlers::households::HouseholdMember,
		crate::core::handlers::households::CreateHouseholdRequest,
		crate::core::handlers::households::UpdateHouseholdRequest,
		crate::core::handlers::households::HouseholdMembership,
		crate::core::handlers::initiatives::Initiative,
		crate::core::handlers::initiatives::CreateInitiativeRequest,
		crate::core::handlers::initiatives::UpdateInitiativeRequest,
		crate::core::handlers::tags::Tag,
		crate::core::handlers::tags::CreateTagRequest,
		crate::core::handlers::tags::UpdateTagRequest,
		crate::core::handlers::relations::EntityRelation,
		crate::core::handlers::relations::CreateRelationRequest,
		crate::core::handlers::relations::UpdateStatusRequest,
		crate::core::handlers::relations::ListRelationsQuery,
	),
	),
		tags(
		(name = "Health", description = "Health check and service status"),
		(name = "Auth", description = "Authentication and user identity"),
		(name = "Users", description = "User profile management"),
		(name = "Households", description = "Household and household membership management"),
		(name = "Initiatives", description = "Initiative and goal tracking"),
		(name = "Tags", description = "Tag categorization and management"),
		(name = "Relations", description = "Cross-entity relationships"),
		(name = "Not Implemented", description = "Placeholder endpoints for future implementation"),
		),
		modifiers(&AuthSecurityAddon)
	)]
#[allow(dead_code)]
pub struct ApiDoc;

/// Creates a router that serves API documentation UIs.
///
/// This function returns an Axum router that provides:
/// - Swagger UI at `/docs/swagger` (interactive API documentation)
/// - Scalar UI at `/docs/scalar` (alternative API documentation, when `scalar-ui` feature enabled)
/// - OpenAPI JSON spec at `/docs/openapi.json` (machine-readable spec)
pub fn router<S>() -> Router<S>
where
	S: Clone + Send + Sync + 'static,
{
	let router: Router<S> = SwaggerUi::new("/docs/swagger")
		.url("/docs/openapi.json", ApiDoc::openapi())
		.into();

	#[cfg(feature = "scalar-ui")]
	let router: Router<S> = {
		use utoipa_scalar::{Scalar, Servable};
		let scalar_router: Router<S> = Scalar::with_url("/docs/scalar", ApiDoc::openapi()).into();
		router.merge(scalar_router)
	};

	router
}
