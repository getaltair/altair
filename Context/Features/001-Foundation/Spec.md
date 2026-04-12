# Feature 001: Foundation

| Field | Value |
|---|---|
| **Feature** | 001-Foundation |
| **Source** | `docs/specs/10-PLAN-001-v1.md` — Step 1 |
| **Status** | Draft |
| **Last Updated** | 2026-04-12 |

---

## Overview

Foundation establishes the monorepo infrastructure that every subsequent Altair feature depends on: a working Rust server skeleton, SvelteKit web skeleton, Android project skeleton, a Docker Compose stack running all required services (PostgreSQL, PowerSync, MongoDB, Zitadel, Garage), a CI pipeline, database migration tooling, and the initial user/household schema with a working OIDC login flow.

Nothing else can be built until this step is complete. It is the enabling substrate, not a user-facing feature.

---

## Problem Statement

The Altair monorepo exists as a directory structure with documentation, ADRs, and specs, but contains no runnable code. Before any domain work (Guidance, Knowledge, Tracking) can begin, each stack must compile, each service must start, developers must be able to log in via OIDC, and the CI pipeline must enforce baseline quality gates on every push.

Without Foundation, there is no development environment to build on.

---

## User Stories

- As a developer, I want `docker compose up` to start the full Altair stack so that I have a local development environment without manual service setup.
- As a developer, I want `cargo build` to succeed on the server crate so that I can begin adding API endpoints.
- As a developer, I want `bun run dev` to start the SvelteKit development server so that I can build web UI.
- As a developer, I want `./gradlew build` to succeed on the Android project so that I can begin building mobile screens.
- As a developer, I want CI to run on every push to `main` so that regressions are caught before merging.
- As a self-hoster, I want a guided OIDC login flow via Zitadel so that I can authenticate against my own instance from day one.
- As a developer, I want sqlx migration tooling in place so that schema changes are versioned, reversible, and applied consistently.

---

## Requirements

### Must Have

- **M-1**: Monorepo directory structure per ADR-007: `apps/web/`, `apps/android/`, `apps/server/`, `packages/`, `infra/migrations/`, `infra/docker/`, `infra/compose/`, `infra/scripts/`.
- **M-2**: Rust Cargo workspace at the repo root or `apps/server/` with a server binary crate and a worker binary crate. `cargo build` must succeed.
- **M-3**: SvelteKit 2 / Svelte 5 project scaffolded at `apps/web/`. `bun run dev` must start the development server.
- **M-4**: Android Gradle project scaffolded at `apps/android/`. `./gradlew build` must succeed in Android Studio.
- **M-5**: Docker Compose configuration (`infra/compose/docker-compose.yml`) that starts PostgreSQL, PowerSync Open Edition, MongoDB (PowerSync dependency), Zitadel (OIDC provider), and Garage (S3-compatible object storage). `docker compose up` must bring all services healthy.
- **M-6**: CI pipeline (GitHub Actions) that runs on push to `main`: `cargo build` + `cargo clippy`, `bun run check`, Android lint, and a Docker Compose smoke test.
- **M-7**: sqlx migration tooling configured for the server. `sqlx migrate run` must apply migrations against the Compose PostgreSQL instance.
- **M-8**: Initial database migration creating `users`, `households`, and `household_memberships` tables with UUID primary keys, `created_at`/`updated_at` timestamps, and `deleted_at` soft-delete columns. Migration must include a rollback procedure.
- **M-9**: Zitadel OIDC application configured (realm, client ID, redirect URIs for web and Android) with documented configuration values. End-to-end OIDC login from the SvelteKit app must succeed.
- **M-10**: Seed data script (`infra/scripts/seed.sql` or equivalent) that creates at least one user record after migration, usable for local development and CI smoke tests.
- **M-11**: `packages/contracts/` placeholder structure per ADR-007 (directory + README describing its future purpose; no code generation required in this step).
- **M-12**: No secrets hardcoded in source, migrations, or Docker Compose files — all credentials loaded from environment variables (SEC-5).

### Should Have

- **S-1**: Health endpoint (`GET /health`) on the Axum server returning 200 OK, verifiable in the CI smoke test.
- **S-2**: `mise.toml` updated to include all toolchain pins (Bun version, Rust toolchain channel) needed for CI setup.
- **S-3**: `.env.example` file documenting all required environment variables for a working local stack.

### Implementation Conventions (enforced as requirements)

