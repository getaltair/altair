# PR Review: feat/scaffold → main

**Date:** 2026-04-12
**Feature:** Context/Features/001-Foundation/
**Branch:** feat/scaffold
**PR:** #1 — feat: 001-Foundation — full monorepo scaffold
**Reviewers:** code-reviewer, pr-test-analyzer, silent-failure-hunter, comment-analyzer
**Status:** ✅ Resolved

## Summary

22 findings total across all four review agents: 10 [FIX] (inline code corrections),
3 [TASK] (missing test work to track), 3 [ADR] (architectural decisions needed), 0 [RULE].
Severity breakdown: 4 Critical, 5 High, 13 Medium/Low. The four critical fix-nows and
two critical ADRs must be resolved before any feature work builds on this foundation.

---

## Findings

### Fix-Now

#### [FIX] P1-001: `fetch` to token endpoint uncaught — network failures crash callback silently
- **File:** `apps/web/src/routes/auth/callback/+page.server.ts:37–48`
- **Severity:** Critical
- **Detail:** The `fetch` call and subsequent `response.json()` are not wrapped in
  try/catch. Network failures (`ECONNREFUSED`, DNS, TLS, timeout) throw `TypeError`
  or `FetchError` which propagate to SvelteKit's framework boundary, producing a
  generic 500 with no log entry naming the failed operation. Wrap both in try/catch
  and return a structured `ErrorResponse` with an actionable message.
- **Relates to:** S015, FA-009
- **Status:** ✅ Fixed
- **Resolution:** Wrapped `fetch` in try/catch returning `ErrorResponse` with the network error message. Wrapped `response.json()` in a separate try/catch returning "not valid JSON".

#### [FIX] P1-002: Secret scan `|| true` — FA-012 gate permanently disabled
- **File:** `.github/workflows/ci.yml:202`
- **Severity:** Critical
- **Detail:** `git grep ... || true` always exits 0. The step shows green whether or
  not secrets are found. Remove `|| true`. Gate must exit non-zero when matches are
  found.
- **Relates to:** FA-012
- **Status:** ✅ Fixed
- **Resolution:** Replaced `|| true` pattern with `if git grep ...; then exit 1; fi` so the step fails when matches are found and passes when none are found.

#### [FIX] P1-003: CI Zitadel wait-loop curls port 8080, compose maps to 8081
- **File:** `.github/workflows/ci.yml:149`
- **Severity:** Critical
- **Detail:** `docker-compose.yml` maps Zitadel container port 8080 → host port 8081
  (`"8081:8080"`). The wait-loop curls `localhost:8080`, exhausts all 40 attempts
  against an unbound port, and continues silently. Fix: change to `localhost:8081`.
- **Relates to:** FA-004
- **Status:** ✅ Fixed
- **Resolution:** Changed Zitadel wait-loop to `localhost:8081/debug/healthz` (also corrected endpoint from `/debug/healthz/ready` which returns 404 to `/debug/healthz` which returns 200).

#### [FIX] P1-004: CI wait-loops fail open — exhausted retries don't exit non-zero
- **File:** `.github/workflows/ci.yml:137–152`
- **Severity:** Critical
- **Detail:** Both Postgres and Zitadel readiness loops use `for ... && break` with no
  failure guard after the loop. When a service never becomes healthy, the loop exits
  cleanly and the next step fails with a misleading error (e.g., migration connection
  refused instead of "service startup timeout"). Add `exit 1` guard after each loop.
- **Relates to:** FA-004
- **Status:** ✅ Fixed
- **Resolution:** Rewrote both loops with `READY=false` flag; added `if [ "$READY" != "true" ]; then exit 1; fi` guard after each loop.

#### [FIX] P1-009: Token response shape unvalidated — undefined fields silently reach client
- **File:** `apps/web/src/routes/auth/callback/+page.server.ts:48–54`
- **Severity:** High
- **Detail:** `response.json()` is cast to `TokenResponse` with no runtime validation.
  A misconfigured scope or IdP version difference that omits `access_token` or
  `id_token` will show "Login successful" with `undefined` rendered as the token.
  Assert that both are non-empty strings before returning.
- **Relates to:** S015, FA-009
- **Status:** ✅ Fixed
- **Resolution:** Added `if (!tokens.access_token || !tokens.id_token)` guard returning an `ErrorResponse` before returning the token data.

