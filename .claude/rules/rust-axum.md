# Rust / Axum Conventions

Applies to: `apps/server/`

## Framework

- Rust with Axum web framework
- Tokio async runtime
- sqlx for database access (PostgreSQL)
- serde for serialization

## Project Structure

- `src/main.rs` — Entry point, router assembly
- `src/config.rs` — Configuration and environment
- `src/error.rs` — Centralized error types
- `src/contracts.rs` — Shared type contracts
- `src/db/` — Database connection and pool
- `src/auth/` — Authentication (JWT, middleware, handlers)
- `src/core/` — Core domain modules (knowledge, relations, tags, households, initiatives)
- `src/guidance/` — Guidance domain modules (quests, routines, epics, focus sessions, daily checkins)
- Each module follows: `mod.rs`, `models.rs`, `service.rs`, `handlers.rs`

## Module Pattern

Every domain module contains:
- `mod.rs` — Re-exports
- `models.rs` — Request/response types, domain structs (derive `Serialize`, `Deserialize`)
- `service.rs` — Business logic, database queries
- `handlers.rs` — Axum route handlers (thin; delegate to service)

## Error Handling

- Concrete error types via `thiserror` or manual `impl`; avoid `Box<dyn Error>`
- Handlers return `Result<impl IntoResponse, AppError>`
- Map database errors to appropriate HTTP status codes
- No `unwrap()` in production code; use `?`, `expect()` with message, or explicit match
- No `panic!()` or `unreachable!()` outside of tests

## Safety

- `unsafe` blocks require `// SAFETY:` documentation comments
- `clone()` on large types requires justification
- `.lock().unwrap()` on Mutex is forbidden; handle poisoned mutex
- No `std::process::exit()` outside of main

## Async

- No blocking calls inside `async` functions (no `std::thread::sleep`, no blocking I/O)
- `tokio::spawn` must have error handling on the `JoinHandle`
- Always `.await` futures; detect missing `.await`

## Formatting & Linting

- `cargo fmt` for formatting
- `cargo clippy` with all warnings as errors
- `cargo audit` for dependency vulnerability scanning

## Testing

- Unit tests in `#[cfg(test)]` modules within each file
- Integration tests in `tests/` directory
- Use `sqlx::test` for database-backed tests with transaction rollback
- `CARGO_TARGET_DIR=/tmp/cargo-target` to prevent worktree contamination
