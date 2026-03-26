# Plan: PowerSync Setup + Sync Proof-of-Life

## Task Description

Implement Step 7 of the Altair implementation plan: stand up the PowerSync sync service, configure sync stream definitions, add a JWT token endpoint to the Rust backend, integrate the `@powersync/web` SDK into the SvelteKit client, and prove the full round-trip works (server write -> client sync -> offline read -> offline mutation -> reconnect sync-back). This is the highest-risk integration point in the project and must be proven before building domain feature UIs.

## Objective

A working PowerSync sync pipeline: the backend issues JWTs for PowerSync client auth, PowerSync replicates scoped data from Postgres to client SQLite, and the web client can read data offline and sync offline mutations back to Postgres on reconnect.

## Problem Statement

Altair's offline-first architecture depends on PowerSync bridging Postgres (server) to SQLite (clients). Until this pipeline is proven, all downstream feature work (Steps 8-18) is blocked for client integration. The sync layer is the single highest-risk component -- if it doesn't work, the architecture decision (ADR-003) needs revisiting.

## Solution Approach

Three parallel work streams converging into an integration test:

1. **Infrastructure** -- PowerSync service in Docker Compose, configuration YAML with sync stream definitions
2. **Backend** -- JWT signing endpoint for PowerSync client auth, new config fields for JWT secret/PowerSync URL
3. **Web Client** -- `@powersync/web` SDK integration, local SQLite schema, auto-subscribe on login, debug view

The proof-of-life test validates the full round-trip across all three.

## Relevant Files

### Existing Files (read/modify)

- `infra/compose/docker-compose.yml` -- uncomment and configure PowerSync service block
- `apps/server/Cargo.toml` -- add `jsonwebtoken` crate dependency
- `apps/server/src/config.rs` -- add JWT secret + PowerSync URL config fields
- `apps/server/src/api/mod.rs` -- register new `/auth/powersync-token` route
- `apps/server/src/auth/handlers.rs` -- add `powersync_token` handler
- `apps/server/src/auth/service.rs` -- add JWT generation logic
- `apps/server/src/auth/models.rs` -- add PowerSync JWT claims struct
- `apps/web/package.json` -- add `@powersync/web` dependency
- `apps/web/src/hooks.server.ts` -- no changes expected but reference for auth flow
- `packages/contracts/registry/sync-streams.json` -- reference for stream names
- `packages/contracts/generated/typescript/syncStreams.ts` -- use in web client
- `packages/contracts/generated/rust/sync_stream.rs` -- reference in backend
- `docs/sync/altair-powersync-sync-spec.md` -- authoritative spec for stream definitions
- `docs/adr/ADR-003-sync-layer-selection.md` -- architectural context

### New Files

- `infra/powersync/powersync.yaml` -- PowerSync service configuration with sync stream definitions
- `apps/server/src/auth/jwt.rs` -- JWT signing module (PowerSync token generation)
- `apps/web/src/lib/sync/powersync-client.ts` -- PowerSync web SDK client setup + connector
- `apps/web/src/lib/sync/schema.ts` -- Local SQLite schema matching synced Postgres tables
- `apps/web/src/lib/sync/index.ts` -- barrel export for sync module
- `apps/web/src/routes/(app)/debug/sync/+page.svelte` -- debug view showing synced data
- `apps/web/src/routes/(app)/debug/sync/+page.server.ts` -- server load for debug page (auth gate)

## Implementation Phases

### Phase 1: Infrastructure + Backend (parallel tracks)

**Track A -- Infrastructure:**

- Create `infra/powersync/powersync.yaml` with PowerSync service config
- Define all sync streams from `altair-powersync-sync-spec.md` section 8
- Configure Postgres replication connection, JWT validation, and client API port
- Uncomment and update PowerSync service block in `docker-compose.yml`

**Track B -- Backend JWT endpoint:**

- Add `jsonwebtoken` crate to `apps/server/Cargo.toml`
- Add `jwt_secret` and `powersync_url` fields to `Config`
- Create `apps/server/src/auth/jwt.rs` with PowerSync JWT signing logic
- Add `POST /auth/powersync-token` handler returning a signed JWT
- JWT claims: `sub` (user_id), `iat`, `exp` (short-lived, 5 min), `user_id`, `household_ids`

### Phase 2: Web Client Integration

