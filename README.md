# Altair

[![CI](https://github.com/getaltair/altair/actions/workflows/ci.yml/badge.svg)](https://github.com/getaltair/altair/actions/workflows/ci.yml)

A self-hosted personal operating system for managing knowledge, goals, and resources across everyday life.

Altair integrates three primary domains:

- **Guidance** -- Goals, initiatives, routines, quests, and focus sessions
- **Knowledge** -- Notes, linked information, snapshots, and semantic search
- **Tracking** -- Household inventory, locations, categories, and shopping lists

## Principles

- **Offline-first** -- Clients work independently and sync when connected. No internet required for core functionality.
- **Self-hosted** -- You own your data. Runs on a Raspberry Pi (4GB+) or any small VPS.
- **Sync-safe** -- Conflicts are detected and surfaced, never silently overwritten.
- **AI-optional** -- AI features enhance the experience but are never required. The app works fully without them.

## Architecture

```
                  +-------------------+
                  |    PostgreSQL      |
                  |  (source of truth) |
                  +--------+----------+
                           |
                  +--------+----------+
                  |    PowerSync       |
                  |   (sync layer)     |
                  +--------+----------+
                           |
              +------------+------------+
              |                         |
     +--------+--------+      +--------+--------+
     |   Web Client     |      | Android Client  |
     |   SvelteKit 2    |      | Kotlin/Compose  |
     |   (SQLite/OPFS)  |      | (SQLite/Room)   |
     +---------+--------+      +--------+--------+
               |                         |
               +------------+------------+
                            |
                   +--------+----------+
                   |   Axum Server      |
                   |   (REST API)       |
                   +--------+----------+
                            |
              +-------------+-------------+
              |                           |
     +--------+--------+        +--------+--------+
     |  Zitadel (OIDC)  |        |  RustFS (S3)    |
     |  (auth/identity)  |        |  (attachments)  |
     +-----------------+        +-----------------+
```

## Tech Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Web | SvelteKit 2 / Svelte 5 / TypeScript | Browser client |
| Android | Kotlin / Jetpack Compose | Mobile client |
| Server | Rust / Axum | REST API |
| Database | PostgreSQL | Primary data store |
| Sync | PowerSync Open Edition | Offline-first sync |
| Auth | Zitadel (OIDC) | Identity and authentication |
| Storage | RustFS (S3-compatible) | Attachment and file storage |
| Search | PostgreSQL FTS + pgvector | Keyword and semantic search |

## Requirements

### Minimum (1-2 users)

- 4GB RAM (Raspberry Pi 4 or $5/mo VPS)
- 4-core ARM64 or x86_64
- 32GB storage + external for attachments
- Docker and Docker Compose

### Recommended (3-5 users)

- 8GB RAM (Raspberry Pi 5, NAS, or $15/mo VPS)
- 64GB+ storage

## Local Development Setup

### Prerequisites

- Docker and Docker Compose v2
- [mise](https://mise.jl.dev) — manages Bun, Rust, and Java toolchains
- sqlx CLI for running migrations

### Environment Setup

Copy the example env file and fill in credentials:

```bash
cp infra/compose/.env.example infra/compose/.env
# Edit infra/compose/.env — set POSTGRES_PASSWORD, ZITADEL_MASTERKEY (≥32 chars),
# GARAGE_RPC_SECRET, GARAGE_ADMIN_TOKEN, and update DATABASE_URL to match
```

See `infra/compose/.env.example` for all required variables and their descriptions.

### Install Toolchains

```bash
mise install    # installs Bun, Rust (stable), and Java 17 (temurin)
cargo install sqlx-cli --no-default-features --features postgres
```

### Start Infrastructure

```bash
cd infra/compose && docker compose up -d
# First run: allow ~30s for Zitadel to initialize
```

### Run Migrations

```bash
# From repo root — set DATABASE_URL to match your infra/compose/.env values
DATABASE_URL=postgres://altair_user:yourpassword@localhost:5432/altair_db \
  sqlx migrate run --source infra/migrations
```

### Apply Seed Data (optional, for local dev)

```bash
psql $DATABASE_URL -f infra/scripts/seed.sql
```

### Start Applications

Open separate terminals for each:

```bash
# Rust API server — listens on port 8000
cd apps/server
DATABASE_URL=postgres://altair_user:yourpassword@localhost:5432/altair_db cargo run

# SvelteKit web client — listens on port 5173
cd apps/web && bun install && bun dev

# Android — open apps/android/ in Android Studio, then run on emulator or device
```

## Project Structure

```
altair/
  apps/
    web/              SvelteKit 2 web client
    android/          Kotlin/Compose mobile client
    server/           Rust/Axum API server
  packages/
    api-contracts/    Shared API type definitions
    design-tokens/    Design system tokens
  infra/
    docker/           Dockerfiles
    compose/          Docker Compose configurations
    migrations/       PostgreSQL migrations (sqlx)
    scripts/          Deployment and seed scripts
  docs/               PRDs, architecture specs
    specs/            Generated spec templates
  Context/
    Decisions/        Architecture Decision Records
    Features/         Feature specs and implementation plans
    Backlog/          Ideas and bugs
    Reviews/          Code review findings
```

## Architecture Decisions

Key decisions are documented as ADRs in `Context/Decisions/`:

| ADR | Decision |
|-----|----------|
| [001](Context/Decisions/ADR-001-defer-desktop-to-v2.md) | Desktop (Tauri) deferred to v2 |
| [002](Context/Decisions/ADR-002-deployment-targets.md) | Deployment: 4GB min, 8GB recommended |
| [003](Context/Decisions/ADR-003-sync-protocol-conflict-resolution.md) | Sync: LWW with conflict logging |
| [004](Context/Decisions/ADR-004-search-embedding-strategy.md) | Search: Postgres FTS + external embeddings |
| [005](Context/Decisions/ADR-005-attachment-storage.md) | Storage: S3-compatible abstraction |
| [006](Context/Decisions/ADR-006-auth-session-model.md) | Auth: OAuth 2.0 / OIDC via Zitadel |
| [007](Context/Decisions/ADR-007-monorepo-structure.md) | Monorepo layout |
| [008](Context/Decisions/ADR-008-notification-ownership-delivery.md) | Notifications: server-owned + client fallback |

## Domains

### Guidance

Manage goals through a hierarchy of initiatives, epics, quests, and routines. Track focus sessions and daily check-ins. Routines automatically spawn recurring quests.

### Knowledge

Capture and connect information with markdown notes, `[[wiki-links]]`, backlinks, and automatic snapshots. Search across all content with full-text and optional semantic search.

### Tracking

Monitor household inventory across locations and categories. Log consumption, purchases, and moves. Get low-stock alerts. Manage shopping lists linked to inventory.

### Cross-Domain

All three domains connect through a shared entity relation system. Tag anything. Link notes to quests, items to initiatives, or any entity to any other. Search spans all domains.

## Contributing

Altair uses conventional commits and a spec-driven development workflow. See [CLAUDE.md](CLAUDE.md) for development conventions and [DESIGN.md](DESIGN.md) for the design system.

## License

[GNU Affero General Public License v3.0](LICENSE)
