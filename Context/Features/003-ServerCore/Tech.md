# Tech Plan: Server Core

**Spec:** `Context/Features/003-ServerCore/Spec.md`
**Stacks involved:** Rust/Axum, SvelteKit 2/Svelte 5, PostgreSQL, Docker Compose

---

## Architecture Overview

The server is both identity provider and resource server. Auth is entirely within the server
crate — no external identity service. All domain routes are protected by an Axum request
extractor that validates the JWT and injects `AuthUser` into the handler.

The feature has three layers of work:

1. **Infrastructure cleanup** — Remove Zitadel/postgres-init from Compose; update env vars;
   delete OIDC web artifacts.
2. **Server implementation** — DB pool, auth module, AppError expansion, core CRUD, migrations.
3. **Web scaffold** — Minimal form-based login/register replacing the OIDC redirect flow.

```
Request
  └─> Axum router
        └─> Auth extractor (JWT from cookie or Authorization header)
              └─> AuthUser { user_id, household_ids } injected into extensions
                    └─> Handler → Service → sqlx query (PgPool from AppState)
```

PowerSync validates client JWTs using the server's JWKS endpoint instead of Zitadel's. The
`POWERSYNC_JWT_JWKS_URL` env var points at the server (`http://altair-server:3000/api/auth/.well-known/jwks.json`).

---

## Key Decisions

### Decision 1: RSA key pair management

**Context:** RS256 requires an RSA private key for signing and the public key in JWKS format for
verification. Keys generated at startup are lost on restart, invalidating all tokens.

**Options considered:**
- **Option A: Generate at startup, accept token invalidation on restart** — Zero operational
  overhead, but every server restart logs everyone out. Unacceptable for production.
- **Option B: Generate once, store private key as PEM in `.env`** — Persists across restarts;
  consistent with "secrets from environment" (invariant SEC-5); human-readable; easy to rotate
  by replacing the env var.
- **Option C: Key file on disk (mount into container)** — More operationally flexible but adds
  volume mount complexity to Docker Compose and a new file format to document.

**Chosen:** Option B — PEM string in `JWT_PRIVATE_KEY` environment variable.

**Rationale:** Consistent with how `DATABASE_URL` and other secrets work in this stack. A single
env var is easier to document for self-hosters than a file mount. Key rotation is `docker compose
down && update .env && docker compose up`. The `.env.example` will include a generation command:
`openssl genrsa -out - 2048 | base64 -w0`.

**Implementation:** Server reads `JWT_PRIVATE_KEY` (PEM, base64-encoded), decodes, and constructs
`EncodingKey::from_rsa_pem`. The public key is derived from it for JWKS and `DecodingKey`.
Both live in `AppState`.

**Related ADRs:** ADR-012 (RS256 preferred for production)

---

### Decision 2: Auth extraction — extractor vs. tower middleware layer

**Options considered:**
- **Option A: Tower middleware layer** — Runs on every request before handlers; centralised;
  rejects unauthenticated requests early. Requires special-casing public routes (health, auth
  endpoints) with `without_auth` wrappers.
- **Option B: Axum `FromRequestParts` extractor on `AuthUser`** — Lazy; only runs when the
  handler declares `AuthUser` as a parameter. Public routes simply don't declare it. Idiomatic
  Axum.

**Chosen:** Option B — `FromRequestParts` extractor.

**Rationale:** Axum's extractor pattern is the idiomatic approach and avoids the "allowlist" anti-
pattern where every new public route must explicitly opt out of auth middleware. The extractor
validates the JWT from either the `Authorization: Bearer` header (Android) or the `access_token`
httpOnly cookie (web). Returns `401 Unauthorized` if neither is present or the token is invalid.

**`AuthUser` struct:**
```rust
pub struct AuthUser {
    pub user_id: Uuid,
    pub household_ids: Vec<Uuid>,
}
```

---

### Decision 3: Refresh token storage — raw vs. hashed

**Options considered:**
- **Option A: Store raw opaque token** — Simple; comparison is direct string equality.
- **Option B: Store SHA-256 hash of the token** — If the `refresh_tokens` table is read by an
  attacker, raw tokens cannot be replayed. The raw token is only returned to the client once
  (at issue time) and never stored server-side.

