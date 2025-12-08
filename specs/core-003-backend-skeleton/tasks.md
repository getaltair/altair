# Tasks: core-003-backend-skeleton

**Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)
**Generated**: 2025-12-06

## Phase 1: Core Types and Configuration

**Goal**: Establish ApiError and AppConfig types in altair-core

- [x] **P1.1**: Create ApiError type for IPC error handling

  - **Acceptance**: File exists with complete type definition, compiles without errors, includes unit tests
  - **Files**: `backend/crates/altair-core/src/api_error.rs`
  - **Details**:
    - Define struct with `code: String`, `message: String`, `details: Option<serde_json::Value>`
    - Add `#[derive(Debug, Clone, serde::Serialize, specta::Type)]`
    - Implement `From<altair_core::Error>` for automatic conversion
    - Use SCREAMING_SNAKE_CASE for code strings (e.g., "DB_CONNECTION_FAILED", "CONFIG_LOAD_FAILED")
    - Add constructor methods: `new(code, message)`, `with_details(code, message, details)`
    - Write unit tests for serialization and conversion

- [x] **P1.2**: Create AppConfig type for application configuration

  - **Acceptance**: File exists with complete type definition, compiles, loads from TOML and defaults
  - **Files**: `backend/crates/altair-core/src/config.rs`
  - **Details**:
    - Define struct with `database_path: PathBuf`, `log_level: String`, `log_dir: PathBuf`, `log_retention_days: u8`
    - Add `#[derive(Debug, Clone, serde::Deserialize)]`
    - Implement `Default` with platform-aware paths using `directories` crate
    - Add `load_from_file(path: PathBuf) -> Result<Self>` method
    - Add `load_or_default() -> Self` that tries config file, falls back to defaults
    - Default log level: "INFO", retention: 7 days
    - Write unit tests for TOML parsing and default values

- [x] **P1.3**: Update altair-core dependencies

  - **Acceptance**: Cargo.toml updated, `cargo check -p altair-core` passes
  - **Files**: `backend/crates/altair-core/Cargo.toml`
  - **Details**:
    - Add `directories = "5.0"` for platform paths
    - Add `toml = "0.8"` for config parsing
    - Ensure `specta` with derive feature is present
    - Add `serde_json` if not already present

- [x] **P1.4**: Export new types from altair-core
  - **Acceptance**: Types available via `use altair_core::*`, `cargo test -p altair-core` passes
  - **Files**: `backend/crates/altair-core/src/lib.rs`
  - **Details**:
    - Add `pub mod api_error;` and `pub mod config;`
    - Re-export: `pub use api_error::ApiError;`
    - Re-export: `pub use config::AppConfig;`
    - Run `cargo test -p altair-core` to verify all tests pass

---

## Phase 2: Database Connection Layer

**Goal**: Implement real SurrealDB embedded connection in altair-db

- [x] **P2.1**: Enable SurrealDB dependency in altair-db

  - **Acceptance**: Cargo.toml updated, dependency resolution succeeds
  - **Files**: `backend/crates/altair-db/Cargo.toml`
  - **Details**:
    - Add `surrealdb = { version = "2.2", features = ["kv-surrealkv"] }`
    - Add `tokio = { version = "1.42", features = ["sync", "rt-multi-thread"] }`
    - Run `cargo check -p altair-db` to verify dependency resolution

- [x] **P2.2**: Create SurrealDB connection module

  - **Acceptance**: Connection module compiles, can connect to embedded DB, includes unit tests
  - **Files**: `backend/crates/altair-db/src/connection.rs`
  - **Details**:
    - Import `surrealdb::{Surreal, engine::local::Db, engine::local::SurrealKV}`
    - Define `SurrealConnection` struct wrapping `Surreal<Db>`
    - Implement `new(config: &DatabaseConfig) -> Result<Self>`
    - Connect using: `Surreal::new::<SurrealKV>(config.url).await?`
    - Call `db.use_ns(&config.namespace).use_db(&config.database).await?`
    - Implement `DatabaseClient` trait methods
    - Add `ping() -> Result<()>` method for health checks
    - Write unit tests with in-memory database

