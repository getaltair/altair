# Altair — Implementation Plan

**Purpose:** Step-by-step feature-based build order for spec-driven development with Claude Code. Each step references the relevant docs, lists its dependencies, and defines its "done" criteria.

**Key Constraint:** Solo developer using AI coding agents heavily. Steps are scoped so each is a self-contained prompt-friendly unit with clear inputs, outputs, and validation criteria.

**Backend:** Rust (Axum) modular monolith. PostgreSQL primary DB. Self-hosted Docker Compose deployment.

**Sync:** PowerSync (Postgres → SQLite). Offline-first clients.

**Clients:** SvelteKit 2 / Svelte 5 (Web + Tauri Desktop), Android native (Kotlin + Compose).

---

## Dependency Graph (Visual)

```
                    ┌──────────────────────────────────┐
                    │  STEP 1: Monorepo Scaffold        │
                    │  + Shared Contracts                │
                    └──────────┬───────────────────────┘
                               │
                  ┌────────────┼────────────┐
                  ▼            ▼            ▼
         ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
         │ STEP 2:      │ │ STEP 3:      │ │ STEP 4:      │
         │ Backend      │ │ Web Client   │ │ Android      │
         │ Foundation   │ │ Scaffold     │ │ Scaffold     │
         │ (Axum +      │ │ (SvelteKit)  │ │ (Kotlin +    │
         │  Postgres)   │ │              │ │  Compose)    │
         └──────┬───────┘ └──────┬───────┘ └──────┬───────┘
                │                │                 │
                ▼                │                 │
         ┌──────────────┐       │                 │
         │ STEP 5:      │       │                 │
         │ Auth +       │       │                 │
         │ Identity     │       │                 │
         └──────┬───────┘       │                 │
                │                │                 │
                ▼                │                 │
         ┌──────────────┐       │                 │
         │ STEP 6:      │       │                 │
         │ Core Domain  │       │                 │
         │ (backend)    │       │                 │
         └──────┬───────┘       │                 │
                │                │                 │
                ▼                │                 │
         ┌──────────────┐       │                 │
         │ STEP 7:      │       │                 │
         │ PowerSync    │       │                 │
         │ Setup        │◄──────┘                 │
         └──────┬───────┘                         │
                │                                  │
       ┌────────┼────────────────┐                │
       ▼        ▼                ▼                │
┌───────────┐ ┌───────────┐ ┌───────────┐        │
│ STEP 8:   │ │ STEP 9:   │ │ STEP 10:  │        │
│ Guidance  │ │ Knowledge │ │ Tracking  │        │
│ Backend   │ │ Backend   │ │ Backend   │        │
└─────┬─────┘ └─────┬─────┘ └─────┬─────┘        │
      │              │              │               │
      └──────────────┼──────────────┘               │
                     ▼                              │
              ┌──────────────┐                      │
              │ STEP 11:     │                      │
              │ Relationships│                      │
              │ Backend      │                      │
              └──────┬───────┘                      │
                     │                              │
            ┌────────┼────────┐                     │
            ▼                 ▼                     │
     ┌──────────────┐  ┌──────────────┐            │
     │ STEP 12:     │  │ STEP 13:     │◄───────────┘
     │ Web Client   │  │ Android      │
     │ Guidance +   │  │ Client       │
     │ Core UI      │  │ Core         │
     └──────┬───────┘  └──────┬───────┘
            │                  │
            ▼                  ▼
     ┌──────────────┐  ┌──────────────┐
     │ STEP 14:     │  │ STEP 15:     │
     │ Web Client   │  │ Android      │
     │ Knowledge +  │  │ Knowledge +  │
     │ Tracking     │  │ Tracking     │
     └──────┬───────┘  └──────┬───────┘
            │                  │
            └────────┬─────────┘
                     ▼
              ┌──────────────┐
              │ STEP 16:     │
              │ Attachments  │
              └──────┬───────┘
                     │
              ┌──────┴───────┐
              ▼              ▼
       ┌───────────┐  ┌───────────┐
       │ STEP 17:  │  │ STEP 18:  │
       │ Search    │  │ Desktop   │
       │           │  │ (Tauri)   │
       └─────┬─────┘  └───────────┘
             │
             ▼
       ┌───────────┐
       │ STEP 19:  │
       │ AI        │
       │ Enrichment│
       └─────┬─────┘
             │
             ▼
       ┌───────────┐
       │ STEP 20:  │
       │ Notif +   │
       │ Household │
       └───────────┘
```

---

## Parallel Tracks

| Track A: Backend | Track B: Web Client | Track C: Android Client | Track D: Desktop |
|---|---|---|---|
| Steps 1, 2, 5–11, 16–17, 19–20 | Steps 3, 7 (partial), 12, 14, 18 | Steps 4, 13, 15 | Step 18 |

Backend is the critical path. Web and Android client scaffolds can proceed in parallel with backend foundation work. Client feature work begins once PowerSync sync is proven (Step 7).

**Critical path to first web prototype:** Steps 1 → 2 → 5 → 6 → 7 → 12 (~18 days)
**Critical path to first Android prototype:** Steps 1 → 2 → 5 → 6 → 7 → 4 → 13 (~20 days)

---

## Monorepo Layout

```text
altair/
  apps/
    server/                    # Rust Axum backend
    web/                       # SvelteKit 2 web client
    desktop/                   # Tauri 2 + shared Svelte UI
    android/                   # Kotlin + Jetpack Compose
  packages/
    contracts/                 # Shared contract registries + generated bindings
      registry/
        entity-types.json
        relation-types.json
        sync-streams.json
      schemas/
        relation-record.schema.json
        attachment-record.schema.json
      generated/
        typescript/
        kotlin/
        rust/
    api-contracts/             # HTTP API type definitions (OpenAPI or equivalent)
    design-tokens/             # Shared colors, spacing, typography tokens
  infra/
    docker/                    # Dockerfiles
    compose/                   # Docker Compose configs
    migrations/                # PostgreSQL migrations
    powersync/                 # PowerSync sync stream configs
    scripts/                   # Codegen, seed data, dev utilities
  docs/
    prd/
    architecture/
    adr/
    specs/
    DESIGN.md                  # Visual design system (colors, typography, components)
```

### Shared Contract Strategy

Entity type strings, relation types, sync stream names, and status enums are defined once in `packages/contracts/registry/*.json` and generated into TypeScript, Kotlin, and Rust constants. No inline magic strings.

> **Pragmatic note:** Start with manual synchronization of constants in the early steps. Introduce codegen when you have three or more consumers. Don't let tooling block progress on Steps 2–7.

---

## STEP 1: Monorepo Scaffold + Shared Contracts

**Dependencies:** None
**Priority:** P0
**Docs:** `altair-architecture-spec.md` §21 Repository Strategy, `altair-shared-contracts-spec.md`

### What to build

- Monorepo root with workspace configuration
- `packages/contracts/` with canonical JSON registries
- `apps/server/` empty Rust project (Cargo workspace member)
- `apps/web/` empty SvelteKit project placeholder
- `apps/android/` empty Gradle project placeholder
- `infra/` directory structure with Docker Compose skeleton
- `docs/` directory with all existing specs committed
- `docs/DESIGN.md` — visual design system (color palette, typography, component styles)
- `.editorconfig`, linting configs, CI placeholder
- `README.md` with setup instructions

### Registry files to create

| File | Contents | Source Doc |
|---|---|---|
| `entity-types.json` | All entity type identifiers (user, household, guidance_*, knowledge_*, tracking_*) | `altair-entity-type-registry.md` |
| `relation-types.json` | Relation type identifiers (references, supports, requires, etc.) | `altair-shared-contracts-spec.md` §6 |
| `sync-streams.json` | Sync stream names (my_profile, my_household_data, initiative_detail, etc.) | `altair-powersync-sync-spec.md` §8 |

### Initial codegen

A simple script that reads each JSON registry and emits:
- `generated/typescript/entityTypes.ts`, `relationTypes.ts`, `syncStreams.ts`
- `generated/kotlin/EntityType.kt`, `RelationType.kt`, `SyncStream.kt`
- `generated/rust/entity_type.rs`, `relation_type.rs`, `sync_stream.rs`