**Chosen:** Option B — store `SHA-256(raw_token)` as `token_hash TEXT UNIQUE`.

**Rationale:** Refresh tokens have 7-day lifetime. A DB dump leak would give an attacker 7 days
to use stolen tokens. Hashing eliminates this. The hashing overhead (`sha2` crate, ~microseconds)
is negligible. Pattern mirrors how session stores handle session IDs.

**Token generation:** 32 random bytes (`rand::thread_rng().gen::<[u8; 32]>()`), hex-encoded →
64-character string returned to client.

---

### Decision 4: Login identifier — email vs. username

**Context:** The Spec refers to "username + password" but the ERD and existing `users` migration
use `email VARCHAR(255) NOT NULL UNIQUE` as the login identifier.

**Chosen:** Use `email` as the login identifier, consistent with the ERD.

**Rationale:** Email is already the unique column in the schema. Introducing a separate `username`
field adds a migration and a second unique identifier. For a household app, email works as a
login credential and the `display_name` field covers how users appear in the UI. Registration
accepts `email` + `display_name` + `password`.

---

### Decision 5: Users table migration approach

**Context:** The existing migration (`20260412000002_create_users.up.sql`) was written for OIDC
and contains `oidc_sub TEXT NOT NULL UNIQUE` with no `password_hash`. Invariant D-2 prohibits
modifying applied migrations.

**Chosen:** Add a new migration (`000005_migrate_users_to_builtin_auth.up.sql`) that:
1. Drops `oidc_sub` column (OIDC-specific, superseded by ADR-012)
2. Adds `password_hash TEXT` (nullable first) then sets `NOT NULL` — safe on an empty table;
   documented as requiring no live user rows (dev only at this stage)
3. Adds `is_admin BOOLEAN NOT NULL DEFAULT false`
4. Adds `status TEXT NOT NULL DEFAULT 'active'` (CHECK constraint: `active`, `pending`)

**Rationale:** Additive migration preserves invariant D-2. The `oidc_sub` drop is acceptable
because no real user data exists yet (Feature 003 is before any domain feature). The two-step
nullable→not-null pattern for `password_hash` is safety hygiene even though the table is empty.

---

### Decision 6: First-user admin race condition

**Context:** The "first registration is admin" logic requires atomically checking whether any
users exist and creating the first user as admin.

**Chosen:** Accept benign race, no special locking.

**Rationale:** For a household self-hosted server (1-5 users), simultaneous first-registration
is a vanishingly rare edge case. Two concurrent first-time admins is a harmless outcome — both
users are admin, which is valid. The cost of serializable isolation or advisory locks to prevent
this would outweigh the benefit. Document the behaviour in a code comment.

---

## Stack-Specific Details

### Rust/Axum (`apps/server/server/`)

**New dependencies to add via `cargo add`:**

| Crate | Version | Purpose |
|---|---|---|
| `argon2` | latest | Argon2id password hashing |
| `jsonwebtoken` | latest | JWT encode/decode, RS256, JWKS |
| `rand` | latest | Refresh token generation |
| `sha2` | latest | SHA-256 hashing of refresh tokens |
| `hex` | latest | Hex encoding for refresh token string |
| `rsa` | latest | RSA key pair parsing for JWKS derivation |

**Module structure to create:**

```
apps/server/server/src/
├── auth/
│   ├── mod.rs         -- re-exports; router assembly: auth_router()
│   ├── models.rs      -- RegisterRequest, LoginRequest, TokenResponse,
│   │                      AuthUser (FromRequestParts impl), JwksResponse
│   ├── service.rs     -- hash_password, verify_password, issue_token_pair,
│   │                      verify_access_token, rotate_refresh_token, revoke_refresh_token
│   └── handlers.rs    -- register, login, refresh, logout, me, jwks
├── core/
│   ├── mod.rs
│   ├── initiatives/
│   │   ├── mod.rs
│   │   ├── models.rs  -- Initiative, CreateInitiativeRequest, UpdateInitiativeRequest
│   │   ├── service.rs -- list, get, create, update, delete (all user-scoped)
│   │   └── handlers.rs
│   ├── tags/
│   │   ├── mod.rs
│   │   ├── models.rs
│   │   ├── service.rs
│   │   └── handlers.rs
│   └── relations/
│       ├── mod.rs
│       ├── models.rs  -- EntityRelation, CreateRelationRequest
│       ├── service.rs -- validate against contracts registry, CRUD
│       └── handlers.rs
├── db/
│   └── mod.rs         -- build_pool() → PgPool; run_migrations()
├── config.rs          -- add JWT_PRIVATE_KEY field
├── error.rs           -- expand with Unauthorized, Forbidden, BadRequest, Conflict,
│                          UnprocessableEntity (ADR-011)
└── main.rs            -- AppState { db: PgPool, enc_key: EncodingKey,
                           dec_key: DecodingKey }; wire db + auth + core routers
```

