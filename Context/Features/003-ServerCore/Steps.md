# Implementation Steps: Server Core

**Spec:** `Context/Features/003-ServerCore/Spec.md`
**Tech:** `Context/Features/003-ServerCore/Tech.md`

---

## Progress
- **Status:** Complete
- **Current task:** —
- **Last milestone:** M9 — Auth integration test coverage complete (2026-04-13)
- **Results:** 71 cargo tests pass (63 unit + 8 auth integration); 13 Vitest tests pass; 0 clippy warnings; bun check 0 errors; all 18 FA assertions covered; note: sqlx::test tests require DATABASE_URL env var pointing to running postgres

---

## Team Orchestration

### Team Members

- **builder-infra**
  - Role: Infrastructure cleanup — Docker Compose, env files, PowerSync config
  - Agent Type: general-purpose
  - Resume: false

- **builder-rust**
  - Role: Axum server — AppError, AppState, auth module, migrations, core CRUD
  - Agent Type: general-purpose
  - Resume: true

- **builder-web**
  - Role: SvelteKit auth scaffold — delete OIDC artifacts, login/register forms, hooks
  - Agent Type: general-purpose
  - Resume: true

- **validator**
  - Role: Quality validation — read-only inspection of all outputs
  - Agent Type: general-purpose
  - Resume: false

---

## Tasks

### Phase 1: Infrastructure Cleanup

Delete Zitadel from Compose and OIDC code from the web app. These are independent work streams
and run in parallel.

- [ ] S001: Remove `zitadel` and `postgres-init` services from `infra/compose/docker-compose.yml`
  - **Assigned:** builder-infra
  - **Depends:** none
  - **Parallel:** true
  - **Detail:** Delete the `zitadel` and `postgres-init` service blocks entirely. Remove the Zitadel
    section from `volumes:` if present (there is none — confirm no `zitadel_data` volume). Remove
    any `depends_on: postgres-init` references. The resulting services are: `postgres`, `mongodb`,
    `powersync`, `rustfs`. Leave the `altair-server` addition for a later step (see Tech.md note
    on server-in-compose).

- [ ] S001-T: Validate docker-compose.yml structure after Zitadel removal
  - **Assigned:** builder-infra
  - **Depends:** S001
  - **Parallel:** false
  - **Scenarios:** `docker compose config` exits 0 with no Zitadel references; exactly 4 services
    defined; no dangling `depends_on` references (FA-001)