### Docker Compose skeleton

```yaml
services:
  postgres:
    image: postgres:16
  powersync:
    image: journeyapps/powersync-service
  garage:
    image: dxflrs/garage:v2.2.0
  server:
    build: ./apps/server
```

### Done when

- [ ] Monorepo root exists with workspace config
- [ ] All three JSON registry files contain correct values from spec docs
- [ ] Codegen script emits valid TypeScript, Kotlin, and Rust files
- [ ] `apps/server/` is a valid Cargo project that compiles (empty main)
- [ ] `infra/compose/docker-compose.yml` has Postgres and Garage services defined
- [ ] `docs/` contains all spec documents
- [ ] README has local dev setup instructions

---

## STEP 2: Backend Foundation (Axum + Postgres)

**Dependencies:** Step 1 (monorepo structure exists)
**Priority:** P0
**Docs:** `altair-architecture-spec.md` §11 Backend Architecture, §19 Deployment Architecture

### What to build

Rust Axum application with database connectivity, migration framework, and health endpoint. No domain logic yet — just the skeleton that everything builds on.

### Crate structure

```text
apps/server/
  Cargo.toml                  # Workspace member
  src/
    main.rs                   # Axum server entrypoint
    config.rs                 # Env-based config (database_url, port, etc.)
    db/
      mod.rs                  # Connection pool setup (sqlx)
      migrations/             # sqlx migrations
    api/
      mod.rs                  # Router composition
      health.rs               # GET /health
    error.rs                  # Unified error type
    telemetry.rs              # Structured logging (tracing)
```

### Key dependencies to pin

| Crate | Purpose |
|---|---|
| `axum` | HTTP framework |
| `sqlx` + `sqlx-postgres` | Async Postgres with compile-time checked queries |
| `tokio` | Async runtime |
| `tracing` + `tracing-subscriber` | Structured logging |
| `serde` + `serde_json` | Serialization |
| `dotenvy` | Env file loading |
| `uuid` | Entity IDs |
| `chrono` | Timestamps |
| `tower-http` | CORS, tracing middleware, compression |

### Database setup

- PostgreSQL 16 via Docker Compose
- Initial migration: create `schema_version` tracking (sqlx handles this)
- Connection pool via `sqlx::PgPool`
- Health endpoint verifies DB connectivity

### Done when

- [ ] `docker compose up` starts Postgres and the Axum server
- [ ] `GET /health` returns 200 with `{"status": "ok", "db": "connected"}`
- [ ] sqlx migration framework runs on startup
- [ ] Structured JSON logs emitted via tracing
- [ ] Server reads config from environment variables
- [ ] Dockerfile builds a release binary

---

## STEP 3: Web Client Scaffold (SvelteKit)

**Dependencies:** Step 1 (monorepo structure)
**Priority:** P0 (can run in parallel with Step 2)
**Docs:** `altair-architecture-spec.md` §10.2 Web Client Architecture, `DESIGN.md`

### What to build

SvelteKit 2 project with TypeScript, basic layout shell, generated contract types wired in, and Tailwind configured with the Altair design system color palette.

### Project structure

```text
apps/web/
  src/
    lib/
      contracts/              # Symlink or copy of generated/typescript
      api/                    # HTTP client wrapper
      stores/                 # Svelte stores
      components/
        layout/               # Shell, nav, sidebar
    routes/
      +layout.svelte          # App shell
      +page.svelte            # Landing / today view placeholder
      guidance/
      knowledge/
      tracking/
      settings/
  static/
  svelte.config.js
  vite.config.ts
  tailwind.config.js
```

### Key dependencies

| Library | Purpose |
|---|---|
| SvelteKit 2 | Framework |
| Svelte 5 | Reactivity |
| TypeScript | Type safety |
| Tailwind CSS | Utility-first styling |
| `@fontsource/manrope` | Display/headline font (from DESIGN.md) |
| `@fontsource-variable/plus-jakarta-sans` | Body/label font (from DESIGN.md) |
| `@powersync/web` | PowerSync client SDK (installed, not configured yet) |

### Application shell

- Top-level layout with navigation sidebar: Guidance, Knowledge, Tracking, Search, Settings
- Empty route stubs for each domain
- Tailwind theme extended with Altair color palette from `DESIGN.md` §2 (primary, secondary, tertiary, surfaces, semantic colors)
- Font families configured: Manrope for display/headings, Plus Jakarta Sans for body
- Dark mode support via Tailwind (using `DESIGN.md` dark mode tonal hierarchy)
- Responsive layout (mobile-first)
- Auth gate placeholder (redirects to login if no session)

### Done when

- [ ] `pnpm dev` serves the SvelteKit app
- [ ] App shell renders with navigation sidebar
- [ ] All domain route stubs exist and render placeholder content
- [ ] Generated TypeScript contracts import without errors
- [ ] Tailwind CSS works with custom Altair color tokens
- [ ] Manrope and Plus Jakarta Sans fonts render correctly
- [ ] Dark mode toggle switches between light/dark palettes
- [ ] Layout is responsive at mobile and desktop breakpoints

---

## STEP 4: Android Client Scaffold (Kotlin + Compose)

**Dependencies:** Step 1 (monorepo structure)
**Priority:** P0 (can run in parallel with Steps 2–3)
**Docs:** `altair-architecture-spec.md` §10.4 Android Client Architecture, `DESIGN.md`

### What to build

Android project with Jetpack Compose, single-module structure, Koin DI, generated contract constants, and a Material 3 theme adapted from the Altair design system.

### Module structure

Single-module with package-based separation. No multi-module split needed for an Android-only app at this stage.

```text
apps/android/
  app/
    src/main/kotlin/com/altair/app/
      AltairApp.kt                # Application class + Koin
      MainActivity.kt             # Single activity
      di/                          # Koin modules
      navigation/                  # Nav graph
      domain/models/               # Domain data classes
      domain/repositories/         # Repository interfaces
      data/local/                  # Room DB + DAOs
      data/sync/                   # PowerSync
      data/repositories/           # Repository implementations
      ui/guidance/                 # Guidance screens
      ui/knowledge/                # Knowledge screens
      ui/tracking/                 # Tracking screens
      contracts/                   # Generated contract constants
  gradle/libs.versions.toml        # Version catalog
```

> **Pragmatic note:** The architecture spec mentions desktop uses Tauri + Svelte, not Kotlin. WearOS is Android — not KMP. There is no KMP target, so there's no reason to split into Gradle modules for code sharing. Package discipline in a single module gives you the same logical separation without the Gradle overhead. Extract modules later only if you hit a real reason.

### Key dependencies

| Library | Purpose | Version |
|---|---|---|
| Kotlin | Language | 2.3.20 |
| Compose BOM | UI toolkit | 2026.03.00 |
| Koin BOM | DI | 4.2.0 |
| KSP | Annotation processing (for Room) | 2.3.6 |
| Room | Local SQLite | 2.7.1 (pinned, added Step 13) |
| Navigation Compose | Screen routing | 2.9.0 (pinned, added Step 13) |
| WorkManager | Background jobs | 2.10.1 (pinned, added Step 13) |
| PowerSync Android SDK | Sync | Latest (added Step 13) |
| Timber | Logging | 5.0.1 |

### Application shell

- Single-activity Compose app
- Koin DI wired in Application class
- Timber logging planted
- Generated Kotlin contract constants available in `contracts` package
- Custom Material 3 `ColorScheme` mapped from `DESIGN.md` §2 color palette
- Manrope and Plus Jakarta Sans bundled as font assets
- Dark mode support using `DESIGN.md` dark tonal hierarchy

### Android design system adaptation notes

The `DESIGN.md` was authored for web. These adaptations apply to Android:

