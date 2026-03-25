# Repository Primer

## What This Repo Is

Altair is a family/household management platform for tracking households, initiatives, guidance quests, knowledge notes, and tracking items. It's a cross-platform monorepo supporting web, Android (Kotlin Compose), and desktop (Tauri) clients with shared backend services (Rust).

## Product / Feature Structure

| Feature / Module | Entry Point | Description |
| ---------------- | ------------------- | ---------------------------------------- |
| **Web App** | `apps/web/` | SvelteKit 2 web application with Svelte 5 runes |
| **Android App** | `apps/android/` | Kotlin Compose Android mobile application |
| **Desktop App** | `apps/web/src-tauri/` | Tauri desktop builds from web codebase |
| **Server** | `apps/server/` | Rust backend server (WIP) |
| **Worker** | `apps/worker/` | Rust background worker for async tasks (WIP) |

## Tech Stack

### Core Framework

- **SvelteKit 2** with Svelte 5 runes and TypeScript strict mode
- **Rust 2024** edition for backend services (server, worker)
- **Kotlin** with Jetpack Compose for Android
- **Tauri 2** for cross-platform desktop builds
- **bun** as JavaScript/TypeScript package manager
- **mise** for unified tool management (bun, java, prek, rust)

### Project Structure

```
apps/
  web/              # SvelteKit 2 web app with Svelte 5 runes
  android/           # Kotlin Compose Android application
  server/           # Rust backend server (WIP)
  worker/           # Rust background worker (WIP)
docs/
  schema/           # Database schema SQL files
  sync/             # Database seed files
Cargo.toml          # Rust workspace definition
mise.toml           # mise tool configuration
```

### Database

- **PostgreSQL** with `pgcrypto` extension enabled
- **Drizzle ORM** with TypeScript schema in `apps/web/src/lib/server/db/`
- **Initial schema** includes: users, households, initiatives, guidance_quests, knowledge_notes, tracking_items, entity_relations
- **Auth schema** via Better Auth: user, session, account, verification tables

### Authentication

- **Better Auth** with email/password authentication enabled
- **Drizzle adapter** for PostgreSQL integration
- **SvelteKit cookies plugin** for session management
- **Server hooks** in `apps/web/src/hooks.server.ts` handle auth sessions
- **User/session** exposed via `App.Locals` (see `apps/web/src/app.d.ts`)

### Payments / Billing

Not yet implemented.

### Analytics / Monitoring

Not yet implemented.

### Styling

- **Tailwind CSS v4** with Vite plugin integration
- **No component library** yet - building custom components
- **Dynamic Svelte compiler options** enable runes mode (excluding node_modules)

### Key Architectural Decisions

1. **Monorepo with shared types** - Database schema and auth configuration are shared across Rust and TypeScript services
2. **Rust workspace** - `Cargo.toml` defines workspace with `apps/server`, `apps/worker`, `apps/web/src-tauri`
3. **Dual build targets** - Web app can build to Node adapter (web) or Static adapter (desktop/Tauri) via `BUILD_TARGET` env var
4. **Better Auth SvelteKit integration** - Uses `svelteKitHandler` with custom hooks for session injection into `event.locals`
5. **Paraglide i18n** - Server middleware transforms page chunks with locale and text direction
6. **Test project split** - Vitest config separates client (Playwright browser) and server (Node environment) test runs
7. **Shared SQL schema** - `docs/schema/altair-initial-schema.sql` serves as reference for both Drizzle and native SQL migrations

## Important Paths

| Path | Purpose |
| ---- | ------- |
| `apps/web/src/hooks.server.ts` | Server hooks chain (Paraglide, Better Auth) |
| `apps/web/src/lib/server/auth.ts` | Better Auth configuration |
| `apps/web/src/lib/server/db/schema.ts` | Drizzle database schema definitions |
| `apps/web/src/lib/server/db/auth.schema.ts` | Better Auth schema (user, session, account, verification) |
| `apps/web/src/app.d.ts` | SvelteKit type definitions including `Locals.user` and `Locals.session` |
| `apps/web/svelte.config.js` | SvelteKit adapter switching (Node vs Static for Tauri) |
| `apps/web/vite.config.ts` | Vite config with Tailwind, devtools, Paraglide, and Playwright test setup |
| `apps/web/drizzle.config.ts` | Drizzle Kit configuration for PostgreSQL |
| `apps/web/eslint.config.js` | ESLint configuration with TypeScript and Svelte rules |
| `apps/web/playwright.config.ts` | Playwright E2E test configuration |
| `docs/schema/altair-initial-schema.sql` | Reference SQL schema for the application |
| `docs/sync/altair-dev-seed.sql` | Database seed data for development |
| `Cargo.toml` | Rust workspace definition (server, worker, web/src-tauri) |
| `mise.toml` | mise tool management (bun, java, prek, rust) |