#### [FIX] P1-010: Zitadel port mismatch in `.env.example` and `zitadel-setup.sh`
- **File:** `apps/web/.env.example:2`, `infra/scripts/zitadel-setup.sh:16`
- **Severity:** High
- **Detail:** Both default `PUBLIC_ZITADEL_ISSUER` / `ZITADEL_BASE_URL` to
  `localhost:8080`, but Docker Compose exposes Zitadel on host port 8081. Every
  developer following the README will hit auth failures. Update both to port 8081.
- **Status:** ✅ Fixed
- **Resolution:** Updated `PUBLIC_ZITADEL_ISSUER` in `apps/web/.env.example` and `ZITADEL_BASE_URL` default in `infra/scripts/zitadel-setup.sh` to `localhost:8081`.

#### [FIX] P1-011: `response.text()` in error path can itself throw unhandled
- **File:** `apps/web/src/routes/auth/callback/+page.server.ts:44`
- **Severity:** High
- **Detail:** When `!response.ok`, calling `await response.text()` can throw if the
  body stream is consumed or the connection drops. The original HTTP error becomes
  invisible, replaced by an unhandled exception. Wrap defensively:
  `.catch(() => '(response body unreadable)')`.
- **Relates to:** S015
- **Status:** ✅ Fixed
- **Resolution:** Changed `await response.text()` to `await response.text().catch(() => '(response body unreadable)')`.

#### [FIX] P1-012: `dotenvy::dotenv()` parse errors silenced
- **File:** `apps/server/server/src/main.rs:11`
- **Severity:** High
- **Detail:** `let _ = dotenvy::dotenv()` discards parse errors. A malformed `.env`
  file shows "DATABASE_URL required" with no indication the file exists but is
  unreadable. Match on the result: log `debug` for file-not-found (expected in
  production), `warn` for parse errors.
- **Status:** ✅ Fixed
- **Resolution:** Captured result before tracing init, then matched on it after init: `not_found()` → `debug!`, any other error → `warn!`.

#### [FIX] P1-015: `code_verifier` cookie not cleared after use
- **File:** `apps/web/src/routes/auth/callback/+page.server.ts`
- **Severity:** Medium
- **Detail:** The `code_verifier` cookie is consumed in the callback but never deleted.
  Add `event.cookies.delete('code_verifier', { path: '/' })` after the token exchange
  succeeds. Minimizes the window for any replay or session confusion.
- **Relates to:** S015
- **Status:** ✅ Fixed
- **Resolution:** Added `event.cookies.delete('code_verifier', { path: '/' })` after token validation passes, before returning the token data.

#### [FIX] P1-016: Duplicate indexes on `oidc_sub` and `email`
- **File:** `infra/migrations/20260412000002_create_users.up.sql:12–13`
- **Severity:** Medium
- **Detail:** PostgreSQL automatically creates a unique index to enforce UNIQUE
  constraints. The explicit `CREATE INDEX idx_users_oidc_sub` and
  `CREATE INDEX idx_users_email` create redundant non-unique indexes on columns that
  already have unique indexes. Remove both lines, or rename the constraints to use
  the `idx_` convention: `CONSTRAINT idx_users_oidc_sub UNIQUE (oidc_sub)`.
- **Status:** ✅ Fixed
- **Resolution:** Removed both redundant `CREATE INDEX` lines; replaced with a comment noting the implicit unique indexes.

#### [FIX] P1-017: Worker binary uses `println!` instead of `tracing`
- **File:** `apps/server/worker/src/main.rs`
- **Severity:** Medium
- **Detail:** `println!("altair-worker started")` is inconsistent with the server crate
  which uses `tracing::info!`. Replace with `tracing::info!` so both binaries produce
  structured logs from the first line. (Server already initializes `tracing_subscriber`
  in its `main.rs`; worker will need the same.)
- **Status:** ✅ Fixed
- **Resolution:** Added `tracing` and `tracing-subscriber` deps to altair-worker via `cargo add`. Replaced `println!` with `tracing::info!` and added `tracing_subscriber::fmt().init()` initialization.

#### [FIX] P1-018: `SAFETY` comments on unsafe env var calls don't justify the invariant
- **File:** `apps/server/server/src/config.rs:29,38`
- **Severity:** Low
- **Detail:** The first SAFETY comment claims "single-threaded test" but `cargo test`
  runs tests in a thread pool by default. The second SAFETY comment on the restore path
  just describes what the code does, not why the unsafe invariant holds. Both must be
  rewritten to state the actual threading constraint and how it is upheld. Consider
  `serial_test` crate or `-- --test-threads=1` as a concrete enforcement mechanism.