- **Material 3 ColorScheme mapping:** Map the Altair palette into `lightColorScheme()` / `darkColorScheme()`. Primary = Deep Muted Teal-Navy (`#446273`), Surface = Frost-Touched Pearl (`#f8fafa`), OnSurface = Deep Ink Charcoal (`#2a3435`), Error = Warm Terracotta (`#9f403d`), etc.
- **Touch transitions:** Use 150ms for direct touch feedback (tap, press) instead of the 300ms "Breathe" duration. Keep 300ms for progressive disclosure and expand/collapse animations.
- **Component borders:** Material 3 `OutlinedTextField` and similar components have built-in outlines. Override where practical (filled text fields match the design), accept minor differences where fighting the framework isn't worth it.
- **Rounded corners:** The design's `1rem` minimum maps well to Material 3's `ShapeDefaults.Medium` (12dp) and `Large` (16dp). Use `RoundedCornerShape(16.dp)` for cards and buttons.
- **No shadows:** Set `Card(elevation = CardDefaults.cardElevation(0.dp))` and rely on tonal layering as the design system specifies.

### Done when

- [ ] Project builds and runs on emulator
- [ ] Empty Compose scaffold renders with custom Altair color scheme
- [ ] Koin modules resolve correctly at runtime
- [ ] Timber logs appear in Logcat
- [ ] Generated Kotlin entity type constants import without errors
- [ ] Manrope and Plus Jakarta Sans fonts render in the UI
- [ ] Both debug and release variants build

---

## STEP 5: Auth + Identity Service

**Dependencies:** Step 2 (Axum server running with Postgres)
**Priority:** P0
**Docs:** `altair-architecture-spec.md` §18 Security, §9.1 Identity Context

### What to build

User registration, login, session management, and per-user authorization middleware. This gates everything else.

### Database migrations

| Table | Key Columns | Purpose |
|---|---|---|
| `users` | id (UUID), email, display_name, password_hash, created_at, updated_at | User accounts |
| `sessions` | id, user_id, token_hash, expires_at, device_info | Session tracking |
| `households` | id, name, created_by, created_at | Household containers |
| `household_memberships` | id, household_id, user_id, role, joined_at | User ↔ Household mapping |

### API endpoints

| Method | Path | Purpose |
|---|---|---|
| POST | `/auth/register` | Create account (email + password) |
| POST | `/auth/login` | Authenticate, return session token |
| POST | `/auth/logout` | Invalidate session |
| GET | `/auth/me` | Get current user profile |
| PUT | `/auth/me` | Update profile |
| POST | `/core/households` | Create household |
| POST | `/core/households/:id/members` | Invite member |
| GET | `/core/households` | List user's households |

### Backend modules

```text
src/
  auth/
    mod.rs
    handlers.rs               # Register, login, logout, me
    service.rs                # Password hashing (Argon2id), session creation
    middleware.rs             # Extract user from token, reject unauthenticated
    models.rs                 # User, Session structs
  core/
    households/
      handlers.rs
      service.rs
      models.rs
```

### Auth middleware

Every request (except `/auth/register`, `/auth/login`, `/health`) must pass through auth middleware that:
1. Extracts bearer token from `Authorization` header
2. Validates session exists and is not expired
3. Injects `AuthenticatedUser { user_id, household_ids }` into request extensions
4. Returns 401 if invalid

### Password security

- Argon2id hashing via `argon2` crate
- No plaintext passwords stored or logged
- Session tokens are random 256-bit values, stored as SHA-256 hash

### Done when

- [ ] User can register with email/password
- [ ] User can login and receive a bearer token
- [ ] Authenticated requests include user context
- [ ] Unauthenticated requests to protected endpoints return 401
- [ ] User can create a household
- [ ] Household membership tracks which users belong to which households
- [ ] Password is hashed with Argon2id
- [ ] Sessions expire after configurable duration
- [ ] Logout invalidates session

---

## STEP 6: Core Domain Backend

**Dependencies:** Step 5 (auth + users + households exist)
**Priority:** P0
**Docs:** `altair-core-prd.md`, `altair-schema-design-spec.md`, `altair-entity-type-registry.md`

### What to build

Shared core entities that all three product domains depend on: initiatives, tags, attachments metadata, and the entity_relations table structure. No domain-specific data yet — just the shared foundation.

### Database migrations

| Table | Key Columns | Purpose | Source |
|---|---|---|---|
| `initiatives` | id, user_id, household_id (nullable), name, description, status, created_at, updated_at | Cross-domain organizing container | `altair-schema-design-spec.md` |
| `tags` | id, user_id, household_id (nullable), name, color, created_at | User/household-scoped labels | `altair-schema-design-spec.md` |
| `attachments` | id, entity_type, entity_id, filename, content_type, storage_key, size_bytes, processing_state, created_at | Attachment metadata (no binary) | `altair-shared-contracts-spec.md` §10 |
| `entity_relations` | id, from_entity_type, from_entity_id, to_entity_type, to_entity_id, relation_type, source_type, status, confidence, evidence_json, created_by_user_id, created_by_process, created_at, updated_at, last_confirmed_at | Cross-domain relationship records | `ADR-004` |

### Critical indexes

```sql
CREATE INDEX idx_initiatives_user ON initiatives(user_id);
CREATE INDEX idx_initiatives_household ON initiatives(household_id);
CREATE INDEX idx_tags_user ON tags(user_id);
CREATE INDEX idx_attachments_entity ON attachments(entity_type, entity_id);
CREATE INDEX idx_relations_from ON entity_relations(from_entity_type, from_entity_id);
CREATE INDEX idx_relations_to ON entity_relations(to_entity_type, to_entity_id);
CREATE INDEX idx_relations_status ON entity_relations(status);
```

### Validation rules

- `entity_type` values must be from the canonical entity type registry — reject unknown types at write time
- `relation_type` must be from the canonical relation type registry
- `source_type` must be from the canonical source type list
- `status` must be from the canonical status list
- `confidence` must be 0.0–1.0 (nullable for user-created relations)

### API endpoints

| Method | Path | Purpose |
|---|---|---|
| POST | `/core/initiatives` | Create initiative |
| GET | `/core/initiatives` | List user/household initiatives |
| GET | `/core/initiatives/:id` | Get initiative detail |
| PUT | `/core/initiatives/:id` | Update initiative |
| DELETE | `/core/initiatives/:id` | Soft-delete initiative |
| CRUD | `/core/tags` | Tag management |
| GET | `/core/relations` | Query relations (filtered by entity, type, status) |
| POST | `/core/relations` | Create relation |
| PUT | `/core/relations/:id` | Update relation (accept/reject/dismiss) |

### Done when

- [ ] Initiatives CRUD works with user and household scoping
- [ ] Tags CRUD works with user and household scoping
- [ ] `entity_relations` table exists with all columns from ADR-004
- [ ] Relation create endpoint validates entity_type, relation_type, source_type, status against registry
- [ ] Unknown entity types are rejected with 400
- [ ] Relations can be queried by from-entity, to-entity, or both
- [ ] Relation status can be updated (accept/reject/dismiss)
- [ ] Attachment metadata table exists (binary upload deferred to Step 16)
- [ ] All endpoints require authentication and enforce user/household authorization

---

## STEP 7: PowerSync Setup + Sync Proof-of-Life

**Dependencies:** Step 6 (core tables exist to sync), Step 3 (web client to test with)
**Priority:** P0
**Docs:** `ADR-003-sync-layer-selection.md`, `altair-powersync-sync-spec.md`

### What to build

PowerSync service configuration, sync stream definitions, and a proof-of-life round-trip: create data on the server → sync to web client SQLite → read locally offline.

This is the highest-risk integration point in the project. Prove it works before building domain features.

### PowerSync service configuration

- PowerSync service in Docker Compose pointing at Postgres
- JWT auth integration (PowerSync authenticates clients using JWTs signed by the backend)
- Backend endpoint to issue PowerSync JWT tokens for authenticated users

### Sync stream definitions

Implement the starter streams from `altair-powersync-sync-spec.md` §8:

