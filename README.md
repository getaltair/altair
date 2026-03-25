# Altair

Personal operating system for managing knowledge, goals, and resources.

## Structure

- `apps/server` — Rust (Axum) backend
- `apps/web` — SvelteKit web client
- `apps/desktop` — Tauri desktop client (shares web UI)
- `apps/android` — Kotlin + Jetpack Compose
- `packages/contracts` — Shared entity type registries and generated bindings
- `infra/` — Docker Compose, migrations, PowerSync config
- `docs/` — PRDs, architecture specs, ADRs

## Prerequisites

- Rust (stable)
- Node.js 20+ and pnpm
- Docker and Docker Compose
- just (command runner): `cargo install just`

## Quick Start

```sh
just setup    # install dependencies
just dev      # start dev environment
```
