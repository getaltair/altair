# SvelteKit / Svelte 5 / TypeScript Conventions

Applies to: `apps/web/`

## Framework

- SvelteKit 2 with Svelte 5
- TypeScript strict mode
- Vite for bundling
- Vitest for unit tests, Playwright for E2E

## Svelte 5 Runes

- Use `$state()` for reactive declarations, not legacy `let` reactivity
- Use `$derived()` for computed values, not `$:` labels
- Use `$effect()` for side effects, not `afterUpdate` or reactive statements
- Use `$props()` in components, not `export let`
- Use `$bindable()` for two-way binding props
- Snippet blocks (`{#snippet}`) replace named slots

## File Naming

- Components: `PascalCase.svelte` (e.g., `QuestCard.svelte`)
- Routes: kebab-case directories following SvelteKit conventions
- Stores: `kebab-case.svelte.ts` (e.g., `sync.svelte.ts`)
- Types: `kebab-case.ts` in `src/lib/types/`
- Utils: `kebab-case.ts` in `src/lib/utils/`
- Tests: co-located as `*.spec.ts` or `*.e2e.ts`

## Imports

- Use `$lib/` alias for imports from `src/lib/`
- Group imports: svelte/sveltekit, external deps, internal modules, types
- Barrel exports via `index.ts` in feature directories

## Reactive Patterns

- Stores use `.svelte.ts` extension for rune-based reactivity
- Prefer fine-grained reactivity over coarse state objects
- Avoid mutating state objects directly; use explicit update functions
- Keep effects minimal; derive where possible
- Any `$effect()` containing an async iterator (`for await`) must include an `AbortController` and return a cleanup function that calls `controller.abort()`. This prevents iterator accumulation when the effect re-triggers (e.g. HMR, parent component remount). The cleanup return is also required even when using an `active` boolean flag.
- All async IIFEs inside `$effect()` must be followed by `.catch((err) => console.error('[module] watch failed:', err))` to surface PowerSync or SQLite errors rather than silently discarding them.

## Error Handling

- Use SvelteKit `error()` helper for HTTP errors in load functions
- Typed error boundaries at route level
- Never swallow errors silently in async operations

## Styling

- Scoped styles in `<style>` blocks by default
- Tailwind CSS via utility classes where appropriate
- CSS custom properties for theme tokens (see DESIGN.md)

## Testing

- Vitest for unit testing stores, utils, and component logic
- Playwright for E2E testing user flows
- Mock PowerSync in unit tests; use real sync in integration tests