- [x] **P2.3**: Create database health check module

  - **Acceptance**: Health module compiles, returns accurate status, measures response time
  - **Files**: `backend/crates/altair-db/src/health.rs`
  - **Details**:
    - Define `DatabaseHealth` struct: `connected: bool`, `response_time_ms: u64`
    - Implement `check_database_health(conn: &SurrealConnection) -> Result<DatabaseHealth>`
    - Use `std::time::Instant` to measure ping response time
    - Return health struct with connection status and timing
    - Add error handling for disconnected state
    - Write unit tests for both connected and disconnected scenarios

- [x] **P2.4**: Update altair-db exports
  - **Acceptance**: Types exported, `cargo test -p altair-db` passes
  - **Files**: `backend/crates/altair-db/src/lib.rs`
  - **Details**:
    - Add `pub mod connection;` and `pub mod health;`
    - Re-export: `pub use connection::SurrealConnection;`
    - Re-export: `pub use health::{DatabaseHealth, check_database_health};`
    - Verify existing `DatabaseConfig` and `DatabaseClient` trait are still exported
    - Run full test suite

---

## Phase 3: Logging Infrastructure

**Goal**: Set up tracing with JSON output and log rotation

- [x] **P3.1**: Add logging dependencies to workspace

  - **Acceptance**: Dependencies added, workspace compiles
  - **Files**: `Cargo.toml` (workspace root)
  - **Details**:
    - Add to `[workspace.dependencies]`:
      - `tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }`
      - `tracing-appender = "0.2"`
    - Run `cargo check` from workspace root

- [x] **P3.2**: Create logging setup module

  - **Acceptance**: Logging initializes correctly, writes to file and console, rotates daily
  - **Files**: `backend/crates/altair-core/src/logging.rs`
  - **Details**:
    - Define `LogGuard` struct wrapping `tracing_appender::non_blocking::WorkerGuard`
    - Implement `init_logging(config: &AppConfig) -> Result<LogGuard>`
    - Set up file appender with `tracing_appender::rolling::daily(config.log_dir, "altair.log")`
    - Configure JSON formatter for file output
    - Configure human-readable formatter for console in debug builds
    - Use `EnvFilter` with configured log level (default INFO)
    - Return guard to prevent premature shutdown
    - Add documentation about guard lifetime requirements

- [x] **P3.3**: Export logging setup from altair-core
  - **Acceptance**: Logging module exported, integrated into lib.rs
  - **Files**: `backend/crates/altair-core/src/lib.rs`
  - **Details**:
    - Add `pub mod logging;`
    - Re-export: `pub use logging::{init_logging, LogGuard};`
    - Add altair-core dependency on workspace tracing-subscriber and tracing-appender
    - Update `backend/crates/altair-core/Cargo.toml`

---

## Phase 4: AppState and Command Infrastructure

**Goal**: Create AppState and health_check command in one app (guidance)

- [x] **P4.1**: Create AppState module for guidance app

  - **Acceptance**: AppState compiles, initializes DB and logging, includes error handling
  - **Files**: `apps/guidance/src-tauri/src/state.rs`
  - **Details**:
    - Define `AppState` struct: `db: SurrealConnection`, `config: AppConfig`, `_log_guard: LogGuard`
    - Implement `AppState::new(config: AppConfig) -> Result<Self>`
    - Initialize logging first: `let log_guard = init_logging(&config)?;`
    - Connect to database: `let db = SurrealConnection::new(&config.into())?;`
    - Log successful initialization with tracing
    - Return struct with all fields
    - Add Drop implementation to log shutdown

- [x] **P4.2**: Create health_check command module

  - **Acceptance**: Command compiles, returns correct health status, includes version
  - **Files**: `apps/guidance/src-tauri/src/commands/health.rs`, `apps/guidance/src-tauri/src/commands/mod.rs`
  - **Details**:
    - Create `commands/` directory and `mod.rs`
    - In `health.rs`:
      - Import `altair_core::ApiError`, `altair_commands::HealthStatus`
      - Define `#[tauri::command]` function
      - Add `#[specta::specta]` for TypeScript generation
      - Signature: `async fn health_check(state: tauri::State<'_, AppState>) -> Result<HealthStatus, String>`
      - Query database health to check database
      - Get version from `env!("CARGO_PKG_VERSION")`
      - Return `HealthStatus` with all fields populated
    - In `mod.rs`: `pub mod health;` and `pub use health::health_check;`

