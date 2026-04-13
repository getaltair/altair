# Feature 003: Server Core

| Field | Value |
|---|---|
| **Feature** | 003-ServerCore |
| **Version** | 1.0 |
| **Status** | Draft |
| **Date** | 2026-04-12 |
| **ADRs** | ADR-011, ADR-012 |
| **Source Docs** | `docs/specs/01-PRD-001-core.md`, `docs/specs/03-invariants.md`, `docs/specs/05-erd.md`, `docs/specs/10-PLAN-001-v1.md` (Step 3) |

---

## Overview

Feature 003 delivers the server backbone that every other feature depends on: authentication,
per-user data isolation, core domain CRUD (initiatives, tags, entity relations), and the database
migrations for all domain tables. It also cleans up the OIDC scaffolding from Steps 1–2 that is
now superseded by ADR-012.

The server was previously scaffolded with a health endpoint and an AppError skeleton. This feature
fills in everything required before domain work (Steps 5–7) can begin.

---

## Problem Statement

After the architecture review (ADR-012), the stack has a gap: the Docker Compose configuration
still includes Zitadel and postgres-init services, the web app contains OIDC scaffolding (PKCE
helpers, callback route, login redirect), and the server has no auth system. Clients cannot
authenticate, and no domain data can be stored or retrieved. The whole stack must be functional
and testable end-to-end before domain-specific work starts.

---

## User Stories

- As a self-hoster, I want `docker compose up` to start a 4-container stack so I don't need to
  configure or run an identity provider.
- As a new user, I want to register with a username and password so I can access the system.
- As a returning user, I want to log in and receive tokens so the web and Android clients can make
  authenticated API calls.
- As a developer, I want all domain tables created via migrations so domain feature branches have
  a stable schema to build on.
- As a developer, I want CRUD endpoints for initiatives, tags, and entity relations so I can
  verify data isolation and contracts before building domain features.

---

## Requirements

### Must Have

**Infrastructure cleanup:**
- M-1: Remove `zitadel` and `postgres-init` services from `infra/compose/docker-compose.yml`
- M-2: Update `infra/compose/.env.example`: remove Zitadel vars; add `JWT_SECRET`; update
  `POWERSYNC_JWT_JWKS_URL` to point to the server JWKS endpoint
- M-3: Remove web OIDC artifacts: `apps/web/src/lib/auth/pkce.ts`,
  `apps/web/src/lib/auth/pkce.spec.ts`, `apps/web/src/routes/auth/callback/`,
  and the OIDC redirect logic in `apps/web/src/routes/auth/login/+page.server.ts`
- M-4: Replace the login route with a form-based login page that submits to the server's
  `/api/auth/login` endpoint

**Server — auth:**
- M-5: `POST /api/auth/register` — accepts username + password; if no users exist, creates the
  user as admin (`is_admin = true`, `status = active`) and returns tokens; otherwise creates the
  user as pending (`is_admin = false`, `status = pending`) and returns HTTP 202 with a message
  indicating the account awaits admin approval (no tokens issued)
- M-6: `POST /api/auth/login` — validates credentials and checks `status = active`; returns access
  token + refresh token on success; returns HTTP 403 if account is pending approval
- M-7: `POST /api/auth/refresh` — validates refresh token, issues new token pair (rotation),
  revokes old refresh token
- M-8: `POST /api/auth/logout` — revokes the current refresh token
- M-9: `GET /api/auth/me` — returns the authenticated user's profile
- M-10: `GET /api/auth/.well-known/jwks.json` — returns the server's public key set for
  PowerSync JWT validation
- M-11: Passwords hashed with Argon2id (OWASP minimum: 19MiB memory, 2 iterations, 1 parallelism;
  per ADR-012)
- M-12: Access tokens: JWT, RS256 or HS256, 15-minute expiry, claims `sub` (user UUID) and
  `household_ids` (array); per ADR-012
- M-13: Refresh tokens: opaque random string, 7-day expiry, stored in `refresh_tokens` table,
  rotated on each use (old token revoked)
- M-14: Auth middleware applied to all routes except `/api/auth/register`, `/api/auth/login`,
  `/api/auth/.well-known/jwks.json`, and `/health` (invariant SEC-2)

**Server — database:**
- M-15: sqlx `PgPool` wired into Axum application state; `DATABASE_URL` from config
- M-16: `sqlx migrate run` applied on startup (or via CI); all migrations reversible (invariant D-1)
- M-17: `users` table migration adds `is_admin BOOLEAN NOT NULL DEFAULT false` and
  `status TEXT NOT NULL DEFAULT 'active'` columns (additive, per invariant D-2)
- M-18: Migration for `refresh_tokens` table (id, user_id FK, token_hash, expires_at, revoked_at,
  created_at)
- M-19: Migrations for all domain tables: `initiatives`, `tags`, `entity_tags`, `attachments`,
  `entity_relations`, guidance tables (epics, quests, routines, focus_sessions, daily_checkins),
  knowledge tables (notes, note_snapshots), tracking tables (locations, categories, items,
  item_events, shopping_lists, shopping_list_items)

**Server — core CRUD:**
- M-20: `initiatives` CRUD (`GET /api/initiatives`, `POST`, `GET /:id`, `PATCH /:id`,
  `DELETE /:id`) scoped to the authenticated user (invariant SEC-1)