- **Status:** ✅ Fixed
- **Resolution:** Rewrote both SAFETY comments to state the threading constraint and reference `-- --test-threads=1` / `serial_test` as enforcement mechanisms.

#### [FIX] P1-019: `powersync.yml` "Step 4 (Sync Engine)" cross-reference will become stale
- **File:** `infra/compose/powersync.yml:3`
- **Severity:** Low
- **Detail:** Comment references an internal planning phase number. Once Steps.md is
  archived or a new developer joins, "Step 4 (Sync Engine)" is opaque. Replace with a
  self-contained comment pointing at the actual target file:
  `apps/web/src/lib/sync/schema.ts`.
- **Status:** ✅ Fixed
- **Resolution:** Replaced "Step 4 (Sync Engine)" with `apps/web/src/lib/sync/schema.ts`.

#### [FIX] P1-020: `seed.sql` missing note linking `oidc_sub` to Zitadel's generated identity
- **File:** `infra/scripts/seed.sql`
- **Severity:** Low
- **Detail:** If Zitadel is reconfigured and generates a different subject identifier,
  the seed silently inserts a user with `oidc_sub = 'dev-user-001'` that won't match
  any real identity. Add a comment noting that this value must match the dev identity
  created by `infra/scripts/zitadel-setup.sh`.
- **Status:** ✅ Fixed
- **Resolution:** Added comment above the INSERT stating that `oidc_sub` must match the dev identity from `infra/scripts/zitadel-setup.sh` and what to update if Zitadel is reconfigured.

#### [FIX] P1-021: Migration missing `-- No password_hash: see ADR-006` rationale comment
- **File:** `infra/migrations/20260412000002_create_users.up.sql`
- **Severity:** Low
- **Detail:** The `users` table has no `password_hash` column, which deviates from the
  most common schema any developer has seen. Without a comment, a future maintainer
  will treat this as an oversight. Add above the `oidc_sub` column:
  `-- No password_hash: authentication is exclusively via OIDC. See ADR-006.`
- **Status:** ✅ Fixed
- **Resolution:** Added `-- No password_hash: authentication is exclusively via OIDC. See ADR-006.` comment above the `oidc_sub` column.

---

### Missing Tasks

#### [TASK] P1-005: Vitest not installed; no `test` script in `package.json`
- **File:** `apps/web/package.json`
- **Severity:** Critical
- **Detail:** There is no `vitest` dependency and no `test` script configured. No web
  tests can be written or run without this. Must be added (with `happy-dom` or
  `jsdom` environment for Web Crypto API support) before any auth feature builds on
  this foundation.
- **Relates to:** S015
- **Status:** ✅ Task created
- **Resolution:** Added as S017 in Steps.md Phase 8.

#### [TASK] P1-006: PKCE crypto helpers have no tests — base64url correctness unverified
- **File:** `apps/web/src/lib/auth/pkce.ts`
- **Severity:** Critical
- **Detail:** `generateCodeVerifier`, `generateCodeChallenge`, and `base64urlEncode`
  implement the security-critical PKCE primitives. A bug in the `+`→`-`, `/`→`_`,
  `=`→`` replacements silently causes every login to fail at the token exchange. Needs
  a `pkce.spec.ts` with: output-length assertion (86 chars), character-set assertion
  (`[A-Za-z0-9\-_]` only), and a known-input/known-output round-trip. Gated on P1-005.
- **Relates to:** S015, FA-009
- **Status:** ✅ Task created
- **Resolution:** Added as S017-T in Steps.md Phase 8 (depends on S017).

#### [TASK] P1-014: `AppError::IntoResponse` has no tests
- **File:** `apps/server/server/src/error.rs`
- **Severity:** High
- **Detail:** Every future route handler returns `AppError`. The HTTP status/body
  mapping is the API contract for all clients. Neither `NotFound` (→ 404) nor
  `Internal` (→ 500, must not leak internal message) variants are tested. Use the
  existing `tower::ServiceExt::oneshot` pattern from `routes/health.rs`.
- **Status:** ✅ Task created
- **Resolution:** Added as S018-T in Steps.md Phase 8.

