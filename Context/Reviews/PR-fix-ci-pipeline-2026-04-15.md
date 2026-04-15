# PR Review: fix+ci-pipeline → main

**Date:** 2026-04-15
**Feature:** N/A (CI infrastructure fix)
**Branch:** worktree-fix+ci-pipeline
**PR:** #8 — fix(ci): resolve six pipeline failures
**Reviewers:** code-reviewer, pr-test-analyzer, silent-failure-hunter
**Status:** ✅ Resolved

## Summary

6 findings across this CI fix PR: 5 [FIX] (2 critical/high rollback loop issues, 1 high
invalid seed hash, 2 medium hardening items) and 1 [TASK] (missing error-code branch tests
in error.rs). All findings are in CI workflow, infra scripts, or server test code.

## Findings

### Fix-Now

#### [FIX] P9-001: Rollback loop has no iteration cap — can hang forever
- **File:** `.github/workflows/ci.yml:239-245`
- **Severity:** Critical
- **Detail:** The `while true` loop breaks only on a matched stdout string. If sqlx-cli
  emits output that matches neither "no migrations available to revert" nor "error" (e.g.,
  future sqlx version rewording the terminal message, transient connectivity noise), the
  loop burns the full 6-hour Actions timeout with no diagnostic output. There are 29
  migrations; a hard cap of 35 would guarantee termination. Additionally, command
  substitution `result=$(...)` always exits 0 even if the inner command fails — the loop
  should check the process exit code, not just stdout text.
- **Fix:** Replace `while true` with an iteration-capped loop that checks exit status:
  ```bash
  REVERTS=0; MAX_REVERTS=35
  while true; do
    [ "$REVERTS" -ge "$MAX_REVERTS" ] && { echo "FAIL: exceeded $MAX_REVERTS iterations"; exit 1; }
    output=$(DATABASE_URL=... sqlx migrate revert --source infra/migrations 2>&1); STATUS=$?
    echo "$output"; REVERTS=$((REVERTS+1))
    echo "$output" | grep -qi "no migrations available to revert" && break
    [ "$STATUS" -ne 0 ] && exit 1
  done
  ```
- **Status:** ✅ Fixed
- **Resolution:** Replaced `while true` with an iteration-capped loop (max 35). Checks `$STATUS` (exit code) instead of grepping for "error" to avoid false positives. Added inline comment explaining both decisions.

#### [FIX] P9-002: `grep -qi "error"` fires on migration names, not just real errors
- **File:** `.github/workflows/ci.yml:244`
- **Severity:** High
- **Detail:** The pattern `grep -qi "error"` will match any output containing the
  substring "error" case-insensitively — including migration names (e.g.,
  `add_error_codes_table`), sqlx diagnostic lines, or future CLI output. This is a latent
  false-positive that would cause a rollback step to exit 1 on a successful revert. This
  finding is addressed by the same fix as P9-001 (check exit code instead of stdout text).
- **Status:** ✅ Fixed
- **Resolution:** Resolved together with P9-001 — exit-code check replaces the `grep -qi "error"` pattern entirely.

#### [FIX] P9-003: Placeholder argon2id hash in seed.sql is syntactically invalid
- **File:** `infra/scripts/seed.sql:9`
- **Severity:** High
- **Detail:** The hash segment `placeholder-ci-only-not-a-real-hash` contains hyphens,
  which are not valid base64url characters. `PasswordHash::new()` in the auth service will
  fail to parse it, producing HTTP 500 and a `tracing::error` "possible DB corruption or
  migration bug" log on any local login attempt. CI passes because the smoke test never
  calls the auth endpoint. The seed file states it is for "local development and CI smoke
  tests", but the local dev use case is broken with a misleading error.
- **Fix:** Either (a) generate a real argon2id hash for a known CI-only password and
  document the credentials in a comment, or (b) add an explicit comment that this user
  cannot authenticate and is seeded for smoke-test purposes only.
