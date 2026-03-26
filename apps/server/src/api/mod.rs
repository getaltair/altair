mod health;

use axum::{
    Router,
    routing::{get, post, put},
};
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};

use crate::auth::handlers as auth_handlers;
use crate::core::households::handlers as household_handlers;
use crate::core::initiatives::handlers as initiative_handlers;
use crate::core::relations::handlers as relation_handlers;
use crate::core::tags::handlers as tag_handlers;
use crate::tracking::categories::handlers as category_handlers;
use crate::tracking::items::handlers as item_handlers;
use crate::tracking::locations::handlers as location_handlers;
use crate::tracking::shopping_lists::handlers as shopping_list_handlers;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub config: crate::config::Config,
}

impl axum::extract::FromRef<AppState> for sqlx::PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl axum::extract::FromRef<AppState> for crate::config::Config {
    fn from_ref(state: &AppState) -> Self {
        state.config.clone()
    }
}

/// Create and configure the main API router
///
/// Sets up middleware (CORS, tracing, compression) and registers all routes.
///
/// # Arguments
///
/// * `state` - Application state containing the database pool and configuration
///
/// # Middleware
///
/// - **CORS**: Currently allows all origins
/// - **Trace**: Request tracing for observability
/// - **Compression**: GZIP compression for responses
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Public routes
        .route("/health", get(health::health))
        .route("/auth/register", post(auth_handlers::register))
        .route("/auth/login", post(auth_handlers::login))
        // Protected routes (auth enforced via AuthenticatedUser extractor in each handler)
        .route("/auth/logout", post(auth_handlers::logout))
        .route(
            "/auth/me",
            get(auth_handlers::get_me).put(auth_handlers::update_me),
        )
        .route("/auth/powersync-token", post(auth_handlers::powersync_token))
        .route(
            "/core/households",
            post(household_handlers::create_household).get(household_handlers::list_households),
        )
        .route(
            "/core/households/{id}/members",
            post(household_handlers::invite_member),
        )
        // Initiative routes
        .route(
            "/core/initiatives",
            post(initiative_handlers::create_initiative)
                .get(initiative_handlers::list_initiatives),
        )
        .route(
            "/core/initiatives/{id}",
            get(initiative_handlers::get_initiative)
                .put(initiative_handlers::update_initiative)
                .delete(initiative_handlers::delete_initiative),
        )
        // Tag routes
        .route(
            "/core/tags",
            post(tag_handlers::create_tag).get(tag_handlers::list_tags),
        )
        .route(
            "/core/tags/{id}",
            put(tag_handlers::update_tag).delete(tag_handlers::delete_tag),
        )
        // Relation routes
        .route(
            "/core/relations",
            post(relation_handlers::create_relation)
                .get(relation_handlers::query_relations),
        )
        .route(
            "/core/relations/{id}",
            put(relation_handlers::update_relation_status),
        )
        // Tracking item routes
        .route(
            "/tracking/items/low-stock",
            get(item_handlers::list_low_stock_items),
        )
        .route(
            "/tracking/items",
            post(item_handlers::create_item).get(item_handlers::list_items),
        )
        .route(
            "/tracking/items/{id}",
            get(item_handlers::get_item)
                .put(item_handlers::update_item)
                .delete(item_handlers::delete_item),
        )
        .route(
            "/tracking/items/{id}/events",
            post(item_handlers::create_item_event)
                .get(item_handlers::list_item_events),
        )
        // Tracking location routes
        .route(
            "/tracking/locations",
            post(location_handlers::create_location)
                .get(location_handlers::list_locations),
        )
        .route(
            "/tracking/locations/{id}",
            get(location_handlers::get_location)
                .put(location_handlers::update_location)
                .delete(location_handlers::delete_location),
        )
        // Tracking category routes
        .route(
            "/tracking/categories",
            post(category_handlers::create_category)
                .get(category_handlers::list_categories),
        )
        .route(
            "/tracking/categories/{id}",
            get(category_handlers::get_category)
                .put(category_handlers::update_category)
                .delete(category_handlers::delete_category),
        )
        // Shopping list routes
        .route(
            "/tracking/shopping-lists",
            post(shopping_list_handlers::create_list)
                .get(shopping_list_handlers::list_lists),
        )
        .route(
            "/tracking/shopping-lists/{id}",
            get(shopping_list_handlers::get_list)
                .put(shopping_list_handlers::update_list)
                .delete(shopping_list_handlers::delete_list),
        )
        .route(
            "/tracking/shopping-lists/{id}/items",
            post(shopping_list_handlers::add_list_item)
                .get(shopping_list_handlers::list_list_items),
        )
        .route(
            "/tracking/shopping-lists/{id}/items/{item_id}",
            put(shopping_list_handlers::update_list_item)
                .delete(shopping_list_handlers::remove_list_item),
        )
        .route(
            "/tracking/shopping-lists/{id}/items/{item_id}/check",
            post(shopping_list_handlers::toggle_check),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
