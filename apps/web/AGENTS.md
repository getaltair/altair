# Web App Guidelines

**Generated:** 2026-03-11 | **Commit:** 5409048 | **Branch:** main

> SvelteKit 2 + Tauri 2 + Tailwind CSS 4

## OVERVIEW

Single codebase for web (SSR) and desktop (Tauri). Dual adapter build.

## STRUCTURE

```
apps/web/
├── src/
│   ├── lib/
│   │   ├── server/     # Auth, DB (server-only)
│   │   └── paraglide/   # i18n (generated)
│   ├── routes/         # SvelteKit routes
│   └── hooks.server.ts # Auth + i18n middleware
└── src-tauri/          # Tauri desktop config
```

## WHERE TO LOOK

| Task | Location |
|------|----------|
| Auth config | `src/lib/server/auth.ts` |
| DB schema | `src/lib/server/db/schema.ts` |
| API routes | `src/routes/api/` |
| Tauri commands | `src-tauri/src/lib.rs` |
| Tauri config | `src-tauri/tauri.conf.json` |

## COMMANDS

```bash
bun run dev              # Dev server
bun run build            # Web build (node adapter)
bun run build:desktop    # Tauri build (static adapter)
bun run check            # svelte-check
bun run lint             # ESLint + Prettier
bun run test:unit        # Vitest
bun run test:e2e         # Playwright
bun run db:start         # PostgreSQL (Docker)
bun run db:push          # Drizzle schema push
```

## CONVENTIONS

### Dual Adapter Build

- `bun run build` → `adapter-node` (SSR web)
- `bun run build:desktop` → `adapter-static` (Tauri)

Controlled by `BUILD_TARGET` env var in `svelte.config.js`.

### Auth Pattern

Better-Auth with Drizzle adapter. Session in `event.locals`.

```typescript
// hooks.server.ts - ALWAYS this order
export const handle = sequence(handleParaglide, handleBetterAuth);
```

### Vitest Dual Project

- **Client**: `src/**/*.svelte.{test,spec}.{js,ts}` (browser)
- **Server**: `src/**/*.{test,spec}.{js,ts}` (node, excludes component tests)

### i18n (Paraglide)

Messages in `src/lib/paraglide/messages/`. Use `$lib/paraglide/runtime` for direction.

## ANTI-PATTERNS

- **NEVER** import server code in client (`.server.ts` files)
- **NEVER** use `localStorage` in SvelteKit SSR code
- **NEVER** skip `handleParaglide` in middleware chain

## MCP TOOLS (Svelte)

When working with Svelte/SvelteKit:

1. **list-sections** — Use FIRST to discover docs
2. **get-documentation** — Fetch relevant sections
3. **svelte-autofixer** — ALWAYS run on new Svelte code
4. **playwright-link** — NEVER if code written to project files

## NOTES

- Tailwind v4 uses CSS config (no tailwind.config.js)
- Prettier + ESLint configured (see `eslint.config.js`)
- `no-undef: off` in ESLint (TypeScript handles it)
