use axum::Router;

use crate::AppState;

pub mod daily_checkins;
pub mod epics;
pub mod focus_sessions;
pub mod quests;
pub mod routines;

/// Top-level router for the Guidance domain.
/// Sub-routers for quests, routines, epics, focus sessions, and daily check-ins
/// will be merged here as each module is implemented.
pub fn router() -> Router<AppState> {
    Router::new()
        .merge(epics::router())
        .merge(daily_checkins::router())
        .merge(quests::router())
        .merge(routines::router())
        .merge(focus_sessions::router())
}
