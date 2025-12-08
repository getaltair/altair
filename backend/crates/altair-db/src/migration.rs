//! Database migration system for SurrealDB
//!
//! This module provides a migration runner that automatically discovers and applies
//! SurrealQL migration files from the `backend/migrations/` directory.
//!
//! ## Migration File Naming
//!
//! Migration files must follow the pattern: `NNN_description.surql`
//! - `NNN`: Zero-padded three-digit number (001, 002, etc.)
//! - `description`: Brief description with underscores
//! - Extension: `.surql`
//!
//! ## Example
//!
//! ```no_run
//! use altair_db::migration::MigrationRunner;
//! use surrealdb::engine::any;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let db = any::connect("mem://").await?;
//! db.use_ns("altair").use_db("main").await?;
//! let mut runner = MigrationRunner::new(db, "../migrations");
//! runner.run().await?;
//! # Ok(())
//! # }
//! ```

use altair_core::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use tracing::{debug, info};

/// Migration file metadata
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Migration {
    /// Migration version number (extracted from filename)
    pub version: i32,
    /// Full filename (e.g., "001_initial_schema.surql")
    pub name: String,
    /// Full path to the migration file
    pub path: PathBuf,
}

/// Migration record stored in the `_migrations` table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRecord {
    /// Migration version number
    pub version: i32,
    /// Migration filename
    pub name: String,
    /// Timestamp when the migration was applied
    pub applied_at: String,
}

/// Migration runner for SurrealDB
///
/// Discovers and applies `.surql` migration files in numerical order.
/// Tracks applied migrations in the `_migrations` table.
pub struct MigrationRunner {
    /// SurrealDB client
    db: Surreal<Any>,
    /// Path to migrations directory
    migrations_dir: PathBuf,
}

impl MigrationRunner {
    /// Create a new migration runner
    ///
    /// # Arguments
    ///
    /// * `db` - SurrealDB client instance
    /// * `migrations_dir` - Path to directory containing `.surql` migration files
    pub fn new<P: AsRef<Path>>(db: Surreal<Any>, migrations_dir: P) -> Self {
        Self {
            db,
            migrations_dir: migrations_dir.as_ref().to_path_buf(),
        }
    }

    /// Run all pending migrations
    ///
    /// This method:
    /// 1. Ensures the `_migrations` tracking table exists
    /// 2. Discovers all migration files
    /// 3. Checks which migrations have already been applied
    /// 4. Applies pending migrations in order
    /// 5. Records each successful migration
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Migration directory doesn't exist
    /// - Migration file cannot be read
    /// - SQL execution fails
    /// - Recording migration fails
    pub async fn run(&mut self) -> Result<()> {
        info!("Starting migration runner");

        // Ensure _migrations table exists
        self.ensure_migrations_table().await?;

        // Discover all migration files
        let migrations = self.discover_migrations()?;
        if migrations.is_empty() {
            info!("No migration files found in {:?}", self.migrations_dir);
            return Ok(());
        }

        info!("Found {} migration files", migrations.len());

        // Get already applied migrations
        let applied = self.get_applied_migrations().await?;
        let applied_versions: Vec<i32> = applied.iter().map(|m| m.version).collect();

        debug!("Applied migrations: {:?}", applied_versions);

        // Apply pending migrations
        for migration in migrations {
            if applied_versions.contains(&migration.version) {
                debug!("Skipping already applied migration: {}", migration.name);
                continue;
            }

            info!("Applying migration: {}", migration.name);
            self.apply_migration(&migration).await?;
        }

        info!("Migration run complete");
        Ok(())
    }

    /// Ensure the `_migrations` tracking table exists
    ///
    /// Creates the table if it doesn't exist. This method is idempotent.
    pub async fn ensure_migrations_table(&self) -> Result<()> {
        debug!("Ensuring _migrations table exists");

        let sql = r#"
            DEFINE TABLE IF NOT EXISTS _migrations SCHEMAFULL;
            DEFINE FIELD IF NOT EXISTS version ON TABLE _migrations TYPE int;
            DEFINE FIELD IF NOT EXISTS name ON TABLE _migrations TYPE string;
            DEFINE FIELD IF NOT EXISTS applied_at ON TABLE _migrations TYPE datetime DEFAULT time::now();
            DEFINE INDEX IF NOT EXISTS idx_migrations_version ON TABLE _migrations FIELDS version UNIQUE;
        "#;

        self.db
            .query(sql)
            .await
            .map_err(|e| Error::database(format!("Failed to create _migrations table: {}", e)))?;

        debug!("_migrations table ready");
        Ok(())
    }

