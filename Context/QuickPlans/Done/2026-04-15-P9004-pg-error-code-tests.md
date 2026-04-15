# Quick Plan: P9-004 — Tests for Database error-code branches

**Date:** 2026-04-15
**File:** `apps/server/server/src/error.rs`

---

## Task
Add unit tests for the `23505` → `Conflict(409)` and `23503` → `BadRequest(400)` branches
in `impl From<sqlx::Error> for AppError`.

## Goal
Ensure the PG error-code matching logic is regression-tested so that a future change to
match arm order or code strings fails a test rather than silently producing wrong HTTP status
codes on duplicate-key inserts.

## Approach

Extract the code-matching logic into a private pure function, then test that function directly.

`sqlx::Error::Database` cannot be constructed outside sqlx internals, so testing the `From`
impl end-to-end requires a live DB connection (`#[sqlx::test]`). Extracting to a pure function
avoids that dependency and makes the test deterministic.

### 1. Extract helper in `error.rs`

```rust
/// Maps a PostgreSQL error code to an AppError where a specific status code
/// is warranted. Returns None for unknown codes (caller falls through to Internal).
fn pg_code_to_app_error(code: &str) -> Option<AppError> {
    match code {
        "23505" => Some(AppError::Conflict("duplicate key".to_string())),
        "23503" => Some(AppError::BadRequest("foreign key constraint violation".to_string())),
        _ => None,
    }
}
```

### 2. Update `From<sqlx::Error>` to call it

Replace the inline match in the `Database` branch with a call to `pg_code_to_app_error`:

```rust
if let sqlx::Error::Database(ref db_err) = e {
    if let Some(app_err) = db_err.code().as_deref().and_then(pg_code_to_app_error) {
        return app_err;
    }
}
```

### 3. Add unit tests

Three tests in the existing `#[cfg(test)]` block:

- `pg_23505_maps_to_conflict` — asserts `Some(AppError::Conflict(_))`
- `pg_23503_maps_to_bad_request` — asserts `Some(AppError::BadRequest(_))`
- `pg_unknown_code_returns_none` — asserts `None` for an unrecognized code

No `#[tokio::test]` needed — pure function, synchronous.

## Verification

```bash
cargo test -p altair-server -- error::tests --test-threads=1
```

All existing tests continue to pass. Three new tests appear and pass.

## Risks

- None significant. This is a pure refactor of existing logic with no behavior change;
  the extracted function returns the same values the inline match did.
- `as_deref()` chaining: verify `db_err.code()` returns `Option<Cow<str>>` — confirmed
  from the current `as_deref()` call already in the code.
