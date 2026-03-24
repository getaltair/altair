# Altair

## Overview

Altair is a personal operating system for managing knowledge, goals, and resources across everyday life. It integrates three primary domains:

- **Guidance** — Goals, initiatives, routines, and task management
- **Knowledge** — Notes and linked information
- **Tracking** — Inventory and resource monitoring

The platform operates across multiple device classes with offline-first synchronization:

- **Tier 1**: Android Mobile, Web Application
- **Tier 2**: Linux Desktop, Windows Desktop (Tauri)
- **Tier 3**: WearOS, iOS (future)

## Architecture

Altair follows a multi-tier, multi-platform architecture:

- **Web Platform** (`apps/web/`) — SvelteKit 5 web app that serves as both browser application and Tauri desktop frontend
- **Desktop Backend** (`apps/web/src-tauri/`) — Tauri 2 native integration for Linux/Windows
- **Server** (`apps/server/`) — Rust backend server with Axum for API endpoints
- **Worker** (`apps/worker/`) — Rust background worker for async processing
- **Mobile** (`apps/android/`) — Native Android app with Kotlin/Jetpack Compose
- **Database** — PostgreSQL for server, SQLite for client (via Drizzle ORM)
- **Sync** — PowerSync for offline-first synchronization across devices
- **Shared Packages** (`packages/`) — Common code shared across platforms

## Tech Stack

| Stack | Directory | Purpose |
|------|----------|--------|
| SvelteKit 2 / Svelte 5 / TypeScript / Tailwind CSS v4 | `apps/web/` | Web application and Tauri frontend |
| Tauri 2 / Rust | `apps/web/src-tauri/` | Desktop app wrapper (Linux/Windows) with native integrations |
| Rust / Axum | `apps/server/` | Backend API server |
| Rust | `apps/worker/` | Background worker for async processing |
| Android / Kotlin / Jetpack Compose | `apps/android/` | Native Android application |
| PostgreSQL / SQLite / Drizzle ORM | `apps/web/src/lib/server/db/` | Database layer (server: PostgreSQL, client: SQLite) |
| PowerSync | — | Offline-first synchronization |
| Better-Auth | `apps/web/src/lib/server/auth.ts` | Authentication |
| Paraglide JS | — | Internationalization |
| Vitest / Playwright | — | Testing (unit and E2E) |
| prek | — | Pre-commit hooks for code quality |

## Key Directories

- `Context/Features/` — Feature specifications, tech plans, and implementation steps
- `Context/Decisions/` — Architecture Decision Records (ADRs)
- `Context/Backlog/` — Ideas and bugs for future work
- `.claude/rules/` — Stack-specific coding conventions
- `apps/` — Platform applications (web, server, worker, android)
- `packages/` — Shared packages
- `docs/` — Architecture docs, PRDs, ADRs
- `infra/` — Infrastructure config (Docker, etc.)

## Development Workflow

This project uses a 4-phase planning workflow:
1. **Spec** — Define what and why (Spec.md with testable assertions)
2. **Tech Research** — Decide how to build (Tech.md with architecture decisions)
3. **Steps** — Break into tasks with tests and docs (Steps.md with milestones)
4. **Implementation** — Build, verify at milestones, commit

Quick tasks skip to a single-file quick plan.

## Conventions

- Conventional commits with task and ADR references
- ADRs required for any deviation from spec
- Milestone checkpoints verify spec alignment
- Test and documentation tasks are planned alongside implementation
- See `.claude/rules/` for stack-specific coding conventions
- Pre-commit hooks run via `prek` (formatting, linting, type checking)

## Setup

### Prerequisites
- [Bun](https://bun.sh) v1.3+
- [Rust](https://rustup.rs) (latest stable)
- [Docker](https://docker.com) (for PostgreSQL)
- [prek](https://prek.j178.dev/) (pre-commit hooks)

### Quick Start

```bash
# Install dependencies
bun install

# Install pre-commit hooks
prek install

# Start database
cd apps/web && bun run db:start

# Run web dev server
cd apps/web && bun run dev
```

### Key Commands

| Command | Purpose |
|---------|---------|
| `bun install` | Install JS/TS dependencies |
| `bun run dev` | Start SvelteKit dev server |
| `bun run test` | Run all tests (Vitest + Playwright) |
| `bun run check` | TypeScript type checking |
| `bun run lint` | ESLint + Prettier |
| `prek run` | Run pre-commit hooks manually |
| `cargo build` | Build Rust apps |
| `cargo test` | Run Rust tests |
| `cargo clippy` | Rust linting |

## Documentation

- [Core PRD](docs/prd/altair-core-prd.md) — Product requirements
- [Guidance PRD](docs/prd/altair-guidance-prd.md) — Goals and tasks domain
- [Knowledge PRD](docs/prd/altair-knowledge-prd.md) — Notes domain
- [Tracking PRD](docs/prd/altair-tracking-prd.md) — Inventory domain
- [Architecture Spec](docs/architecture/altair-architecture-spec.md) — Technical architecture
- [Implementation Plan](docs/altair-implementation-plan.md) — Development roadmap

## Active Work

- **Feature:** —
- **Status:** Not started
