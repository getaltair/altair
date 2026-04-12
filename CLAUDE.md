# Altair

## Overview

Altair is a personal operating system for managing knowledge, goals, and resources across everyday life. It integrates three primary domains: **Guidance** (goals, initiatives, routines), **Knowledge** (notes and linked information), and **Tracking** (inventory and resource monitoring).

The platform is offline-first, sync-aware, and self-hosted. Clients operate independently and synchronize changes when connectivity returns. Sync conflicts must never silently lose data.

## Architecture

Monorepo with three client/server applications sharing a common data model:

- **Web** — SvelteKit 2 / Svelte 5 SPA with PowerSync for offline sync
- **Mobile** — Native Android with Kotlin / Jetpack Compose, SQLite local DB
- **Server** — Rust / Axum REST API backed by PostgreSQL + self-hosted S3-compatible object storage

Desktop (Tauri 2) is deferred to v2 (see ADR-001).

Clients sync through PowerSync. The server is the source of truth via Postgres.

## Tech Stack

| Stack | Directory | Purpose |
|---|---|---|
| SvelteKit 2 / Svelte 5 / TypeScript | `apps/web/` | Web client |
| Kotlin / Jetpack Compose | `apps/android/` | Android mobile client |
| Rust / Axum | `apps/server/` | Backend API |
| PostgreSQL | server-side | Main database |
| PowerSync | sync layer | Client-server DB sync |
| SQLite | Android local | Offline-first local storage |
| S3-compatible (Garage/RustFS) | server-side | Attachment/file storage |

## Key Directories

- `docs/` — PRDs and architecture specifications
- `Context/Features/` — Feature specifications, tech plans, and implementation steps
- `Context/Decisions/` — Architecture Decision Records (ADRs)
- `Context/Backlog/` — Ideas and bugs for future work
- `Context/Reviews/` — Code review findings
- `.claude/rules/` — Stack-specific coding conventions

## Development Workflow

This project uses a 4-phase planning workflow:

1. **Spec** — Define what and why (`Spec.md` with testable assertions)
2. **Tech Research** — Decide how to build (`Tech.md` with architecture decisions)
3. **Steps** — Break into tasks with tests and docs (`Steps.md` with milestones)
4. **Implementation** — Build, verify at milestones, commit

Quick tasks skip to a single-file quick plan.

### Available Commands

| Command | Purpose |
|---|---|
| `/blueprint` | Full 4-phase planning: Spec, Tech, Steps, Implementation |
| `/quick` | Lightweight single-file plan for small tasks |
| `/spec-init` | Initialize spec-driven design templates |
| `/impl` | Execute Steps.md via hub-and-spoke agents |
| `/commit` | Git commit with conventional format |
| `/qa` | Run stack-specific QA agents in parallel |
| `/review-capture` | Capture review findings with 4-category system |
| `/review-resolve` | Work through captured review findings |
| `/adr` | Create or review Architecture Decision Records |
| `/backlog` | Add to or prioritize the backlog |

### Complexity Routing

- **Simple** (< 50 lines, single file): Direct implementation, no plan needed
- **Medium** (50-300 lines, 2-5 files): `/quick` plan
- **Complex** (> 300 lines, cross-cutting): `/blueprint` full planning cycle

## Conventions

- Conventional commits with task and ADR references
- ADRs required for any deviation from spec
- Milestone checkpoints verify spec alignment
- Test and documentation tasks are planned alongside implementation
- See `.claude/rules/` for stack-specific coding conventions

## Setup

```bash
# Tools managed via mise
mise install          # Install bun + rust toolchain

# Web app
cd apps/web && bun install && bun dev

# Server
cd apps/server && cargo run

# Android
# Open apps/android/ in Android Studio

# Desktop (v2 — not in active development)
# cd apps/desktop && cargo tauri dev
```

## Active Work

- **Feature:** Architecture review and planning
- **Status:** All 8 ADRs accepted. Implementation plan updated. Ready for Step 1 (Foundation).
- **Next:** Resolve remaining open questions (GAPS.md), then begin Step 1 per `docs/specs/10-PLAN-001-v1.md`