- Install `@powersync/web` in `apps/web/`
- Create local SQLite schema (`apps/web/src/lib/sync/schema.ts`) matching auto-subscribed tables
- Create PowerSync connector (`apps/web/src/lib/sync/powersync-client.ts`) that:
  - Fetches JWT from backend `/auth/powersync-token` endpoint
  - Connects to PowerSync service
  - Auto-subscribes to baseline streams on login
  - Handles upload of local mutations back to the backend API
- Create debug page (`/debug/sync`) showing sync status and table row counts

### Phase 3: Integration Test + Validation

- Create an initiative via the backend API
- Verify it appears in the web client's local SQLite within 5 seconds
- Disconnect from network, verify data still readable locally
- Create a tag while offline
- Reconnect and verify the tag syncs back to Postgres
- Verify authorization: client cannot access other users' data

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
  - Name: `infra-builder`
  - Role: Create PowerSync service configuration and Docker Compose integration
  - Agent Type: general-purpose
  - Resume: true

- Specialist
  - Name: `backend-jwt`
  - Role: Implement JWT signing endpoint for PowerSync client authentication in the Rust backend
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: `web-sync`
  - Role: Integrate @powersync/web SDK into SvelteKit client with local SQLite schema and debug view
  - Agent Type: frontend-specialist
  - Resume: true

- Quality Engineer (Validator)
  - Name: `validator`
  - Role: Validate completed work against acceptance criteria (read-only inspection mode)
  - Agent Type: quality-engineer
  - Resume: false

## Step by Step Tasks

- IMPORTANT: Execute every step in order, top to bottom. Each task maps directly to a `TaskCreate` call.
- Before you start, run `TaskCreate` to create the initial task list that all team members can see and execute.

### 1. Create Git Branch

- **Task ID**: create-branch
- **Depends On**: none
- **Assigned To**: `infra-builder`
- **Agent Type**: general-purpose
- **Parallel**: false
- Create and checkout branch `feat/step-7-powersync-setup` from `main`

### 2. PowerSync Service Configuration

- **Task ID**: powersync-config
- **Depends On**: create-branch
- **Assigned To**: `infra-builder`
- **Agent Type**: general-purpose
- **Parallel**: true (can run alongside tasks 3 and 4)
- Create directory `infra/powersync/`
- Create `infra/powersync/powersync.yaml` with:
  - Postgres replication connection config (host: `postgres`, port: 5432, database: `altair`, user: `altair`, password: `altair_dev`)
  - JWT validation config (RS256 or HS256 with shared secret -- use HS256 for simplicity in dev)
  - Client API on port 8080
  - All sync stream definitions from `docs/sync/altair-powersync-sync-spec.md` section 8:
    - **Auto-subscribed streams**: `my_profile`, `my_memberships`, `my_personal_data`, `my_household_data`, `my_relations`, `my_attachment_metadata`
    - **On-demand streams**: `initiative_detail`, `note_detail`, `item_history`, `quest_detail`
  - Each stream must include proper SQL queries filtered by `token_parameters.user_id` or household membership
  - Reference `packages/contracts/registry/sync-streams.json` for canonical stream names
- Uncomment and update PowerSync service block in `infra/compose/docker-compose.yml`:
  - Image: `journeyapps/powersync-service:latest`
  - Port: 8080:8080
  - Volume mount: `../powersync:/config`
  - Environment: `POWERSYNC_CONFIG=/config/powersync.yaml`
  - Depends on: postgres (service_healthy)
  - Add `POWERSYNC_JWT_SECRET` environment variable
- Add `powersync-data` volume if needed

**Context for agent**: Read `docs/sync/altair-powersync-sync-spec.md` for full stream definitions. Read `docs/schema/altair-initial-schema.sql` and `apps/server/migrations/` for table schemas. Use Context7 MCP to fetch current PowerSync documentation for the YAML config format. Use the `documentation-research` skill to look up PowerSync Sync Streams YAML configuration format.

### 3. Backend JWT Endpoint

- **Task ID**: backend-jwt-endpoint
- **Depends On**: create-branch
- **Assigned To**: `backend-jwt`
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside tasks 2 and 4)
- Add `jsonwebtoken = "9"` to `apps/server/Cargo.toml` dependencies
- Add config fields to `apps/server/src/config.rs`:
  - `jwt_secret: String` (from `POWERSYNC_JWT_SECRET` env var)
  - `powersync_url: String` (from `POWERSYNC_URL` env var, default `http://localhost:8080`)
