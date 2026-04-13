# PR Review: feat/server-core → main

**Date:** 2026-04-13
**Feature:** Context/Features/003-ServerCore/
**Branch:** feat/server-core
**Reviewers:** code-reviewer, silent-failure-hunter, pr-test-analyzer, type-design-analyzer
**Status:** ✅ Resolved

## Summary

22 findings total from a 4-agent review of PR#3 (built-in auth + core domain API). 13 fix-now
items span critical silent failures, security gaps, and convention violations. 3 missing tasks
cover auth integration test coverage. 3 architectural concerns need ADRs (TOCTOU race, RLS
deferral, DB/API type coupling). 1 convention gap covers sqlx error mapping repeated across
the codebase. Plus 9 suggestions pending interactive triage.

---

## Findings

### Fix-Now

#### [FIX] P4-001: Refresh token cookie path blocks cookie-based logout
- **File:** `apps/server/server/src/auth/handlers.rs:317-320`
- **Severity:** Critical
- **Detail:** The `refresh_token` cookie is set with `Path=/api/auth/refresh`. The logout
  endpoint is `POST /api/auth/logout`. Browsers only send cookies to matching paths, so the
  refresh token cookie is never sent to `/api/auth/logout` — cookie-based logout is silently
  broken. The logout handler tries to read the cookie and finds nothing. Fix: widen path to
  `/api/auth/` so both `/api/auth/refresh` and `/api/auth/logout` receive the cookie.
  Also update the cookie-clearing path string in the logout handler (line 244) to match.
- **Status:** ✅ Fixed
- **Resolution:** Changed `Path=/api/auth/refresh` to `Path=/api/auth/` in `set_auth_cookies` and logout clear strings.

#### [FIX] P4-002: `#[derive(Debug)]` on credential and token types leaks secrets to logs
- **File:** `apps/server/server/src/auth/models.rs:4, 11, 17`
- **Severity:** Critical
- **Detail:** `RegisterRequest`, `LoginRequest`, and `TokenResponse` all derive `Debug`.
  Any `tracing::debug!("{:?}", req)`, panic message, or test assertion will emit plaintext
  passwords and live tokens. Replace with manual `impl Debug` that redacts `password`,
  `access_token`, and `refresh_token` fields.
- **Status:** ✅ Fixed
- **Resolution:** Removed `#[derive(Debug)]` from `RegisterRequest`, `LoginRequest`, `TokenResponse`, `RefreshRequest`, `LogoutRequest`. Added manual `impl fmt::Debug` with `[redacted]` for all sensitive fields.

#### [FIX] P4-003: `verify_password` collapses hash-parse failure and wrong-password into identical Unauthorized
- **File:** `apps/server/server/src/auth/service.rs:29-34`
- **Severity:** Critical
- **Detail:** Both `PasswordHash::new` failure (malformed/corrupt stored hash) and
  `verify_password` failure (wrong password) use `|_| AppError::Unauthorized` with no logging.
  A corrupt hash in the DB — indicating a migration bug or write error — surfaces identically
  to a wrong password with zero diagnostic trace. A user account becomes permanently
  inaccessible with nothing logged. Fix: map `PasswordHash::new` failure to
  `AppError::Internal` with `tracing::error!`; keep wrong-password as `AppError::Unauthorized`.
- **Status:** ✅ Fixed
- **Resolution:** `PasswordHash::new` failure now maps to `AppError::Internal` with `tracing::error!`. Wrong password remains `AppError::Unauthorized`. Added `test_verify_corrupt_hash_returns_internal` test.

#### [FIX] P4-004: JWT decode failure in extractor discards all error context
- **File:** `apps/server/server/src/auth/extractor.rs:48`
- **Severity:** Critical
- **Detail:** `.map_err(|_| AppError::Unauthorized)` on `jsonwebtoken::decode` discards the
  error kind entirely. `ExpiredSignature`, `InvalidSignature`, `InvalidAlgorithm`, and
  `MalformedToken` are all indistinguishable. An algorithm confusion attempt or wrong
  deployment key produces no log entry. Fix: match on `e.kind()` and log non-expiry failures
  at `warn` level before returning `AppError::Unauthorized`.