- **IC-1**: All Rust dependencies must be added via `cargo add <crate>` — never by manually editing `Cargo.toml`.
- **IC-2**: All JavaScript/TypeScript dependencies must be added via `bun add <package>` — never by manually editing `package.json`.
- **IC-3**: All Android dependencies must be added via the Gradle DSL or `./gradlew` tasks — never by manually editing `build.gradle.kts` dependency blocks.
- **IC-4**: Project scaffolding must use the canonical CLI initializers (`cargo new`, `bun create svelte`, Android Studio project wizard or `npx create-*`) — never `cat`/`echo` to fabricate project files.
- **IC-5**: These tools exist for a reason: they resolve versions, populate lock files, and set up correct metadata. Bypassing them creates dependency drift and lock file inconsistencies.

### Won't Have (this iteration)

- Full authentication API endpoints (Step 3: Server Core)
- PowerSync sync stream configuration (Step 4: Sync Engine)
- Any domain business logic — Guidance, Knowledge, Tracking (Steps 5–7)
- Client UI screens (Steps 8–9)
- Search or attachment infrastructure (Step 10)
- Production Docker Compose configuration (Step 13)
- `packages/api-contracts/` or `packages/design-tokens/` populated (blocked on domain models)

---

## Testable Assertions

| ID | Assertion | Verification |
|---|---|---|
| FA-001 | `cargo build` succeeds for the server crate from a clean checkout | Run `cargo build` in CI; exit 0 |
| FA-002 | `bun run dev` starts the SvelteKit development server without errors | Run in CI; assert process is listening on expected port within 15s |
| FA-003 | `./gradlew build` succeeds for the Android project | Run in CI Android lane; exit 0 |
| FA-004 | `docker compose up` brings all five services (Postgres, PowerSync, MongoDB, Zitadel, Garage) to healthy status | Run `docker compose ps`; all services report `healthy` |
| FA-005 | `sqlx migrate run` applies the initial migration without errors against the Compose Postgres instance | Run as part of CI smoke test; exit 0 |
| FA-006 | The `users` table exists in PostgreSQL after migration, with `id`, `created_at`, `updated_at`, `deleted_at` columns | Query `information_schema.columns` in CI |
| FA-007 | The `households` and `household_memberships` tables exist after migration | Query `information_schema.tables` in CI |
| FA-008 | Seed script inserts at least one user record visible in `SELECT * FROM users` | Assert row count ≥ 1 after seeding in CI |
| FA-009 | OIDC login flow from the SvelteKit app redirects to Zitadel and returns a valid access token on successful authentication | Manual end-to-end test with local Compose stack |
| FA-010 | Initial migration includes a rollback procedure that cleanly removes all created tables | Run `sqlx migrate revert`; assert tables no longer exist |
| FA-011 | CI pipeline runs and passes on a push to `main` with no code changes beyond scaffold | Verify GitHub Actions workflow completes green |
| FA-012 | No environment secrets (passwords, client secrets, API keys) appear in any committed file | `git grep` for known secret patterns returns no matches |

---

## Open Questions

- [ ] **OQ-F-1**: Should Zitadel run against its own PostgreSQL database or share the Altair PostgreSQL instance? (Memory/complexity trade-off — ADR-002 deployment targets relevant.)
- [ ] **OQ-F-2**: Should the Cargo workspace root be at the repo root (with `apps/server/` as a member) or nested inside `apps/server/`? ADR-007 implies a nested workspace is acceptable.
- [ ] **OQ-F-3**: Should PowerSync Open Edition be configured as a separate service with its own YAML config, or does the PowerSync container self-configure from environment variables?
- [ ] **OQ-F-4**: What is the minimum Zitadel configuration needed for step 1 (basic OIDC flow for web), and what can be deferred until Android client work (Step 8)?

---

## Dependencies

| Dependency | Type | Notes |
|---|---|---|
| ADR-007: Monorepo Structure | Structural | Directory layout and workspace strategy |
| ADR-006: Auth and Session Model | Architectural | Zitadel as OIDC provider; Axum as relying party |
| ADR-002: Deployment Targets | Constraints | Memory budget for full Compose stack (4GB min, 8GB recommended) |
| `docs/specs/10-PLAN-001-v1.md` Step 1 | Source | Done-when criteria and scope definition |
| `docs/specs/03-invariants.md` | Constraints | SEC-5 (no hardcoded secrets), D-1 (migration rollback), D-2 (immutable migrations) |

---

## Revision History

| Date | Change | ADR |
|---|---|---|
| 2026-04-12 | Initial spec | — |
