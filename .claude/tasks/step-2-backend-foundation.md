# Plan: Step 2 - Backend Foundation (Axum + Postgres)

## Task Description

Build the foundational Rust Axum server with PostgreSQL database connectivity, migration framework, and health endpoint. This creates the skeleton that all future backend development builds upon — no domain logic yet, just infrastructure.

## Objective

Create a working Axum web server that:

- Connects to PostgreSQL via sqlx with compile-time checked queries
- Runs database migrations on startup
- Exposes a health endpoint that verifies DB connectivity
- Emits structured JSON logs via tracing
- Reads configuration from environment variables
- Can be built as a Docker container for deployment

## Problem Statement

The current `apps/server/` is a placeholder with only a `println!` in main.rs. Dependencies are already specified in Cargo.toml, but there is no:

- Server setup or router configuration
- Database connection pool
- Migration framework integration
- Configuration management
- Structured logging
- Health monitoring endpoint

## Solution Approach

Implement a modular Axum application with clear separation of concerns:

1. **Config Module**: Centralized configuration via `dotenvy` for environment variables (DATABASE_URL, PORT, RUST_LOG, etc.)
2. **Error Module**: Unified error type with thiserror for HTTP response mapping
3. **Telemetry Module**: Structured logging setup with tracing and tracing-subscriber
4. **DB Module**: sqlx connection pool with migration runner
5. **API Module**: Router composition with health endpoint
6. **Main Entry**: Server bootstrap that wires everything together

## Relevant Files

### Existing Files

- `apps/server/Cargo.toml` - Already contains all required dependencies
- `apps/server/src/main.rs` - Placeholder to be replaced
- `infra/compose/docker-compose.yml` - PostgreSQL service already configured
- `Cargo.toml` - Workspace definition

### New Files to Create

- `apps/server/src/config.rs` - Environment-based configuration
- `apps/server/src/error.rs` - Unified error type with HTTP response mapping
- `apps/server/src/telemetry.rs` - Structured logging setup
- `apps/server/src/db/mod.rs` - Database connection pool
- `apps/server/src/db/migrations/mod.rs` - Migration definitions
- `apps/server/src/api/mod.rs` - Router composition
- `apps/server/src/api/health.rs` - Health endpoint handler
- `apps/server/Dockerfile` - Multi-stage build for release binary
- `apps/server/.env.example` - Example environment configuration

### Files to Modify

- `apps/server/src/main.rs` - Replace placeholder with full server bootstrap

## Implementation Phases

### Phase 1: Foundation Modules

Create the core infrastructure modules that don't depend on each other:

- Config module for environment variable loading
- Error module with thiserror integration
- Telemetry module for logging setup

### Phase 2: Database Integration

Implement database connectivity:

- DB module with sqlx connection pool
- Migration framework setup with schema_version tracking
- Initial migration creating `_sqlx_migrations` table

### Phase 3: API Layer

Build the HTTP layer:

- Health endpoint that queries the database
- Router composition with middleware (CORS, tracing, compression)
- API module structure for future route additions

### Phase 4: Main Bootstrap & Docker

Wire everything together:

- Main.rs server setup with graceful shutdown
- Dockerfile for production builds
- Environment variable documentation

## Team Orchestration

### Team Members

- Specialist (Backend Builder)
  - Name: backend-foundation-builder
  - Role: Implement Axum server with Postgres connectivity
  - Agent Type: backend-engineer
  - Resume: false

- Quality Engineer (Validator)
  - Name: backend-validator
  - Role: Validate completed backend work against acceptance criteria (read-only inspection mode)
  - Agent Type: quality-engineer
  - Resume: false

## Step by Step Tasks

### 1. Create Error Module

- **Task ID**: create-error-module
- **Depends On**: none
- **Assigned To**: backend-foundation-builder
- **Agent Type**: backend-engineer
- **Parallel**: false

Create `apps/server/src/error.rs` with:

- `AppError` enum with variants for common error types (Database, Internal, NotFound, BadRequest, Unauthorized)
- Implement `std::error::Error` and `std::fmt::Display` for each variant
- Implement `axum::response::IntoResponse` trait to convert errors to HTTP responses with proper status codes
- Use `thiserror` for user-friendly error messages and backtraces
- Return JSON responses: `{"error": "message", "code": "error_code"}`

Example error mapping:

- DatabaseError → 500 Internal Server Error
- NotFound → 404 Not Found
- BadRequest → 400 Bad Request
- Unauthorized → 401 Unauthorized
- InternalError → 500 Internal Server Error with backtrace in dev mode

### 2. Create Config Module

- **Task ID**: create-config-module
- **Depends On**: none
- **Assigned To**: backend-foundation-builder
  - **Agent Type**: backend-engineer
- **Parallel**: true (can run with task 1)

Create `apps/server/src/config.rs` with:

- `Config` struct with fields: `database_url`, `port`, `log_level`, `environment`
- Load environment variables using `dotenvy`:
  - `DATABASE_URL` (required) - PostgreSQL connection string
  - `PORT` (default: 3000) - Server port
  - `RUST_LOG` (default: "info") - Log level
  - `APP_ENV` (default: "development") - Environment (development, production)