**AppState:**
```rust
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub enc_key: jsonwebtoken::EncodingKey,   // for signing JWTs
    pub dec_key: jsonwebtoken::DecodingKey,   // for verifying JWTs
    pub jwks_json: String,                    // pre-serialized JWKS response
}
```

**Route layout:**
```
GET  /health                                  -- existing; no auth
POST /api/auth/register                       -- no auth
POST /api/auth/login                          -- no auth
POST /api/auth/refresh                        -- no auth (refresh token in body)
POST /api/auth/logout                         -- requires AuthUser
GET  /api/auth/me                             -- requires AuthUser
GET  /api/auth/.well-known/jwks.json          -- no auth

GET  /api/initiatives                         -- requires AuthUser
POST /api/initiatives                         -- requires AuthUser
GET  /api/initiatives/:id                     -- requires AuthUser
PATCH /api/initiatives/:id                    -- requires AuthUser
DELETE /api/initiatives/:id                   -- requires AuthUser

GET  /api/tags                                -- requires AuthUser
POST /api/tags                                -- requires AuthUser
DELETE /api/tags/:id                          -- requires AuthUser

POST /api/entity_relations                    -- requires AuthUser
GET  /api/entity_relations                    -- requires AuthUser
DELETE /api/entity_relations/:id              -- requires AuthUser
```

**Argon2id parameters** (per ADR-012, OWASP minimum):
```rust
Params::new(19 * 1024, 2, 1, None)  // 19MiB memory, 2 iterations, 1 parallelism
```

**JWT claims:**
```rust
#[derive(Serialize, Deserialize)]
struct Claims {
    sub: Uuid,              // user UUID
    household_ids: Vec<Uuid>,
    iat: i64,
    exp: i64,
    // kid: String         // key ID matching JWKS entry
}
```

Access token lifetime: 15 minutes. JWT header includes `kid` matching the JWKS entry.

---

### SvelteKit (`apps/web/`)

**Files to delete:**
- `src/lib/auth/pkce.ts`
- `src/lib/auth/pkce.spec.ts`
- `src/routes/auth/callback/+page.server.ts`
- `src/routes/auth/callback/+page.svelte`

**Files to modify:**
- `src/routes/auth/login/+page.server.ts` — replace OIDC redirect with a SvelteKit form `actions`
  block: POST email + password to `/api/auth/login`; on success, set `access_token` and
  `refresh_token` cookies (httpOnly, SameSite=Lax); `redirect(302, '/')`.

**Files to create:**
- `src/routes/auth/login/+page.svelte` — login form with email + password fields; uses design
  tokens for basic legibility (colours, spacing); error display; no full DESIGN.md polish
  (deferred to Feature 009).
- `src/routes/auth/register/+page.server.ts` — form action: POST email + display_name + password
  to `/api/auth/register`; handles 201 (first user, redirect to home) and 202 (pending, show
  "awaiting approval" message).
- `src/routes/auth/register/+page.svelte` — registration form.
- `src/hooks.server.ts` — read `access_token` cookie, decode JWT, set `event.locals.user` for
  SSR auth checks; refresh on expiry using `refresh_token` cookie.
- `src/app.d.ts` — extend `App.Locals` with `user: { id: string; email: string } | null`.

**Environment variables:** Remove `PUBLIC_ZITADEL_CLIENT_ID` and `PUBLIC_ZITADEL_ISSUER` from
any `.env` files. Add `PUBLIC_API_BASE_URL=http://localhost:3000` for API calls.

---

### Infrastructure (`infra/`)

