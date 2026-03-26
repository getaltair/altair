# Plan: Address All PowerSync PR Review Feedback

## Task Description

Address ALL findings from the comprehensive 6-agent PR review of `feat/step-7-powersync-setup`. The review identified 8 critical issues, 6 important issues, and 8 suggestions across the Rust backend, TypeScript frontend sync client, debug page, PowerSync YAML configuration, and Docker Compose infrastructure. Every finding must be resolved before the branch can merge.

## Objective

All review findings resolved: critical bugs fixed (JWT encoding, auth mechanism, URL routing, schema mismatches, singleton safety, production secret), important issues addressed (error handling, auth gate, test coverage), and suggestions applied (code style, type safety, comment cleanup). The branch compiles, tests pass, and the PowerSync integration is functionally correct end-to-end.

## Problem Statement

The PR review revealed that the PowerSync integration cannot function as-is due to multiple interacting bugs:

1. The PowerSync YAML sync queries reference columns/tables from the target design spec that do not yet exist in actual Postgres migrations
2. The JWT `k` field in YAML expects base64url encoding but gets a raw string
3. The frontend connector sends cookies but the backend requires Bearer tokens
4. The upload URL prefix (`/api/`) doesn't match backend routes (`/core/`)
5. Failed uploads are silently marked complete, causing permanent data loss
6. The singleton can enter a permanently broken state on init failure
7. The JWT secret silently falls back to a known value in production

Additionally, there is zero frontend test coverage and several code quality issues.

## Solution Approach

Three parallel work streams (backend, frontend, infra) followed by test creation and validation:

1. **Infra fixes** -- Pare down PowerSync YAML to match actual Postgres migrations, fix JWT secret base64url encoding in Docker Compose, update reference SQL
2. **Backend fixes** -- Require JWT secret in production, replace SystemTime panic with Result, make PowerSyncClaims private, extract shared test helper, filter soft-deleted memberships, clean up getter comments
3. **Frontend fixes** -- Fix auth mechanism (Bearer token), fix URL prefix, add response.ok checks, validate response shape, fix singleton failure handling, fix closePowerSync, add default switch case, fix debug page (auth gate, error logging, SQL safety, nested ternaries, comment cleanup)
4. **Frontend tests** -- Add powersync-client.spec.ts and schema.spec.ts
5. **Validation** -- cargo build/test, bun build/test/lint

## Relevant Files

### Existing Files (modify)

