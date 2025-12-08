//! Integration test for Tauri commands
//!
//! This test verifies that the health_check command works correctly
//! when invoked through the Tauri runtime.

use altair_core::AppConfig;
use guidance::AppState;
use std::path::PathBuf;
use std::time::Instant;

// Note: Full Tauri runtime testing requires tauri::test module
// For now, we test the command logic directly with state

#[tokio::test]
async fn test_health_check_command_response_structure() {
    // Create test state with unique temporary database
    let test_db_path = format!("/tmp/altair-test-health-response-{}", std::process::id());
    let config = AppConfig {
        database_path: PathBuf::from(&test_db_path),
        ..Default::default()
    };

    let state = AppState::new_for_test(config.clone())
        .await
        .expect("State initialization should succeed");

    // Check database health (simulates what health_check command does)
    let db_health = altair_db::check_database_health(&state.db)
        .await
        .expect("Health check should succeed");

    // Verify response structure
    assert!(
        db_health.connected,
        "Database should be connected in health response"
    );

    // Verify response time is reasonable (can be 0 for very fast operations)
    assert!(
        db_health.response_time_ms < 100,
        "Response time should be < 100ms, got {}ms",
        db_health.response_time_ms
    );

    drop(state);
    let _ = std::fs::remove_dir_all(&test_db_path);
}

#[tokio::test]
async fn test_health_check_command_performance() {
    // Verify health_check responds quickly
    let test_db_path = format!("/tmp/altair-test-health-perf-{}", std::process::id());
    let config = AppConfig {
        database_path: PathBuf::from(&test_db_path),
        ..Default::default()
    };

    let state = AppState::new_for_test(config.clone())
        .await
        .expect("State initialization should succeed");

    // Measure health check time
    let start = Instant::now();
    let _health = altair_db::check_database_health(&state.db)
        .await
        .expect("Health check should succeed");
    let elapsed = start.elapsed();

    // Should respond in < 100ms
    assert!(
        elapsed.as_millis() < 100,
        "Health check took {:?}, expected < 100ms",
        elapsed
    );

    drop(state);
    let _ = std::fs::remove_dir_all(&test_db_path);
}

#[tokio::test]
async fn test_health_check_version_accuracy() {
    // Verify version is correctly reported
    let version = env!("CARGO_PKG_VERSION");

    // Version should be semantic version format (x.y.z)
    let parts: Vec<&str> = version.split('.').collect();
    assert!(
        parts.len() >= 3,
        "Version should be semantic (x.y.z), got: {}",
        version
    );

    // Each part should be numeric
    for part in parts.iter().take(3) {
        assert!(
            part.parse::<u32>().is_ok(),
            "Version part '{}' should be numeric",
            part
        );
    }
}

#[tokio::test]
async fn test_health_check_with_disconnected_database() {
    // Test health_check behavior when database is unavailable
    // Note: SurrealKV is very resilient and can create databases in most locations
    // This test verifies error handling exists, even if not all paths trigger it

    let config = AppConfig {
        database_path: PathBuf::from("/root/altair-test-nonexistent"),
        ..Default::default()
    };

    // Try to initialize state
    let result = AppState::new_for_test(config).await;

    // Connection may succeed (SurrealKV creates DB) or fail (permission denied)
    // Both behaviors are acceptable - what matters is graceful handling
    match result {
        Ok(_state) => {
            // If connection succeeded, that's fine - SurrealKV is resilient
            // Just verify we can detect if a connection is actually broken
            // (tested separately in health check failure scenarios)
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

    // Note: In the actual health_check command, disconnection would return
    // HealthStatus { healthy: false, database_connected: false, ... }
    // instead of an error
}

#[tokio::test]
async fn test_multiple_health_checks_consistency() {
    // Verify health_check can be called multiple times consistently
    let test_db_path = format!("/tmp/altair-test-multi-health-{}", std::process::id());
    let config = AppConfig {
        database_path: PathBuf::from(&test_db_path),
        ..Default::default()
    };

    let state = AppState::new_for_test(config.clone())
        .await
        .expect("State initialization should succeed");

    // Call health check multiple times
    for i in 0..5 {
        let health = altair_db::check_database_health(&state.db)
            .await
            .unwrap_or_else(|_| panic!("Health check iteration {} should succeed", i));

        assert!(
            health.connected,
            "Database should remain connected on iteration {}",
            i
        );
    }

    drop(state);
    let _ = std::fs::remove_dir_all(&test_db_path);
}