#### Auto-subscribed streams
| Stream | Tables Included | Scope |
|---|---|---|
| `my_profile` | users (self row only) | user_id |
| `my_memberships` | household_memberships | user_id |
| `my_personal_data` | initiatives, tags (user-owned) | user_id |
| `my_household_data` | households, initiatives, tags (household-owned), tracking_locations, tracking_categories, tracking_items, tracking_shopping_lists, tracking_shopping_list_items | household_id via membership |
| `my_relations` | entity_relations (scoped to user's initiatives + households) | user_id + household_id |
| `my_attachment_metadata` | attachments (scoped to synced entities) | user_id + household_id |

#### On-demand streams
| Stream | Tables Included | Scope |
|---|---|---|
| `initiative_detail` | guidance_epics, guidance_quests, knowledge_notes, tracking_items (for initiative) | initiative_id |
| `note_detail` | knowledge_notes, knowledge_note_snapshots, attachments, entity_relations (note-scoped) | note_id |
| `item_history` | tracking_items, tracking_item_events, attachments, entity_relations (item-scoped) | item_id |
| `quest_detail` | guidance_quests, tags, attachments, entity_relations, guidance_focus_sessions | quest_id |

### Web client PowerSync integration

- Install `@powersync/web` in the SvelteKit app
- Configure PowerSync client with backend JWT endpoint
- Create local SQLite schema matching synced tables
- Subscribe to auto-subscribed streams on login
- Display synced data in a debug view

### Proof-of-life test

1. Create an initiative via the backend API
2. Verify it appears in the web client's local SQLite within 5 seconds
3. Disconnect the web client from the network
4. Verify the data is still readable locally
5. Create a tag while offline
6. Reconnect and verify the tag syncs to Postgres

### Authorization rules

Every stream query must filter by `auth.user_id()` or household membership. A client cannot request data for users or households it does not belong to.

### Done when

- [ ] PowerSync service starts in Docker Compose and connects to Postgres
- [ ] Backend issues signed JWTs for PowerSync client auth
- [ ] Web client connects to PowerSync and syncs auto-subscribed streams
- [ ] Initiative created via API appears in web client local SQLite within 5s
- [ ] Web client reads data offline after sync
- [ ] Offline mutations (create tag) sync back to Postgres on reconnect
- [ ] On-demand stream subscription works for initiative_detail
- [ ] Authorization: client cannot access other users' data via PowerSync

---

## STEP 8: Guidance Domain Backend

**Dependencies:** Step 6 (core domain tables), Step 5 (auth)
**Priority:** P0
**Docs:** `altair-guidance-prd.md`, `altair-schema-design-spec.md`, `altair-entity-type-registry.md`

### What to build

Backend tables, services, and API endpoints for the Guidance domain: epics, quests, routines, focus sessions, and daily check-ins.

### Database migrations

| Table | Key Columns | Purpose |
|---|---|---|
| `guidance_epics` | id, initiative_id, user_id, name, description, status, priority, created_at, updated_at | Large efforts grouping quests |
| `guidance_quests` | id, epic_id (nullable), initiative_id (nullable), user_id, household_id (nullable), name, description, status, priority, due_date, estimated_minutes, created_at, updated_at | Actionable units of work |
| `guidance_routines` | id, user_id, household_id (nullable), name, description, frequency, status, created_at, updated_at | Recurring habits/behaviors |
| `guidance_focus_sessions` | id, quest_id, user_id, started_at, ended_at, duration_minutes, notes | Timed work sessions |
| `guidance_daily_checkins` | id, user_id, date, energy_level, mood, notes, created_at | Daily self-assessment |

### Tag association tables

| Table | Purpose |
|---|---|
| `quest_tags` | Quest ↔ Tag many-to-many |
| `routine_tags` | Routine ↔ Tag many-to-many |

### API endpoints

| Method | Path | Purpose |
|---|---|---|
| CRUD | `/guidance/epics` | Epic management |
| CRUD | `/guidance/quests` | Quest management |
| POST | `/guidance/quests/:id/complete` | Mark quest complete |
| CRUD | `/guidance/routines` | Routine management |
| POST | `/guidance/routines/:id/trigger` | Create quest instances from routine |
| CRUD | `/guidance/focus-sessions` | Focus session tracking |
| POST | `/guidance/daily-checkins` | Record daily check-in |
| GET | `/guidance/today` | Today's quests + routines + check-in status |

### Key business rules

- Quests can belong to an epic, an initiative, both, or neither
- Quests can be user-scoped or household-scoped (shared chores)
- Routines generate quest instances when triggered
- Daily check-in is one per user per day (UNIQUE constraint on user_id + date)
- Quest status transitions: pending → in_progress → completed / cancelled

### Done when

- [ ] All Guidance tables created with correct columns and indexes
- [ ] Epic/Quest/Routine CRUD endpoints work with proper auth
- [ ] Quest completion endpoint updates status and records timestamp
- [ ] Routine trigger creates quest instances
- [ ] Daily check-in enforces one-per-day constraint
- [ ] `/guidance/today` returns today's relevant quests and routines
- [ ] Household-scoped quests visible to all household members
- [ ] Tag associations work for quests and routines

---

## STEP 9: Knowledge Domain Backend

**Dependencies:** Step 6 (core domain tables), Step 5 (auth)
**Priority:** P0 (can run in parallel with Step 8)
**Docs:** `altair-knowledge-prd.md`, `altair-schema-design-spec.md`

### What to build

Backend tables, services, and API for the Knowledge domain: notes and note snapshots.

### Database migrations

| Table | Key Columns | Purpose |
|---|---|---|
| `knowledge_notes` | id, user_id, household_id (nullable), initiative_id (nullable), title, content (text/markdown), content_type, is_pinned, created_at, updated_at | Primary information units |
| `knowledge_note_snapshots` | id, note_id, content, created_at, created_by_process | Point-in-time captures of note content |
| `note_tags` | note_id, tag_id | Note ↔ Tag many-to-many |
| `note_attachments` | note_id, attachment_id | Note ↔ Attachment many-to-many |

### API endpoints

| Method | Path | Purpose |
|---|---|---|
| CRUD | `/knowledge/notes` | Note management |
| GET | `/knowledge/notes/:id/snapshots` | Get note revision history |
| POST | `/knowledge/notes/:id/snapshots` | Create manual snapshot |
| GET | `/knowledge/notes/:id/relations` | Get relations from/to this note |
| GET | `/knowledge/notes/:id/backlinks` | Get entities that link TO this note |

### Key business rules

- Notes support markdown content
- Snapshots are immutable once created
- Auto-snapshot on significant edits (configurable — backend creates snapshot if content delta exceeds threshold)
- Backlinks are derived from `entity_relations` where `to_entity_type = 'knowledge_note'`
- Notes can be scoped to user, household, or initiative

### Done when

- [ ] Notes CRUD works with proper auth and scoping
- [ ] Snapshots can be created and listed for a note
- [ ] Note backlinks query returns all entities that relate TO this note
- [ ] Note tags and attachments associations work
- [ ] Notes filterable by initiative, household, pinned status

---

## STEP 10: Tracking Domain Backend

**Dependencies:** Step 6 (core domain tables), Step 5 (auth)
**Priority:** P0 (can run in parallel with Steps 8–9)
**Docs:** `altair-tracking-prd.md`, `altair-schema-design-spec.md`

### What to build

Backend tables, services, and API for the Tracking domain: locations, categories, items, item events, shopping lists.

### Database migrations

| Table | Key Columns | Purpose |
|---|---|---|
| `tracking_locations` | id, user_id, household_id, name, description, parent_location_id (nullable), created_at | Storage locations (hierarchical) |
| `tracking_categories` | id, user_id, household_id, name, description, parent_category_id (nullable), created_at | Item categorization (hierarchical) |
| `tracking_items` | id, user_id, household_id, category_id, location_id, name, description, quantity, unit, min_quantity (nullable), barcode (nullable), status, created_at, updated_at | Physical/digital resources |
| `tracking_item_events` | id, item_id, user_id, event_type, quantity_change, notes, created_at | Consumption/change records |
| `tracking_shopping_lists` | id, user_id, household_id, name, status, created_at, updated_at | Shopping list containers |
| `tracking_shopping_list_items` | id, shopping_list_id, item_id (nullable), name, quantity, unit, is_checked, created_at | Shopping list line items |
| `item_tags` | item_id, tag_id | Item ↔ Tag many-to-many |
| `item_attachments` | item_id, attachment_id | Item ↔ Attachment many-to-many |

### API endpoints

| Method | Path | Purpose |
|---|---|---|
| CRUD | `/tracking/locations` | Location management (hierarchical) |
| CRUD | `/tracking/categories` | Category management (hierarchical) |
| CRUD | `/tracking/items` | Item management |
| POST | `/tracking/items/:id/events` | Record consumption/change event |
| GET | `/tracking/items/:id/events` | Get item event history |
| GET | `/tracking/items/low-stock` | Items below min_quantity threshold |
| CRUD | `/tracking/shopping-lists` | Shopping list management |
| CRUD | `/tracking/shopping-lists/:id/items` | Shopping list item management |
| POST | `/tracking/shopping-lists/:id/items/:item_id/check` | Toggle check state |

### Key business rules

- Items track quantity; events record changes (consumed, restocked, moved, etc.)
- Low-stock detection: items where `quantity < min_quantity`
- Locations and categories support one level of hierarchy (parent_id)
- Shopping list items can reference a tracked item or be freeform text
- All tracking entities are household-scoped by default
- Item events are append-only (immutable log)

### Done when

- [ ] Location and category hierarchical CRUD works
- [ ] Item CRUD with location and category assignment works
- [ ] Item events create and update item quantity
- [ ] Low-stock query returns items below threshold
- [ ] Shopping list CRUD with check/uncheck works
- [ ] All endpoints enforce household-level authorization
- [ ] Item event history is queryable and sorted by date

---

## STEP 11: Relationships Backend (Wiring Cross-Domain Links)

**Dependencies:** Steps 8, 9, 10 (all domain tables exist)
**Priority:** P0
**Docs:** `ADR-004-relationship-modeling-strategy.md`, `altair-entity-type-registry.md`, `altair-shared-contracts-spec.md` §6

### What to build

Make the `entity_relations` system functional across all three domains. This is where Altair's cross-domain value lives.

### API enhancements

| Method | Path | Purpose |
|---|---|---|
| GET | `/core/relations/for/:entity_type/:entity_id` | All relations from/to a specific entity |
| GET | `/core/relations/graph/:entity_type/:entity_id` | One-hop relationship graph for an entity |
| PUT | `/core/relations/:id/accept` | Accept a suggested relation |
| PUT | `/core/relations/:id/dismiss` | Dismiss a suggested relation |
| PUT | `/core/relations/:id/reject` | Reject a suggested relation |
| GET | `/core/relations/suggested` | All suggested relations for the user (for review UI) |

### Cross-domain query examples to support

These queries validate that the relationship model actually works:

1. "What notes reference this item?" — `entity_relations WHERE to_entity_type = 'tracking_item' AND to_entity_id = X`
2. "What items does this quest require?" — `entity_relations WHERE from_entity_type = 'guidance_quest' AND relation_type = 'requires'`
3. "What is related to this initiative?" — `entity_relations WHERE from_entity_id = X OR to_entity_id = X` (both directions)
4. "Show all AI-suggested relations pending review" — `entity_relations WHERE source_type = 'ai' AND status = 'suggested'`

### Seed data

Create a development seed dataset that exercises cross-domain relationships per `altair-powersync-sync-spec.md` §10:

- One user, one household, two members
- One personal initiative, one household initiative
- A note that references a tracking item
- A quest that requires a tracking item
- A note that supports an initiative
- An item related to a note
- One AI-suggested relation (status: suggested)
- One user-dismissed relation (status: dismissed)

### Done when

- [ ] Relations queryable by entity (both directions)
- [ ] One-hop graph query returns all directly related entities with metadata
- [ ] Accept/dismiss/reject updates relation status correctly
- [ ] Suggested relations query works (for future review UI)
- [ ] Seed dataset creates representative cross-domain relationships
- [ ] All four cross-domain query examples above return correct results
- [ ] Relation sync via PowerSync includes relations for user's scoped entities

---

## STEP 12: Web Client — Guidance + Core UI

**Dependencies:** Step 7 (PowerSync syncing data), Step 8 (Guidance backend)
**Priority:** P0
**Docs:** `altair-guidance-prd.md`, `altair-core-prd.md`, `altair-architecture-spec.md` §10.2, `DESIGN.md`

### What to build

First real web client screens. The Guidance domain is the natural starting point because "what should I do today?" is the highest-frequency user interaction.

### Design system implementation (first sub-task)

Before building screens, implement the `DESIGN.md` design system in Tailwind and Svelte components:

- **Tailwind theme extension:** Map the full Altair color palette from `DESIGN.md` §2 into `tailwind.config.js` custom colors (primary, secondary, tertiary, surface hierarchy, semantic colors)
- **CSS custom properties:** Define `--color-primary`, `--color-surface`, etc. for runtime dark mode switching
- **Base component library:** Build foundational Svelte components that encode the design rules:
  - `Card` — no borders, no shadows, tonal layering via surface hierarchy
  - `Button` — pill-shaped (1.5rem radius), primary fill / ghost secondary variants
  - `Input` — no border at rest, filled background, ghost border on focus
  - `SectionLabel` — editorial-style all-caps `label-sm` with 0.1em letter-spacing
- **Navigation sidebar:** Icon-based vertical nav per `DESIGN.md` §4, Material Symbols Outlined icons, tonal background distinction
- **Typography scale:** Implement the Manrope (display/headlines) + Plus Jakarta Sans (body/labels) hierarchy with the specified sizes and line heights
- **Signature gradient:** CSS utility for the 135° primary-to-container gradient on CTAs
- **Scrollbar styling:** Ultra-thin 4-6px thumb, transparent track, Silver Haze color
- **Transition defaults:** 300ms cubic-bezier(0.4, 0, 0.2, 1) as the global "Breathe" transition

This component library is reused across all subsequent web client steps (14, 18).

### Pages to build

| Route | Screen | Purpose |
|---|---|---|
| `/` | Today View | Today's quests, routines, daily check-in |
| `/guidance/initiatives` | Initiative List | All user/household initiatives |
| `/guidance/initiatives/:id` | Initiative Detail | Epics, quests, notes, items for this initiative |
| `/guidance/quests` | Quest List | All quests (filterable by status, initiative) |
| `/guidance/quests/:id` | Quest Detail | Quest info, related entities, focus sessions |
| `/guidance/routines` | Routine List | All routines |
| `/guidance/routines/:id` | Routine Detail | Routine info, generated quests |
| `/settings` | Settings | User profile, household management, sync status |

### Today View (primary screen)

The Today View follows the `DESIGN.md` "Digital Sanctuary" aesthetic: generous whitespace, tonal card layering on Frost-Touched Pearl background, Manrope display heading for the greeting, no borders or dividers between sections.

```
┌─────────────────────────────────────────────────┐
│                                                  │
│  Good morning, Robert              Thu, Mar 26   │
│                                                  │
│                                                  │
│  TODAY'S QUESTS                      3 of 7 done │
│                                                  │
│  ┌─────────────────────────────────────────────┐ │
│  │ ✓ Review PR feedback              Personal  │ │
│  │                                              │ │
│  │ ○ Draft sync spec update          ARGUS     │ │
│  │                                              │ │
│  │ ○ Check UPS battery status        Home      │ │
│  └─────────────────────────────────────────────┘ │
│                                                  │
│                                                  │
│  ROUTINES                                        │
│                                                  │
│  ┌─────────────────────────────────────────────┐ │
│  │ ○ Morning routine (3 items)                 │ │
│  │                                              │ │
│  │ ✓ Evening review                            │ │
│  └─────────────────────────────────────────────┘ │
│                                                  │
│                                                  │
│  CHECK-IN                                        │
│                                                  │
│  ┌─────────────────────────────────────────────┐ │
│  │ Energy: ●●●○○  Mood: 😊  [Edit]            │ │
│  └─────────────────────────────────────────────┘ │
│                                                  │
└─────────────────────────────────────────────────┘
```

Section labels use the editorial `label-sm` all-caps treatment. Cards are Pure White on Frost-Touched Pearl — depth via tone, not shadow. Spacing between sections is 2rem minimum.

### Data access pattern

All reads come from local PowerSync SQLite. Writes go to local SQLite first, then sync to Postgres via PowerSync.

### Key UI patterns

- Quest completion: tap → local update → optimistic UI → sync in background
- Initiative detail: subscribe to `initiative_detail` on-demand stream when navigating in
- Offline indicator in app shell when sync is disconnected
- Toast/notification on sync errors

### Done when

- [ ] Tailwind theme configured with full Altair color palette from DESIGN.md
- [ ] Base component library (Card, Button, Input, SectionLabel) built and matches design system
- [ ] Navigation sidebar renders with correct icons and tonal styling
- [ ] Manrope + Plus Jakarta Sans typography hierarchy working
- [ ] Today View shows today's quests and routines from local PowerSync data
- [ ] Quest completion writes locally and syncs to server
- [ ] Initiative list and detail views render with correct data
- [ ] Quest list with status filtering works
- [ ] Routine list shows routines; detail shows generated quests
- [ ] Settings page shows user profile, household info, sync status
- [ ] App works fully offline (reads + writes queue)
- [ ] On-demand streams subscribe/unsubscribe on navigation
- [ ] Empty states for all lists
- [ ] Light and dark modes both match DESIGN.md tonal hierarchy

---

## STEP 13: Android Client — Core + Guidance

**Dependencies:** Step 4 (Android scaffold), Step 7 (PowerSync), Step 8 (Guidance backend)
**Priority:** P0
**Docs:** `altair-architecture-spec.md` §10.4, `altair-guidance-prd.md`, `DESIGN.md`

### What to build

Android equivalent of Step 12. Room as local DB, PowerSync for sync, Compose UI styled with the Altair design system (adapted for Material 3).

### Design system implementation (first sub-task)

Before building screens, implement the Altair theme in Compose:

- **`Color.kt`:** Define all Altair palette colors as Compose `Color` constants
- **`Theme.kt`:** Custom `lightColorScheme()` and `darkColorScheme()` mapped from `DESIGN.md` §2
- **`Type.kt`:** Typography scale using Manrope (display/headline) and Plus Jakarta Sans (body/label), with sizes and line heights from `DESIGN.md` §3
- **`Shape.kt`:** Override `ShapeDefaults` — minimum 12dp corners for medium, 16dp for large, 24dp pill for buttons
- **Base composables:** `AltairCard` (zero elevation, tonal layering), `AltairButton` (pill-shaped, primary/ghost variants), `AltairTextField` (filled, no outline at rest)
- **Adaptation:** Use 150ms for touch feedback animations, 300ms for expand/collapse. Accept minor Material 3 component differences where overriding is disproportionate effort.

### Domain models (in `domain` package)

Pure Kotlin data classes mirroring the server schema, using generated contract constants:

| Entity | Key Fields |
|---|---|
| `User` | id, email, displayName |
| `Initiative` | id, userId, householdId, name, description, status |
| `Quest` | id, epicId, initiativeId, userId, householdId, name, status, priority, dueDate |
| `Routine` | id, userId, householdId, name, frequency, status |
| `Tag` | id, userId, householdId, name, color |
| `DailyCheckin` | id, userId, date, energyLevel, mood, notes |
| `EntityRelation` | id, fromEntityType, fromEntityId, toEntityType, toEntityId, relationType, sourceType, status, confidence |

### Room database

- `AltairDatabase` with all tables matching PowerSync-synced schema
- DAOs for each entity with common queries
- PowerSync Android SDK integration for sync

### Screens to build

| Screen | Purpose |
|---|---|
| Today | Today's quests + routines + check-in |
| Initiative List | All initiatives |
| Initiative Detail | Quests, related entities |
| Quest Detail | Quest info, completion, related entities |
| Routine List + Detail | Routine management |
| Settings | Profile, households, sync status |

### Key Android-specific behaviors

- WorkManager for background sync
- Notification channel setup (used later in Step 20)
- Share intent receiver (capture text into notes — wired later)
- Custom Altair Material 3 theme (not dynamic color — use the explicit DESIGN.md palette)

### Done when

- [ ] Custom Altair `ColorScheme` renders correctly in light and dark modes
- [ ] Manrope + Plus Jakarta Sans fonts render in Compose UI
- [ ] Base composables (`AltairCard`, `AltairButton`, `AltairTextField`) built
- [ ] PowerSync syncs data to Room on login
- [ ] Today screen shows quests and routines from local Room data
- [ ] Quest completion works offline and syncs
- [ ] Initiative list and detail views work
- [ ] Routine list and detail views work
- [ ] Daily check-in creates and syncs
- [ ] Settings shows sync status and user info
- [ ] App survives rotation and process death
- [ ] Background sync via WorkManager runs periodically

---

## STEP 14: Web Client — Knowledge + Tracking

**Dependencies:** Step 12 (web core UI exists), Steps 9–10 (Knowledge + Tracking backends)
**Priority:** P1
**Docs:** `altair-knowledge-prd.md`, `altair-tracking-prd.md`

### What to build

Knowledge and Tracking domain screens in the web client.

### Knowledge pages

| Route | Screen | Purpose |
|---|---|---|
| `/knowledge/notes` | Note List | All notes (searchable, filterable) |
| `/knowledge/notes/:id` | Note Detail/Editor | View/edit note content (markdown) |
| `/knowledge/notes/:id/history` | Snapshot History | Note revision timeline |
| `/knowledge/notes/:id/relations` | Note Relations | Related entities + backlinks |

### Knowledge UI features

- Markdown editor with preview (use a Svelte markdown component)
- Backlinks panel showing what entities link TO this note
- Related entities panel showing what this note links TO
- Tag management on notes
- Snapshot history timeline

### Tracking pages

| Route | Screen | Purpose |
|---|---|---|
| `/tracking/items` | Item List | All tracked items (filterable by location, category) |
| `/tracking/items/:id` | Item Detail | Item info, event history, related entities |
| `/tracking/locations` | Location Tree | Hierarchical location view |
| `/tracking/categories` | Category Tree | Hierarchical category view |
| `/tracking/shopping-lists` | Shopping Lists | Active shopping lists |
| `/tracking/shopping-lists/:id` | Shopping List Detail | List items with check/uncheck |
| `/tracking/low-stock` | Low Stock Dashboard | Items below threshold |

### Tracking UI features

- Item quantity adjustment (increment/decrement with event logging)
- Shopping list with real-time check/uncheck (syncs across devices)
- Low-stock alert indicators on items
- Location and category tree views
- Item event history timeline

### Cross-domain relationship display

Both Knowledge and Tracking detail views should show:
- "Related to" panel listing entity_relations
- Ability to create new manual relations (link note to item, etc.)
- Visual distinction between user-created and AI-suggested relations

### Done when

- [ ] Note list with search and filtering works
- [ ] Note editor supports markdown with preview
- [ ] Note backlinks and relations panels display correctly
- [ ] Snapshot history shows note revisions
- [ ] Item list with location/category filtering works
- [ ] Item quantity adjustment creates events and syncs
- [ ] Shopping list CRUD with check/uncheck works and syncs
- [ ] Low-stock dashboard shows items below threshold
- [ ] Location and category tree views render hierarchically
- [ ] Cross-domain relations visible on note and item detail screens
- [ ] Manual relation creation works (link note ↔ item, note ↔ quest, etc.)

---

## STEP 15: Android Client — Knowledge + Tracking

**Dependencies:** Step 13 (Android core exists), Steps 9–10 (backends)
**Priority:** P1
**Docs:** `altair-knowledge-prd.md`, `altair-tracking-prd.md`

### What to build

Android equivalents of Step 14's knowledge and tracking screens, plus Android-specific capture features.

### Knowledge screens

| Screen | Purpose |
|---|---|
| Note List | Filterable list of notes |
| Note Editor | Create/edit markdown notes |
| Note Detail | View note with backlinks + relations |

### Tracking screens

| Screen | Purpose |
|---|---|
| Item List | Filterable item inventory |
| Item Detail | Item info + event history + relations |
| Shopping List | Check/uncheck items |
| Quick Capture | Barcode scan or photo → create item |

### Android-specific capture features

- **Camera capture:** Take photo → create note with image attachment metadata
- **Barcode scanner:** Scan → look up or create tracking item
- **Share intent:** Receive shared text/URLs from other apps → create note
- **Voice note:** Record audio → create note with audio attachment metadata

> **Note:** Attachment binaries are not uploaded yet (that's Step 16). At this stage, capture creates the metadata records and stores the binary locally.

### Done when

- [ ] Note list, editor, and detail screens work offline
- [ ] Item list and detail with event history works
- [ ] Shopping list check/uncheck syncs across devices
- [ ] Camera capture creates note with attachment metadata
- [ ] Barcode scanner creates or finds tracking items
- [ ] Share intent creates notes from external app content
- [ ] Cross-domain relations visible on detail screens
- [ ] All screens survive rotation and process death

---

## STEP 16: Attachment Service

**Dependencies:** Step 14 or 15 (attachment metadata exists from captures), Step 2 (backend running)
**Priority:** P1
**Docs:** `altair-architecture-spec.md` §11 (Files Module), `altair-shared-contracts-spec.md` §10

### What to build

Binary attachment upload, download, and processing pipeline. Object storage (Garage for self-hosted), processing state machine, and client upload/download flows.

### Backend components

- Garage object storage in Docker Compose (already running from Step 1)
- Upload endpoint: `POST /attachments/upload` (multipart)
- Download endpoint: `GET /attachments/:id/download` (signed URL or proxy)
- Processing state machine: pending → uploaded → processing → ready → failed
- Thumbnail generation worker for images
- Storage abstraction (S3-compatible interface for Garage / AWS / local)

### API endpoints

| Method | Path | Purpose |
|---|---|---|
| POST | `/attachments/upload` | Upload binary, create/update attachment record |
| GET | `/attachments/:id/download` | Get signed download URL or proxy download |
| GET | `/attachments/:id/thumbnail` | Get thumbnail for images |
| DELETE | `/attachments/:id` | Mark attachment deleted, schedule cleanup |

### Client integration

- Web: upload via fetch, display images inline, download links
- Android: upload queued via WorkManager, local cache for downloaded files
- Both clients: show processing state indicator (uploading, processing, ready)
- Attachments synced only as metadata via PowerSync — binaries are fetched on demand

### Processing pipeline

1. Client uploads binary to `/attachments/upload`
2. Server stores in Garage, updates attachment record to `uploaded`
3. Background worker picks up, generates thumbnails for images, sets `processing` → `ready`
4. Future: OCR, AI description, embedding generation

### Done when

- [ ] Garage running in Docker Compose with altair-attachments bucket
- [ ] Upload endpoint accepts multipart file and stores in Garage
- [ ] Download endpoint returns file via signed URL
- [ ] Attachment processing state transitions correctly
- [ ] Image thumbnails generated by background worker
- [ ] Web client can upload and display images/files
- [ ] Android client queues uploads via WorkManager
- [ ] Attachment metadata syncs via PowerSync; binaries fetched on demand
- [ ] Deleted attachments cleaned from object storage

---

## STEP 17: Search

**Dependencies:** Step 11 (relationships exist to search across), Step 14 (UI to display results)
**Priority:** P1
**Docs:** `altair-architecture-spec.md` §9.6 Search Context, `altair-core-prd.md` §Search

### What to build

Cross-domain search: keyword full-text search first, semantic/vector search later.

### Phase 1: PostgreSQL full-text search

Use Postgres `tsvector` and `tsquery` for keyword search across all domains.

| Table | Indexed Columns |
|---|---|
| `knowledge_notes` | title, content |
| `guidance_quests` | name, description |
| `guidance_epics` | name, description |
| `tracking_items` | name, description, barcode |
| `initiatives` | name, description |

### API endpoints

| Method | Path | Purpose |
|---|---|---|
| GET | `/search?q=...&types=...` | Cross-domain keyword search |
| GET | `/search/suggestions?q=...` | Autocomplete/type-ahead |

### Search result shape

```json
{
  "results": [
    {
      "entity_type": "knowledge_note",
      "entity_id": "uuid",
      "title": "HVAC Filter Replacement",
      "snippet": "...matching text excerpt...",
      "score": 0.87,
      "updated_at": "2026-03-25T..."
    }
  ],
  "total": 42
}
```

### Phase 2: Semantic search (future prep)

- Add `pgvector` extension to Postgres
- `embeddings` table: entity_type, entity_id, embedding (vector), model, created_at
- Embedding generation as background job (triggered on content change)
- Hybrid search: combine keyword + vector scores
- This phase can be deferred past v1 — prep the table and extension now

### Client integration

- Global search bar in web app shell and Android app bar
- Results grouped by entity type
- Click result → navigate to entity detail
- Search works against local PowerSync data for basic filtering; full search hits the backend API

### Done when

- [ ] Full-text search indexes created on key tables
- [ ] `/search` endpoint returns cross-domain results ranked by relevance
- [ ] Search results include entity type, title, snippet, and score
- [ ] Results filterable by entity type
- [ ] Web client global search bar queries backend and displays results
- [ ] Android search screen works similarly
- [ ] pgvector extension installed and embeddings table exists (even if unused yet)

---

## STEP 18: Desktop Client (Tauri)

**Dependencies:** Step 14 (web client Knowledge + Tracking complete — shared UI)
**Priority:** P2
**Docs:** `altair-architecture-spec.md` §10.3 Desktop Client Architecture

### What to build

Tauri 2 wrapper around the shared SvelteKit UI, with desktop-specific enhancements.

### What Tauri adds over web

- SQLite via PowerSync for true offline-first (not just browser cache)
- Local file system access for imports/exports
- System tray presence
- Native notifications
- Stronger local attachment cache
- Multi-window potential (future)

### Project setup

```text
apps/desktop/
  src-tauri/
    Cargo.toml
    src/
      main.rs                 # Tauri app entry
      commands.rs             # Tauri commands (file access, etc.)
    tauri.conf.json
  src/                        # Shared Svelte UI (symlink or import from apps/web)
```

### Desktop-specific features

- File import: drag-and-drop or file picker → create note/attachment
- Export: export notes as markdown files, items as CSV
- System tray: sync status indicator, quick-capture shortcut
- Local AI adapter placeholder (future)

### Build targets

- Linux: `.deb` and `.AppImage`
- Windows: `.msi` installer

### Done when

- [ ] Tauri app builds and runs on Linux
- [ ] Shared Svelte UI renders identically to web
- [ ] PowerSync syncs to local SQLite (not just browser IndexedDB)
- [ ] File import (drag-and-drop) creates notes/attachments
- [ ] Export notes as markdown files works
- [ ] System tray shows sync status
- [ ] Linux `.AppImage` and Windows `.msi` build successfully

---

## STEP 19: AI Enrichment Pipeline

**Dependencies:** Step 17 (search + embeddings infrastructure), Step 11 (relationships to create)
**Priority:** P2
**Docs:** `altair-architecture-spec.md` §11 (AI Module), `ADR-004` §Inferred relationships

### What to build

Background AI pipeline that enriches content and suggests relationships. AI is optional and degrades gracefully — Altair must work fully without it.

### Pipeline components

1. **Embedding generation:** On note/item/quest create/update, generate embeddings via pluggable AI provider, store in pgvector
2. **Relationship suggestion:** Compare new/updated entity embeddings against existing entities, suggest relations above confidence threshold
3. **Content enrichment:** Summarize long notes, extract key entities, suggest tags
4. **OCR/transcription:** Process image and audio attachments (deferred if no AI provider configured)

### Backend modules

```text
src/
  ai/
    mod.rs
    provider.rs               # Trait for AI providers (OpenAI, Ollama, etc.)
    embedding.rs              # Embedding generation + storage
    relationship_suggest.rs   # Similarity-based relation suggestions
    enrichment.rs             # Tag/summary/entity extraction
    jobs.rs                   # Background job definitions
```

### Configuration

- AI provider configured via environment (API key + endpoint)
- If no AI provider configured, all AI features silently no-op
- Provider adapters: OpenAI API, Ollama (local), future others
- Rate limiting and cost tracking per provider

### Relationship suggestion flow

1. Entity created/updated → embedding generated
2. Background job compares embedding against entities in the same user/household scope
3. If similarity > threshold, create `entity_relations` record with `source_type = 'ai'`, `status = 'suggested'`, `confidence = similarity_score`
4. User reviews in "Suggested Relations" UI (built in Steps 12/14)

### Done when

- [ ] Embedding generation works with at least one provider (OpenAI or Ollama)
- [ ] Embeddings stored in pgvector table
- [ ] Relationship suggestion creates `entity_relations` records with AI source type
- [ ] Suggestions appear in the "Suggested Relations" UI for user review
- [ ] All AI features no-op gracefully if no provider configured
- [ ] Background job processes queue without blocking API
- [ ] Provider is configurable via environment variables

---

## STEP 20: Notifications + Household Shared State

**Dependencies:** Step 13 (Android client for push), Step 12 (web client), Steps 8–10 (all domains)
**Priority:** P2
**Docs:** `altair-architecture-spec.md` §11 (Notify Module), `ADR-003` §Multi-user shared state

### What to build

Push notifications for Android (quest reminders, low-stock alerts, routine triggers) and refined household shared state behavior.

### Notification types

| Trigger | Channel | Content |
|---|---|---|
| Quest due date approaching | Push (Android) | "Quest X is due tomorrow" |
| Routine trigger time | Push (Android) | "Time for your morning routine" |
| Item low stock | Push (Android) + In-app | "Running low on X (2 remaining)" |
| AI suggestion ready | In-app | "New suggested relation: Note ↔ Item" |
| Household member action | In-app | "Jane completed 'Take out trash'" |

### Backend

- Notification preferences table (per-user, per-type enable/disable)
- FCM (Firebase Cloud Messaging) integration for Android push
- Notification generation workers triggered by domain events
- Notification history table for in-app notification center

### Android integration

- FCM token registration on login
- Notification channels (Guidance, Tracking, Household, AI)
- Notification actions (mark quest complete from notification)

### Household shared state refinements

- Shared quest assignment (assign household quest to specific member)
- Shared shopping list real-time sync (check item → all members see it)
- Household activity feed (recent actions by all members)
- Household member management (invite, remove, role changes)

### Done when

- [ ] Android receives push notifications for quest reminders
- [ ] Low-stock notifications fire when items drop below threshold
- [ ] Notification preferences allow per-type enable/disable
- [ ] In-app notification center shows recent notifications
- [ ] Household quest assignment works
- [ ] Shared shopping list check/uncheck propagates to all members in real-time
- [ ] Household activity feed shows recent member actions
- [ ] Notification actions (complete quest from notification) work

---

## Integration Testing Milestones

### Milestone 1: First Sync Round-Trip (after Steps 6 + 7)

- Create an initiative via the API → appears in web client local SQLite within 5s
- **This validates the entire backend + sync pipeline. Do it ASAP.**

### Milestone 2: Offline Quest Completion (after Step 12)

- Complete a quest in web client while offline → reconnect → quest status syncs to server → appears on other devices

### Milestone 3: Cross-Domain Relationship (after Step 11 + 14)

- Create a note that references a tracking item → relationship visible on both the note detail and item detail views
- Verifies entity_relations + UI integration end-to-end

### Milestone 4: Household Shared Inventory (after Steps 10 + 14 + 20)

- User A adds item to shopping list → User B sees it on their device within 5s
- User B checks item → User A sees it checked

### Milestone 5: AI Suggestion Loop (after Step 19)

- Create a note about HVAC filters → AI suggests relation to HVAC tracking item → user accepts in review UI → relation persists

### Milestone 6: Full Daily Workflow (after Steps 12–15 + 20)

- Morning: check daily routines → complete quests → review notes
- Shopping: check low-stock → add to shopping list → check off at store
- Evening: capture notes → see AI-suggested relations → review and accept

---

## Timeline Mapping

| Step | Priority | Est. Effort | Can Parallel With |
|---|---|---|---|
| 1. Monorepo Scaffold + Contracts | P0 | 1 day | — |
| 2. Backend Foundation (Axum + Postgres) | P0 | 2 days | 3, 4 |
| 3. Web Client Scaffold (SvelteKit) | P0 | 1 day | 2, 4 |
| 4. Android Client Scaffold | P0 | 1 day | 2, 3 |
| 5. Auth + Identity | P0 | 2.5 days | — |
| 6. Core Domain Backend | P0 | 2 days | — |
| 7. PowerSync Setup + Proof-of-Life | P0 | 3 days | — |
| 8. Guidance Backend | P0 | 2.5 days | 9, 10 |
| 9. Knowledge Backend | P0 | 2 days | 8, 10 |
| 10. Tracking Backend | P0 | 2.5 days | 8, 9 |
| 11. Relationships Backend | P0 | 1.5 days | — |
| 12. Web Client — Guidance + Core UI | P0 | 4 days | 13 |
| 13. Android Client — Core + Guidance | P0 | 4 days | 12 |
| 14. Web Client — Knowledge + Tracking | P1 | 4 days | 15 |
| 15. Android Client — Knowledge + Tracking | P1 | 4 days | 14 |
| 16. Attachments | P1 | 3 days | — |
| 17. Search | P1 | 2.5 days | 18 |
| 18. Desktop (Tauri) | P2 | 3 days | 17 |
| 19. AI Enrichment | P2 | 3 days | — |
| 20. Notifications + Household | P2 | 3 days | — |
| **Total** | | **~50 days** | |

> **Critical path to first working web prototype (today view + sync):** Steps 1 → 2 → 5 → 6 → 7 → 8 → 12 = ~18 working days

> **Critical path to Android prototype:** Add Step 4 + 13 after Step 7 = ~20 working days

> **Steps 8, 9, 10 are parallel** — all three domain backends can be built simultaneously after Step 6. Combined wall-clock time: ~3 days instead of ~7.

> **Steps 12+13 and 14+15 are parallel pairs** — web and Android feature work can proceed simultaneously.

---

## Design Decisions Summary

| # | Decision | Rationale |
|---|---|---|
| 1 | PostgreSQL as primary DB | Ecosystem maturity, PowerSync compatibility, self-hosting ease. ADR-002. |
| 2 | PowerSync for sync | Best fit for Postgres → SQLite offline-first architecture. ADR-003. |
| 3 | Relationships as first-class domain records | Core product value depends on cross-domain links being durable and queryable. ADR-004. |
| 4 | Monorepo with shared contracts | One source of truth for entity types, relation types, sync streams. No drift. |
| 5 | Web + Desktop share Svelte UI; Android is native | Right tool per platform. Mobile needs camera, notifications, widgets. Web/desktop share naturally. |
| 6 | Backend is Rust modular monolith | Single deployable, clean module boundaries, extract services only when justified. |
| 7 | AI is optional and degrades gracefully | Altair must work fully without AI configured. AI enriches but never gates. |
| 8 | Attachments are metadata-first | PowerSync syncs metadata rows. Binaries stored in object storage, fetched on demand. |
| 9 | Entity type registry enforced at write time | Backend rejects unknown entity types. Prevents ad hoc string drift across codebases. |
| 10 | Sync scopes based on clear boundaries (user, household, initiative) | Avoids graph-traversal-based sync scope definitions. Keeps sync predictable. |
| 11 | "Digital Sanctuary" design system from DESIGN.md | Shared visual language across web and Android. Web implements directly; Android adapts through Material 3 `ColorScheme` mapping with faster touch transitions and accepted component differences. |
| 12 | Android is single Gradle module | No KMP target exists (WearOS is Android, desktop is Tauri/Svelte). Package-based separation provides the same logical boundaries without Gradle overhead. |

---

## Risk Register

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| PowerSync integration complexity | Medium | High | Step 7 is dedicated proof-of-life. Fail fast if sync doesn't work. |
| Sync scope design becomes unwieldy | Medium | Medium | Start with simple scopes, add on-demand streams only when proven needed. |
| Android + Web feature parity drift | Medium | Medium | Shared contracts prevent schema drift. Accept UI-level differences. |
| Relationship model over-complexity | Low | Medium | Design guardrails from ADR-004. Don't create relations for every FK. |
| AI scope creep | Medium | Medium | AI is last priority (Step 19). Ship without it first. |
| Solo developer bandwidth | High | High | Steps sized for AI-assisted development. Parallel tracks are aspirational — serialize if needed. |