- `infra/powersync/powersync.yaml` -- Fix sync queries to match actual Postgres migrations, remove non-existent tables
- `infra/compose/docker-compose.yml` -- Base64url-encode the JWT secret default
- `apps/server/src/config.rs` -- Require JWT secret in production, remove trivial getter doc comments
- `apps/server/src/auth/jwt.rs` -- Replace expect panic with Result, make PowerSyncClaims fields private, add constructor
- `apps/server/src/auth/handlers.rs` -- Use getter methods consistently
- `apps/server/src/auth/service.rs` -- Add deleted_at IS NULL filter to get_user_household_ids (for future-proofing, since YAML now won't have deleted_at either -- skip if no deleted_at column exists yet)
- `apps/web/src/lib/sync/powersync-client.ts` -- Fix auth (Bearer token), fix URL prefix (/core/), add response.ok checks, validate response shape, fix singleton, fix closePowerSync, add default switch case, remove banner comments
- `apps/web/src/lib/sync/schema.ts` -- Already correct (matches migrations); update header comment to clarify relationship with design spec
- `apps/web/src/routes/(app)/debug/sync/+page.svelte` -- Fix catch blocks, add table name validation, replace nested ternaries with lookup maps, remove trivial HTML comments
- `apps/web/src/routes/(app)/debug/sync/+page.server.ts` -- Gate behind dev environment check
- `docs/schema/altair-initial-schema.sql` -- Update to match actual migrations (source of confusion for reviewers)

### New Files

- `apps/web/src/lib/sync/powersync-client.spec.ts` -- Unit tests for AltairConnector and singleton lifecycle
- `apps/web/src/lib/sync/schema.spec.ts` -- Structural tests for schema definitions

## Implementation Phases

### Phase 1: Foundation (Schema Alignment)

Fix the PowerSync YAML to match the actual Postgres migrations. This is the most fundamental issue -- everything else depends on the schema being correct. The actual Postgres schema is defined by the sqlx migrations in `apps/server/migrations/`, NOT the reference SQL in `docs/schema/`.

**Actual Postgres tables and columns (from migrations):**

- `users`: id, email, display_name, password_hash, created_at, updated_at
- `sessions`: id, user_id, token_hash, expires_at, device_info, created_at
- `households`: id, name, created_by, created_at
- `household_memberships`: id, household_id, user_id, role, joined_at
- `initiatives`: id, user_id, household_id, name, description, status, created_at, updated_at
- `tags`: id, user_id, household_id, name, color, created_at
- `attachments`: id, entity_type, entity_id, filename, content_type, storage_key, size_bytes, processing_state, created_at
- `entity_relations`: id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, relation_type, source_type, status, confidence, evidence_json, created_by_user_id, created_by_process, created_at, updated_at, last_confirmed_at

**Tables that do NOT exist yet (remove from YAML):**

- guidance_routines, guidance_epics, guidance_focus_sessions, guidance_quests (only in design spec)
- knowledge_notes, knowledge_note_snapshots, note_tags, note_attachments
- tracking_locations, tracking_categories, tracking_items, tracking_item_events, tracking_shopping_lists, tracking_shopping_list_items
- item_attachments, item_tags, quest_tags, quest_attachments

**Column name corrections for YAML (actual migration name -> what YAML currently uses):**

- `households.created_by` (NOT `owner_user_id`)
- `initiatives.user_id` (NOT `owner_user_id`)
- `initiatives.name` (NOT `title`)
- `household_memberships.joined_at` (NOT `created_at`)
- `tags.user_id` (NOT `owner_user_id`)
- No `deleted_at`, `updated_at` on most tables (only on users, initiatives, entity_relations)
- No `slug`, `description` on households
- No `is_active` on users or household_memberships

**The local SQLite schema (schema.ts) already matches the actual migrations correctly.** Do not change it.

### Phase 2: Core Implementation (Parallel Fixes)

Three parallel work streams:

**Stream A: Infrastructure (YAML + Docker Compose)**

- Rewrite PowerSync YAML sync queries to match actual Postgres columns
- Remove all streams/queries referencing non-existent tables
- Keep only tables that have migrations: users, households, household_memberships, initiatives, tags, attachments, entity_relations
- Reduce on-demand streams to only what can work (initiative_detail with existing columns; remove note_detail, item_history, quest_detail)
- Base64url-encode the JWT secret in Docker Compose default
- Update the YAML `k` field handling or document that the env var must be base64url-encoded
- Update `docs/schema/altair-initial-schema.sql` to match actual migrations

**Stream B: Rust Backend Fixes**

1. `config.rs`: Require `POWERSYNC_JWT_SECRET` in production (use `require_env` when `APP_ENV == "production"`), log warning in dev when using default. Remove trivial getter doc comments (lines 62-120 comments only, keep the methods).
2. `jwt.rs`: Replace `.expect("System clock is before UNIX epoch")` with `.map_err(|e| AppError::Internal(...))?`. Make `PowerSyncClaims` fields `pub(crate)` instead of `pub`. Keep `Deserialize` for tests but document the constraint.
3. `handlers.rs`: Use `config.powersync_url()` getter instead of `config.powersync_url.clone()` for consistency with other handlers.
4. Extract a shared `test_config()` into a test utilities module or `config.rs` so `jwt.rs`, `db/mod.rs` tests can reuse it instead of duplicating.

**Stream C: TypeScript Frontend Fixes**

1. `powersync-client.ts` -- AltairConnector.fetchCredentials:
   - Fix auth: The backend requires `Authorization: Bearer <token>`. The web app's auth uses Better Auth cookies for SvelteKit routes, but the Rust backend uses its own Bearer token auth. For the PowerSync token endpoint, the connector needs to obtain the session token and send it as a Bearer header. Since the web app stores the session token from login/register responses, add an `Authorization` header. If the token is not available client-side (cookie-only auth), then the approach needs a SvelteKit server endpoint proxy. For now, since the Rust backend is the auth source (not Better Auth), update the comment and add a `TODO` noting this needs the session token passed in, or create a SvelteKit API route that proxies the request with the server-side session.
   - **Simplest correct fix**: Create a SvelteKit API route (`/api/powersync-token`) that proxies to the Rust backend, forwarding the Bearer token from the SvelteKit session. The connector calls this SvelteKit endpoint with `credentials: 'include'` (cookies). OR, if the client already has the bearer token (e.g., stored in a cookie or local storage), pass it directly.
   - **Pragmatic approach for now**: Since this is a debug/proof-of-life phase and the auth integration between SvelteKit (Better Auth) and Rust backend is still being wired, update the comment to accurately describe the current state and add a clear TODO. The fetchCredentials should document that it requires a Bearer token and that the integration path is TBD.
   - Include response body in error messages
   - Wrap `response.json()` in try/catch, validate required fields

2. `powersync-client.ts` -- AltairConnector.uploadData:
   - Fix URL prefix: `${BACKEND_URL}/core/${table}` instead of `${BACKEND_URL}/api/${table}`
   - Add `response.ok` check after every fetch, throw with status and body on failure
   - Add `default` case to switch that throws for unknown operation types
   - Remove banner comment separators (keep JSDoc comments)

3. `powersync-client.ts` -- Singleton:
   - `initPowerSync`: Create client, try init+connect, on failure close client and re-throw (do not assign to `_db` until fully initialized)
   - `closePowerSync`: Null `_db` first, then close, catch and log close errors

4. `+page.svelte`:
   - Replace nested ternaries with lookup maps for `statusColor` and `statusLabel`
   - Add error logging in catch blocks: `console.error('[sync-debug] ...', err)`
   - Add table name allowlist check: `if (!/^[a-z_]+$/.test(table)) continue;`
   - Remove redundant `database` truthiness check in interval callback
   - Remove trivial HTML section comments (`<!-- Header -->`, etc.)

5. `+page.server.ts`:
   - Import `dev` from `$app/environment`
   - Gate: if not dev and no user, redirect to login. If dev and no user, log warning.

### Phase 3: Integration & Polish (Tests + Validation)

**Frontend Tests:**

`powersync-client.spec.ts`:

- Test fetchCredentials success returns correct shape
- Test fetchCredentials non-OK response throws with status and body
- Test fetchCredentials invalid JSON response throws descriptive error
- Test fetchCredentials missing fields throws validation error
- Test uploadData checks response.ok and throws on failure
- Test uploadData completes transaction on success
- Test uploadData default case throws for unknown op type
- Test initPowerSync returns same instance on second call
- Test initPowerSync resets on failure (allows retry)
- Test closePowerSync nulls singleton
- Test getPowerSyncDb returns null before init

`schema.spec.ts`:

- Test AppSchema contains expected table names
- Test SYNCED_TABLE_NAMES matches Object.keys(AppSchema.props)
- Test each table has expected columns (spot-check critical ones)

**Validation:**

- `cargo build` -- Rust compiles
- `cargo test` -- All Rust tests pass
- `cd apps/web && bun run build` -- SvelteKit builds
- `cd apps/web && bun run test` -- Vitest tests pass
- `cd apps/web && bun run lint` -- ESLint passes

## Team Orchestration

- You operate as the team lead and orchestrate the team to execute the plan.
- You're responsible for deploying the right team members with the right context to execute the plan.
- IMPORTANT: You NEVER operate directly on the codebase. You use `Task` and `Task*` tools to deploy team members to the building, validating, testing, deploying, and other tasks.
  - This is critical. Your job is to act as a high level director of the team, not a builder.
  - Your role is to validate all work is going well and make sure the team is on track to complete the plan.
  - You'll orchestrate this by using the Task\* Tools to manage coordination between the team members.
  - Communication is paramount. You'll use the Task\* Tools to communicate with the team members and ensure they're on track to complete the plan.
- Take note of the session id of each team member. This is how you'll reference them.

### Team Members

- Specialist
  - Name: builder-infra
  - Role: Fix PowerSync YAML sync queries, Docker Compose JWT encoding, and reference SQL to match actual Postgres migrations
  - Agent Type: general-purpose
  - Resume: true

- Specialist
  - Name: builder-backend
  - Role: Fix Rust backend security (JWT secret production requirement), correctness (SystemTime panic, PowerSyncClaims encapsulation), and code quality (shared test helper, getter consistency, comment cleanup)
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-frontend
  - Role: Fix TypeScript sync client (auth mechanism, URL prefix, response validation, singleton safety, closePowerSync), debug page (auth gate, error logging, SQL safety, nested ternaries, comments)
  - Agent Type: frontend-specialist
  - Resume: true

- Specialist
  - Name: builder-tests
  - Role: Write frontend unit tests for powersync-client and schema modules
  - Agent Type: quality-engineer
  - Resume: true

- Quality Engineer (Validator)
  - Name: validator
  - Role: Validate completed work against acceptance criteria (read-only inspection mode)
  - Agent Type: quality-engineer
  - Resume: false

## Step by Step Tasks

### 1. Fix PowerSync YAML and Infrastructure Config

- **Task ID**: fix-infra
- **Depends On**: none
- **Assigned To**: builder-infra
- **Agent Type**: general-purpose
- **Parallel**: true (can run alongside backend and frontend fixes)
- Rewrite `infra/powersync/powersync.yaml` sync queries to match actual Postgres migration columns EXACTLY (see Phase 1 column mapping above)
- Remove ALL queries referencing tables that do not have migrations: guidance_routines, guidance_epics, guidance_focus_sessions, guidance_quests, knowledge_notes, knowledge_note_snapshots, note_tags, note_attachments, tracking_locations, tracking_categories, tracking_items, tracking_item_events, tracking_shopping_lists, tracking_shopping_list_items, item_attachments, item_tags, quest_tags, quest_attachments
- Keep only these tables in sync streams: users, households, household_memberships, initiatives, tags, attachments, entity_relations
- Fix column names: `created_by` not `owner_user_id` for households, `user_id` not `owner_user_id` for initiatives/tags, `name` not `title` for initiatives, `joined_at` not `created_at` for household_memberships
- Remove columns that don't exist: timezone, default_household_id, is_active, deleted_at, slug, start_date, target_date, completed_at (except where they actually exist per migrations)
- Remove on-demand streams that reference non-existent tables (note_detail, item_history, quest_detail). Keep initiative_detail but fix its columns.
- Fix JWT secret in Docker Compose: the `k` field in JWKS requires base64url encoding. Either base64url-encode the default env var value, or add a comment documenting that the POWERSYNC_JWT_SECRET env var must be the base64url-encoded form of the signing key.
- Update `docs/schema/altair-initial-schema.sql` to match the actual migrations (add password_hash to users, fix households.created_by, fix initiatives columns, add sessions table reference, etc.)
- Update stream count in the header comment to reflect the reduced set
- Read ALL migration files in `apps/server/migrations/` to verify column names before writing

### 2. Fix Rust Backend Security and Correctness

- **Task ID**: fix-backend
- **Depends On**: none
- **Assigned To**: builder-backend
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside infra and frontend fixes)
- `apps/server/src/config.rs`:
  - Change `jwt_secret` loading: if `APP_ENV == "production"`, use `require_env("POWERSYNC_JWT_SECRET")`. Otherwise, use the dev default with a `tracing::warn!` when falling back.
  - Same pattern for `powersync_url`: warn in dev when using localhost default.
  - Remove trivial doc comments from getter methods (e.g., "Get the database URL as a string slice") -- the method names are self-documenting. Keep the methods themselves.