- Create `apps/server/src/auth/jwt.rs`:
  - Define `PowerSyncClaims` struct with fields: `sub` (user_id as string), `iat`, `exp`, `user_id` (UUID string), `household_ids` (Vec of UUID strings)
  - Function `generate_powersync_token(config: &Config, user_id: Uuid, household_ids: Vec<Uuid>) -> Result<String>` that signs a JWT with HS256 using `config.jwt_secret`, 5-minute expiry
- Add handler in `apps/server/src/auth/handlers.rs`:
  - `pub async fn powersync_token(auth: AuthenticatedUser, State(pool): State<PgPool>, State(config): State<Config>) -> Result<Json<PowerSyncTokenResponse>, AppError>`
  - Fetches user's household IDs via `auth::service::get_user_household_ids`
  - Calls `jwt::generate_powersync_token` and returns `{ token: "...", powersync_url: "..." }`
- Add `PowerSyncTokenResponse` model to `apps/server/src/auth/models.rs`
- Register route `POST /auth/powersync-token` in `apps/server/src/api/mod.rs` (protected route)
- Update `apps/server/src/auth/mod.rs` to export the new `jwt` module
- Add unit test for JWT generation (valid claims, correct expiry, signature verifiable)

**Context for agent**: Read `apps/server/src/auth/service.rs` for `get_user_household_ids`. Read `apps/server/src/config.rs` for config pattern. Read `apps/server/src/auth/handlers.rs` and `apps/server/src/api/mod.rs` for routing pattern. Read `apps/server/src/auth/middleware.rs` for `AuthenticatedUser` extractor. Use Context7 MCP to check `jsonwebtoken` crate docs for HS256 signing.

### 4. Web Client PowerSync Integration

- **Task ID**: web-powersync-client
- **Depends On**: create-branch
- **Assigned To**: `web-sync`
- **Agent Type**: frontend-specialist
- **Parallel**: true (can run alongside tasks 2 and 3)
- Install `@powersync/web` in `apps/web/`: `cd apps/web && bun add @powersync/web`
- Create `apps/web/src/lib/sync/schema.ts`:
  - Define local SQLite schema using `@powersync/web` column types
  - Include tables matching auto-subscribed streams: `users`, `household_memberships`, `households`, `initiatives`, `tags`, `entity_relations`, `attachments`, `tracking_locations`, `tracking_categories`, `tracking_items`, `tracking_shopping_lists`, `tracking_shopping_list_items`
  - Column types must match Postgres schema (reference `docs/schema/altair-initial-schema.sql` and `apps/server/migrations/`)
- Create `apps/web/src/lib/sync/powersync-client.ts`:
  - Implement `PowerSyncBackendConnector` interface:
    - `fetchCredentials()` -- calls backend `/auth/powersync-token` and returns `{ endpoint, token }`
    - `uploadData(database)` -- iterates `CrudTransaction` queue and POSTs mutations to the appropriate backend API endpoints
  - Export factory function `createPowerSyncClient(backendUrl: string)` that:
    - Creates `PowerSyncDatabase` with the local schema
    - Initializes with the connector
    - Returns the database instance
- Create `apps/web/src/lib/sync/index.ts` -- barrel exports
- Create debug page `apps/web/src/routes/(app)/debug/sync/+page.svelte`:
  - Show PowerSync connection status (connected/disconnected/syncing)
  - Show row counts for each synced table
  - Show last sync timestamp
  - Button to manually trigger sync
  - Display raw data from one table (e.g., initiatives) for verification
- Create `apps/web/src/routes/(app)/debug/sync/+page.server.ts`:
  - Auth gate: redirect to login if no session

**Context for agent**: Read `apps/web/src/lib/api/client.ts` for the existing API client pattern. Read `packages/contracts/generated/typescript/syncStreams.ts` for stream enum constants. Read `apps/web/src/routes/(app)/+layout.server.ts` for auth pattern. Read `docs/sync/altair-powersync-sync-spec.md` for table list and scope rules. Read `apps/web/package.json` for existing dependencies. Use Context7 MCP to fetch `@powersync/web` documentation for SDK setup, `PowerSyncDatabase`, and `PowerSyncBackendConnector` interface.

