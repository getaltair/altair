# Altair

## Overview

Altair is a personal operating system for managing knowledge, goals, and resources across everyday life. It integrates three primary domains: **Guidance** (goals, initiatives, routines), **Knowledge** (notes and linked information), and **Tracking** (inventory and resource monitoring).

The platform is offline-first, sync-aware, and self-hosted. Clients operate independently and synchronize changes when connectivity returns. Sync conflicts must never silently lose data.

## Architecture

Monorepo with three client/server applications sharing a common data model:

- **Web** -- SvelteKit 2 / Svelte 5 SPA with PowerSync for offline sync
- **Mobile** -- Native Android with Kotlin / Jetpack Compose, SQLite local DB
- **Server** -- Rust / Axum REST API backed by PostgreSQL + self-hosted S3-compatible object storage

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
| RustFS (S3-compatible) | server-side | Attachment/file storage |

## Key Directories

- `docs/` -- PRDs and architecture specifications
- `docs/specs/` -- Spec-driven design documents (PRDs, domain model, invariants, ERD, etc.)
- `Context/Features/` -- Feature specifications, tech plans, and implementation steps
- `Context/Decisions/` -- Architecture Decision Records (ADRs)
- `Context/Backlog/` -- Ideas and bugs for future work
- `Context/Reviews/` -- Code review findings
- `.claude/rules/` -- Stack-specific coding conventions

## Development Workflow

This project uses a 4-phase planning workflow:

1. **Spec** -- Define what and why (`Spec.md` with testable assertions)
2. **Tech Research** -- Decide how to build (`Tech.md` with architecture decisions)
3. **Steps** -- Break into tasks with tests and docs (`Steps.md` with milestones)
4. **Implementation** -- Build, verify at milestones, commit

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
- **Trivial** (single file, obvious fix) -- Execute directly
- **Moderate** (2-5 files, clear scope) -- `/quick` then execute
- **Complex** (multi-phase, 5+ files) -- `/blueprint` then `/impl`
- **Collaborative** (cross-domain integration) -- `/blueprint` then `/team-impl`

## Conventions

- Conventional commits with task and ADR references
- ADRs required for any deviation from spec
- Milestone checkpoints verify spec alignment
- Test and documentation tasks are planned alongside implementation
- See `.claude/rules/` for stack-specific coding conventions

## Coding Guidelines

Guidelines to reduce common LLM coding mistakes. Biased toward caution over speed. For trivial tasks, use judgment.

### 1. Think Before Coding

Don't assume. Don't hide confusion. Surface tradeoffs.

- State assumptions explicitly. If uncertain, ask.
- If multiple interpretations exist, present them -- don't pick silently.
- If a simpler approach exists, say so. Push back when warranted.
- If something is unclear, stop. Name what's confusing. Ask.

### 2. Read Before You Write

Don't guess what a file contains. Don't rewrite what you haven't read.

- Read the full file (or relevant sections) before modifying it.
- Check for existing implementations before writing new ones.
- Look for patterns the codebase already uses -- follow them.
- If a file is long, read the specific function/section you're changing plus its callers.
- Never assume file contents from the filename alone.

### 3. Simplicity First

Minimum code that solves the problem. Nothing speculative.

- No features beyond what was asked.
- No abstractions for single-use code.
- No "flexibility" or "configurability" that wasn't requested.
- No error handling for impossible scenarios.
- If you write 200 lines and it could be 50, rewrite it.

Ask: "Would a senior engineer say this is overcomplicated?" If yes, simplify.

### 4. Surgical Changes

Touch only what you must. Clean up only your own mess.

When editing existing code:
- Don't "improve" adjacent code, comments, or formatting.
- Don't refactor things that aren't broken.
- Match existing style, even if you'd do it differently.
- If you notice unrelated issues, mention them -- don't fix them.

When your changes create orphans:
- Remove imports/variables/functions that YOUR changes made unused.
- Don't remove pre-existing dead code unless asked.

The test: every changed line should trace directly to the request.

Note: During /impl orchestrated execution, file ownership is enforced by agent prompts. These guidelines apply especially during direct coding and /quick tasks.

### 5. Goal-Driven Execution

Define success criteria. Loop until verified.

Transform tasks into verifiable goals:
- "Add validation" -- write tests for invalid inputs, then make them pass
- "Fix the bug" -- write a test that reproduces it, then make it pass
- "Refactor X" -- ensure tests pass before and after

For multi-step tasks, state a brief plan with verification at each step. Strong success criteria let you loop independently. Weak criteria ("make it work") require constant clarification.

Note: For /impl orchestrated work, Steps.md milestones already define success criteria and assertion checks. This guideline is most valuable for /quick and direct tasks.

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

# Desktop (v2 -- not in active development)
# cd apps/desktop && cargo tauri dev
```

## Active Work

- **Feature:** Architecture review and planning
- **Status:** All 8 ADRs accepted. Implementation plan updated. Ready for Step 1 (Foundation).
- **Next:** Resolve remaining open questions (GAPS.md), then begin Step 1 per `docs/specs/10-PLAN-001-v1.md`