- `apps/server/src/auth/jwt.rs`:
  - Replace `.expect("System clock is before UNIX epoch")` with `.map_err(|e| AppError::Internal(format!("System clock error: {e}")))?`
  - Change `PowerSyncClaims` fields from `pub` to `pub(crate)` to restrict construction to within the crate
- `apps/server/src/auth/handlers.rs`:
  - Change `config.powersync_url.clone()` to `config.powersync_url().to_string()` for consistency with existing handler patterns that use getter methods
- Extract a shared `test_config()` function. Add a `#[cfg(test)] pub fn test_default() -> Config` method on `Config` itself in config.rs, then update jwt.rs and db/mod.rs tests to use `Config::test_default()` instead of duplicating the struct literal.
- Run `cargo build` and `cargo test` to verify all changes compile and tests pass

### 3. Fix TypeScript Sync Client and Debug Page

- **Task ID**: fix-frontend
- **Depends On**: none
- **Assigned To**: builder-frontend
- **Agent Type**: frontend-specialist
- **Parallel**: true (can run alongside infra and backend fixes)
- `apps/web/src/lib/sync/powersync-client.ts`:
  - **fetchCredentials auth fix**: Update the comment from "validates the session cookie" to accurately describe that the backend requires a Bearer token. Since the SvelteKit app uses Better Auth cookies and the Rust backend uses its own Bearer token auth, and the auth bridge is not yet wired, add a clear TODO explaining the integration gap. For now, keep `credentials: 'include'` and note that this will work once a SvelteKit proxy endpoint or shared auth is implemented.
  - **fetchCredentials error improvement**: Include response body in error: `const body = await response.text().catch(() => ''); throw new Error(\`Failed to fetch PowerSync token (${response.status}): ${body}\`);`
  - **fetchCredentials response validation**: Wrap `response.json()` in try/catch. After parsing, validate `data.token` and `data.powersync_url` are truthy strings before returning.
  - **uploadData URL prefix**: Change `${BACKEND_URL}/api/${table}` to `${BACKEND_URL}/core/${table}` for all three fetch calls (PUT, PATCH, DELETE). The backend routes are mounted under `/core/`.
  - **uploadData response checks**: After each fetch call, check `response.ok`. If not ok, read body and throw: `if (!resp.ok) { const body = await resp.text().catch(() => ''); throw new Error(\`${op.op} ${table}/${id} failed (${resp.status}): ${body}\`); }`
  - **uploadData default case**: Add `default: throw new Error(\`Unhandled CRUD operation type: ${op.op}\`);` to the switch statement
  - **Remove banner comments**: Remove the `// ---------------------------------------------------------------------------` separator lines (3 occurrences). Keep JSDoc comments.
  - **initPowerSync fix**: Restructure to create client in a local variable, try init+connect, only assign to `_db` after full success. On failure, attempt `client.close()` in a catch-and-ignore block, then re-throw.
  - **closePowerSync fix**: Save reference to `_db`, set `_db = null` immediately, then try to close. Catch and log any close errors.