- Implement `Config::from_env()` or `Config::load()` function
- Parse port to `u16`
- Validate that required env vars are present, return error if missing
- Add `Config::database_url()` getter that returns `&str`

### 3. Create Telemetry Module

- **Task ID**: create-telemetry-module
- **Depends On**: none
- **Assigned To**: backend-foundation-builder
- **Agent Type**: backend-engineer
- **Parallel**: true (can run with tasks 1, 2)

Create `apps/server/src/telemetry.rs` with:

- Function `init_tracing(log_level: &str)` that sets up global tracing subscriber
- Use `tracing-subscriber` with:
  - `EnvFilter` layer controlled by `RUST_LOG` env var
  - `fmt::layer` with JSON format
  - Reloadable configuration support
- Initialize with sensible defaults: show timestamps, show source file location in debug
- Export function that can be called from main.rs before server starts

### 4. Create DB Module

- **Task ID**: create-db-module
- **Depends On**: create-error-module, create-config-module
- **Assigned To**: backend-foundation-builder
- **Agent Type**: backend-engineer
- **Parallel**: false

Create `apps/server/src/db/mod.rs` with:

- Connection pool using `sqlx::PgPool`
- `create_pool(config: &Config) -> Result<PgPool, AppError>` function
- Parse `DATABASE_URL` and create `PgPoolOptions` with:
  - Min/max connections (default 5-20)
  - Acquire timeout
  - Test connections on acquire
- Error handling: convert `sqlx::Error` to `AppError::Database`

Create `apps/server/src/db/migrations/mod.rs` with:

- `run_migrations(pool: &PgPool) -> Result<(), AppError>` function
- Use `sqlx::migrate!` macro for compile-time checked migrations
- Initial migration: `_sqlx_migrations` table if not exists (sqlx handles this automatically)
- Note: Actual migrations will be added in later steps; just setup the framework now

### 5. Create API Health Endpoint

- **Task ID**: create-health-endpoint
- **Depends On**: none
- **Assigned To**: backend-foundation-builder
- **Agent Type**: backend-engineer
- **Parallel**: true (can run with tasks 1, 2, 3)

Create `apps/server/src/api/health.rs` with:

- `health(pool: PgPool)` handler function
- Query database: `SELECT 1` or similar simple query to verify connectivity
- Return JSON: `{"status": "ok", "db": "connected", "timestamp": "..."}`
- Handle database errors by returning `"db": "disconnected"` with 503 status
- Use `axum::Json` for response serialization

### 6. Create API Router Module

- **Task ID**: create-api-router
- **Depends On**: create-health-endpoint
- **Assigned To**: backend-foundation-builder
- **Agent Type**: backend-engineer
- **Parallel**: false

Create `apps/server/src/api/mod.rs` with:

- `create_router(pool: PgPool) -> Router` function
- Register health endpoint at `GET /health`
- Set up middleware using `tower-http`:
  - CORS middleware (allow all for now, tighten in auth step)
  - Trace middleware for request tracing
  - Compression middleware (gzip)
- Use axum's `Router::new()` with `route` macro or builder pattern
- Return composed router

### 7. Implement Main Server Bootstrap

- **Task ID**: implement-main-bootstrap
- **Depends On**: create-config-module, create-telemetry-module, create-db-module, create-api-router
- **Assigned To**: backend-foundation-builder
- **Agent Type**: backend-engineer
- **Parallel**: false

Replace `apps/server/src/main.rs` with:

- Load environment variables and initialize `Config`
- Initialize telemetry/tracing
- Create database connection pool
- Run migrations on startup
- Create API router with DB pool
- Set up graceful shutdown signal handling (SIGTERM, SIGINT)
- Bind server to configured host/port
- Log server startup message with address
- Keep tokio runtime alive

Example main flow:

```rust
#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;
    telemetry::init(&config.log_level)?;
    let pool = db::create_pool(&config)?;
    db::run_migrations(&pool).await?;
    let app = api::create_router(pool);
    // ... bind and serve
}
```

### 8. Create Dockerfile

- **Task ID**: create-dockerfile
- **Depends On**: none
- **Assigned To**: backend-foundation-builder
- **Agent Type**: backend-engineer
- **Parallel**: true

Create `apps/server/Dockerfile` with:

- Multi-stage build (builder stage + runtime stage)
- Builder stage:
  - Use `rust:1.82-alpine` or similar base
  - Copy Cargo.toml and Cargo.lock
  - Build dependencies
  - Copy source and build release binary
- Runtime stage:
  - Use minimal base (e.g., `alpine:3.20` or `debian:bookworm-slim`)
  - Copy compiled binary from builder
  - Set up non-root user for security
  - Expose port 3000
  - Set `CMD ["/altair-server"]`

### 9. Create Environment Example

- **Task ID**: create-env-example
- **Depends On**: create-config-module
- **Assigned To**: backend-foundation-builder
- **Agent Type**: backend-engineer
- **Parallel**: true

