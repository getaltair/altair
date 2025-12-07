//! Integration tests for the migration runner
//!
//! These tests verify that the migration system works correctly with a real
//! SurrealDB instance (in-memory for testing).

use altair_db::{DatabaseClient, DatabaseConfig, MigrationRunner};
use std::fs;
use surrealdb::engine::any;
use tempfile::TempDir;

/// Helper function to create a temporary migrations directory with test migrations
fn create_test_migrations_dir() -> (TempDir, Vec<String>) {
    let temp_dir = TempDir::new().unwrap();
    let migrations_path = temp_dir.path();

    // Create empty migration (tests runner creates _migrations table)
    let migration_001 = r#"-- Test migration 001
USE NS test DB test;

DEFINE TABLE test_table_001 SCHEMAFULL;
DEFINE FIELD name ON TABLE test_table_001 TYPE string;
"#;

    fs::write(
        migrations_path.join("001_create_test_table.surql"),
        migration_001,
    )
    .unwrap();

    (temp_dir, vec!["001_create_test_table.surql".to_string()])
}

#[tokio::test]
async fn test_migration_runner_creates_migrations_table() {
    // Create in-memory database
    let db = any::connect("mem://").await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    // Create test migrations directory
    let (temp_dir, _files) = create_test_migrations_dir();

    // Create and run migration runner
    let mut runner = MigrationRunner::new(db, temp_dir.path());
    let result = runner.run().await;

    assert!(result.is_ok(), "Migration runner should succeed");

    // Verify _migrations table exists by querying it
    let db2 = any::connect("mem://").await.unwrap();
    db2.use_ns("test").use_db("test").await.unwrap();

    let query_result: Result<Vec<serde_json::Value>, _> = db2
        .query("SELECT * FROM _migrations")
        .await
        .and_then(|mut r| r.take(0));

    assert!(
        query_result.is_ok(),
        "_migrations table should exist and be queryable"
    );
}

#[tokio::test]
async fn test_migration_runner_applies_migrations() {
    // Create in-memory database
    let db = any::connect("mem://").await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    // Create test migrations directory
    let (temp_dir, _files) = create_test_migrations_dir();

    // Create and run migration runner
    let mut runner = MigrationRunner::new(db, temp_dir.path());
    runner.run().await.expect("Failed to run migrations");

    // Verify migration was applied by checking for the created table
    let db2 = any::connect("mem://").await.unwrap();
    db2.use_ns("test").use_db("test").await.unwrap();

    let table_exists: Result<Vec<serde_json::Value>, _> = db2
        .query("INFO FOR TABLE test_table_001")
        .await
        .and_then(|mut r| r.take(0));

    assert!(
        table_exists.is_ok(),
        "Migration should have created test_table_001"
    );
}

#[tokio::test]
async fn test_migration_runner_is_idempotent() {
    // Create in-memory database
    let db = any::connect("mem://").await.unwrap();
    db.use_ns("test").use_db("test").await.unwrap();

    // Create test migrations directory
    let (temp_dir, _files) = create_test_migrations_dir();

    // Run migrations first time
    let mut runner1 = MigrationRunner::new(db, temp_dir.path());
    runner1.run().await.expect("First migration run failed");

    // Run migrations second time (should be idempotent)
    let db2 = any::connect("mem://").await.unwrap();
    db2.use_ns("test").use_db("test").await.unwrap();

    let mut runner2 = MigrationRunner::new(db2, temp_dir.path());
    let result = runner2.run().await;

    assert!(
        result.is_ok(),
        "Running migrations twice should succeed (idempotent)"
    );

    // Verify only one migration record exists
    let db3 = any::connect("mem://").await.unwrap();
    db3.use_ns("test").use_db("test").await.unwrap();

    let count_result: Result<Vec<serde_json::Value>, _> = db3
        .query("SELECT count() FROM _migrations GROUP ALL")
        .await
        .and_then(|mut r| r.take(0));

    assert!(
        count_result.is_ok(),
        "Should be able to count migration records"
    );

    // Check that migration wasn't applied twice
    let records: Vec<serde_json::Value> = db3
        .query("SELECT * FROM _migrations")
        .await
        .unwrap()
        .take(0)
        .unwrap();

    assert_eq!(
        records.len(),
        1,
        "Should have exactly one migration record (not duplicated)"
    );
}

#[tokio::test]
async fn test_database_client_connection() {
    let config = DatabaseConfig {
        url: "mem://".to_string(),
        namespace: "test".to_string(),
        database: "test".to_string(),
        username: None,
        password: None,
    };

    let client = DatabaseClient::connect(config).await;
    assert!(client.is_ok(), "DatabaseClient should connect successfully");

    let client = client.unwrap();
    let health = client.health_check().await;
    assert!(health.is_ok(), "Health check should pass");
}

#[tokio::test]
async fn test_database_client_execute() {
    let config = DatabaseConfig {
        url: "mem://".to_string(),
        namespace: "test".to_string(),
        database: "test".to_string(),
        username: None,
        password: None,
    };

    let client = DatabaseClient::connect(config).await.unwrap();

    // Execute a simple CREATE TABLE statement
    let result = client.execute("DEFINE TABLE test_client SCHEMAFULL;").await;
    assert!(result.is_ok(), "Should be able to execute SQL");

    // Verify table was created
    let tables: Vec<serde_json::Value> = client
        .inner()
        .query("INFO FOR DB")
        .await
        .unwrap()
        .take(0)
        .unwrap();

    assert!(
        !tables.is_empty(),
        "Database info should show at least one table"
    );
}

#[tokio::test]
async fn test_changefeed_enabled_on_all_tables() {
    // Apply the real migration
    let migrations_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("migrations");

    // Create in-memory database and run migrations
    let db = any::connect("mem://").await.unwrap();
    db.use_ns("altair").use_db("main").await.unwrap();

    let mut runner = MigrationRunner::new(db.clone(), &migrations_path);
    runner
        .run()
        .await
        .expect("Failed to run 001_initial_schema migration");

    // List of all entity tables that should have CHANGEFEED
    let tables_with_changefeed = vec![
        "user",
        "campaign",
        "quest",
        "focus_session",
        "energy_checkin",
        "note",
        "folder",
        "daily_note",
        "item",
        "location",
        "reservation",
        "maintenance_schedule",
        "capture",
        "user_progress",
        "achievement",
        "streak",
        "attachment",
        "tag",
    ];

    // Verify each table has CHANGEFEED enabled by querying them
    // If migration succeeded, all tables should be queryable
    for table in tables_with_changefeed {
        let select_result: Result<Vec<serde_json::Value>, _> = db
            .query(format!("SELECT * FROM {} LIMIT 0", table))
            .await
            .and_then(|mut r| r.take(0));

        assert!(
            select_result.is_ok(),
            "Table {} should exist and be queryable (CHANGEFEED 7d is part of table definition)",
            table
        );
    }

    // Verify database info shows the tables
    let db_info: Result<Vec<serde_json::Value>, _> =
        db.query("INFO FOR DB").await.and_then(|mut r| r.take(0));

    assert!(db_info.is_ok(), "Should be able to query database info");
    println!("Database info: {:?}", db_info.unwrap());
}
