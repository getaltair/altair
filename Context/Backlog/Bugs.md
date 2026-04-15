# Bugs

Known bugs and regressions. Defer to GitHub Issues for tracked bugs.

| Bug | Severity | Component | Reported | Issue |
|-----|----------|-----------|----------|-------|

## `From<sqlx::Error>` Database-code branches have no direct unit tests

`apps/server/server/src/error.rs` implements `From<sqlx::Error>` with two `Database`-variant
branches: code `23505` → `Conflict(409)` and code `23503` → `BadRequest(400)`. Neither branch
has a unit test. Because `sqlx::Error::Database` cannot be constructed outside sqlx internals,
the cleanest approaches are:

1. Extract the code-matching logic into a pure function `fn pg_code_to_app_error(code: &str) -> Option<AppError>` that is unit-testable independently.
2. Add a `#[sqlx::test]` integration test that inserts a duplicate row to trigger `23505` and
   verifies the 409 response.

**Risk:** If the match arm order changes or the code string changes, production callers silently
get 500 on duplicate-key inserts instead of the expected 409, breaking the A-018 retry contract.

**Discovered in:** PR #8 review, finding P9-004.

---

## config::tests env var race with #[sqlx::test]

`config::tests` uses `std::env::remove_var("DATABASE_URL")` to test missing-env scenarios.
These tests are marked `#[serial]` which only prevents them from running concurrently with
OTHER `#[serial]` tests — not with `#[sqlx::test]` tests in other modules that run on
separate threads and need `DATABASE_URL` set.

**Workaround in CI:** `cargo test -- --test-threads=1`

**Proper fix:** Replace `#[serial_test::serial]` with `#[serial_test::exclusive]` on the
config tests that call `remove_var`. The `exclusive` attribute prevents ANY other test from
running concurrently while it holds the lock.
