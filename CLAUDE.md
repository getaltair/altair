# CLAUDE.md

> **Project context for Claude Code** — Read this first

---

## Project Overview

**Altair** is an ADHD-focused productivity ecosystem with three apps:

| App | Purpose | Key Entities |
|-----|---------|--------------|
| **Guidance** | Task management (Quest-Based Agile) | Quest, Campaign |
| **Knowledge** | Personal knowledge management | Note, Folder |
| **Tracking** | Inventory management | Item, Location |

Plus **Quick Capture** for zero-friction input across all apps.

---

## Tech Stack

| Layer | Technology |
|-------|------------|
| Database | SurrealDB 2.x (embedded + cloud) |
| Object Storage | S3-compatible (Minio local, Backblaze B2 cloud) |
| Desktop | Tauri 2.0 + Svelte |
| Backend | Rust + Axum (localhost:3847) |
| Mobile | Tauri 2.0 Android |
| IPC | Tauri Commands (not REST for desktop) |
| Type Safety | tauri-specta (Rust → TypeScript) |
| Embeddings | Local ONNX (all-MiniLM-L6-v2) |

---

## Key Architectural Decisions

Before making changes, understand these decisions (see `docs/decision-log.md` for
full rationale):

1. **SurrealDB over SQLite** — Native graph queries, built-in vector search,
   change feeds for sync
2. **Tauri IPC over REST** — Desktop apps use Tauri commands, not HTTP
   (type-safe, no overhead)
3. **Local embeddings always on** — ~25MB ONNX model, no cloud dependency
   for semantic search
4. **Last-Write-Wins sync** — Simple conflict resolution, single-user focus
5. **Plugin architecture** — Auth and AI providers are trait-based plugins
6. **Soft delete everywhere** — `status: archived`, never hard delete

---

## Project Structure

```bash
altair/
├── apps/
│   ├── guidance/           # Tauri - Quest management
│   ├── knowledge/          # Tauri - PKM
│   ├── tracking/           # Tauri - Inventory
│   └── mobile/             # Tauri Android
├── packages/
│   ├── ui/                 # Svelte design system
│   ├── bindings/           # Generated TypeScript (tauri-specta)
│   ├── db/                 # SurrealDB schema + queries
│   ├── sync/               # Change feed sync
│   ├── storage/            # S3 client
│   └── search/             # Embeddings + hybrid search
├── backend/
│   ├── src/
│   │   ├── commands/       # Tauri IPC handlers
│   │   ├── api/            # REST handlers (mobile only)
│   │   ├── auth/           # Auth plugins
│   │   ├── sync/           # Sync engine
│   │   ├── embeddings/     # ONNX inference
│   │   ├── providers/      # AI plugins (optional)
│   │   └── storage/        # S3 integration
│   └── migrations/         # SurrealDB migrations
├── specs/                  # Specifications (SDD)
├── docs/                   # Architecture, domain model, etc.
└── CLAUDE.md               # You are here
```

---

## Terminology (Glossary)

Use these terms consistently:

| ✅ Use | ❌ Don't Use | Why |
|--------|-------------|-----|
| Quest | Task, Todo | Quest has energy cost, adventure framing |
| Campaign | Project, Epic | Campaign contains quests |
| Note | Document, Page | Note is the PKM entity |
| Item | Product, Asset | Item is the inventory entity |
| Capture | Inbox item, Draft | Capture is pending classification |
| Archive | Delete | Soft delete, recoverable |

See `docs/glossary.md` for full terminology.

---

## Domain Model

### Core Relationships

```text
Campaign →contains→ Quest →references→ Note
                         →requires→ Item
Note →links_to→ Note (wiki-links, bidirectional)
Note →documents→ Item
Item →stored_in→ Location
```

### Key Rules

- **All entities have `owner`** — Record-level auth via user reference
- **All tables have `CHANGEFEED 7d`** — Enables sync
- **All deletes are soft** — `status: archived`
- **References are soft links** — Deleting source doesn't delete target

See `docs/domain-model.md` for complete model.

---

## Development Patterns

