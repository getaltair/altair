pub mod categories;
pub mod household;
pub mod item_events;
pub mod items;
pub mod locations;
pub mod shopping_list_items;
pub mod shopping_lists;

use axum::Router;

use crate::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(locations::router())
        .merge(categories::router())
        .merge(items::router())
        .merge(item_events::router())
        .merge(shopping_lists::router())
        .merge(shopping_list_items::router())
}
