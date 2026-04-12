# ADR-001: Defer Desktop (Tauri) App to v2

## Status

Accepted

## Date

2026-04-12

## Context

Altair's architecture originally specified four client platforms for v1: Web (SvelteKit), Desktop (Tauri wrapping SvelteKit), Android (Kotlin/Compose), and Server (Rust/Axum).

The primary motivation for a desktop app was **local file storage** — avoiding a dependency on cloud object storage (S3) for attachments like PDFs and images. Desktop could store files on the local filesystem, making the system fully self-contained without external storage infrastructure.

However, the server's attachment strategy now targets **self-hosted S3-compatible storage** (Garage, RustFS, or similar). This eliminates the architectural need for local filesystem access as a storage escape hatch — the self-hosted server already provides file storage without external cloud dependencies.

Additionally:
- Web covers the same SvelteKit UI with zero distribution overhead
- Multi-window workflows are increasingly available via browser split-tab features (Chrome)
- Android covers the portable/accessible use case with native local storage
- Desktop adds distribution cost (binary builds, auto-updates, platform testing for Linux + Windows) without unique capability

## Decision

Defer the Tauri desktop application to v2. Focus v1 on three platforms:

1. **Web** — SvelteKit 2 / Svelte 5 SPA with PowerSync
2. **Android** — Kotlin / Jetpack Compose with local SQLite
3. **Server** — Rust / Axum with PostgreSQL + self-hosted S3-compatible object storage

## Consequences

### Positive

- Reduced v1 surface area (3 platforms instead of 4)
- No Tauri build pipeline, IPC layer, or platform-specific bug surface in v1
- Team can focus on sync reliability (the highest-risk area) instead of desktop packaging
- Self-hosted S3-compatible storage serves all clients uniformly

### Negative

- Users who prefer a native desktop app must use the web client in v1
- Obsidian-like local file workflows deferred
- Desktop-specific features (system tray, OS integration, offline without browser) unavailable in v1

### Neutral

- No architectural changes required for v2 desktop addition — SvelteKit UI will exist, Tauri wraps it
- Desktop app conventions (`.claude/rules/tauri.md`) remain in the repo for v2 readiness
- PowerSync client integration for desktop shares the web implementation

## Alternatives Considered

1. **Build desktop in v1** — Rejected due to distribution overhead without unique architectural value given self-hosted object storage decision
2. **PWA instead of desktop** — Viable for "app in dock" use case but doesn't provide local filesystem access; revisit alongside v2 desktop decision
3. **Drop desktop permanently** — Premature; local-first file workflows have genuine value worth evaluating once v1 is stable
