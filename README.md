# Altair

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
     |  Zitadel (OIDC)  |        |  Garage (S3)    |
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
| Storage | Garage / RustFS (S3-compatible) | Attachment and file storage |
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

## Quick Start

### Prerequisites

```bash
# Install mise (toolchain manager)
curl https://mise.run | sh

# Install project toolchains
mise install    # Installs Bun + Rust
```

### Development

```bash
# Start infrastructure (Postgres, PowerSync, Zitadel, Garage)
docker compose up -d

# Server
cd apps/server && cargo run

# Web client
cd apps/web && bun install && bun dev

# Android
# Open apps/android/ in Android Studio
```

### Production (Self-Hosted)

```bash
# Deploy full stack
docker compose -f infra/compose/docker-compose.prod.yml up -d
```

See the [self-hosting guide](docs/self-hosting.md) for detailed setup instructions.

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
