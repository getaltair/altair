---
paths:
  - "apps/worker/**"
  - "apps/server/**"
  - "**/*.rs"
---

# Rust Conventions

## Language
- Rust stable channel (latest)
- Edition 2024
- Clippy: `cargo clippy -- -D warnings` must pass
- Axum web framework for server

## Code Style
- Formatter: rustfmt (default configuration)
- `use` imports: grouped by std → external crates → internal modules
- Error types: `thiserror` for library errors, `anyhow` for application errors
- Naming: snake_case for functions/variables, PascalCase for types, SCREAMING_SNAKE for constants

## Axum Patterns
- Route handlers organized by domain (guidance, knowledge, tracking, relations, attachments)
- Use `State<T>` extractor for shared dependencies (DB pool, config)
- Layer extractors: Auth → Domain guards → Handler
- Return `Result<T, (StatusCode, Json)>` for API errors
- Use `axum::extract::Path` and `axum::extract::Query` for typed parameters

## Database (sqlx)
- **Server owns all database migrations** — migrations live in `apps/server/migrations/`
- Use sqlx for migrations and query building
- Migrations are ordered: `0001_initial.sql`, `0002_*.sql`, etc.
- Each migration is idempotent and reversible (include down migration)
- Run migrations via `sqlx database migrate run`
- Rollback via `sqlx database migrate revert`

## Patterns
- Prefer `?` operator over `unwrap()` in application code
- `unwrap()` acceptable in tests and after explicit validation
- Every `unsafe` block must have a `// SAFETY:` comment explaining the invariant
- `clone()` on large types needs a comment justifying it
- Prefer iterators over indexed loops
- Use `#[must_use]` on functions where ignoring the return value is likely a bug

## Async
- Tokio runtime for async operations
- Never block inside async functions
- `JoinHandle` errors must be handled
- Use `tokio::select!` for concurrent operations with cancellation

## Testing
- Unit tests: `#[cfg(test)]` module in each source file
- Integration tests: `tests/` directory at crate root
- Mock external dependencies — don't hit real APIs in tests

## Shared Contracts
- Use canonical entity types from `packages/contracts/generated/rust/`
- Use canonical relation types from `packages/contracts/generated/rust/`
- Never invent new shared identifiers inline
- See `docs/altair-shared-contracts-spec.md` for contract strategy
