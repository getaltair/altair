// Mobile app library - Unified Altair mobile experience

use tauri::Manager;

// Module declarations
mod commands;
mod state;

// Imports
use altair_core::AppConfig;
use commands::health_check;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Set up tauri-specta bindings generation
    let builder = tauri_specta::Builder::<tauri::Wry>::new()
        .commands(tauri_specta::collect_commands![health_check]);

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