- [x] **P4.3**: Update guidance app lib.rs

  - **Acceptance**: App compiles, initializes state, registers command, exports bindings
  - **Files**: `apps/guidance/src-tauri/src/lib.rs`
  - **Details**:
    - Add module declarations: `mod state;` and `mod commands;`
    - Import `use state::AppState;` and `use commands::health_check;`
    - Update tauri_specta builder to include `health_check`
    - In `setup` closure:
      - Load config: `let config = AppConfig::load_or_default();`
      - Initialize state: `let state = AppState::new(config)?;`
      - Add to Tauri managed state: `app.manage(state);`
    - Verify bindings export path: `packages/bindings/src/guidance.ts`

- [x] **P4.4**: Update guidance Cargo.toml dependencies
  - **Acceptance**: All dependencies present, app builds successfully
  - **Files**: `apps/guidance/src-tauri/Cargo.toml`
  - **Details**:
    - Add `altair-db = { workspace = true }` (if not present)
    - Ensure `altair-core` has all needed features
    - Add workspace dependencies: `tracing`, `tokio = { workspace = true, features = ["rt-multi-thread"] }`
    - Run `cargo build -p guidance` to verify

---

## Phase 5: Replicate to All Apps

**Goal**: Copy pattern to knowledge, tracking, and mobile apps

- [x] **P5.1**: Replicate to knowledge app

  - **Acceptance**: Knowledge app builds, health_check registered, bindings exported
  - **Files**:
    - `apps/knowledge/src-tauri/src/state.rs`
    - `apps/knowledge/src-tauri/src/commands/health.rs`
    - `apps/knowledge/src-tauri/src/commands/mod.rs`
    - `apps/knowledge/src-tauri/src/lib.rs`
    - `apps/knowledge/src-tauri/Cargo.toml`
  - **Details**:
    - Copy `state.rs` from guidance (no changes needed)
    - Copy `commands/` directory structure
    - Update `lib.rs` following guidance pattern
    - Update bindings export path to `packages/bindings/src/knowledge.ts`
    - Add dependencies to Cargo.toml
    - Run `cargo build -p knowledge`

- [x] **P5.2**: Replicate to tracking app

  - **Acceptance**: Tracking app builds, health_check registered, bindings exported
  - **Files**:
    - `apps/tracking/src-tauri/src/state.rs`
    - `apps/tracking/src-tauri/src/commands/health.rs`
    - `apps/tracking/src-tauri/src/commands/mod.rs`
    - `apps/tracking/src-tauri/src/lib.rs`
    - `apps/tracking/src-tauri/Cargo.toml`
  - **Details**:
    - Copy `state.rs` from guidance (no changes needed)
    - Copy `commands/` directory structure
    - Update `lib.rs` following guidance pattern
    - Update bindings export path to `packages/bindings/src/tracking.ts`
    - Add dependencies to Cargo.toml
    - Run `cargo build -p tracking`

- [x] **P5.3**: Replicate to mobile app
  - **Acceptance**: Mobile app builds, health_check registered, bindings exported
  - **Files**:
    - `apps/mobile/src-tauri/src/state.rs`
    - `apps/mobile/src-tauri/src/commands/health.rs`
    - `apps/mobile/src-tauri/src/commands/mod.rs`
    - `apps/mobile/src-tauri/src/lib.rs`
    - `apps/mobile/src-tauri/Cargo.toml`
  - **Details**:
    - Copy `state.rs` from guidance (no changes needed)
    - Copy `commands/` directory structure
    - Update `lib.rs` following guidance pattern
    - Update bindings export path to `packages/bindings/src/mobile.ts`
    - Add dependencies to Cargo.toml
    - Run `cargo build -p mobile`