- `apps/web/src/routes/(app)/debug/sync/+page.svelte`:
  - Replace nested ternary for `statusColor` with a `Record<string, string>` lookup map
  - Replace nested ternary for `statusLabel` with a `Record<string, string>` lookup map
  - In `refreshRowCounts` catch: capture error and log it: `catch (err) { console.error(\`[sync-debug] Failed to count ${table}:\`, err); counts[table] = -1; }`
  - In `refreshInitiatives` catch: capture error and log it: `catch (err) { console.error('[sync-debug] Failed to query initiatives:', err); initiatives = []; }`
  - Add table name validation guard: `if (!/^[a-z_]+$/.test(table)) continue;` before the SQL query
  - Remove redundant `database` truthiness check in interval callback (line 117, `database` is a const and cannot be null)
  - Remove trivial HTML comments: `<!-- Header -->`, `<!-- Status card -->`, `<!-- Row counts -->`, `<!-- Sample data -->`
  - Wrap interval callback body in try/catch to prevent unhandled promise rejections
- `apps/web/src/routes/(app)/debug/sync/+page.server.ts`:
  - Import `dev` from `$app/environment`
  - Replace commented-out auth gate with: `if (!dev && !locals.user) { redirect(302, '/login'); }` and keep the dev warning for when user is missing in dev mode
