// Mobile app library - Unified Altair mobile experience

use tauri::Manager;

// Module declarations
mod commands;
mod state;

// Imports
use altair_core::AppConfig;
use commands::health_check;
use state::AppState;

// Domain type imports for bindings generation (all types for unified mobile experience)
#[allow(unused_imports)]
use altair_db::schema::{
    capture::Capture,
    enums::{
        CaptureSource, CaptureStatus, CaptureType, EnergyCost, EntityStatus, FocusSessionStatus,
        ItemStatus, MediaType, QuestColumn, ReservationStatus, StreakType, UserRole,
    },
    gamification::{Achievement, Streak, UserProgress},
    item::{GeoPoint, Item, Location, MaintenanceSchedule, Reservation},
    note::{DailyNote, Folder, Note},
    quest::{Campaign, EnergyCheckIn, FocusSession, Quest},
    shared::{Attachment, Tag, User, UserPreferences},
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Set up tauri-specta bindings generation
    let builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![health_check])
        // Quest domain types
        .typ::<Quest>()
        .typ::<Campaign>()
        .typ::<FocusSession>()
        .typ::<EnergyCheckIn>()
        // Knowledge domain types
        .typ::<Note>()
        .typ::<Folder>()
        .typ::<DailyNote>()
        // Tracking domain types
        .typ::<Item>()
        .typ::<Location>()
        .typ::<GeoPoint>()
        .typ::<Reservation>()
        .typ::<MaintenanceSchedule>()
        // Gamification types
        .typ::<UserProgress>()
        .typ::<Achievement>()
        .typ::<Streak>()
        // Shared types
        .typ::<User>()
        .typ::<UserPreferences>()
        .typ::<Attachment>()
        .typ::<Tag>()
        .typ::<Capture>()
        // Enums
        .typ::<QuestColumn>()
        .typ::<EnergyCost>()
        .typ::<FocusSessionStatus>()
        .typ::<ItemStatus>()
        .typ::<ReservationStatus>()
        .typ::<EntityStatus>()
        .typ::<CaptureStatus>()
        .typ::<CaptureType>()
        .typ::<CaptureSource>()
        .typ::<StreakType>()
        .typ::<MediaType>()
        .typ::<UserRole>();

    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default(),
            "../../../packages/bindings/src/mobile.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            // Initialize application state
            let config = AppConfig::load_or_default();

            // Create runtime for async state initialization
            let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

            let state = runtime
                .block_on(AppState::new(config))
                .expect("Failed to initialize application state");

            // Add state to Tauri's managed state
            app.manage(state);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