### Adding a Tauri Command

```rust
// backend/src/commands/quest.rs
#[tauri::command]
pub async fn create_quest(
    state: State<'_, AppState>,
    title: String,
    energy_cost: EnergyCost,
) -> Result<Quest, ApiError> {
    // Implementation
}

// Register in main.rs
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        commands::quest::create_quest,
        // ...
    ])
```

TypeScript types auto-generate via tauri-specta.

### Adding a Database Table

1. Create migration: `migrations/NNN_add_table.surql`
2. Define schema with `CHANGEFEED 7d`
3. Add indexes for common queries
4. Update `packages/db/` with query functions

### Adding a New Feature

Follow spec-driven development:

1. Check `docs/spec-backlog.md` for next spec
2. Write spec using template in `specs/`
3. Generate plan from spec
4. Break into tasks
5. Implement

---

## Code Style

### Rust

- Use `Result<T, ApiError>` for fallible operations
- Prefer `async` for I/O operations
- Use `#[instrument]` from `tracing` for observability
- Run `cargo fmt` and `cargo clippy` before committing

### TypeScript/Svelte

- Use generated types from `packages/bindings/`
- Prefer `invoke()` over manual type definitions
- Use Tailwind for styling (utility classes only)
- Run `pnpm lint` before committing

### SurrealQL

- Use `SCHEMAFULL` tables (explicit schema)
- Add `ASSERT` constraints for validation
- Include `CHANGEFEED 7d` on all synced tables
- Use graph edges for relationships (`->edge->`)

---

## Common Tasks

### Run Development

```bash
# Start backend + all apps
pnpm dev

# Start specific app
pnpm --filter guidance dev

# Run backend only
cd backend && cargo run
```

### Run Tests

```bash
# All tests
pnpm test

# Rust tests
cd backend && cargo test

# Specific package
pnpm --filter @altair/db test
```

### Database Operations

```bash
# Run migrations (usually automatic on backend start)
cargo run --bin migrate

# Connect to embedded DB via standalone server (debugging)
surreal start --user root --pass root surrealkv:~/.local/share/altair/db
surreal sql --conn ws://localhost:8000 --ns altair --db main

# Reset database (development only)
rm -rf ~/.local/share/altair/db && cargo run --bin migrate
```

> **Note:** In normal development, SurrealDB runs embedded inside the backend
> process. The standalone server is only needed for direct SQL debugging.

### Generate Types

```bash
# Regenerate TypeScript bindings
cargo run --bin generate-bindings

# Types output to packages/bindings/
```

---

## Documentation

| Doc | Location | Purpose |
|-----|----------|---------|
| Technical Architecture | `docs/technical-architecture.md` | How the system works |
| Domain Model | `docs/domain-model.md` | Entities and relationships |
| User Flows | `docs/user-flows.md` | What users do |
| Glossary | `docs/glossary.md` | Consistent terminology |
| Decision Log | `docs/decision-log.md` | Why decisions were made |
| Spec Backlog | `docs/spec-backlog.md` | What to build next |

---

## AI Agent Guidelines

See `AGENTS.md` for:

- How to use Claude Code effectively on this project
- Spec-driven development workflow (spectrena)
- When to ask vs. when to act
- Code review expectations

---

## Quick Reference

### Ports

| Service | Port |
|---------|------|
| Backend API | 3847 |
| SurrealDB | 8000 |
| Minio | 9000 |
| Minio Console | 9001 |

### Environment Variables

```bash
# .env.local
SURREAL_URL=ws://localhost:8000
SURREAL_NS=altair
SURREAL_DB=main
S3_ENDPOINT=http://localhost:9000
S3_BUCKET=altair-local
```

### Key Files

| File | Purpose |
|------|---------|
| `backend/src/main.rs` | Backend entry point |
| `backend/migrations/` | Database schema |
| `packages/db/src/schema.ts` | TypeScript schema types |
| `packages/bindings/src/` | Generated Tauri command types |
| `apps/*/src/routes/` | Svelte routes |
- Always use 'pnpm'. NEVER use 'npm' or 'npx'.