- `apps/web/src/lib/sync/schema.ts`:
  - Update header comment to clarify: "Column definitions here match the current Postgres migration columns. The PowerSync YAML sync rules and the design spec (docs/schema/altair-schema-design-spec.md) may reference additional columns from the target schema that do not yet exist."
- Run `cd apps/web && bun run build` and `bun run lint` to verify

### 4. Write Frontend Unit Tests

- **Task ID**: write-tests
- **Depends On**: fix-frontend
- **Assigned To**: builder-tests
- **Agent Type**: quality-engineer
- **Parallel**: false (must wait for frontend fixes to be applied first)
- Create `apps/web/src/lib/sync/powersync-client.spec.ts`:
  - Mock `fetch` using `vi.stubGlobal` or `vi.fn()`
  - Mock `@powersync/web` PowerSyncDatabase class
  - Test `fetchCredentials` success path returns `{ endpoint, token }`
  - Test `fetchCredentials` throws on non-OK response with status and body
  - Test `fetchCredentials` throws on invalid JSON response
  - Test `fetchCredentials` throws on missing required fields
  - Test `uploadData` calls correct URLs with `/core/` prefix
  - Test `uploadData` throws on non-OK fetch response (does not call `transaction.complete()`)
  - Test `uploadData` calls `transaction.complete()` on success
  - Test `uploadData` throws on unknown operation type
  - Test `initPowerSync` returns same instance on repeated calls
  - Test `initPowerSync` resets on failure (allows retry on next call)
  - Test `closePowerSync` sets singleton to null
  - Test `getPowerSyncDb` returns null before init
- Create `apps/web/src/lib/sync/schema.spec.ts`:
  - Test `AppSchema` contains exactly the expected 7 table names
  - Test `SYNCED_TABLE_NAMES` matches `Object.keys(AppSchema.props)`
  - Test `SYNCED_TABLE_NAMES` is readonly (TypeScript compile check)
  - Spot-check critical table columns (e.g., household_memberships has household_id, user_id, role, joined_at)
