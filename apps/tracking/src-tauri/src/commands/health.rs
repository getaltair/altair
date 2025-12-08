//! Health check command for Tracking app
//!
//! Provides the health_check command that returns system status including:
//! - Application version
//! - Database connection status
//! - Overall health indicator

use crate::state::AppState;
use altair_commands::HealthStatus;
use altair_db::check_database_health;

/// Check application health status
///
/// This command is used by the frontend to verify the backend is running
/// and all critical systems are operational. It checks:
/// - Database connectivity
/// - Response time
/// - Application version
///
/// # Returns
///
/// Always returns `Ok(HealthStatus)` with:
/// - `healthy`: Overall health indicator (true if all checks pass)
/// - `version`: Application version from Cargo.toml
/// - `database_connected`: Whether database is accessible
/// - `sync_enabled`: Whether sync engine is active (always false in this phase)
///
/// This command never fails - if health checks fail, it returns a status
/// indicating the failure rather than returning an error. The Result is
/// required by Tauri for async commands with state.
#[tauri::command]
#[specta::specta]
pub async fn health_check(state: tauri::State<'_, AppState>) -> Result<HealthStatus, String> {
    tracing::debug!("Health check requested");

    // Get application version
    let version = env!("CARGO_PKG_VERSION").to_string();

    // Check database health (handle errors gracefully)
    let db_health = match check_database_health(&state.db).await {
        Ok(health) => health,
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            // Return unhealthy status instead of erroring
            return Ok(HealthStatus {
                healthy: false,
                version,
                database_connected: false,
                sync_enabled: false,
            });
        }
    };

    // Determine overall health
    let healthy = db_health.connected;

    tracing::debug!(
        "Health check complete: healthy={}, db_connected={}, response_time_ms={}",
        healthy,
        db_health.connected,
        db_health.response_time_ms
    );

    Ok(HealthStatus {
        healthy,
        version,
        database_connected: db_health.connected,
        sync_enabled: false, // Not implemented in this phase
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::AppState;
    use altair_core::AppConfig;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_health_check_with_connected_db() {
        // Create test state with in-memory database
        let config = AppConfig {
            database_path: PathBuf::from("mem://"),
            ..Default::default()
        };

        let state = AppState::new_for_test(config)
            .await
            .expect("State initialization failed");

        // In tests, we directly check database health instead of calling the command
        // since creating Tauri State requires the full Tauri runtime
        let db_health = check_database_health(&state.db)
            .await
            .expect("Health check failed");

        // Verify health check works
        assert!(db_health.connected, "Database should be connected");
    }
}