**`infra/compose/docker-compose.yml`:**
- Remove `postgres-init` service entirely
- Remove `zitadel` service entirely
- Add `altair-server` service (build from `apps/server/`; exposes port 3000; depends on postgres)
- Resulting services: `postgres`, `mongodb`, `powersync`, `rustfs`, `altair-server`

**Note:** PLAN-001/ADR-002 lists the stack as 4 containers (postgres, mongodb, powersync, rustfs).
The Axum server is typically run outside Compose during development (via `cargo run`) and only
added to Compose for a full-stack integration test. The compose file should support both modes via
a `--profile server` flag or a separate `docker-compose.prod.yml` override. Decision to be
finalised during Steps.md — both approaches are valid.

**`infra/compose/.env.example`:**

Remove:
- `ZITADEL_MASTERKEY`
- `ZITADEL_ADMIN_PAT`
- `ZITADEL_CLIENT_ID`
- `ZITADEL_JWKS_URL`

Add:
```
# Server auth (RS256 JWT)
# Generate: openssl genrsa 2048 | base64 -w0
JWT_PRIVATE_KEY=<base64-encoded PEM>

# PowerSync: now uses the server's JWKS endpoint
POWERSYNC_JWT_JWKS_URL=http://altair-server:3000/api/auth/.well-known/jwks.json
```

**`infra/compose/powersync.yml`:**
- Update comment: "JWKS endpoint from Altair server (ADR-012)" replacing the Zitadel reference
- `${POWERSYNC_JWT_JWKS_URL}` continues to work via env var substitution — no structural change

---

### Database migrations (`infra/migrations/`)

Migrations numbered from 000005 (continuing existing sequence). Each migration has `.up.sql` and
`.down.sql`.

| # | Migration | Notes |
|---|---|---|
| 000005 | `migrate_users_to_builtin_auth` | Drop `oidc_sub`; add `password_hash`, `is_admin`, `status` |
| 000006 | `create_refresh_tokens` | Auth token rotation table |
| 000007 | `create_initiatives` | Core domain |
| 000008 | `create_tags` | Core domain |
| 000009 | `create_entity_tags` | Junction tables (quest_tags, note_tags, item_tags, initiative_tags) |
| 000010 | `create_attachments` | Attachment metadata (binaries in object storage per ADR-005) |
| 000011 | `create_entity_relations` | Cross-domain graph table |
| 000012 | `create_guidance_epics` | Guidance domain |
| 000013 | `create_guidance_quests` | Guidance domain |
| 000014 | `create_guidance_routines` | Guidance domain |
| 000015 | `create_guidance_focus_sessions` | Guidance domain |
| 000016 | `create_guidance_daily_checkins` | Guidance domain (UNIQUE on user_id, checkin_date) |
| 000017 | `create_knowledge_notes` | Knowledge domain |
| 000018 | `create_knowledge_note_snapshots` | Immutable — no `updated_at` (invariant E-6) |
| 000019 | `create_tracking_locations` | Tracking domain |
| 000020 | `create_tracking_categories` | Tracking domain |
| 000021 | `create_tracking_items` | Tracking domain |
| 000022 | `create_tracking_item_events` | Append-only — no `updated_at`/`deleted_at` (invariant D-5) |
| 000023 | `create_tracking_shopping_lists` | Tracking domain |
| 000024 | `create_tracking_shopping_list_items` | Tracking domain |

All schemas follow `docs/specs/05-erd.md`. Indices in `05-erd.md` are included in the relevant
table migration (not separate index migrations).

---

## Integration Points

### PowerSync ↔ Server JWKS

PowerSync calls `${POWERSYNC_JWT_JWKS_URL}` to fetch the server's public key and validate client
JWTs. The server must be reachable from within the Docker Compose network when PowerSync starts.

The JWKS response format (RFC 7517 JWK Set):
```json
{
  "keys": [{
    "kty": "RSA",
    "use": "sig",
    "alg": "RS256",
    "kid": "altair-v1",
    "n": "<base64url-encoded modulus>",
    "e": "AQAB"
  }]
}
```

PowerSync's `client_auth.jwks_urls` array in `powersync.yml` already handles this via the env
var. No structural change to `powersync.yml` is needed beyond the comment update.

### Web ↔ Server auth cookies

The SvelteKit `hooks.server.ts` intercepts every SSR request, reads the `access_token` cookie,
and populates `event.locals.user`. This is used by `+layout.server.ts` to gate protected routes.