- Run `cd apps/web && bun run test` to verify all tests pass

### 5. Validate All Changes

- **Task ID**: validate-all
- **Depends On**: fix-infra, fix-backend, fix-frontend, write-tests
- **Assigned To**: validator
- **Agent Type**: quality-engineer
- **Parallel**: false
- Run `cargo build` -- verify Rust compiles without errors or warnings
- Run `cargo test` -- verify all Rust tests pass
- Run `cd apps/web && bun run build` -- verify SvelteKit builds
- Run `cd apps/web && bun run test` -- verify all Vitest tests pass
- Run `cd apps/web && bun run lint` -- verify ESLint passes
- Verify PowerSync YAML only references tables/columns that exist in `apps/server/migrations/`
- Verify `docs/schema/altair-initial-schema.sql` matches actual migrations
- Verify no `response.ok` checks are missing in `powersync-client.ts`
- Verify `initPowerSync` properly resets `_db` on failure
- Verify `closePowerSync` nulls `_db` before closing
- Verify debug page auth gate uses `dev` import
- Verify no nested ternaries remain in `+page.svelte`
- Report pass/fail for each acceptance criterion
- Operate in validation mode: inspect and report only, do not modify files

## Acceptance Criteria

1. `cargo build` succeeds with no errors
2. `cargo test` passes all tests (existing + updated)
3. `bun run build` succeeds in `apps/web/`
4. `bun run test` passes all tests (existing + new frontend tests)
5. `bun run lint` passes in `apps/web/`
6. PowerSync YAML references ONLY columns/tables that exist in actual Postgres migrations
7. `docs/schema/altair-initial-schema.sql` matches actual migration-created schema
8. JWT secret is required via `require_env` when `APP_ENV == "production"`
9. SystemTime uses `map_err` instead of `expect` in jwt.rs
10. All `fetch` calls in `uploadData` check `response.ok`
11. Upload URL prefix is `/core/` not `/api/`
12. `initPowerSync` does not leave a broken singleton on partial failure
13. `closePowerSync` nulls `_db` before calling close
14. Debug page has auth gate using `$app/environment` dev check
15. Debug page catch blocks log errors
16. No nested ternaries in `+page.svelte`
17. `powersync-client.spec.ts` has 10+ tests covering fetchCredentials, uploadData, and singleton
18. `schema.spec.ts` has 3+ structural tests

## Validation Commands

Execute these commands to validate the task is complete:

- `cd /home/rghamilton3/workspace/getaltair/altair && cargo build` -- Verify Rust compiles
- `cd /home/rghamilton3/workspace/getaltair/altair && cargo test` -- Run Rust tests
- `cd /home/rghamilton3/workspace/getaltair/altair/apps/web && bun run build` -- Verify SvelteKit builds
- `cd /home/rghamilton3/workspace/getaltair/altair/apps/web && bun run test` -- Run Vitest tests
- `cd /home/rghamilton3/workspace/getaltair/altair/apps/web && bun run lint` -- Run ESLint

## Notes

1. The local SQLite schema (`schema.ts`) already correctly matches the actual Postgres migrations. Do NOT change column names in schema.ts.
2. The PowerSync YAML is the primary source of schema drift -- it was written against the design spec, not the actual migrations.
3. The `docs/schema/altair-initial-schema.sql` reference file is also outdated vs actual migrations (e.g., missing password_hash on users, wrong column names).
4. The auth integration between SvelteKit (Better Auth cookies) and Rust backend (Bearer tokens) is an open architectural question. The fetchCredentials fix should document this gap clearly rather than implement a half-solution.
5. The `household_memberships` table does not have a `deleted_at` column in actual migrations, so the `get_user_household_ids` query in `service.rs` does not need a `deleted_at IS NULL` filter. The YAML sync queries should also not filter by `deleted_at` for this table.
6. For the JWT base64url encoding issue: PowerSync's JWKS `k` field per RFC 7517 must be base64url-encoded. The simplest fix is to base64url-encode the raw secret string and use that as the env var default in Docker Compose. The Rust server continues to sign with the raw bytes. Document this clearly.
7. Test files should use Vitest conventions (describe/it/expect) and should mock external dependencies (fetch, PowerSyncDatabase) rather than requiring a running backend.