Create `apps/server/.env.example` with:

- Commented examples for all required environment variables
- Database URL format for docker compose: `postgresql://altair:altair_dev@localhost:5432/altair`
- Default values for optional variables
- Usage instructions

Example content:

```
DATABASE_URL=postgresql://altair:altair_dev@localhost:5432/altair
PORT=3000
RUST_LOG=info
APP_ENV=development
```

### 10. Update Docker Compose for Server Service

- **Task ID**: update-docker-compose
- **Depends On**: create-dockerfile
- **Assigned To**: backend-foundation-builder
- **Agent Type**: backend-engineer
- **Parallel**: false

Update `infra/compose/docker-compose.yml` to add:

- Server service:
  - Build context: `./apps/server`
  - Depends on: `postgres`
  - Health condition: check `/health` endpoint
  - Environment variables for database connection
  - Ports: `3000:3000`
  - Restart policy: `unless-stopped`

Example service definition:

```yaml
server:
  build:
    context: ./apps/server
    dockerfile: Dockerfile
  ports:
    - "3000:3000"
  environment:
    DATABASE_URL: postgresql://altair:altair_dev@postgres:5432/altair
    RUST_LOG: info
    APP_ENV: development
  depends_on:
    postgres:
      condition: service_healthy
  healthcheck:
    test: ["CMD-SHELL", "wget -qO- http://localhost:3000/health || exit 1"]
    interval: 10s
    timeout: 5s
    retries: 5
  restart: unless-stopped
```

### 11. Final Validation

- **Task ID**: validate-all
- **Depends On**: create-error-module, create-config-module, create-telemetry-module, create-db-module, create-health-endpoint, create-api-router, implement-main-bootstrap, create-dockerfile, create-env-example, update-docker-compose
- **Assigned To**: backend-validator
- **Agent Type**: quality-engineer
- **Parallel**: false

Run validation commands and verify:

- `docker compose -f infra/compose/docker-compose.yml up -d` starts all services
- `curl http://localhost:3000/health` returns 200 with `{"status": "ok", "db": "connected", ...}`
- Server logs show structured JSON output
- Database migrations table exists in Postgres
- Docker container builds successfully: `docker compose build server`
- All rust code compiles: `cargo build -p server --release`
- Environment variables are properly documented

Operate in validation mode: inspect and report only, do not modify files.

## Acceptance Criteria

- [ ] `docker compose up` starts PostgreSQL and Axum server
- [ ] `GET /health` returns 200 with `{"status": "ok", "db": "connected", "timestamp": "..."}`
- [ ] sqlx migration framework runs on startup successfully
- [ ] Structured JSON logs emitted via tracing (visible in `docker compose logs`)
- [ ] Server reads configuration from environment variables
- [ ] Dockerfile builds a release binary successfully
- [ ] `.env.example` documents all required environment variables
- [ ] Server health fails gracefully if database is unavailable
- [ ] CORS, tracing, and gzip compression middleware is configured
- [ ] Graceful shutdown works on SIGTERM/SIGINT

## Validation Commands

```bash
# Build the server
cargo build -p server --release

# Start services
docker compose -f infra/compose/docker-compose.yml up -d

# Wait for healthy status
docker compose -f infra/compose/docker-compose.yml ps

# Test health endpoint
curl http://localhost:3000/health

# Check logs for structured JSON output
docker compose -f infra/compose/docker-compose.yml logs server

# Build Docker image
docker compose -f infra/compose/docker-compose.yml build server

# Verify database table exists (connect to pg and check)
docker compose -f infra/compose/docker-compose.yml exec postgres psql -U altair -c "\d altair \dt"

# Cleanup
docker compose -f infra/compose/docker-compose.yml down
```

## Notes

### Environment Variables Required

- `DATABASE_URL` - PostgreSQL connection string (e.g., `postgresql://user:pass@host:5432/dbname`)
- `PORT` - Server port (default: 3000)
- `RUST_LOG` - Log level (default: "info")
- `APP_ENV` - Environment identifier (default: "development")

### Dependency Notes

All required dependencies are already in `apps/server/Cargo.toml`:

- axum 0.8.8 with macros
- sqlx 0.8.6 with runtime-tokio, postgres, uuid, chrono, json, migrate
- tokio 1.50.0 with full features
- tower-http 0.6.8 with cors, trace, compression-gzip
- tracing 0.1.44 and tracing-subscriber 0.3.23 with env-filter, json
- uuid 1.22.0 with v4, serde
- chrono 0.4.44 with serde
- serde 1.0.228 with derive
- serde_json 1.0.149
- thiserror 2.0.18

### Docker Compose Notes

The postgres service is already configured in `infra/compose/docker-compose.yml`:

- Image: postgres:18-alpine
- Port: 5432:5432
- Database: altair, User: altair, Password: altair_dev
- Health check using pg_isready
- Named volume: pgdata for persistence

The server service needs to be added to depend on postgres and connect using the credentials from the environment.
