# ADR-007: Monorepo Structure

## Status

Accepted

## Date

2026-04-12

## Context

Altair is a multi-platform project with a Rust server, SvelteKit web app, and Kotlin Android app sharing a common data model. Coordinated schema changes, shared API contracts, and unified documentation favor a monorepo over separate repositories.

The project uses `mise` for toolchain management (Bun + Rust). No build orchestration exists yet.

## Decision

### Repository Layout

```
altair/
├── apps/
│   ├── web/                    # SvelteKit 2 + Svelte 5 (Bun)
│   ├── android/                # Kotlin + Jetpack Compose (Gradle)
│   └── server/                 # Rust + Axum (Cargo)
├── packages/
│   ├── api-contracts/          # Shared API type definitions
│   └── design-tokens/          # Design system tokens (from DESIGN.md)
├── infra/
│   ├── docker/                 # Dockerfiles per service
│   ├── compose/                # Docker Compose configs (dev, prod)
│   ├── migrations/             # PostgreSQL migrations (sqlx)
│   └── scripts/                # Seed data, deployment helpers
├── docs/                       # PRDs, architecture specs
│   └── specs/                  # Generated spec templates
├── Context/                    # Working context (features, ADRs, backlog)
│   ├── Decisions/              # ADRs
│   ├── Features/               # Feature specs, tech plans, steps
│   ├── Backlog/                # Ideas and bugs
│   └── Reviews/                # Code review findings
├── CLAUDE.md
├── DESIGN.md
└── mise.toml
```

### Workspace Strategy

Each app manages its own build toolchain:

- **`apps/web/`** — Bun workspace. `package.json` with Bun as runtime. SvelteKit's own build system (Vite).
- **`apps/server/`** — Cargo workspace. Standard Rust project with `Cargo.toml`. Background worker is a separate binary in the same workspace.
- **`apps/android/`** — Gradle project. Android Studio manages build. Independent of Bun/Cargo.
- **No top-level Bun workspace** — The web app is the only JS/TS project in v1. A top-level workspace adds complexity without benefit until there are multiple JS packages.

### Shared Contracts

**What to share:**
- API request/response types (DTOs)
- Entity type registry (canonical identifiers used across all platforms)
- Design tokens (CSS custom properties, color values, typography scales)
- Database schema (migrations are the shared source of truth)
- PowerSync sync schema definition

**How contracts are shared:**
- `packages/api-contracts/` contains canonical type definitions
- Server generates OpenAPI spec from Axum route definitions
- Web client types generated from OpenAPI spec (or hand-maintained TypeScript matching Rust types)
- Android models maintained manually to match — Kotlin has no codegen from Rust types without overhead that isn't justified for a small team
- Entity type registry is a simple enum/constant file duplicated per platform with a test asserting parity

**What stays platform-native:**
- UI components and platform behavior
- Platform-specific storage and auth integration
- Build configuration and tooling

### Migrations

PostgreSQL migrations live in `infra/migrations/`, managed by `sqlx`. Single source of truth for schema. Migrations apply to the shared Postgres instance and must maintain PowerSync compatibility (see ADR-003).

## Consequences

### Positive

- Single repo for coordinated schema changes across server + clients
- Shared docs, ADRs, and specs live alongside code
- No cross-repo version synchronization needed
- Simple `mise install` sets up all toolchains

### Negative

- Android Gradle and Rust Cargo are unaware of each other — no unified build command
- Contract drift between platforms possible without explicit parity tests
- Repo size will grow as Android assets and binaries accumulate

### Neutral

- Desktop app (v2) slots into `apps/desktop/` when ready — no structural changes needed
- Top-level Bun workspace can be introduced later if `packages/` grows JS packages
- CI will need per-app build pipelines (not a single unified build)
