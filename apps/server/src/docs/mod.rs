//! OpenAPI documentation module.
//!
//! This module defines the OpenAPI specification for the Altair API,
//! which is auto-generated using the utoipa crate and its axum integration.
//!
//! Note: The docs router is temporarily disabled due to type compatibility issues
//! between utoipa-swagger-ui's Router type and the main application's Router type.

use axum::Router;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};
use utoipa_swagger_ui::SwaggerUi;

/// Modifier struct to add authentication security scheme to OpenAPI spec.
///
/// This implements the `Modify` trait to customize the OpenAPI components
/// by adding a cookie-based API key security scheme for Better-Auth session tokens.
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
#[allow(dead_code)]
#[derive(OpenApi)]
#[openapi(
	info(
		title = "Altair API",
		version = "0.1.0",
		description = "Personal OS API for managing knowledge, goals, and resources",
		contact(name = "Altair Project", email = "noreply@altair.dev")
	),
	paths(
		rate::handlers::health::health_check,
		rate::auth::handlers::me,
		rate::handlers::users::me,
		rate::core::handlers::households::list,
		rate::core::handlers::households::get_household,
		rate::core::handlers::households::create,
		rate::core::handlers::households::update,
		rate::core::handlers::households::delete_household,
		rate::core::handlers::households::list_memberships,
		rate::core::handlers::initiatives::list,
		rate::core::handlers::initiatives::get_initiative,
		rate::core::handlers::initiatives::create,
		rate::core::handlers::initiatives::update,
		rate::core::handlers::initiatives::delete_initiative,
		rate::core::handlers::tags::list,
		rate::core::handlers::tags::get_tag,
		rate::core::handlers::tags::create,
		rate::core::handlers::tags::update,
		rate::core::handlers::tags::delete_tag,
		rate::core::handlers::relations::list,
		rate::core::handlers::relations::get_single,
		rate::core::handlers::relations::create,
		rate::core::handlers::relations::delete_relation,
		rate::core::handlers::relations::update_status,
		rate::guidance::list_quests,
		rate::guidance::list_routines,
		rate::knowledge::list_notes,
		rate::tracking::list_items,
		rate::attachments::list_attachments,
		rate::sync::get_checkpoint,
		rate::search::search,
	),
		components(schemas(
		crate::handlers::health::HealthResponse,
		crate::handlers::users::AppUser,
		crate::core::handlers::households::{Household, HouseholdMember},
		crate::core::handlers::households::CreateHouseholdRequest,
		crate::core::handlers::households::UpdateHouseholdRequest,
		crate::core::handlers::households::Household,
		crate::core::handlers::households::DeleteHouseholdRequest,
		crate::core::handlers::households::HouseholdMembership,
		crate::core::handlers::initiatives::{Initiative},
		crate::core::handlers::initiatives::CreateInitiativeRequest,
		crate::core::handlers::initiatives::UpdateInitiativeRequest,
		crate::core::handlers::tags::{Tag},
		crate::core::handlers::tags::CreateTagRequest,
		crate::core::handlers::tags::UpdateTagRequest,
		crate::core::handlers::relations::{EntityRelation},
		crate::core::handlers::relations::{CreateRelationRequest, UpdateStatusRequest},
		crate::core::handlers::relations::ListRelationsQuery,
		crate::core::handlers::relations::ErrorResponse,
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
pub struct ApiDoc;

/// Creates a router that serves Swagger UI and OpenAPI documentation.
///
/// This function returns an Axum router that provides:
/// - Swagger UI at `/docs/swagger` (interactive API documentation)
/// - OpenAPI JSON spec at `/docs/openapi.json` (machine-readable spec)
///
/// Note: Currently disabled due to type compatibility issues
/// between utoipa-swagger-ui's Router type and the main application's Router type.
#[allow(dead_code)]
pub fn router() -> Router {
	let router: Router = SwaggerUi::new("/docs/swagger")
		.url("/docs/openapi.json", ApiDoc::openapi())
		.into();

	// TODO: utoipa-scalar doesn't implement From<Scalar<OpenApi>> for Router
	// Re-enable when utoipa-scalar adds proper Axum Router support
	// #[cfg(feature = "scalar-ui")]
	// {
	// 	router = Scalar::with_url("/docs/scalar", ApiDoc::openapi()).into();
	// }

	router
}