---

## Phase 6: TypeScript Binding Generation

**Goal**: Generate and validate TypeScript bindings

- [x] **P6.1**: Trigger TypeScript binding generation

  - **Acceptance**: All apps export bindings to packages/bindings/src/
  - **Files**: Generated: `packages/bindings/src/{guidance,knowledge,tracking,mobile}.ts`
  - **Details**:
    - Generated TypeScript bindings for all four apps
    - Each binding exports `healthCheck()` command function
    - Each binding exports `HealthStatus` type definition
    - All bindings have valid TypeScript syntax

- [x] **P6.2**: Update bindings package index.ts

  - **Acceptance**: Common types re-exported from index, TypeScript compiles
  - **Files**: `packages/bindings/src/index.ts`
  - **Details**:
    - Re-exported `HealthStatus` type from guidance
    - Organized exports by app (guidance, knowledge, tracking, mobile)
    - Added JSDoc comments for each app namespace

- [x] **P6.3**: Validate TypeScript compilation

  - **Acceptance**: Zero TypeScript errors, package builds successfully
  - **Files**: All files in `packages/bindings/src/`
  - **Details**:
    - Ran `pnpm --filter @altair/bindings typecheck` - passed
    - Ran `pnpm --filter @altair/bindings build` - succeeded
    - dist/ output created successfully

- [x] **P6.4**: Create example usage documentation
  - **Acceptance**: README updated with usage examples
  - **Files**: `packages/bindings/README.md`
  - **Details**:
    - Added health_check usage examples
    - Documented HealthStatus structure
    - Showed type-safe command invocation per app
    - Updated status to Phase 6 complete

---

## Phase 7: Integration Testing

**Goal**: Verify end-to-end functionality

- [ ] **P7.1**: Write backend startup integration test

  - **Acceptance**: Test passes, measures startup time < 2s
  - **Files**: `apps/guidance/src-tauri/tests/integration_test.rs`
  - **Details**:
    - Create integration test that initializes AppState
    - Measure time from config load to ready state
    - Assert startup time < 2000ms
    - Verify database connection succeeds
    - Verify logging is initialized
    - Clean up test database after run

- [ ] **P7.2**: Write health_check command integration test

  - **Acceptance**: Test invokes command, validates response
  - **Files**: `apps/guidance/src-tauri/tests/command_test.rs`
  - **Details**:
    - Set up Tauri test environment
    - Invoke `health_check` command
    - Assert response contains version
    - Assert database status is reported
    - Assert response time < 50ms
    - Verify response serializes to TypeScript types

- [ ] **P7.3**: Document test procedures and results
  - **Acceptance**: Testing documentation complete, all platforms verified
  - **Files**: `specs/core-003-backend-skeleton/test-results.md`
  - **Details**:
    - Document how to run tests locally
    - Document expected outputs
    - List all test scenarios and outcomes
    - Include CI/CD test results for Linux, macOS, Windows
    - Document any platform-specific issues encountered
    - Add troubleshooting section

---

## Task Summary

| Phase                        | Tasks  | Status       |
| ---------------------------- | ------ | ------------ |
| Phase 1: Core Types          | 4      | ✅ Completed |
| Phase 2: Database Connection | 4      | ✅ Completed |
| Phase 3: Logging             | 3      | ✅ Completed |
| Phase 4: AppState + Commands | 4      | ✅ Completed |
| Phase 5: Replicate to Apps   | 3      | ✅ Completed |
| Phase 6: TypeScript Bindings | 4      | ✅ Completed |
| Phase 7: Integration Testing | 3      | ⏳ Pending   |
| **Total**                    | **25** |              |

## Progress Tracking

Update this section as you complete tasks:

- **Completed**: 22/25 (88%)
- **In Progress**: 0/25
- **Blocked**: 0/25

## Next Action

Phase 6 complete! TypeScript bindings generated and validated for all four apps. All apps can now invoke `healthCheck()` with full type safety. Next: Begin Phase 7 with **P7.1**: Write backend startup integration test