## Build Commands

```bash
# Development
bun dev              # Start SvelteKit dev server (apps/web/)

# Rust services
cargo build           # Build all workspace members (server, worker, web/src-tauri)
cargo build -p server # Build server only
cargo build -p worker # Build worker only

# Android
./gradlew build       # Build Android app (from apps/android/)

# Desktop (Tauri)
BUILD_TARGET=desktop bun run build  # Build Tauri desktop app from web codebase
bun run tauri dev   # Run Tauri dev server

# Database (Drizzle)
bunx drizzle-kit generate  # Generate migrations from schema
bunx drizzle-kit push      # Apply migrations to database
bunx drizzle-kit studio    # Open Drizzle Studio GUI

# Testing
bun run test          # Run Vitest tests (client + server)
bun run test:e2e      # Run Playwright E2E tests

# Linting
bun run lint          # Run ESLint
bun run format        # Run Prettier (via ESLint config)
```

## Environment Variables

**Required for build:**

- `DATABASE_URL` - PostgreSQL connection string for Drizzle ORM
- `BETTER_AUTH_SECRET` - Secret key for Better Auth session encryption
- `ORIGIN` - Canonical app URL for Better Auth (e.g., http://localhost:5173)

**Required for full functionality:**

(Not yet implemented - email, payments, monitoring, etc.)

**Optional:**

- `BUILD_TARGET` - Build target: "web" (default, Node adapter) or "desktop" (Static adapter for Tauri)

## Common Gotchas

1. **Build target affects adapter** - Set `BUILD_TARGET=desktop` before building for Tauri. SvelteKit uses different adapters (Node vs Static).
2. **Drizzle env validation** - `drizzle.config.ts` throws if `DATABASE_URL` is not set. Ensure `.env` file exists.
3. **Session injection** - Better Auth sessions are injected via server hooks into `event.locals`. Access via `event.locals.session` and `event.locals.user` in routes.
4. **Rust workspace** - Cargo workspace is defined in root `Cargo.toml`. Individual app `Cargo.toml` files are workspace members.
5. **Test configuration** - Vitest runs two test projects: client (Playwright browser) and server (Node environment). Client tests exclude server code and vice versa.
6. **mise for tools** - Use mise to manage bun, java (for Android), prek, rust versions via `mise.toml`.
7. **Dynamic compile options** - Svelte 5 runes are enabled for all files except `node_modules` via `vitePlugin.dynamicCompileOptions`.

## Workflows

1. **Feature development** - Work primarily in `apps/web/` for shared logic. Android (`apps/android/`) is separate but shares domain concepts.
2. **Database changes** - Modify TypeScript schema in `apps/web/src/lib/server/db/`, then run `bunx drizzle-kit generate` and `push`. Update reference SQL in `docs/schema/` accordingly.
3. **Auth changes** - Modify `apps/web/src/lib/server/auth.ts`. Session is automatically injected into all routes via `hooks.server.ts`.
4. **Desktop development** - Use `BUILD_TARGET=desktop bun run tauri dev` to develop Tauri desktop app using same web codebase.
5. **Testing** - Use Vitest for unit tests and Playwright for E2E tests. Tests are split into client and server environments.

## Notes

1. The project is early-stage - Rust backend services (`apps/server/` and `apps/worker/`) are WIP (work in progress) with minimal implementations.
2. Android app uses Jetpack Compose and is largely independent from the web codebase.
3. Tauri desktop builds reuse the entire web codebase via Static adapter - no separate desktop UI needed.
4. Database schema (`docs/schema/altair-initial-schema.sql`) shows the intended domain model (households, initiatives, guidance quests, knowledge notes, tracking items, entity relations).
5. Better Auth uses Drizzle adapter with `provider: 'pg'` for PostgreSQL integration.
6. The project uses mise for tool management - run `mise install` after cloning to set up tools.