    /// Discover all migration files in the migrations directory
    ///
    /// Returns a sorted vector of migrations (sorted by version number).
    ///
    /// # Migration File Pattern
    ///
    /// Files must match: `NNN_description.surql`
    /// - `NNN`: Three-digit zero-padded number (001-999)
    /// - `description`: Any valid filename characters
    /// - Extension: `.surql`
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Migrations directory doesn't exist
    /// - Directory cannot be read
    /// - Migration filename is invalid
    pub fn discover_migrations(&self) -> Result<Vec<Migration>> {
        debug!("Discovering migrations in {:?}", self.migrations_dir);

        if !self.migrations_dir.exists() {
            return Err(Error::database(format!(
                "Migrations directory does not exist: {:?}",
                self.migrations_dir
            )));
        }

        let mut migrations = Vec::new();

        let entries = std::fs::read_dir(&self.migrations_dir).map_err(|e| {
            Error::database(format!(
                "Failed to read migrations directory {:?}: {}",
                self.migrations_dir, e
            ))
        })?;

        for entry in entries {
            let entry = entry
                .map_err(|e| Error::database(format!("Failed to read directory entry: {}", e)))?;

            let path = entry.path();

            // Skip directories
            if path.is_dir() {
                continue;
            }

            // Check extension
            if path.extension().and_then(|s| s.to_str()) != Some("surql") {
                debug!("Skipping non-.surql file: {:?}", path);
                continue;
            }

            let filename = path
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| Error::database("Invalid filename"))?
                .to_string();

            // Parse version from filename (NNN_description.surql)
            let version = self.parse_version(&filename)?;

            migrations.push(Migration {
                version,
                name: filename,
                path,
            });
        }

        // Sort by version
        migrations.sort();

        debug!("Discovered {} migrations", migrations.len());
        Ok(migrations)
    }

    /// Parse version number from migration filename
    ///
    /// Expects format: `NNN_description.surql`
    fn parse_version(&self, filename: &str) -> Result<i32> {
        let parts: Vec<&str> = filename.splitn(2, '_').collect();
        if parts.len() < 2 {
            return Err(Error::database(format!(
                "Invalid migration filename '{}': must match pattern NNN_description.surql",
                filename
            )));
        }

        parts[0].parse::<i32>().map_err(|_| {
            Error::database(format!(
                "Invalid version number in filename '{}': expected NNN (001-999)",
                filename
            ))
        })
    }

    /// Get list of already applied migrations
    async fn get_applied_migrations(&self) -> Result<Vec<MigrationRecord>> {
        let sql = "SELECT version, name, applied_at FROM _migrations ORDER BY version";

        let mut response =
            self.db.query(sql).await.map_err(|e| {
                Error::database(format!("Failed to query _migrations table: {}", e))
            })?;

        let records: Vec<MigrationRecord> = response
            .take(0)
            .map_err(|e| Error::database(format!("Failed to parse migration records: {}", e)))?;

        Ok(records)
    }

    /// Apply a single migration
    ///
    /// Reads the migration file, executes the SQL, and records the migration.
    async fn apply_migration(&self, migration: &Migration) -> Result<()> {
        // Read migration file
        let sql = std::fs::read_to_string(&migration.path).map_err(|e| {
            Error::database(format!(
                "Failed to read migration file {:?}: {}",
                migration.path, e
            ))
        })?;

        // Execute migration SQL
        self.db.query(&sql).await.map_err(|e| {
            Error::database(format!(
                "Failed to execute migration {}: {}",
                migration.name, e
            ))
        })?;

        // Record migration
        let record_sql = format!(
            "INSERT INTO _migrations (version, name, applied_at) VALUES ({}, '{}', time::now())",
            migration.version, migration.name
        );

        self.db.query(&record_sql).await.map_err(|e| {
            Error::database(format!(
                "Failed to record migration {}: {}",
                migration.name, e
            ))
        })?;

        info!("Successfully applied migration: {}", migration.name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_migration_runner_creation() {
        let db = surrealdb::engine::any::connect("mem://").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        let runner = MigrationRunner::new(db, "../migrations");
        assert!(
            runner
                .migrations_dir
                .to_str()
                .unwrap()
                .contains("migrations")
        );
    }

    #[tokio::test]
    async fn test_ensure_migrations_table() {
        let db = surrealdb::engine::any::connect("mem://").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();

        let runner = MigrationRunner::new(db, "../migrations");
        let result = runner.ensure_migrations_table().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_parse_version() {
        let db = surrealdb::engine::any::connect("mem://").await.unwrap();
        db.use_ns("test").use_db("test").await.unwrap();
        let runner = MigrationRunner::new(db, "../migrations");

        assert_eq!(runner.parse_version("001_initial.surql").unwrap(), 1);
        assert_eq!(runner.parse_version("042_feature.surql").unwrap(), 42);
        assert_eq!(runner.parse_version("100_major.surql").unwrap(), 100);

        // Invalid filenames
        assert!(runner.parse_version("invalid.surql").is_err());
        assert!(runner.parse_version("abc_test.surql").is_err());
    }
}