- [ ] S002: Update `infra/compose/.env.example` to remove Zitadel vars and add JWT vars
  - **Assigned:** builder-infra
  - **Depends:** S001
  - **Parallel:** false
  - **Detail:** Remove: `ZITADEL_MASTERKEY`, `ZITADEL_ADMIN_PAT`, `ZITADEL_CLIENT_ID`,
    `ZITADEL_JWKS_URL`. Update: `POWERSYNC_JWT_JWKS_URL=http://altair-server:3000/api/auth/.well-known/jwks.json`.
    Add under a `# Server auth (RS256 JWT)` heading:
    `JWT_PRIVATE_KEY=<base64-encoded PEM>` with generation command in comment:
    `# Generate: openssl genrsa 2048 | base64 -w0`.
    Add a clearly marked dev-only pre-generated key as a comment block (label: "DO NOT USE IN
    PRODUCTION"). Add `SERVER_URL=http://localhost:3000` for web app API calls.

- [ ] S003: Update `infra/compose/powersync.yml` comment
  - **Assigned:** builder-infra
  - **Depends:** S001
  - **Parallel:** true
  - **Detail:** Replace the comment `# JWKS endpoint from Zitadel for validating client JWTs`
    with `# JWKS endpoint from Altair server — see ADR-012`. No structural change; env var
    substitution continues to work.

- [ ] S004: Delete OIDC artifacts from web app
  - **Assigned:** builder-web
  - **Depends:** none
  - **Parallel:** true
  - **Detail:** Delete the following files/directories:
    - `apps/web/src/lib/auth/pkce.ts`
    - `apps/web/src/lib/auth/pkce.spec.ts`
    - `apps/web/src/routes/auth/callback/+page.server.ts`
    - `apps/web/src/routes/auth/callback/+page.svelte`
    - Replace `apps/web/src/routes/auth/login/+page.server.ts` with an empty stub that exports
      `export const actions = {};` — the real form action is wired in Phase 7. This keeps the
      route valid so `bun run check` passes now.
    Remove the `PUBLIC_ZITADEL_CLIENT_ID` and `PUBLIC_ZITADEL_ISSUER` references. Add
    `PUBLIC_API_BASE_URL` to `apps/web/src/app.d.ts` or a `.env.example` if needed for type
    checking.

- [ ] S004-T: Verify web app type-checks cleanly after OIDC deletion
  - **Assigned:** builder-web
  - **Depends:** S004
  - **Parallel:** false
  - **Scenarios:** `bun run check` exits 0 in `apps/web/`; no references to
    `PUBLIC_ZITADEL_CLIENT_ID`, `PUBLIC_ZITADEL_ISSUER`, or `pkce` in source tree; callback
    route directory does not exist (FA-017)

---

🏁 **MILESTONE 1: Infrastructure cleaned up**
Verify: FA-001 (`docker compose config` shows 4 services), FA-017 (`bun run check` passes)
**Contracts:**
- `infra/compose/docker-compose.yml` — 4-service stack for downstream integration tests
- `infra/compose/.env.example` — updated env var reference for team and self-hosters

---

### Phase 2: Server Foundations

Wire the Axum application state and expand AppError before writing any handlers.

- [ ] S005: Add new Cargo dependencies to `apps/server/server/Cargo.toml`
  - **Assigned:** builder-rust
  - **Depends:** none
  - **Parallel:** false
  - **Detail:** Use `cargo add` from `apps/server/`:
    `cargo add argon2`, `cargo add jsonwebtoken`, `cargo add rand --features std`,
    `cargo add sha2`, `cargo add hex`, `cargo add rsa --features pem`.
    Verify `cargo build` succeeds after adding.

- [ ] S006: Expand `AppError` in `apps/server/server/src/error.rs` (ADR-011)
  - **Assigned:** builder-rust
  - **Depends:** S005
  - **Parallel:** false
  - **Detail:** Add variants: `Unauthorized` → 401, `Forbidden` → 403,
    `BadRequest(String)` → 400, `Conflict(String)` → 409,
    `UnprocessableEntity(String)` → 422. Keep existing `NotFound` and `Internal`.
    Update `IntoResponse` match arm. `BadRequest`, `Conflict`, and `UnprocessableEntity`
    return the contained message in the JSON body. `Internal` continues to return the
    generic message (no detail leak, already tested).

- [ ] S006-T: Test all AppError variants produce correct HTTP status and response body
  - **Assigned:** builder-rust
  - **Depends:** S006
  - **Parallel:** false
  - **Scenarios:** `Unauthorized` → 401 `{"error":"Unauthorized"}`; `Forbidden` → 403;
    `BadRequest("bad input")` → 400 `{"error":"bad input"}`; `Conflict("duplicate")` → 409;
    `UnprocessableEntity("invalid type")` → 422; `Internal` → 500 without leaking detail
    (extends existing tests in error.rs)

- [ ] S007: Add `JWT_PRIVATE_KEY` to `Config` and build `AppState`
  - **Assigned:** builder-rust
  - **Depends:** S005
  - **Parallel:** true
  - **Detail:** In `config.rs`: add `jwt_private_key_pem: String` field read from
    `JWT_PRIVATE_KEY` env var (base64-decoded). In `main.rs`: define
    `AppState { db: PgPool, enc_key: EncodingKey, dec_key: DecodingKey, jwks_json: String }`.
    Derive `Clone`. Parse the PEM key at startup to build `EncodingKey::from_rsa_pem` and
    extract the public key for `DecodingKey` and the pre-serialized JWKS response.
    Fail fast with a clear message if `JWT_PRIVATE_KEY` is absent or malformed.

- [ ] S007-T: Config and key parsing tests
  - **Assigned:** builder-rust
  - **Depends:** S007
  - **Parallel:** false
  - **Scenarios:** Missing `JWT_PRIVATE_KEY` → startup error with actionable message;
    valid PEM key parses successfully; `EncodingKey` and `DecodingKey` round-trip a test JWT;
    JWKS JSON string contains `"kty":"RSA"` and `"alg":"RS256"`

- [ ] S008: Add `db/mod.rs` with `build_pool()` and `run_migrations()`
  - **Assigned:** builder-rust
  - **Depends:** S007
  - **Parallel:** false
  - **Detail:** `build_pool(database_url)` → `PgPool` using `sqlx::postgres::PgPoolOptions`.
    `run_migrations(pool)` calls `sqlx::migrate!("../../infra/migrations").run(pool)` (path
    relative to `apps/server/server/`). Wire both into `main()`. Verify `cargo build` succeeds.

---

🏁 **MILESTONE 2: Server foundations complete**
Verify: `cargo build` succeeds; `cargo test` passes for error.rs and config.rs
**Contracts:**
- `apps/server/server/src/error.rs` — full AppError taxonomy for all downstream handlers
- `apps/server/server/src/main.rs` — AppState shape (db, enc_key, dec_key, jwks_json)
- `apps/server/server/src/config.rs` — Config with JWT_PRIVATE_KEY

---

### Phase 3: Auth Schema Migrations

Create the migrations that the auth module requires.

- [ ] S009: Migration 000005 — alter `users` table for built-in auth
  - **Assigned:** builder-rust
  - **Depends:** S008
  - **Parallel:** false
  - **Detail:** Create `infra/migrations/20260412000005_migrate_users_to_builtin_auth.up.sql`:
    ```sql
    ALTER TABLE users DROP COLUMN IF EXISTS oidc_sub;
    ALTER TABLE users ADD COLUMN password_hash TEXT;
    ALTER TABLE users ALTER COLUMN password_hash SET NOT NULL;
    ALTER TABLE users ADD COLUMN is_admin BOOLEAN NOT NULL DEFAULT false;
    ALTER TABLE users ADD COLUMN status TEXT NOT NULL DEFAULT 'active'
      CHECK (status IN ('active', 'pending'));
    ```
    Header comment: `-- Requires empty users table; safe for dev/CI. See ADR-012.`
    Down migration: reverse each ALTER in reverse order; restore `oidc_sub TEXT NOT NULL UNIQUE`
    with a placeholder value to satisfy the NOT NULL.

- [ ] S009-T: Migration 000005 applies and reverts cleanly
  - **Assigned:** builder-rust
  - **Depends:** S009
  - **Parallel:** false
  - **Scenarios:** Up migration exits 0 on fresh DB; `users` table has `password_hash`,
    `is_admin`, `status` columns and no `oidc_sub`; down migration restores original schema
    (FA-013, FA-014 partial)

- [ ] S010: Migration 000006 — create `refresh_tokens` table
  - **Assigned:** builder-rust
  - **Depends:** S009
  - **Parallel:** false
  - **Detail:** Columns: `id UUID PK DEFAULT gen_random_uuid()`, `user_id UUID FK → users NOT NULL`,
    `token_hash TEXT UNIQUE NOT NULL`, `device_hint TEXT NULL`,
    `expires_at TIMESTAMPTZ NOT NULL`, `revoked_at TIMESTAMPTZ NULL`,
    `created_at TIMESTAMPTZ NOT NULL DEFAULT now()`. Index on `user_id`. Index on `expires_at`
    (for cleanup). No `updated_at` trigger needed. Down migration: `DROP TABLE refresh_tokens`.

- [ ] S010-T: Migration 000006 applies and reverts
  - **Assigned:** builder-rust
  - **Depends:** S010
  - **Parallel:** false
  - **Scenarios:** Table created with correct columns; UNIQUE on token_hash enforced; FK to
    users enforced; down migration drops table cleanly

---

🏁 **MILESTONE 3: Auth schema ready**
Verify: migrations 000001–000006 apply cleanly to a fresh DB (`sqlx migrate run`)
**Contracts:**
- `infra/migrations/20260412000005_migrate_users_to_builtin_auth.up.sql` — users table shape
  for auth service
- `infra/migrations/20260412000006_create_refresh_tokens.up.sql` — refresh token table for
  auth service

---

### Phase 4: Auth Implementation

Build the full auth module in dependency order: models → service → handlers → extractor.

- [ ] S011: Create `auth/mod.rs` and `auth/models.rs`
  - **Assigned:** builder-rust
  - **Depends:** S010
  - **Parallel:** false
  - **Detail:** `auth/models.rs` types:
    - `RegisterRequest { email: String, display_name: String, password: String }`
    - `LoginRequest { email: String, password: String }`
    - `TokenResponse { access_token: String, refresh_token: String, token_type: String }`
    - `RegisterResponse` (enum or struct handling 201 vs 202 case)
    - `UserProfile { id: Uuid, email: String, display_name: String, is_admin: bool }`
    - `Claims { sub: Uuid, household_ids: Vec<Uuid>, iat: i64, exp: i64 }`
    - `AuthUser { user_id: Uuid, household_ids: Vec<Uuid> }` (extractor shell — impl in S016)
    - `JwksResponse { keys: Vec<JwkKey> }`, `JwkKey { kty, use_, alg, kid, n, e }`
    All types derive `Serialize`/`Deserialize` as appropriate.
    `auth/mod.rs`: re-exports; `pub fn auth_router() -> Router<AppState>` stub.

- [ ] S012: Create `auth/service.rs` — password hashing and JWT logic
  - **Assigned:** builder-rust
  - **Depends:** S011
  - **Parallel:** false
  - **Detail:**
    - `hash_password(password: &str) -> Result<String, AppError>` — Argon2id with
      `Params::new(19 * 1024, 2, 1, None)` (OWASP minimum per ADR-012)
    - `verify_password(password: &str, hash: &str) -> Result<(), AppError>` — returns
      `Unauthorized` on mismatch; constant-time comparison
    - `issue_access_token(user_id: Uuid, household_ids: Vec<Uuid>, enc_key: &EncodingKey) -> Result<String, AppError>`
      — 15-minute expiry; RS256; `kid: "altair-v1"` in header
    - `generate_refresh_token() -> (String, String)` — returns `(raw_token, token_hash)`;
      raw token is 32 random bytes hex-encoded; hash is `SHA-256(raw_token)` hex-encoded
    - `store_refresh_token(pool, user_id, token_hash, device_hint) -> Result<(), AppError>`
    - `rotate_refresh_token(pool, raw_token, enc_key, user_id, household_ids) -> Result<TokenResponse, AppError>`
      — looks up hash, verifies not revoked/expired, revokes old, stores new, issues new pair
    - `revoke_refresh_token(pool, raw_token) -> Result<(), AppError>`

- [ ] S012-T: Auth service unit tests
  - **Assigned:** builder-rust
  - **Depends:** S012
  - **Parallel:** false
  - **Scenarios:** hash/verify roundtrip succeeds; wrong password → `AppError::Unauthorized`;
    JWT encodes `sub` and `exp` correctly; JWT decode verifies signature; refresh token hash
    differs from raw token; revoked token returns error on rotate

- [ ] S013: JWKS handler — `GET /api/auth/.well-known/jwks.json`
  - **Assigned:** builder-rust
  - **Depends:** S012
  - **Parallel:** false
  - **Detail:** Handler returns pre-serialized `jwks_json` from `AppState` with
    `Content-Type: application/json`. Extract RSA public key from the PEM at startup (in
    `main.rs` AppState construction) using `rsa::RsaPublicKey`: extract modulus `n` and
    exponent `e`, base64url-encode each (no padding), build `JwkKey { kty: "RSA",
    use_: "sig", alg: "RS256", kid: "altair-v1", n: <base64url>, e: <base64url> }`.
    Wire route into `auth_router()`.

- [ ] S013-T: JWKS endpoint test
  - **Assigned:** builder-rust
  - **Depends:** S013
  - **Parallel:** false
  - **Scenarios:** GET returns 200 with `Content-Type: application/json`; response parses as
    `JwksResponse`; `keys` array has exactly one entry; entry has `kty: "RSA"`, `alg: "RS256"`,
    non-empty `n` and `e` fields (FA-015)

- [ ] S014: Register and login handlers
  - **Assigned:** builder-rust
  - **Depends:** S013
  - **Parallel:** false
  - **Detail:**
    `register`:
    - Validate email format and password minimum length (8 chars)
    - Count existing users in transaction; if 0 → `is_admin=true, status='active'`;
      else → `is_admin=false, status='pending'`
    - Hash password with Argon2id
    - INSERT into users; handle UNIQUE violation → `AppError::Conflict`
    - If active: issue token pair, return 201 with `TokenResponse` + set httpOnly cookies
    - If pending: return 202 `{"message": "Account created. An admin must approve before you can log in."}`
    `login`:
    - Fetch user by email; if not found → `AppError::Unauthorized` (no hint whether email exists)
    - Verify password with Argon2id; if mismatch → `AppError::Unauthorized`
    - Check `status = 'active'`; if pending → `AppError::Forbidden("Account pending admin approval")`
    - Issue token pair; set httpOnly cookies for web; return `TokenResponse` in body (for Android)
    Cookie settings: `access_token` (HttpOnly, SameSite=Lax, Path=/, MaxAge=900);
    `refresh_token` (HttpOnly, SameSite=Lax, Path=/api/auth/refresh, MaxAge=604800)

- [ ] S014-T: Register and login integration tests
  - **Assigned:** builder-rust
  - **Depends:** S014
  - **Parallel:** false
  - **Scenarios:** First register → 201 with tokens, user is admin and active (FA-002);
    second register → 202 no tokens, user is pending (FA-003);
    duplicate email → 409 (FA-004);
    login active user → 200 with valid JWT (FA-005);
    login pending user → 403 (FA-006);
    login wrong password → 401 (FA-007)

- [ ] S015: Refresh, logout, and me handlers
  - **Assigned:** builder-rust
  - **Depends:** S014
  - **Parallel:** false
  - **Detail:**
    `refresh` (POST /api/auth/refresh): read `refresh_token` from body or cookie;
    call `rotate_refresh_token`; return new `TokenResponse` + set new cookies; clear old cookies.
    `logout` (POST /api/auth/logout, requires `AuthUser`): revoke refresh token from body or
    cookie; clear both cookies; return 204.
    `me` (GET /api/auth/me, requires `AuthUser`): fetch user by `auth_user.user_id`; return
    `UserProfile`.

- [ ] S015-T: Refresh, logout, me tests
  - **Assigned:** builder-rust
  - **Depends:** S015
  - **Parallel:** false
  - **Scenarios:** Valid refresh → new token pair, old token revoked (FA-010);
    replayed old refresh token → 401; logout clears cookies and revokes token;
    GET /me returns correct user profile (FA-016)

- [ ] S016: `AuthUser` extractor (`FromRequestParts` implementation)
  - **Assigned:** builder-rust
  - **Depends:** S015
  - **Parallel:** false
  - **Detail:** Implement `FromRequestParts<AppState>` for `AuthUser`. Extraction order:
    1. Check `Authorization: Bearer <token>` header
    2. Fall back to `access_token` httpOnly cookie
    If neither present → `AppError::Unauthorized`. If present but invalid/expired →
    `AppError::Unauthorized`. Decode with `DecodingKey` from `AppState`; validate `exp`.
    Populate `AuthUser { user_id: claims.sub, household_ids: claims.household_ids }`.

- [ ] S016-T: AuthUser extractor tests
  - **Assigned:** builder-rust
  - **Depends:** S016
  - **Parallel:** false
  - **Scenarios:** No token → 401 on protected route (FA-008); expired token → 401;
    valid Bearer token → AuthUser populated; valid cookie token → AuthUser populated;
    `user_id` matches `sub` claim

---

🏁 **MILESTONE 4: Auth fully functional**
Verify: FA-002 through FA-010, FA-015, FA-016
Run: `cargo test` for auth module
**Contracts:**
- `apps/server/server/src/auth/models.rs` — `AuthUser`, `TokenResponse` for CRUD handlers
- `apps/server/server/src/auth/mod.rs` — `auth_router()` signature

---

### Phase 5: Domain Migrations

Create all remaining domain table migrations. These establish the schema that Features 005–007
will build on. Migrations are sequential (each depends on the previous).

- [ ] S017: Migrations 000007–000011 — core domain tables
  - **Assigned:** builder-rust
  - **Depends:** S010
  - **Parallel:** false
  - **Detail:** Create up + down migrations for each, following schemas in `docs/specs/05-erd.md`:
    - `000007_create_initiatives` — initiatives table + indices
    - `000008_create_tags` — tags table, UNIQUE(user_id, name)
    - `000009_create_entity_tags` — quest_tags, note_tags, item_tags, initiative_tags junction
      tables (composite PK + created_at per table)
    - `000010_create_attachments` — attachments table
    - `000011_create_entity_relations` — entity_relations table + forward/reverse/user indices
    All tables: `updated_at` trigger, `deleted_at` soft delete where applicable per ERD.

- [ ] S017-T: Core domain migrations apply and revert
  - **Assigned:** builder-rust
  - **Depends:** S017
  - **Parallel:** false
  - **Scenarios:** All 5 up migrations apply cleanly on fresh DB with 000001–000006 already
    applied; all 5 down migrations revert in reverse order without error (FA-013, FA-014 partial)

- [ ] S018: Migrations 000012–000018 — guidance and knowledge tables
  - **Assigned:** builder-rust
  - **Depends:** S017
  - **Parallel:** false
  - **Detail:** Create up + down migrations per `docs/specs/05-erd.md`:
    - `000012_create_guidance_epics`
    - `000013_create_guidance_quests` (FK to epics, routines, initiatives)
    - `000014_create_guidance_routines`
    - `000015_create_guidance_focus_sessions`
    - `000016_create_guidance_daily_checkins` — UNIQUE(user_id, checkin_date)
    - `000017_create_knowledge_notes`
    - `000018_create_knowledge_note_snapshots` — no `updated_at`/`deleted_at` (invariant E-6)
    All guidance/knowledge tables: updated_at trigger where applicable.

- [ ] S018-T: Guidance + knowledge migrations apply and revert
  - **Assigned:** builder-rust
  - **Depends:** S018
  - **Parallel:** false
  - **Scenarios:** 7 migrations apply cleanly; UNIQUE constraint on checkins verified;
    note_snapshots has no updated_at column; all revert cleanly

- [ ] S019: Migrations 000019–000024 — tracking tables
  - **Assigned:** builder-rust
  - **Depends:** S018
  - **Parallel:** false
  - **Detail:** Create up + down migrations per `docs/specs/05-erd.md`:
    - `000019_create_tracking_locations`
    - `000020_create_tracking_categories`
    - `000021_create_tracking_items` — quantity NUMERIC NOT NULL DEFAULT 0
    - `000022_create_tracking_item_events` — no `updated_at`/`deleted_at` (invariant D-5)
    - `000023_create_tracking_shopping_lists`
    - `000024_create_tracking_shopping_list_items`

- [ ] S019-T: Tracking migrations apply and revert
  - **Assigned:** builder-rust
  - **Depends:** S019
  - **Parallel:** false
  - **Scenarios:** 6 migrations apply cleanly; item_events has no updated_at column;
    all 24 migrations apply to a completely fresh DB in one run; all 24 revert cleanly (FA-013,
    FA-014)

---

🏁 **MILESTONE 5: All 24 migrations complete**
Verify: FA-013 (sqlx migrate run on fresh DB exits 0), FA-014 (all revert cleanly)
Run: `sqlx migrate run` + `sqlx migrate revert` (all 24) against a test DB
**Contracts:**
- `infra/migrations/` (000007–000024 up.sql files) — full schema for domain CRUD (Features 005–007)
  and sync engine (Feature 004)

---

### Phase 6: Core CRUD

Implement the three core CRUD modules. Tags and entity_relations are independent of initiatives
and can be built in parallel.

- [ ] S020: Initiatives module — `src/core/initiatives/{mod,models,service,handlers}.rs`
  - **Assigned:** builder-rust
  - **Depends:** S016, S017
  - **Parallel:** false
  - **Detail:**
    `models.rs`: `Initiative`, `CreateInitiativeRequest { title, description?, status? }`,
    `UpdateInitiativeRequest` (all fields optional), `InitiativeListResponse`.
    `service.rs`: All queries include `WHERE user_id = $user_id` (invariant SEC-1).
    `list_initiatives(pool, user_id) → Vec<Initiative>`.
    `get_initiative(pool, id, user_id) → Result<Initiative, AppError>` — 404 if not found
    or belongs to another user.
    `create_initiative`, `update_initiative`, `delete_initiative` (soft delete: set deleted_at).
    `handlers.rs`: thin Axum handlers; extract `AuthUser`; delegate to service.
    Wire into `core_router()` mounted at `/api/initiatives`.

- [ ] S020-T: Initiatives CRUD tests
  - **Assigned:** builder-rust
  - **Depends:** S020
  - **Parallel:** false
  - **Scenarios:** Create initiative → 201; list returns only user's own initiatives (FA-009 —
    User A list does not include User B's initiative); GET by ID with wrong user → 404;
    soft delete sets deleted_at; PATCH updates only provided fields

- [ ] S021: Tags module — `src/core/tags/{mod,models,service,handlers}.rs`
  - **Assigned:** builder-rust
  - **Depends:** S016, S017
  - **Parallel:** true
  - **Detail:** `models.rs`: `Tag`, `CreateTagRequest { name }`.
    `service.rs`: All queries scoped to `user_id`. UNIQUE constraint on (user_id, name) →
    catch sqlx unique violation → `AppError::Conflict("Tag name already exists")`.
    `list_tags`, `create_tag`, `delete_tag` (hard delete — tags are user-owned labels,
    not entities with history).
    Wire into `core_router()` at `/api/tags`.

- [ ] S021-T: Tags CRUD tests
  - **Assigned:** builder-rust
  - **Depends:** S021
  - **Parallel:** false
  - **Scenarios:** Create tag → 201; duplicate tag name for same user → 409;
    same tag name for different user succeeds; delete removes tag; list scoped to user

- [ ] S022: Entity relations module — `src/core/relations/{mod,models,service,handlers}.rs`
  - **Assigned:** builder-rust
  - **Depends:** S016, S017
  - **Parallel:** true
  - **Detail:** `models.rs`: `EntityRelation`, `CreateRelationRequest { from_entity_type,
    from_entity_id, to_entity_type, to_entity_id, relation_type, source_type,
    evidence? }`.
    `service.rs`:
    Registry validation (invariants C-1, C-2): check `from_entity_type` and `to_entity_type`
    against `ENTITY_TYPES` slice from `contracts.rs`; check `relation_type` against
    `RELATION_TYPES` slice. Invalid values → `AppError::UnprocessableEntity`.
    `source_type` validation: must be one of `user`, `ai`, `import`, `rule`, `migration`,
    `system`.
    All queries scoped to `user_id`. `list_relations`, `create_relation`,
    `delete_relation` (soft delete).
    Wire into `core_router()` at `/api/entity_relations`.

- [ ] S022-T: Entity relations validation and CRUD tests
  - **Assigned:** builder-rust
  - **Depends:** S022
  - **Parallel:** false
  - **Scenarios:** Unknown `relation_type` → 422 (FA-011); unknown `from_entity_type` → 422
    (FA-012); unknown `to_entity_type` → 422; valid relation → 201; list scoped to user;
    delete sets deleted_at

---

🏁 **MILESTONE 6: Core CRUD complete**
Verify: FA-009, FA-011, FA-012; `cargo test` passes for core/ module
Run: `cargo test` in `apps/server/`
**Contracts:**
- `apps/server/server/src/core/initiatives/models.rs` — Initiative shape for web client
- `apps/server/server/src/core/relations/models.rs` — EntityRelation shape for web client

---

### Phase 7: Web Auth Scaffold

Replace the deleted OIDC stub with real form-based auth. The login/register stub from S004 is
replaced with working form actions.

- [ ] S023: Implement form action in `apps/web/src/routes/auth/login/+page.server.ts`
  - **Assigned:** builder-web
  - **Depends:** S014
  - **Parallel:** false
  - **Detail:** Replace the empty stub from S004. Export `actions = { default: async (event) => {
    ...} }`. Extract `email` and `password` from `event.request.formData()`. POST to
    `${PUBLIC_API_BASE_URL}/api/auth/login`. On 200: set `access_token` and `refresh_token`
    as httpOnly cookies (re-set from server response Set-Cookie headers, or manually from
    response body if the server returns them in JSON). `redirect(303, '/')`. On 401: return
    `fail(401, { error: 'Invalid email or password' })`. On 403: return
    `fail(403, { error: 'Account pending admin approval' })`.

- [ ] S024: Create `apps/web/src/routes/auth/login/+page.svelte`
  - **Assigned:** builder-web
  - **Depends:** S023
  - **Parallel:** false
  - **Detail:** Form with email + password fields; submit to `?/default` action. Display
    `form.error` if present. Use design tokens from `DESIGN.md` for basic styling
    (background color, text color, border radius, spacing). No full DESIGN.md component polish
    (deferred to Feature 009). The form must be functional and legible.

- [ ] S025: Create register route — `+page.server.ts` + `+page.svelte`
  - **Assigned:** builder-web
  - **Depends:** S014
  - **Parallel:** true
  - **Detail:** `+page.server.ts`: form action POSTing `email`, `display_name`, `password` to
    `/api/auth/register`. On 201: set cookies + `redirect(303, '/')`. On 202: return
    `{ pending: true, message: "Account created. Awaiting admin approval." }` (show in page).
    On 409: return `fail(409, { error: 'Email already registered' })`.
    `+page.svelte`: email + display_name + password fields; show pending message or error.
    Link to login page.

- [ ] S026: Create `apps/web/src/hooks.server.ts` and extend `src/app.d.ts`
  - **Assigned:** builder-web
  - **Depends:** S023, S025
  - **Parallel:** false
  - **Detail:** `hooks.server.ts`: on each request, read `access_token` cookie; if present,
    decode JWT (without verification — verification happens server-side on API calls; this is
    for populating locals for SSR rendering). Set `event.locals.user = { id, email }` or `null`.
    Handle decode failure gracefully (malformed token → `locals.user = null`).
    `app.d.ts`: extend `App.Locals` with `user: { id: string; email: string } | null`.

- [ ] S026-T: Web type-check and auth flow smoke test
  - **Assigned:** builder-web
  - **Depends:** S026
  - **Parallel:** false
  - **Scenarios:** `bun run check` exits 0 (FA-017 final); no references to Zitadel/OIDC/PKCE
    in `apps/web/src/`; login page renders without JS errors; register page renders

- [ ] S026-D: Update `CLAUDE.md` active work section
  - **Assigned:** builder-web
  - **Depends:** S026
  - **Parallel:** false
  - **Detail:** Update the `## Active Work` section to: Feature 003 Server Core — status
    complete. Next: Feature 004 Sync Engine. Add note: auth endpoints at
    `POST /api/auth/register`, `POST /api/auth/login`; web login at `/auth/login`.

---

🏁 **MILESTONE 7: Web auth scaffold complete**
Verify: FA-017 (`bun run check` passes); login and register pages exist and render

---

### Phase 8: Full Validation

- [ ] S027: Run full `cargo test` suite and confirm all pass
  - **Assigned:** validator
  - **Depends:** S022, S026
  - **Parallel:** false
  - **Detail:** Run `cargo test` in `apps/server/`. All tests must pass. Flag any failing tests
    with the exact error. Do not fix — report to builder-rust. (FA-018)

- [ ] S028: Integration verification against all FA assertions
  - **Assigned:** validator
  - **Depends:** S027
  - **Parallel:** false
  - **Detail:** Read-only check. For each FA assertion in Spec.md, verify it is addressed by
    a test or can be manually verified. Produce a checklist:
    - FA-001: `docker compose config` shows 4 services
    - FA-002 through FA-016: covered by test tasks S009-T through S022-T
    - FA-017: covered by S004-T and S026-T
    - FA-018: covered by S027
    Flag any assertion with no corresponding test or implementation.

---

🏁 **MILESTONE 8: Feature complete**
Verify: all FA-001 through FA-018 assertions checked; `cargo test` green; `bun run check` green;
no TODO/FIXME stubs in auth/ or core/ modules

---

---

### Phase 9: Auth Integration Tests (added post-review)

Tasks added from PR review findings P4-014, P4-015, P4-016, P4-025, P4-026, P4-028.

- [ ] S029: Auth handler integration tests — register, login, refresh, logout
  - **Assigned:** builder-rust
  - **Depends:** S015
  - **Relates to:** P4-014
  - **Scenarios (sqlx::test):** first register → 201, user is admin and active; second register
    → 202, user is pending; duplicate email → 409; login active user → 200 with valid JWT
    containing email claim; login pending user → 403; login wrong password → 401;
    logout with valid token → 204, token revoked; replay old token after rotate → 401

- [ ] S030: `rotate_refresh_token` security invariant tests
  - **Assigned:** builder-rust
  - **Depends:** S015
  - **Relates to:** P4-015
  - **Scenarios (sqlx::test):** revoked token returns 401; expired token returns 401; token
    not found returns 401; valid rotation revokes old token and returns new pair

- [ ] S031: `hooks.server.ts` JWT decode unit tests (Vitest)
  - **Assigned:** builder-web
  - **Depends:** S026
  - **Relates to:** P4-016
  - **Scenarios:** truncated cookie (undefined payload) → locals.user is null; JWT with
    valid structure but non-JSON payload → null; token missing exp → null;
    valid token with email claim → user populated with correct email and id

- [ ] S032: Initiative service sqlx::test integration tests replacing SQL string assertions
  - **Assigned:** builder-rust
  - **Depends:** S020
  - **Relates to:** P4-028
  - **Scenarios (sqlx::test):** insert two users' initiatives; assert each user's list
    contains only their own records (FA-009 at DB level); PATCH updates only provided fields;
    GET by wrong user → NotFound; soft delete sets deleted_at

- [ ] S033-T: `extract_token_from_body_or_cookie` priority contract test
  - **Note:** Already added inline in handlers.rs tests (P4-025 resolved inline)

- [ ] S034-T: Extractor malformed `Authorization` header tests
  - **Note:** Already added inline in extractor.rs tests (P4-026 resolved inline)

---

🏁 **MILESTONE 9: Auth integration test coverage complete**
Verify: `cargo test` green with sqlx::test auth integration tests; Vitest passing for hooks

---

## Acceptance Criteria
- [ ] All 18 testable assertions (FA-001–FA-018) verified
- [ ] `cargo test` passes in `apps/server/`
- [ ] `bun run check` passes in `apps/web/`
- [ ] `docker compose config` shows exactly 4 services
- [ ] No TODO/FIXME stubs remaining in `auth/` or `core/` modules
- [ ] All 24 migrations apply to a fresh DB and revert cleanly
- [ ] No Zitadel/OIDC references remaining in active source files

## Validation Commands
```bash
# Infrastructure
docker compose -f infra/compose/docker-compose.yml config | grep -c "image:"  # expect 4

# Server
cd apps/server && cargo test 2>&1 | tail -5
cd apps/server && cargo clippy -- -D warnings

# Web
cd apps/web && bun run check

# Migrations (requires running postgres)
cd apps/server && sqlx migrate run --source ../../infra/migrations
cd apps/server && sqlx migrate revert --source ../../infra/migrations  # repeat 24x

# No OIDC references remaining
grep -r "zitadel\|oidc_sub\|pkce\|code_verifier" apps/web/src/ apps/server/server/src/  # expect no output
```