### 5. Integration Validation

- **Task ID**: validate-integration
- **Depends On**: powersync-config, backend-jwt-endpoint, web-powersync-client
- **Assigned To**: `validator`
- **Agent Type**: quality-engineer
- **Parallel**: false
- Verify all new files exist and follow project conventions
- Verify PowerSync YAML contains all 10 streams (6 auto-subscribed + 4 on-demand)
- Verify each stream query filters by `token_parameters.user_id` or household membership (authorization check)
- Verify JWT endpoint is protected (requires `AuthenticatedUser` extractor)
- Verify JWT claims include `user_id` and `household_ids`
- Verify JWT expiry is short-lived (5 minutes)
- Verify local SQLite schema covers all auto-subscribed tables
- Verify `uploadData` connector handles CRUD mutations
- Verify Docker Compose PowerSync service depends on postgres health
- Verify no secrets are hardcoded (JWT secret comes from env var)
- Verify `apps/server/Cargo.toml` has `jsonwebtoken` dependency
- Verify debug page has auth gate
- Run `cargo check -p altair-server` to verify Rust compiles
- Run `cd apps/web && bun run check` to verify TypeScript compiles
- Operate in validation mode: inspect and report only, do not modify files

### 6. Final Validation

- **Task ID**: validate-all
- **Depends On**: validate-integration
- **Assigned To**: `validator`
- **Agent Type**: quality-engineer
- **Parallel**: false
- Run all validation commands listed below
- Verify all acceptance criteria are met
- Operate in validation mode: inspect and report only, do not modify files

## Acceptance Criteria

- [ ] PowerSync service configuration exists at `infra/powersync/powersync.yaml`
- [ ] PowerSync service starts in Docker Compose and connects to Postgres
- [ ] Backend issues signed JWTs via `POST /auth/powersync-token` for authenticated users
- [ ] JWT claims contain `user_id` and `household_ids` for PowerSync authorization
- [ ] JWT is short-lived (5-minute expiry)
- [ ] `@powersync/web` is installed in the web app
- [ ] Local SQLite schema covers all auto-subscribed tables from the sync spec
- [ ] PowerSync connector implements `fetchCredentials` and `uploadData`
- [ ] Debug page at `/debug/sync` shows connection status and synced data
- [ ] All 6 auto-subscribed streams defined in PowerSync config
- [ ] All 4 on-demand streams defined in PowerSync config
- [ ] Every stream query enforces authorization (filters by user_id or household membership)
- [ ] No hardcoded secrets -- all sensitive values from environment variables
- [ ] Rust backend compiles without errors
- [ ] TypeScript/SvelteKit compiles without errors
- [ ] Docker Compose PowerSync service depends on postgres health check
- [ ] New code follows existing project patterns (module structure, error handling, auth patterns)

## Validation Commands

Execute these commands to validate the task is complete:

- `cargo check -p altair-server` -- Verify Rust backend compiles
- `cd apps/web && bun run check` -- Verify SvelteKit TypeScript compiles
- `cd apps/web && bun run lint` -- Verify lint passes
- `docker compose -f infra/compose/docker-compose.yml config` -- Verify Docker Compose config is valid
- `cargo test -p altair-server` -- Run existing + new backend tests

## Notes

- PowerSync uses Sync Streams (not the older Sync Rules format) per `altair-powersync-sync-spec.md` section 2
- HS256 is chosen for JWT signing in dev for simplicity; RS256 can be adopted for production later
- The `uploadData` connector in the web client will need to map CRUD operations to the correct backend API endpoints. For now, implement a basic version that handles initiative and tag creation (enough for proof-of-life). Full CRUD mapping will expand as domain endpoints are added in Steps 8-11.
- The proof-of-life test described in the implementation plan (create initiative -> sync -> offline read -> offline tag create -> reconnect -> sync back) is a manual integration test. Automated E2E testing of the sync pipeline can be added later.
- PowerSync service image: `journeyapps/powersync-service:latest`. Check PowerSync docs via Context7 for the correct YAML configuration schema.
- The `token_parameters` in PowerSync YAML streams reference JWT claims. The backend must include `user_id` and `household_ids` in JWT claims so stream queries can filter by them.
- Shared contracts (`packages/contracts/registry/sync-streams.json`) already define stream names -- use these canonical values everywhere.
