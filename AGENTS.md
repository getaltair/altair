# Altair Project Guidelines

**Generated:** 2026-03-11 | **Commit:** 5409048 | **Branch:** main

> Personal OS for knowledge, goals, resources. Offline-first sync across mobile/desktop.

## OVERVIEW

Monorepo: SvelteKit web + Tauri desktop + Rust backend + Android mobile.

```
altair/
├── apps/
│   ├── web/          # SvelteKit 2 + Tauri 2 (shared codebase)
│   ├── server/       # Rust backend (Axum) - scaffold
│   ├── worker/       # Rust background worker - scaffold
│   └── android/      # Kotlin/Jetpack Compose - scaffold
├── packages/         # Shared TS packages (empty - planned)
├── docs/             # PRDs, ADRs, architecture
└── infra/            # Infrastructure config (empty - planned)
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| Auth setup | `apps/web/src/lib/server/auth.ts` |
| Database schema | `apps/web/src/lib/server/db/schema.ts`, `auth.schema.ts` |
| SvelteKit hooks | `apps/web/src/hooks.server.ts` |
| Tauri desktop | `apps/web/src-tauri/` |
| i18n config | `apps/web/src/lib/paraglide/` |
| Build configs | `apps/web/svelte.config.js`, `vite.config.ts` |
| Rust workspace | `Cargo.toml` (root) |

## COMMANDS

### Web App (run from `apps/web/`)

```bash
bun run dev                  # Start dev server
bun run build                # Build web (node adapter)
bun run build:desktop        # Build Tauri (static adapter)
bun run check                # Type check (svelte-check)
bun run lint                 # ESLint + Prettier
bun run test:unit            # Vitest (watch)
bun run test:unit -- --run   # Vitest (once)
bun run test:e2e             # Playwright
bun run db:start             # Start PostgreSQL (Docker)
bun run db:push              # Push schema changes
bun run db:studio            # Drizzle Studio
```

### Rust Apps

```bash
cargo build                  # Build all workspaces
cargo test                   # Run tests
cargo clippy                 # Lint
cargo fmt                    # Format
```

### Pre-commit

```bash
prek run                     # Run all hooks
prek install                 # Install hooks
```

## CONVENTIONS

### TypeScript/Svelte

**Imports:** `$lib` for `src/lib/`, `$app/*` for SvelteKit, group by framework → external → local.

**Formatting:** Tabs, no semicolons, single quotes, trailing commas.

**Svelte 5:** Use runes (`$state`, `$derived`, `$effect`, `$props`), `{@render children()}` for slots.

**Types:** `type` over `interface`, explicit return types on exports.

### Rust

- Edition 2024 (server/worker), Edition 2021 (Tauri)
- Hard tabs, 100 char width (see `rustfmt.toml`)
- Resolver version 3

### Testing

| Type | Pattern | Framework |
|------|---------|-----------|
| Component | `*.svelte.{test,spec}.{js,ts}` | Vitest + Playwright browser |
| Server/Unit | `*.{test,spec}.{js,ts}` | Vitest (Node) |
| E2E | `*.e2e.{ts,js}` | Playwright |

### File Naming

- Components: PascalCase (`Welcome.svelte`)
- Routes: SvelteKit conventions (`+page.svelte`)
- Lib files: kebab-case (`auth.ts`)

## ANTI-PATTERNS

- **NEVER** suppress types: `as any`, `@ts-ignore`, `@ts-expect-error`
- **NEVER** empty catch blocks
- **NEVER** delete failing tests to "pass"
- **DO NOT** remove Windows console prevention in `src-tauri/src/main.rs`
- **ALWAYS** use `$lib` alias, never relative paths beyond immediate parent

## UNIQUE STYLES

### Dual Adapter Build

`BUILD_TARGET=web` → `adapter-node` (SSR)
`BUILD_TARGET=desktop` → `adapter-static` (Tauri)

Configured in `svelte.config.js` via environment variable.

### Auth Middleware Chain

```typescript
// hooks.server.ts - ALWAYS in this order
export const handle = sequence(handleParaglide, handleBetterAuth);
```

### ESLint Customization

`no-undef: off` — TypeScript handles this; prevents false positives.

## PRE-COMMIT

Uses [prek](https://prek.j178.dev/) (not standard pre-commit):
- Trailing whitespace (except markdown)
- Prettier (TS/Svelte)
- ESLint
- cargo fmt / cargo clippy

## NOTES

- **Greenfield project** - Rust apps are placeholders
- **packages/ empty** - shared code not yet extracted
- **No CD pipelines** - CI only (build, test, lint)
- **Tauri inside web** - desktop shares web codebase, not separate app
- **src-tauri excluded from CI** - platform-specific deps
