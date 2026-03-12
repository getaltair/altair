# Altair

A personal operating system for managing knowledge, goals, and resources across everyday life.

## Overview

Altair integrates three primary domains:

- **Guidance** — Goals, initiatives, routines, and task management
- **Knowledge** — Notes and linked information
- **Tracking** — Inventory and resource monitoring

The platform operates across multiple device classes with offline-first synchronization:

- **Tier 1**: Android Mobile, Web Application
- **Tier 2**: Linux Desktop, Windows Desktop
- **Tier 3**: WearOS, iOS (future)

## Tech Stack

| Component | Technology |
|-----------|------------|
| Web | SvelteKit 2, Svelte 5, TypeScript |
| Desktop | Tauri 2 (Linux, Windows) |
| Mobile | Android (Kotlin, Jetpack Compose) |
| Backend | Rust, Axum |
| Database | PostgreSQL (server), SQLite (client) |
| Sync | PowerSync |
| Auth | Better-Auth |
| ORM | Drizzle |
| i18n | Paraglide JS |
| Testing | Vitest (unit), Playwright (e2e) |

## Project Structure

```
altair/
├── apps/
│   ├── web/              # SvelteKit + Tauri desktop app
│   ├── server/           # Rust backend server
│   ├── worker/           # Rust background worker
│   └── android/          # Android mobile app
├── packages/             # Shared packages
├── docs/                 # Architecture docs, PRDs, ADRs
│   ├── prd/              # Product requirements
│   ├── architecture/     # Technical architecture
│   ├── adr/              # Architecture decision records
│   └── sync/             # Sync specification
└── infra/                # Infrastructure config
```

## Development

### Prerequisites

- [Bun](https://bun.sh) v1.3+
- [Rust](https://rustup.rs) (latest stable)
- [Docker](https://docker.com) (for database)

### Quick Start

```bash
# Install dependencies
bun install

# Start the database
cd apps/web && bun run db:start

# Run web dev server
cd apps/web && bun run dev
```

## Commands

### Web App (`apps/web/`)

```bash
bun run dev              # Start dev server
bun run build            # Build for web (node adapter)
bun run build:web        # Build for web target
bun run build:desktop    # Build for Tauri desktop
bun run preview          # Preview production build

bun run check            # Type check with svelte-check
bun run lint             # Run ESLint and Prettier
bun run format           # Format with Prettier

bun run test:unit        # Run unit tests (Vitest)
bun run test:e2e         # Run e2e tests (Playwright)
bun run test             # Run all tests

bun run db:start         # Start PostgreSQL container
bun run db:push          # Push schema changes to database
bun run db:generate      # Generate migrations
bun run db:migrate       # Run migrations
bun run db:studio        # Open Drizzle Studio

bun run auth:schema      # Generate Better-Auth schema
```

### Rust Apps (`apps/server/`, `apps/worker/`, `apps/web/src-tauri/`)

```bash
cargo build              # Build all Rust workspaces
cargo test               # Run all Rust tests
cargo test -p server     # Run tests for specific package
cargo clippy             # Run Clippy linter
cargo fmt                # Format Rust code
```

## Developer Setup

### Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| [Bun](https://bun.sh) | v1.3+ | JS/TS runtime & package manager |
| [Rust](https://rustup.rs) | Latest stable | Backend & Tauri apps |
| [Docker](https://docker.com) | Latest | Database (PostgreSQL) |
| [prek](https://prek.j178.dev/) | Latest | Git pre-commit hooks |

### IDE Setup

**VS Code (Recommended)**
1. Install recommended extensions (prompted on first open)
2. Extensions:
   - `svelte.svelte-vscode` - Svelte 5 support
   - `rust-lang.rust-analyzer` - Rust language server
   - `bradlc.vscode-tailwindcss` - Tailwind CSS IntelliSense
   - `esbenp.prettier-vscode` - Code formatting

**IntelliJ IDEA / Android Studio (Android)**
1. Install Kotlin plugin (bundled)
2. Enable EditorConfig support: Settings → Editor → Code Style → Enable EditorConfig

### Initial Setup

```bash
# 1. Install JS/TS dependencies
bun install

# 2. Install pre-commit hooks
prek install

# 3. Start the database
cd apps/web && bun run db:start

# 4. Run dev server
cd apps/web && bun run dev
```

### Formatting & Linting

**Run all linters:**
```bash
prek run
```

**Format all code:**
```bash
# TypeScript
cd apps/web && bun run format

# Rust
cargo fmt --all

# All (via prek)
prek run
```

**Language-specific commands:**

| Language | Format | Lint |
|----------|--------|------|
| TypeScript | `cd apps/web && bun run format` | `cd apps/web && bun run lint` |
| Rust | `cargo fmt --all` | `cargo clippy --all` |
| Kotlin | IDE auto-format | `cd apps/android && ./gradlew ktlintCheck` |

### Pre-Commit Hooks

This project uses [prek](https://prek.j178.dev/) for automated code quality checks:

- **Trailing whitespace** (except markdown)
- **File endings** (final newline)
- **YAML/TOML syntax** validation
- **Merge conflict** detection
- **Private keys** detection
- **Prettier** formatting (TypeScript/Svelte)
- **ESLint** linting (TypeScript/Svelte)
- **cargo fmt** formatting (Rust)
- **cargo clippy** linting (Rust)

Hooks run automatically on `git commit`. To run manually:
```bash
prek run              # All hooks
prek run prettier     # Specific hook
prek run --all-files  # Check all files
```

### Type Checking

```bash
# TypeScript
cd apps/web && bun run check

# Rust
cargo check --all
```

### Testing

```bash
# Unit tests (TypeScript)
cd apps/web && bun run test:unit

# E2E tests (TypeScript)
cd apps/web && bun run test:e2e

# Rust tests
cargo test --all
```

### Pre-Commit Checklist

Before committing, ensure:
- [ ] Code is formatted: `prek run` passes
- [ ] Types check: `cd apps/web && bun run check` passes
- [ ] Tests pass: `cd apps/web && bun run test:unit -- --run` passes
- [ ] Rust lints: `cargo clippy --all` passes

## Documentation

- [Core PRD](docs/prd/altair-core-prd.md) — Product requirements
- [Guidance PRD](docs/prd/altair-guidance-prd.md) — Goals and tasks domain
- [Knowledge PRD](docs/prd/altair-knowledge-prd.md) — Notes domain
- [Tracking PRD](docs/prd/altair-tracking-prd.md) — Inventory domain
- [Architecture Spec](docs/architecture/altair-architecture-spec.md) — Technical architecture
- [Implementation Plan](docs/altair-implementation-plan.md) — Development roadmap

## Key Features

- **Offline-first**: Full operation without connectivity
- **Multi-device sync**: Seamless synchronization across platforms
- **Cross-domain linking**: Connect tasks, notes, and inventory items
- **Household sharing**: Collaborate with family members
- **Self-hostable**: Full control over your data
- **Privacy-focused**: Optional AI that degrades gracefully