Cookie settings:
```
Set-Cookie: access_token=<JWT>; HttpOnly; SameSite=Lax; Path=/; Max-Age=900
Set-Cookie: refresh_token=<opaque>; HttpOnly; SameSite=Lax; Path=/api/auth/refresh; Max-Age=604800
```

The `refresh_token` cookie's `Path` is scoped to `/api/auth/refresh` so it is not sent to
every API request — only the token refresh endpoint.

### Contracts registry ↔ Entity relations validation

`apps/server/server/src/contracts.rs` (from Feature 002) already contains `ENTITY_TYPES` and
`RELATION_TYPES` string constants. The `entity_relations` service validates `from_entity_type`,
`to_entity_type`, and `relation_type` against these slices before inserting.

Note: The ERD's `entity_relations` table uses `from_entity_type`/`to_entity_type` rather than
`source_type`/`target_type` as the Spec describes. The ERD column names are authoritative —
Spec.md FA-011/FA-012 assertions reference "source_type/target_type" as conceptual terms; the
actual columns are `from_entity_type` and `to_entity_type`.

---

## Risks & Unknowns

**Risk: RSA key not set in env**
- The server fails to start if `JWT_PRIVATE_KEY` is missing or malformed.
- **Mitigation:** Fail fast in `Config::from_env()` with a clear error message: "JWT_PRIVATE_KEY
  is required — generate with: openssl genrsa 2048 | base64 -w0". Include a pre-generated
  development key in `.env.example` comments (clearly marked "DO NOT USE IN PRODUCTION").

**Risk: Migration 000005 fails if `users` table has rows**
- `ALTER TABLE users ALTER COLUMN password_hash SET NOT NULL` fails if any row has NULL.
- **Mitigation:** The two-step add-nullable then set-not-null pattern is safe on empty tables.
  Since no user-facing feature exists yet, the table will always be empty when this runs.
  Document in the migration file header: "Requires empty users table — safe for dev/CI."

**Risk: PowerSync can't reach server JWKS endpoint**
- If the server isn't running when PowerSync starts, PowerSync may fail to validate tokens.
- **Mitigation:** PowerSync caches the JWKS after first fetch. The server must be healthy before
  the first client connects. Start order: postgres → server → powersync. Add `depends_on` in
  Compose if the server is included in the compose file.

**Risk: Cookie `SameSite=Lax` blocks cross-origin PowerSync auth**
- PowerSync's web SDK makes requests from the same origin (SvelteKit app), so `SameSite=Lax` is
  fine. Android uses the token from the response body, not cookies. No issue expected.
- **Mitigation:** Verify in integration test (FA-001 + PowerSync connectivity check).

**Implementation note: JWKS serialisation**
- The `rsa` crate is included in the dependency list for this purpose. Use
  `rsa::RsaPublicKey` to extract the modulus (`n`) and exponent (`e`), then base64url-encode
  them for the JWK Set response. The JWKS handler is a good first task to implement and verify
  before building the rest of auth.

---

## Testing Strategy

**Unit tests (in `#[cfg(test)]` modules):**
- `error.rs` — each AppError variant maps to the correct HTTP status and body (extends existing tests)
- `auth/service.rs` — hash/verify roundtrip; JWT issue/verify roundtrip; refresh token hash
- `core/relations/service.rs` — unknown entity type rejects; known entity type accepts

**Integration tests (`apps/server/tests/`):**
- Auth flow: register (first user → admin), register (second user → pending), login active user,
  login pending user (403), refresh token rotation, logout
- Data isolation: User A cannot see User B's initiatives (FA-009)
- Entity relations: invalid type rejected with 422 (FA-011, FA-012)
- Migrations: apply to fresh DB and revert (FA-013, FA-014)
- JWKS: endpoint returns valid JWK Set (FA-015)

Use `sqlx::test` with transaction rollback for DB-backed tests. See `.claude/rules/rust-axum.md`.

**Web tests (`apps/web/`):**
- `bun run check` passes after OIDC file deletion (FA-017)
- No unit tests for the login form in this feature — E2E deferred to Feature 009

---

## Revision History

| Date | Change | ADR |
|---|---|---|
| 2026-04-12 | Initial tech plan | ADR-011, ADR-012 |