- **Status:** ✅ Fixed
- **Resolution:** Now matches on `e.kind()`: `ExpiredSignature` logged at debug, all other kinds logged at `tracing::warn!` with `error_kind` field.

#### [FIX] P4-005: `hooks.server.ts` catch block silently swallows all token decode errors
- **File:** `apps/web/src/hooks.server.ts:25-27`
- **Severity:** Critical
- **Detail:** The bare `catch { event.locals.user = null; }` suppresses all errors from
  `split`, `atob`, and `JSON.parse` with no logging. A proxy truncation or cookie corruption
  affecting all users would cause every authenticated request to fail with zero server-side
  evidence. CLAUDE.md explicitly prohibits swallowing errors silently. Fix: `catch (e) {`
  with `console.error("[hooks.server] Failed to decode access_token cookie:", e)`.
- **Status:** ✅ Fixed
- **Resolution:** Changed `catch {` to `catch (e) {` with `console.error("[hooks.server] Failed to decode access_token cookie:", e)`.

#### [FIX] P4-006: Auth cookies missing `Secure` flag
- **File:** `apps/server/server/src/auth/handlers.rs:313-332`
- **Severity:** High
- **Detail:** `access_token` and `refresh_token` cookies are set `HttpOnly; SameSite=Lax`
  but without `; Secure`. Tokens will transmit over plain HTTP in production. The logout
  handler's cookie-clearing strings (lines 239, 243-245) also omit `Secure`. Fix: make
  `Secure` configurable (on by default in production, off for local dev via `APP_ENV`).
- **Status:** ✅ Fixed
- **Resolution:** Added `secure_cookies: bool` to `Config` (read from `APP_ENV=production`) and `AppState`. `set_auth_cookies` appends `; Secure` when enabled. SvelteKit pages use `secure: !dev` from `$app/environment`.

#### [FIX] P4-007: Blocking Argon2id operations called directly in async context
- **File:** `apps/server/server/src/auth/service.rs:17-26, 29-34` (called from `handlers.rs:58, 137`)
- **Severity:** High
- **Detail:** `hash_password` and `verify_password` are synchronous CPU-intensive operations
  called directly in async handler code. With 19MB memory + 2 iteration parameters they block
  a Tokio worker thread for the full hash duration, reducing concurrency under load.
  Convention: `.claude/rules/rust-axum.md` prohibits blocking calls in async functions.
  Fix: wrap calls in `tokio::task::spawn_blocking`.
- **Status:** ✅ Fixed
- **Resolution:** Both `hash_password` and `verify_password` calls in `register` and `login` handlers are now wrapped in `tokio::task::spawn_blocking`. Rule added to `.claude/rules/rust-axum.md`.

#### [FIX] P4-008: `refresh_tokens` table missing `updated_at` column
- **File:** `infra/migrations/20260412000006_create_refresh_tokens.up.sql`
- **Severity:** High
- **Detail:** Convention (`.claude/rules/postgres.md`) requires all tables to have
  `created_at` and `updated_at` with the `set_updated_at` trigger. `refresh_tokens` has
  `created_at` but no `updated_at`. Unlike truly immutable tables, `refresh_tokens` receives
  UPDATE operations for revocation (`revoked_at`). Fix: add `updated_at` column + trigger
  in a new migration.
- **Status:** ✅ Fixed
- **Resolution:** Created migration `20260412000026_add_refresh_tokens_updated_at.up.sql` adding the column and trigger.

#### [FIX] P4-009: `revoke_refresh_token` silently succeeds when token does not exist
- **File:** `apps/server/server/src/auth/service.rs:130-138`
- **Severity:** High
- **Detail:** `UPDATE refresh_tokens SET revoked_at = NOW() WHERE token_hash = $1` returns
  `rows_affected = 0` when the token is missing or already revoked, yet the function returns
  `Ok(())`. Logout always returns 204 regardless. Replay attempts against the logout endpoint
  go undetected. Fix: check `result.rows_affected()` and log a warning at minimum; consider
  returning `AppError::Unauthorized` for zero-row results.
- **Status:** ✅ Fixed
- **Resolution:** Added `result.rows_affected() == 0` check with `tracing::warn!` logging. Returns `Ok(())` so logout 204 is preserved but the event is now visible in logs.

