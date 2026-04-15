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
- Map `sqlx::Error` via `anyhow::Error::from(e)`, **never** `.to_string()` — `.to_string()` flattens the error chain and leaks schema detail (table/constraint names) into logs. Prefer `impl From<sqlx::Error> for AppError` to eliminate boilerplate and preserve the full error chain.
- **Never** write `.map_err(|e| AppError::Internal(anyhow::Error::from(e)))` inline. Add `impl From<sqlx::Error> for AppError` once per crate (in `error.rs`) and use `?` directly. The inline form appeared ~40 times in the sync module — it is a maintenance hazard and obscures intent.
- **Never** write `let _ = tx.rollback().await` to discard rollback errors. A failed rollback leaves the connection in an ambiguous state and the error disappears silently. Always log rollback failures: `if let Err(rb_err) = tx.rollback().await { tracing::warn!("rollback failed: {:?}", rb_err); }`. The `let _` form is banned — treat it as a bug on sight.

## Safety

- `unsafe` blocks require `// SAFETY:` documentation comments
- `clone()` on large types requires justification
- `.lock().unwrap()` on Mutex is forbidden; handle poisoned mutex
- No `std::process::exit()` outside of main

## Async

- No blocking calls inside `async` functions (no `std::thread::sleep`, no blocking I/O)
- CPU-intensive synchronous operations (e.g. Argon2id hashing/verification) must be wrapped in `tokio::task::spawn_blocking` to avoid blocking a Tokio worker thread
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
