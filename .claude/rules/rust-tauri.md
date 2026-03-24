---
paths:
  - "apps/web/src-tauri/**"
  - "**/*.rs"
---

# Rust/Tauri Conventions

## Language
- Rust stable channel (latest)
- Edition 2021
- Clippy: `cargo clippy -- -D warnings` must pass

## Code Style
- Formatter: rustfmt (default configuration)
- `use` imports: grouped by std → external crates → internal modules
- Error types: `thiserror` for library errors, `anyhow` for application errors
- Naming: snake_case for functions/variables, PascalCase for types, SCREAMING_SNAKE for constants

## Patterns
- Prefer `?` operator over `unwrap()` in application code
- `unwrap()` acceptable in tests and after explicit validation
- Every `unsafe` block must have a `// SAFETY:` comment explaining the invariant
- `clone()` on large types needs a comment justifying it
- Prefer iterators over indexed loops
- Use `#[must_use]` on functions where ignoring the return value is likely a bug

## Tauri-Specific
- All `#[tauri::command]` handlers must validate inputs before processing
- Capability permissions: least-privilege — request only what's needed
- IPC: return `Result<T, String>` from commands for proper error propagation
- State: use `tauri::State<>` for shared state, not global statics
- Windows: scope file system access to app directories
- CSP: configure in `tauri.conf.json`, no `unsafe-inline` for scripts

## Async
- Tokio runtime for async operations
- Never block inside async functions
- `JoinHandle` errors must be handled
- Use `tokio::select!` for concurrent operations with cancellation

## Testing
- Unit tests: `#[cfg(test)]` module in each source file
- Integration tests: `tests/` directory at crate root
- Mock external dependencies — don't hit real APIs in tests
