# AGENTS.md

Altair is an ADHD-focused productivity app built with Tauri 2 + Svelte 5 + SurrealDB (embedded database).

## Commands

```bash
# Install dependencies
pnpm install

# Development (runs both frontend and backend)
pnpm tauri dev

# Build for production
pnpm tauri build

# Run Rust tests
cargo test --manifest-path src-tauri/Cargo.toml

# Run frontend tests
pnpm test

# Lint and typecheck
cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings
cargo fmt --manifest-path src-tauri/Cargo.toml --check
pnpm lint
pnpm check
```

## Project Structure

- `src-tauri/` — Rust backend (Tauri commands, business logic, database)
- `src/` — Svelte frontend (SvelteKit routes, stores, components)
- `docs/` — Architecture docs, PRDs, ADRs (read these for context)

## Conventions

### Rust (Backend)

- Use `thiserror` for error types, implement `Into<tauri::InvokeError>`
- Tauri commands go in `src-tauri/src/commands/`, one file per module
- Business logic goes in `src-tauri/src/modules/`, never in command handlers
- All entities use ULID string IDs (e.g., `"01HXK3..."`)
- Soft delete: set `deleted_at` field, never hard delete in normal operations
- Events published via event bus after database writes succeed

### Svelte (Frontend)

- Use Svelte 5 runes (`$state`, `$derived`, `$effect`), not legacy stores
- Stores go in `src/lib/stores/` with `.svelte.ts` extension
- Components use shadcn-svelte; check `src/lib/components/ui/` before creating new ones
- Call Tauri commands via typed wrappers in `src/lib/api/`, not raw `invoke()`
- Routes follow SvelteKit conventions in `src/routes/`

### Database (SurrealDB)

- Tables are singular snake_case: `quest`, `note`, `item`
- Namespace: `altair`, Database: `main`
- Migrations tracked in `_migration` table, run on startup
- Use record links for hierarchy, graph edges for cross-module relations

## Domain Vocabulary

| Term          | Meaning                                              |
| ------------- | ---------------------------------------------------- |
| Quest         | A focused task with energy cost (1-5), WIP limit = 1 |
| Epic          | A goal containing multiple Quests                    |
| Checkpoint    | Optional sub-step within a Quest                     |
| Energy Budget | Daily capacity (default 5), soft limit               |
| Note          | Markdown content with wiki-links `[[Title]]`         |
| Item          | Physical object in inventory                         |
| Location      | Fixed place (Room → Shelf)                           |
| Container     | Movable storage (Box, Toolbox)                       |

## Gotchas

- **WIP=1**: Only one Quest can be `active` at a time—reject if one exists
- **Energy is soft**: Warn but don't block if completing Quest exceeds budget
- **Embeddings are local**: Use bundled ONNX model via `ort`, not cloud API
- **Transcription is local**: Use bundled Whisper model via `whisper-rs`
- **Note titles unique per folder**: Same title OK in different folders
- **Item location XOR container**: Item has location_id OR container_id, never both

## Verification

Before completing any task:

1. `cargo test --manifest-path src-tauri/Cargo.toml` — all tests pass
2. `cargo clippy --manifest-path src-tauri/Cargo.toml -- -D warnings` — no warnings
3. `pnpm check` — TypeScript types valid
4. `pnpm lint` — no lint errors
5. App launches with `pnpm tauri dev` without errors

## When Stuck

- Read `docs/architecture/` for technical patterns
- Read `docs/requirements/` for acceptance criteria
- Read `docs/adr/` for decision rationale
- Ask a clarifying question rather than guessing
- Open a draft PR with notes if unsure about approach

**NEVER** commit code that fails verification checks.
**NEVER** use cloud AI APIs as defaults—local models are the default.
**NEVER** hard delete records—use soft delete with `deleted_at`.