- M-21: `tags` CRUD scoped to the authenticated user
- M-22: `entity_relations` CRUD — create, read, delete; `source_type`/`target_type`/
  `relation_type` validated against the shared contracts registry (invariants C-1, C-2)

**Server — error handling:**
- M-23: `AppError` expanded with variants: `Unauthorized`, `Forbidden`, `BadRequest(String)`,
  `Conflict(String)`, `UnprocessableEntity(String)` (per ADR-011)
- M-24: No internal error details leaked in HTTP responses (already tested in error.rs)

### Should Have

- S-1: `GET /api/households` and `POST /api/households` — household creation and listing
- S-2: `POST /api/households/:id/members` — invite member to household
- S-3: Username uniqueness enforced at registration (database unique constraint + `Conflict` error)
- S-4: `GET /api/admin/users` — admin-only list of all users with their status (active/pending)
- S-5: `POST /api/admin/users/:id/approve` — admin-only endpoint to activate a pending user

### Won't Have (this feature)

- Domain CRUD for Guidance, Knowledge, or Tracking entities — those are Features 005, 006, 007
- Sync endpoints (`/sync/push`, `/sync/pull`) — Feature 004
- Attachment upload/download — Feature 010
- Web login form styling against DESIGN.md — Feature 009 (Web Client)
- Admin UI or household management UI
- Token revocation beyond logout (e.g., revoke-all-sessions)
- OIDC migration path (documented in ADR-012 for v2)

---

## Testable Assertions

| ID | Assertion | Verification |
|---|---|---|
| FA-001 | `docker compose up` starts exactly 4 services: postgres, powersync, mongodb, rustfs | `docker compose ps` shows 4 running services; no zitadel or postgres-init |
| FA-002 | First `POST /api/auth/register` (empty user table) returns HTTP 201 with `access_token` and `refresh_token`; the created user has `is_admin = true` and `status = active` | Integration test; verify DB row |
| FA-003 | Second `POST /api/auth/register` (user table non-empty) returns HTTP 202 with no tokens; the created user has `status = pending` | Integration test; verify DB row |
| FA-004 | `POST /api/auth/register` with a duplicate username returns HTTP 409 | Integration test |
| FA-005 | `POST /api/auth/login` with correct credentials and `status = active` returns HTTP 200 with valid JWT `access_token` | curl / integration test; decode JWT, verify `sub` is a UUID |
| FA-006 | `POST /api/auth/login` for a user with `status = pending` returns HTTP 403 | Integration test |
| FA-007 | `POST /api/auth/login` with wrong password returns HTTP 401 | Integration test |
| FA-008 | `GET /api/initiatives` without a valid JWT returns HTTP 401 | curl with no/invalid token |
| FA-009 | `GET /api/initiatives` with a valid JWT returns HTTP 200 scoped to the authenticated user | Integration test: User A and User B each create an initiative; User A's response does not include User B's initiative |
| FA-010 | `POST /api/auth/refresh` with a valid refresh token returns new `access_token` and `refresh_token`; the old refresh token is revoked | Integration test: use old token after refresh → 401 |
| FA-011 | `POST /api/entity_relations` with an unknown `relation_type` returns HTTP 422 | Integration test with invalid relation_type string |
| FA-012 | `POST /api/entity_relations` with an unknown `source_type` or `target_type` returns HTTP 422 | Integration test |
| FA-013 | All domain table migrations apply cleanly against a fresh database | `sqlx migrate run` exits 0 on empty DB |
| FA-014 | All domain table migrations are reversible | `sqlx migrate revert` succeeds for each migration |
| FA-015 | `GET /api/auth/.well-known/jwks.json` returns a JSON document with a `keys` array | curl; response contains `keys` array with at least one entry |
| FA-016 | `GET /api/auth/me` with a valid JWT returns the authenticated user's id and username | curl / integration test |
| FA-017 | `bun run check` passes with no type errors after OIDC files are removed from the web app | `bun run check` exits 0 in `apps/web/` |
| FA-018 | `cargo test` passes for the server crate | `cargo test` exits 0 in `apps/server/` |

---

## Open Questions

- [x] **JWT algorithm**: RS256. PowerSync validates JWTs independently via the JWKS endpoint —
  HS256 would require sharing the signing secret with PowerSync, defeating the isolation.
  See M-10, FA-015.
- [x] **Migration location**: Keep `infra/migrations/`. Migrations are a deployment artifact
  shared by server, CI, and Docker Compose — co-locating them in the server crate would imply
  they're private to Rust. sqlx path configured via `SQLX_MIGRATIONS_PATH` or `--source` flag.
- [x] **Web login form scope**: Use design tokens (CSS custom properties from DESIGN.md) for
  legibility. Full component polish against DESIGN.md deferred to Feature 009.
- [x] **First-user bootstrap**: First registration creates an admin (`is_admin = true`,
  `status = active`) and returns tokens. All subsequent registrations create pending users
  (`status = pending`) with no tokens. Admin approves via `POST /api/admin/users/:id/approve`.
  Pending users cannot log in (HTTP 403). See M-5, M-6, M-17, S-4, S-5, FA-002, FA-003, FA-006.

---

## Revision History

| Date | Change | ADR |
|---|---|---|
| 2026-04-12 | Initial spec — replaces original 003 planning (cleaned-up worktree) | ADR-012 |
