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
use crate::core::knowledge::handlers as knowledge_handlers;
use crate::core::tags::handlers as tag_handlers;

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
        // Knowledge note routes
        .route(
            "/knowledge/notes",
            post(knowledge_handlers::create_note).get(knowledge_handlers::list_notes),
        )
        .route(
            "/knowledge/notes/{id}",
            get(knowledge_handlers::get_note)
                .put(knowledge_handlers::update_note)
                .delete(knowledge_handlers::delete_note),
        )
        .route(
            "/knowledge/notes/{id}/snapshots",
            get(knowledge_handlers::list_snapshots)
                .post(knowledge_handlers::create_snapshot),
        )
        .route(
            "/knowledge/notes/{id}/relations",
            get(knowledge_handlers::get_note_relations),
        )
        .route(
            "/knowledge/notes/{id}/backlinks",
            get(knowledge_handlers::get_note_backlinks),
        )
        .route(
            "/knowledge/notes/{note_id}/tags/{tag_id}",
            post(knowledge_handlers::add_note_tag)
                .delete(knowledge_handlers::remove_note_tag),
        )
        .route(
            "/knowledge/notes/{note_id}/attachments/{attachment_id}",
            post(knowledge_handlers::add_note_attachment)
                .delete(knowledge_handlers::remove_note_attachment),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
