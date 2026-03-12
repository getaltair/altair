# Altair Project Guidelines

This is a monorepo containing multiple applications built with TypeScript/JavaScript and Rust.

## Project Structure

```
altair/
├── apps/
│   ├── web/          # SvelteKit + Tauri desktop app
│   ├── server/       # Rust backend server
│   ├── worker/       # Rust background worker
│   └── android/      # Android mobile app
├── packages/         # Shared packages (currently empty)
├── docs/             # Architecture docs, PRDs, ADRs
└── infra/            # Infrastructure config
```

## Build/Lint/Test Commands

### Web App (apps/web/)

All commands should be run from `apps/web/` directory:

```bash
# Development
bun run dev                  # Start dev server

# Building
bun run build                # Build for web (node adapter)
bun run build:web            # Build for web target
bun run build:desktop        # Build for Tauri desktop (static adapter)
bun run preview              # Preview production build

# Type Checking
bun run check                # Run svelte-check
bun run check:watch          # Run svelte-check in watch mode

# Linting & Formatting
bun run lint                 # Run ESLint and Prettier checks
bun run format               # Format with Prettier

# Testing
bun run test:unit            # Run all unit tests with Vitest
bun run test                 # Run unit tests + e2e tests
bun run test:e2e             # Run Playwright e2e tests

# Running a single test
bun run test:unit -- path/to/test.spec.ts
bun run test:unit -- --run path/to/test.spec.ts    # Run once (no watch)
bun run test:unit -- --filter "test name"          # Filter by test name

# Database (requires Docker)
bun run db:start             # Start PostgreSQL container
bun run db:push              # Push schema changes to database
bun run db:generate          # Generate migrations
bun run db:migrate           # Run migrations
bun run db:studio            # Open Drizzle Studio

# Auth
bun run auth:schema          # Generate Better-Auth schema
```

### Rust Apps (apps/server/, apps/worker/, apps/web/src-tauri/)

```bash
# From repository root
cargo build                  # Build all Rust workspaces
cargo test                   # Run all Rust tests
cargo test -p server         # Run tests for specific package
cargo clippy                 # Run Clippy linter
cargo fmt                    # Format Rust code
```

## Code Style Guidelines

### TypeScript/Svelte

#### Imports
- Use `$lib` alias for imports from `src/lib/`
- Use `$app/*` for SvelteKit imports (`$app/state`, `$app/paths`, `$app/environment`, `$app/server`)
- Use `$env/dynamic/private` for server-side env vars, `$env/static/public` for client
- Group imports: framework/standard library first, then external packages, then local imports

```typescript
import { sequence } from '@sveltejs/kit/hooks';
import { building } from '$app/environment';
import { auth } from '$lib/server/auth';
import type { Handle } from '@sveltejs/kit';
```

#### Formatting
- Use tabs for indentation (consistent with Svelte defaults)
- No semicolons at end of statements (follow existing code patterns)
- Single quotes for strings
- Trailing commas in multiline objects/arrays

#### Svelte Components
- Use Svelte 5 runes: `$state`, `$derived`, `$effect`, `$props`
- Use `{@render children()}` for slots
- Component props: `let { prop1, prop2 } = $props();`
- Script tag: `<script lang="ts">` for TypeScript

#### Types
- Strict TypeScript enabled
- Use `type` for type definitions, `interface` only when extending
- Explicit return types for exported functions
- Use `svelte-check` for component type checking

#### Error Handling
- Throw errors with descriptive messages for configuration issues
- Use SvelteKit's `error()` function for HTTP errors in routes
- Validate environment variables at startup with clear error messages

```typescript
if (!env.DATABASE_URL) throw new Error('DATABASE_URL is not set');
```

### Testing

#### Unit Tests (Vitest)
- Server tests: `src/**/*.{test,spec}.{js,ts}` (excludes `.svelte.test.ts`)
- Component tests: `src/**/*.svelte.{test,spec}.{js,ts}`
- Use `vitest-browser-svelte` for component testing

```typescript
import { describe, expect, it } from 'vitest';
import { page } from 'vitest/browser';
import { render } from 'vitest-browser-svelte';
import MyComponent from './MyComponent.svelte';

describe('MyComponent', () => {
    it('renders correctly', async () => {
        render(MyComponent, { prop: 'value' });
        await expect.element(page.getByText('value')).toBeInTheDocument();
    });
});
```

#### E2E Tests (Playwright)
- Files: `**/*.e2e.{ts,js}`
- Import from `@playwright/test`

```typescript
import { expect, test } from '@playwright/test';

test('has expected h1', async ({ page }) => {
    await page.goto('/demo/playwright');
    await expect(page.locator('h1')).toBeVisible();
});
```

### Database (Drizzle)

- Schema files: `src/lib/server/db/*.ts`
- Use `pgTable` for PostgreSQL tables
- Export all schemas from `schema.ts`

```typescript
import { pgTable, serial, text } from 'drizzle-orm/pg-core';

export const task = pgTable('task', {
    id: serial('id').primaryKey(),
    title: text('title').notNull(),
});
```

### Rust

- Edition 2024 for server/worker, Edition 2021 for Tauri
- Follow standard Rust formatting (`cargo fmt`)
- Run `cargo clippy` for linting

## Key Technologies

- **Frontend**: SvelteKit 2, Svelte 5, Tailwind CSS 4
- **Desktop**: Tauri 2
- **Backend**: Rust
- **Database**: PostgreSQL with Drizzle ORM
- **Auth**: Better-Auth
- **i18n**: Paraglide JS
- **Testing**: Vitest (unit), Playwright (e2e)
- **Package Manager**: Bun

## File Naming Conventions

- Svelte components: PascalCase (e.g., `Welcome.svelte`)
- Route files: SvelteKit conventions (`+page.svelte`, `+layout.svelte`, `+page.server.ts`)
- Test files: `*.spec.ts` or `*.test.ts` for unit, `*.e2e.ts` for e2e
- Lib files: lowercase with dashes (e.g., `auth.ts`, `schema.ts`)

## Pre-commit Checklist

1. Run `bun run lint` in apps/web/
2. Run `bun run check` for type checking
3. Run `bun run test:unit -- --run` for tests
4. Run `cargo clippy` and `cargo fmt` for Rust code