#### [FIX] P4-010: Revoked refresh token reuse not logged — security event has no audit trail
- **File:** `apps/server/server/src/auth/service.rs:106`
- **Severity:** High
- **Detail:** `if record.revoked_at.is_some() { return Err(AppError::Unauthorized); }` silently
  discards the event. A stolen token being replayed after rotation produces no log entry.
  Fix: add `tracing::warn!(user_id = %user_id, "Revoked refresh token used — possible replay attack")` before returning the error.
- **Status:** ✅ Fixed
- **Resolution:** Added `tracing::warn!(token_hash = %token_hash, "Revoked refresh token presented — possible replay attack")` before returning Unauthorized.

#### [FIX] P4-011: SvelteKit auth form actions don't guard against malformed server response
- **File:** `apps/web/src/routes/auth/register/+page.server.ts:31-44`, `apps/web/src/routes/auth/login/+page.server.ts:27-42`
- **Severity:** High
- **Detail:** `const body = (await response.json()) as { access_token: string; ... }` is a
  TypeScript type assertion, not a runtime check. If the server responds with 201 but a
  malformed body (serialisation bug, gateway rewrite), `response.json()` throws, the `await`
  rejects, and there is no try-catch — the user sees SvelteKit's default error page rather
  than a meaningful failure message. Fix: wrap JSON parse in try-catch and validate
  `body.access_token` exists before using it.
- **Status:** ✅ Fixed
- **Resolution:** Both pages now wrap `response.json()` in try-catch and check `body.access_token && body.refresh_token` before using the tokens.

#### [FIX] P4-012: Handler validation tests verify Rust's standard library, not the handler
- **File:** `apps/server/server/src/auth/handlers.rs` (inline tests)
- **Severity:** Medium
- **Detail:** `test_email_validation_logic` asserts `"user@example.com".contains('@')` and
  `test_password_length_validation_logic` asserts `"short".len() < 8`. These pass even if the
  validation is removed from the handler. They verify `str::contains` and `str::len`, not
  handler behavior. These tests give false assurance about validation coverage. They should be
  replaced by proper integration tests that POST bad inputs and assert HTTP 400 responses.
- **Status:** ✅ Fixed
- **Resolution:** Replaced with `test_register_rejects_invalid_email`, `test_register_rejects_at_only_email`, and `test_register_rejects_short_password` — all POST to the actual handler and assert HTTP 400.

#### [FIX] P4-013: `read_cookie_from_headers` silently drops non-UTF-8 header value
- **File:** `apps/server/server/src/auth/handlers.rs:298-306`
- **Severity:** Low
- **Detail:** `cookie_header.to_str().ok()?` converts the error to `None` with no log entry.
  A malformed `Cookie` header from a misconfigured proxy causes auth to fail silently as
  Unauthorized with no diagnostic trace. Fix: add a `tracing::debug!` log before returning
  `None` when `to_str()` fails.
- **Status:** ✅ Fixed
- **Resolution:** Changed `to_str().ok()?` to an explicit `match` with `tracing::debug!("Cookie header contains non-UTF-8 bytes...")` before returning `None`.

---

### Missing Tasks