- **Status:** ✅ Fixed
- **Resolution:** Added a block comment at the top of seed.sql (option b) explaining the hash is intentionally unauthentable and warning developers not to attempt login with this user.

### Missing Tasks

#### [TASK] P9-004: `Database` error-code branches in `From<sqlx::Error>` have no unit tests
- **File:** `apps/server/server/src/error.rs:37-46`
- **Severity:** High
- **Detail:** PR #8 expanded error.rs test coverage by adding tests for RowNotFound → 404
  and catch-all → 500. But the two `Database`-variant branches — code 23505 → Conflict
  (409) and code 23503 → BadRequest (400) — have no test exercising the `From<sqlx::Error>`
  impl directly. If the match arm order changes or the code string changes, production
  callers get 500s on duplicate-key inserts instead of 409s, silently breaking the retry
  contract. Because `sqlx::Error::Database` cannot be constructed outside sqlx internals,
  the cleanest fix is factoring the code-matching logic into a pure function
  (`fn pg_code_to_app_error(code: &str) -> Option<AppError>`) that is unit-testable, or
  adding a `#[sqlx::test]` integration test that actually inserts a duplicate row.
- **Relates to:** rust-axum.md convention — `impl From<sqlx::Error> for AppError`
- **Status:** ✅ Task created
- **Resolution:** Added to `Context/Backlog/Bugs.md` with both fix options documented (extract pure function vs. `#[sqlx::test]` integration test).

### Architectural Concerns

*(none)*

### Convention Gaps

*(none)*

#### [FIX] P9-005: JWT key generation relies on implicit pipefail — add explicit guard
- **File:** `.github/workflows/ci.yml:222`
- **Severity:** Medium
- **Detail:** `JWT_KEY=$(openssl genrsa 2048 | base64 -w0)` is protected by `set -eo pipefail`
  today, but this is invisible to readers and silently lost if the step is extracted to a
  script or `shell:` is overridden. A failed keygen would produce an empty `JWT_KEY`, the
  server might still start (health check passes), and auth failures would appear as a
  confusing downstream error rather than pointing at key generation.
- **Fix:** Add an explicit guard after the assignment:
  ```bash
  JWT_KEY=$(openssl genrsa 2048 | base64 -w0)
  [ -z "$JWT_KEY" ] && { echo "FAIL: JWT key generation failed"; exit 1; }
  ```
- **Status:** ✅ Fixed
- **Resolution:** Added the explicit empty-string guard on the line immediately after JWT_KEY assignment.

#### [FIX] P9-006: sqlx-cli installed without version pin
- **File:** `.github/workflows/ci.yml:58, 172`
- **Severity:** Medium
- **Detail:** Both the `rust` and `smoke-test` jobs install sqlx-cli with
  `cargo install sqlx-cli --no-default-features --features postgres` and no `--version`.
  The rollback loop's break condition matches against the specific string "no migrations
  available to revert" — an implementation detail of the CLI, not a stable API. A patch
  release that rewords this output would silently break loop termination, causing the
  infinite-hang scenario described in P9-001.
- **Fix:** Pin to the current known-good version:
  ```bash
  cargo install sqlx-cli --version 0.8.3 --no-default-features --features postgres
  ```
- **Status:** ✅ Fixed
- **Resolution:** Pinned to `--version 0.8.3` at both install sites (rust job line 58 and smoke-test job line 172).

## Resolution Checklist
- [x] All [FIX] findings resolved
- [x] All [TASK] findings added to Steps.md or tracked
- [ ] All [ADR] findings have ADRs created or dismissed
- [ ] All [RULE] findings applied or dismissed
- [ ] Review verified by review-verify agent

## Resolution Summary
**Resolved at:** 2026-04-15
**Session:** resolve-review session for PR #8 ci-pipeline findings

| Category | Total | Resolved |
|---|---|---|
| [FIX] | 5 | 5 |
| [TASK] | 1 | 1 |
| [ADR] | 0 | 0 |
| [RULE] | 0 | 0 |
| **Total** | **6** | **6** |
