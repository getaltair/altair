pub mod categories;
pub mod household;
pub mod item_events;
pub mod items;
pub mod locations;
pub mod shopping_list_items;
pub mod shopping_lists;

use axum::Router;
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;

/// Shared query parameter for handlers that scope a request by household.
///
/// Consolidated here to avoid duplication across tracking handler modules.
#[derive(Debug, Deserialize)]
pub struct HouseholdQuery {
    pub household_id: Uuid,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(locations::router())
        .merge(categories::router())
        .merge(items::router())
        .merge(item_events::router())
        .merge(shopping_lists::router())
        .merge(shopping_list_items::router())
}
