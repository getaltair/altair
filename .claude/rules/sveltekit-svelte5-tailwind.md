---
paths:
  - "apps/web/**"
  - "**/*.svelte"
  - "**/*.ts"
  - "**/*.tsx"
---

# SvelteKit 5 / Svelte 5 / Tailwind CSS v4 Conventions

## Language & Framework
- Svelte 5 with runes ($state, $derived, $effect, $props, $bindable)
- SvelteKit 2 for full-stack application
- TypeScript strict mode enabled

## Code Style
- Formatter: prettier with prettier-plugin-svelte and prettier-plugin-tailwindcss
- Linter: eslint with eslint-plugin-svelte
- Component naming: PascalCase for files and usage
- Svelte 5 runes for all reactivity — no more $store, no more classic reactivity

## Svelte 5 Patterns
- State: `$state()` for reactive primitives and objects
- Derived: `$derived()` for computed values
- Props: `$props()` with generic interface for type safety
- Effects: `$effect()` for side effects, always return cleanup function
- Bindables: `$bindable()` for two-way binding props
- Snippets: `{#snippet}` for reusable template sections
- Render functions: `{@render}` for dynamic components

## SvelteKit Patterns
- Load functions: `+page.server.ts` for server data, `+page.ts` for universal
- Actions: `+page.server.ts` actions use FormData for form submissions
- Validation: validate in load functions, return errors to client
- No direct database access from client routes
- Use `$app/stores` for shared client state

## Styling
- Tailwind CSS v4 utility classes preferred
- No inline styles except for truly dynamic values
- Tailwind v4 CSS-first configuration via `@import` statements
- Use `@tailwindcss/typography` for prose styling where appropriate

## Testing
- Framework: Vitest with vitest-browser-svelte for component tests
- Playwright for E2E tests
- Test files: `*.test.ts` or `*.test.svelte` colocated with component
- Test behavior, not implementation details

## Database (Drizzle ORM)
- Drizzle ORM for all database operations
- Schema: TypeScript in `apps/web/src/lib/server/db/schema.ts`
- Migrations: `bun db:generate` and `bun db:migrate`
- All queries use prepared statements via Drizzle

## Authentication (Better Auth)
- Better Auth configuration in `apps/web/src/lib/server/auth.ts`
- Generate schema with `bun auth:schema`
- Server-side validation for all auth operations

## Error Handling
- Never bare `try/catch` — always specify exception type
- Log errors with structured context before re-raising
- User-facing error messages should be actionable
- Return HTTP status codes appropriately from server routes
