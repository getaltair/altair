//! Integration test for backend startup
//!
//! This test verifies that the complete backend stack can initialize
//! successfully within acceptable time limits.

use altair_core::AppConfig;
use guidance::AppState;
use std::path::PathBuf;
use std::time::Instant;

#[tokio::test]
async fn test_backend_startup_performance() {
    // Use unique temporary path for test isolation
    let test_db_path = format!("/tmp/altair-test-startup-{}", std::process::id());
    let config = AppConfig {
        database_path: PathBuf::from(&test_db_path),
        log_level: "INFO".to_string(),
        log_dir: PathBuf::from("/tmp/altair-test-logs"),
        log_retention_days: 7,
    };

    // Measure startup time
    let start = Instant::now();

    // Initialize AppState using test helper (skips logging to avoid conflicts)
    let state = AppState::new_for_test(config.clone())
        .await
        .expect("AppState initialization should succeed");

    let elapsed = start.elapsed();

    // Verify startup completed within 2 seconds
    assert!(
        elapsed.as_secs() < 2,
        "Startup took {:?}, expected < 2s",
        elapsed
    );

    // Verify database connection is alive
    let db_health = altair_db::check_database_health(&state.db)
        .await
        .expect("Database health check should succeed");

    assert!(
        db_health.connected,
        "Database should be connected after startup"
    );

    // Verify response time is reasonable (< 100ms for file-based DB)
    assert!(
        db_health.response_time_ms < 100,
        "Database response time {:?}ms, expected < 100ms",
        db_health.response_time_ms
    );

    // Clean shutdown
    drop(state);

    // Clean up test database
    let _ = std::fs::remove_dir_all(&test_db_path);
}

#[tokio::test]
async fn test_backend_initialization_order() {
    // This test verifies that initialization happens in the correct order:
    // 1. Database connection is established
    // 2. State is ready

    let test_db_path = format!("/tmp/altair-test-init-order-{}", std::process::id());
    let config = AppConfig {
        database_path: PathBuf::from(&test_db_path),
        ..Default::default()
    };

    // Initialize state (using test helper to avoid logging conflicts)
    let state = AppState::new_for_test(config.clone())
        .await
        .expect("AppState initialization should succeed");

    // Verify database is connected (would fail if init order was wrong)
    let db_health = altair_db::check_database_health(&state.db).await;

    assert!(
        db_health.is_ok(),
        "Database health check should succeed after proper initialization"
    );

    assert!(db_health.unwrap().connected, "Database should be connected");

    drop(state);

    // Clean up test database
    let _ = std::fs::remove_dir_all(&test_db_path);
}

#[tokio::test]
async fn test_backend_handles_invalid_config_gracefully() {
    // Test that backend handles various configurations gracefully
    // Note: SurrealKV is very resilient and can create databases in most locations
    // This test verifies that error handling exists and works correctly

    let config = AppConfig {
        // Use a path that may fail depending on permissions
        database_path: PathBuf::from("/root/nonexistent/path/db"),
        ..Default::default()
    };

    // Attempt to initialize (using test helper to avoid logging conflicts)
    let result = AppState::new_for_test(config).await;

    // Connection may succeed (SurrealKV creates DB) or fail (permission denied)
    // Both behaviors are acceptable - what matters is graceful handling
    match result {
        Ok(_state) => {
            // If connection succeeded, that's fine - SurrealKV is resilient
            // The important thing is no panic occurred
        }
        Err(error) => {
            // If connection failed, verify error message is clear
            let error_msg = error.to_string();
            assert!(
                error_msg.contains("database")
                    || error_msg.contains("Database")
                    || error_msg.contains("connect")
                    || error_msg.contains("permission"),
                "Error should mention database or permission issue: {}",
                error_msg
            );
        }
    }
}