#### [TASK] P4-014: No integration tests for any auth handler — all SQL paths untested
- **File:** `apps/server/server/src/auth/handlers.rs`, `apps/server/server/tests/`
- **Severity:** Critical
- **Detail:** The `tests/` directory is empty. All register, login, refresh, and logout
  handlers execute SQL but are never invoked in tests. The existing handler-level tests
  (#[cfg(test)] in handlers.rs) verify string operations, not behavior. Missing coverage:
  happy-path register (first user → admin), duplicate email → 409, login with wrong password
  → 401, login with pending user → 403, refresh with valid token, logout invalidates token.
  Convention: `.claude/rules/rust-axum.md` requires `sqlx::test` for DB-backed tests.
- **Relates to:** Steps.md auth implementation tasks; FA assertions FA-013/FA-014
- **Status:** ✅ Task created
- **Resolution:** Added as S029 in Steps.md Phase 9.

#### [TASK] P4-015: `rotate_refresh_token` security invariants have no test coverage
- **File:** `apps/server/server/src/auth/service.rs:96-127`
- **Severity:** Critical
- **Detail:** The most security-critical function in the PR is entirely untested at the DB
  level. Missing: revoked token reuse returns 401 (prevents indefinite session extension with
  stolen token), expired token returns 401, token not found returns 401. Without these tests,
  a regression in any of the three failure branches would go undetected.
- **Relates to:** Steps.md auth tasks; FA assertion about refresh token rotation
- **Status:** ✅ Task created
- **Resolution:** Added as S030 in Steps.md Phase 9.

#### [TASK] P4-016: `hooks.server.ts` JWT parsing has no unit test coverage
- **File:** `apps/web/src/hooks.server.ts`
- **Severity:** High
- **Detail:** The hook manually parses JWTs (split, base64 decode, JSON parse) on every
  request with no test coverage. Edge cases that need verification: truncated cookie
  (undefined payload), JWT with valid structure but non-JSON payload, token where `email`
  claim is absent (always "" per S-1 finding). Convention: Vitest for SvelteKit utility logic.
- **Status:** ✅ Task created
- **Resolution:** Added as S031 in Steps.md Phase 9.

---

### Architectural Concerns

#### [ADR] P4-017: TOCTOU race in first-user admin registration
- **File:** `apps/server/server/src/auth/handlers.rs:44-55`
- **Severity:** Critical
- **Detail:** `SELECT COUNT(*)` then separate `INSERT` are not atomic. Two concurrent
  registrations can both observe `count == 0` and both become admin users with `status =
  "active"`. Requires a decision: wrap in a serializable transaction, or consolidate into a
  single INSERT with a subquery that determines admin/pending in one atomic operation. An ADR
  is needed because this changes the SQL approach for the auth bootstrapping path.
- **Relates to:** ADR-012 (built-in auth)
- **Status:** ✅ ADR created
- **Resolution:** ADR-013 (single atomic INSERT with subquery, implementation deferred to S029).

#### [ADR] P4-018: No RLS on any of the 21 new tables
- **File:** `infra/migrations/20260412000005_migrate_users_to_builtin_auth.up.sql` through `20260412000025_create_entity_tags.up.sql`
- **Severity:** Critical
- **Detail:** Convention (`.claude/rules/postgres.md`) requires RLS on all user-facing tables.
  None of the 21 new migrations enable RLS or create policies. User isolation currently relies
  entirely on application-layer `WHERE user_id = $1` clauses. Any missed filter in a future
  handler exposes cross-user data. If this is a deliberate deferral (e.g., waiting for
  PowerSync sync rules to be defined first), it must be documented in an ADR with a timeline.
- **Relates to:** ADR-012; powersync.md sync rules convention
- **Status:** ✅ ADR created
- **Resolution:** ADR-014 (RLS deferred to Feature 004 Sync Engine, with rationale and timeline).

#### [ADR] P4-019: Core domain models serve as both DB row types and API response types
- **File:** `apps/server/server/src/core/initiatives/models.rs:5-15`, `tags/models.rs:6`, `relations/models.rs:6`
- **Severity:** High
- **Detail:** `Initiative`, `Tag`, and `EntityRelation` derive both `sqlx::FromRow` and
  `Serialize`, causing `user_id` (redundant to the caller) and `deleted_at` (internal
  implementation detail) to be serialized into API responses. The auth module correctly
  separates these concerns (separate `UserProfile` response type, local `MeRow` query type
  in handlers.rs). A decision is needed: adopt the same DB-type / response-type separation
  pattern across all domain modules (the auth module already establishes the precedent).
- **Status:** ✅ ADR created
- **Resolution:** ADR-015 (separate row/response types required; refactor of core modules deferred to Feature 005).

---

### Convention Gaps

#### [RULE] P4-020: sqlx errors mapped with `.to_string()` throughout — loses error chain
- **Files:** `apps/server/server/src/auth/service.rs:80-83`, `handlers.rs:48, 78, 133, 194, 272`, multiple service files
- **Severity:** Medium
- **Detail:** The pattern `.map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))`
  appears ~15 times. `.to_string()` flattens the `sqlx::Error` to a string, discarding the
  original error chain and making structured debugging harder. Additionally it leaks schema
  detail (table name, constraint name) into logs. The correct approach is
  `anyhow::Error::from(e)` or adding `impl From<sqlx::Error> for AppError`. This pattern is
  absent from `.claude/rules/rust-axum.md` and will recur in every new module.
- **Suggested rule:** Add to `.claude/rules/rust-axum.md`: "Map sqlx errors via
  `anyhow::Error::from(e)`, never `.to_string()`. Prefer `impl From<sqlx::Error> for AppError`
  to eliminate boilerplate and preserve the error chain."
- **Status:** ✅ Rule updated
- **Resolution:** Added to `.claude/rules/rust-axum.md` Error Handling section. Existing `.to_string()` occurrences left in place (not in scope for this review pass — tracked for future cleanup).

---

#### [FIX] P4-021: Migration numbering gap at slot 000009 — document intentional skip
- **File:** `infra/migrations/`
- **Severity:** Low
- **Detail:** Numbering skips from `20260412000008_create_tags` to `20260412000010_create_attachments`
  with no slot 000009. This is intentional (entity_tags was renumbered from slot 9 → slot 25 to
  resolve FK ordering), but the gap is unexplained to future readers. Add a comment in the
  migrations directory README or a no-op placeholder migration explaining the gap.
- **Status:** ✅ Fixed
- **Resolution:** Created `infra/migrations/_SLOT_000009_RESERVED.txt` explaining the intentional gap and the FK ordering reason.

#### [FIX] P4-022: `hooks.server.ts` email claim is always empty string — real functional bug
- **File:** `apps/web/src/hooks.server.ts:21`
- **Severity:** High
- **Detail:** `event.locals.user = { id: payload.sub, email: payload.email ?? "" }`. The
  Rust `Claims` struct has no `email` field — only `sub`, `household_ids`, `iat`, `exp`.
  `payload.email` is always `undefined`, so `locals.user.email` is always `""`. Any layout or
  route that reads `locals.user.email` to display the logged-in user will always show blank
  with no error. Fix: either add `email` to the `Claims` struct (and re-issue tokens), or fetch
  user email separately (e.g., store it in a session cookie at login, or add a `/api/auth/me`
  call at app load).
- **Status:** ✅ Fixed
- **Resolution:** Added `email: String` to `Claims` struct in `auth/models.rs`. Updated `issue_access_token` to accept and embed the email claim. `register` and `login` handlers pass the user's email. `rotate_refresh_token` fetches email from users table when rotating.

#### [FIX] P4-023: Refresh handler double-lookup — redundant DB round-trip
- **File:** `apps/server/server/src/auth/handlers.rs:186-207`, `service.rs:96-127`
- **Severity:** Low
- **Detail:** The refresh handler fetches `user_id` from the token in a first query, then
  passes it to `rotate_refresh_token` which does its own independent lookup by token hash.
  The first query's `user_id` is used for issuing the new token, but the second query does not
  verify `user_id` matches. Fix: have `rotate_refresh_token` also return `user_id` from its
  own lookup (eliminating the first query), or add `AND user_id = $2` to the rotation query to
  make the user binding explicit.
- **Status:** ✅ Fixed
- **Resolution:** Removed the redundant first query from the refresh handler. `rotate_refresh_token` signature no longer takes `user_id`/`household_ids` — it fetches `user_id` from the token record and email from the users table internally.

#### [FIX] P4-024: Email validation `contains('@')` trivially bypassed
- **File:** `apps/server/server/src/auth/handlers.rs:35-38`
- **Severity:** Medium
- **Detail:** `if !req.email.contains('@')` accepts `"@"`, `"@@"`, `"@b"` as valid. The
  validation implies correctness it does not provide — the error message "Invalid email
  address" fires for far fewer inputs than it should. Use a proper check (e.g., the `validator`
  crate's `validate_email`, or at minimum require non-empty local and domain parts).
- **Status:** ✅ Fixed
- **Resolution:** Replaced with `is_valid_email()` helper requiring non-empty local part, non-empty domain, and domain containing a dot with no leading/trailing dots. Added unit tests covering valid and invalid cases.

#### [FIX] P4-027: Duplicate cookie parser in extractor and handlers
- **File:** `apps/server/server/src/auth/extractor.rs:58-63`, `apps/server/server/src/auth/handlers.rs:298-306`
- **Severity:** Low
- **Detail:** `parse_cookie_value` (extractor) and `read_cookie_from_headers` (handlers) are
  functionally identical string-splitting cookie parsers. Extract to a shared utility function
  in `src/auth/mod.rs` or a new `src/auth/cookies.rs`.
- **Status:** ✅ Fixed
- **Resolution:** Added `pub(crate) fn parse_cookie_value` to `auth/mod.rs`. Both `extractor.rs` and `handlers.rs` now delegate to `super::parse_cookie_value`.

#### [TASK] P4-025: `extract_token_from_body_or_cookie` priority behavior has no test
- **File:** `apps/server/server/src/auth/handlers.rs`
- **Severity:** Low
- **Detail:** When both a body token and a cookie token are present, the body token takes
  precedence (allows programmatic clients to override cookie auth). This is intentional
  behavior but there is no test asserting this contract. A future refactor could silently flip
  the precedence. Add a test that provides both and asserts body wins.
- **Status:** ✅ Task created
- **Resolution:** Added inline tests `test_extract_token_body_wins_over_cookie` and `test_extract_token_falls_back_to_cookie` in handlers.rs (P4-025 resolved inline, noted in S033 in Steps.md).

#### [TASK] P4-026: Extractor missing tests for malformed `Authorization` header
- **File:** `apps/server/server/src/auth/extractor.rs`
- **Severity:** Low
- **Detail:** Two missing edge cases: (1) `Authorization: Basic abc123` — non-Bearer scheme
  should return 401; (2) `Authorization: Bearer` with no trailing token — should return 401.
  Both are handled by the `strip_prefix` path, but the behavior is untested.
- **Status:** ✅ Task created
- **Resolution:** Added `test_non_bearer_scheme_returns_401` and `test_bearer_with_no_token_returns_401` inline in extractor.rs (P4-026 resolved inline, noted in S034 in Steps.md).

#### [TASK] P4-028: Initiative service SQL string tests verify constants, not behavior
- **File:** `apps/server/server/src/core/initiatives/service.rs`
- **Severity:** Medium
- **Detail:** Tests define SQL string constants that duplicate the actual queries, then assert
  those constants contain substrings like `"user_id = $1"`. These break on whitespace changes
  and pass even if the query logic is wrong. Replace with `sqlx::test` integration tests that
  insert two users' data and assert each user sees only their own records. This would also
  validate the user-scoping requirement at the DB level.
- **Status:** ✅ Task created
- **Resolution:** Added as S032 in Steps.md Phase 9.

#### [ADR] P4-029: `is_admin` absent from JWT claims — DB hit on every admin-gated request
- **File:** `apps/server/server/src/auth/models.rs:39-45`, `handlers.rs:268`
- **Severity:** Medium
- **Detail:** The `me` handler re-queries `is_admin` from the DB on every authenticated
  request. As admin-gated routes grow, each will require the same DB round-trip. Adding
  `is_admin: bool` to `Claims` and `AuthUser` would make authorization decisions zero-cost
  after JWT validation. Tradeoff: if admin status is revoked, the old claim remains valid
  until the token expires (15 min). A decision is needed on acceptable staleness for admin
  role changes.
- **Relates to:** ADR-012 (built-in auth design)
- **Status:** ✅ ADR created
- **Resolution:** ADR-016 (is_admin NOT added to claims; DB query accepted; revisit trigger documented).

---

## Resolution Checklist
- [x] All [FIX] findings resolved (P4-001 through P4-009, P4-011 through P4-013, P4-021 through P4-024, P4-027)
- [x] All [TASK] findings added to Steps.md (P4-014 through P4-016, P4-025, P4-026, P4-028)
- [x] All [ADR] findings have ADRs created or dismissed (P4-017 through P4-019, P4-029)
- [x] All [RULE] findings applied or dismissed (P4-020)
- [x] Review verified by review-verify agent

## Resolution Summary
**Resolved at:** 2026-04-13
**Session:** Full resolve pass — all 29 findings addressed

| Category | Total | Resolved |
|---|---|---|
| [FIX] | 18 | 18 |
| [TASK] | 6 | 6 |
| [ADR] | 4 | 4 |
| [RULE] | 1 | 1 |
| **Total** | **29** | **29** |