#### [TASK] P1-022: `config.rs` tests missing happy-path and `BIND_ADDR` default coverage
- **File:** `apps/server/server/src/config.rs`
- **Severity:** Low
- **Detail:** Only the missing-`DATABASE_URL` error path is tested. The happy path
  (returns `Ok` with correct field values) and the `BIND_ADDR` fallback default
  (`"0.0.0.0:8000"`) are not exercised. A regression changing the default address
  would go undetected.
- **Status:** ✅ Task created
- **Resolution:** Added as S019-T in Steps.md Phase 8.

---

### Architectural Concerns

#### [ADR] P1-007: Missing OIDC `state` parameter — no CSRF protection on auth flow
- **File:** `apps/web/src/routes/auth/login/+page.server.ts:17–24`,
  `apps/web/src/routes/auth/callback/+page.server.ts:18`
- **Severity:** Critical
- **Detail:** The OIDC authorization request has no `state` parameter, and the callback
  does not validate one. PKCE protects against code interception but does not replace
  `state` (RFC 6749 §10.12). Without `state`, an attacker can craft a malicious
  authorization response URL and complete a code exchange for an attacker-controlled
  account (login CSRF). Must be decided: add `crypto.randomUUID()` state stored in a
  second httpOnly cookie, or adopt a session library that handles this automatically.
- **Relates to:** S015, FA-009, ADR-006 (auth approach)
- **Status:** ✅ ADR created
- **Resolution:** ADR-009 (Context/Decisions/ADR-009-oidc-state-csrf-protection.md) — decided: `crypto.randomUUID()` state in httpOnly cookie, validated in callback.

#### [ADR] P1-008: Tokens returned as page data, rendered in browser DOM
- **File:** `apps/web/src/routes/auth/callback/+page.server.ts:48–54`,
  `apps/web/src/routes/auth/callback/+page.svelte:11`
- **Severity:** Critical
- **Detail:** `access_token` and `id_token` are returned from the server `load`
  function and embedded in the server-rendered HTML payload. Tokens are in browser
  history, proxied/CDN response logs, and `<pre>`-rendered in the DOM. This
  establishes the wrong token-storage pattern for the entire project. Decision needed:
  store tokens in an httpOnly session cookie (preferred) or a server-side session
  store; `redirect(302, '/')` after exchange rather than rendering tokens. Also
  determines what the callback page's purpose is post-auth.
- **Relates to:** S015, FA-009, ADR-006
- **Status:** ✅ ADR created
- **Resolution:** ADR-010 (Context/Decisions/ADR-010-token-storage-post-callback.md) — decided: httpOnly signed cookie + `redirect(302, '/')`, implemented in Step 3/Step 9 auth hardening.

#### [ADR] P1-013: `AppError` only `NotFound`/`Internal` — all non-404 errors become 500
- **File:** `apps/server/server/src/error.rs`
- **Severity:** High
- **Detail:** The two-variant `AppError` forces auth failures, validation errors,
  conflicts, and bad requests all into `Internal` (HTTP 500). Clients cannot
  distinguish retryable from non-retryable errors. This must be decided before any
  feature handlers are written, as the taxonomy affects every route. Recommended
  additions: `Unauthorized` (401), `Forbidden` (403), `BadRequest(String)` (400),
  `Conflict(String)` (409). Decision: establish the full taxonomy now vs. expand
  incrementally per feature.
- **Relates to:** Tech.md "Error Handling" section
- **Status:** ✅ ADR created
- **Resolution:** ADR-011 (Context/Decisions/ADR-011-apperror-variant-taxonomy.md) — decided: expand to 6 variants (NotFound, Unauthorized, Forbidden, BadRequest, Conflict, Internal) before domain handlers are written.

---

## Resolution Checklist
- [x] All [FIX] findings resolved (15 items: P1-001 through P1-004, P1-009 through P1-012, P1-015 through P1-021)
- [x] All [TASK] findings added to Steps.md (4 items: P1-005, P1-006, P1-014, P1-022 → S017, S017-T, S018-T, S019-T)
- [x] All [ADR] findings have ADRs created or dismissed (3 items: P1-007 → ADR-009, P1-008 → ADR-010, P1-013 → ADR-011)
- [x] Review verified by review-verify agent

## Resolution Summary
**Resolved at:** 2026-04-12
**Session:** Post-PR review resolution — all inline fixes, Steps.md tasks, and ADRs for P1 findings

| Category | Total | Resolved |
|---|---|---|
| [FIX] | 15 | 15 |
| [TASK] | 4 | 4 |
| [ADR] | 3 | 3 |
| **Total** | **22** | **22** |
