# Plan: Fix Web Client Scaffold Review Feedback

## Task Description

Address all critical, important, and suggestion-level findings from the comprehensive PR review of the `feat/web-client-scaffold` branch. The review was conducted by 5 specialized agents (code-reviewer, silent-failure-hunter, type-design-analyzer, test-analyzer, comment-analyzer) and identified 4 critical issues, 7 important issues, and 9 suggestions across the SvelteKit web client scaffold in `apps/web/`.

## Objective

Resolve all critical and important review findings so the web client scaffold is production-ready with proper error handling, correct adapter configuration, type safety, and accurate documentation. Address suggestions where they improve maintainability without over-engineering.

## Problem Statement

The scaffold introduces foundational code (API client, theme store, auth plumbing, layout system) that will be built upon by every future feature. Defects in this foundation layer cascade into every downstream consumer. The review identified:

- **Zero error handling** in the API client (fetch, JSON parse, serialization failures all unhandled)
- **Wrong SvelteKit adapter** (adapter-auto vs documented dual Node/Static pattern for Tauri)
- **Missing hooks.server.ts** causing type/runtime mismatch (undefined vs null for locals)
- **Weak types** that defeat TypeScript strict mode (Record<string, unknown> for auth)
- **Silent failures** in theme store (localStorage crashes in Safari private mode)
- **Inaccurate comments** that will mislead future developers

## Solution Approach

Fix issues in 4 parallel workstreams grouped by domain, followed by a validation pass:

1. **API Client Overhaul** -- error handling, discriminated response union, Content-Type fix, comment accuracy
2. **Theme Store Hardening** -- localStorage try/catch, comment corrections
3. **Auth & Types Foundation** -- hooks.server.ts stub, concrete Locals types, auth gate logging, login form UX
4. **Config & Scaffold Polish** -- dual adapter, package manager alignment, error page, typo fix

## Relevant Files

### Existing Files to Modify

- `apps/web/src/lib/api/client.ts` -- API client: add error handling, discriminated union, fix Content-Type, fix comments
- `apps/web/src/lib/stores/theme.svelte.ts` -- Theme store: wrap localStorage in try/catch, fix JSDoc
- `apps/web/src/app.d.ts` -- Replace Record<string, unknown> with concrete auth interfaces
- `apps/web/src/routes/(app)/+layout.server.ts` -- Add auth bypass warning, normalize locals.user
- `apps/web/src/routes/login/+page.svelte` -- Add submit feedback for placeholder state
- `apps/web/svelte.config.js` -- Switch to dual adapter pattern, fix "execept" typo
- `apps/web/package.json` -- Add adapter-node + adapter-static deps, fix npm->pnpm in scripts
- `apps/web/playwright.config.ts` -- Fix npm->pnpm in webServer command

### New Files to Create

- `apps/web/src/hooks.server.ts` -- Stub server hooks that initialize locals to null
- `apps/web/src/routes/+error.svelte` -- Root-level error page with app branding

## Implementation Phases

### Phase 1: Foundation (Parallel -- 4 workstreams)

All 4 tasks are independent and can execute simultaneously:

**1a. API Client Overhaul** (`client.ts`)
- Create `ApiError` class for enriched error context (url, method, status)
- Model `ApiResponse<T>` as discriminated union: `{ ok: true; data: T } | { ok: false; status: number; error: unknown }`
- Wrap `fetch()` in try/catch, throw `ApiError` on network failure
- Wrap `response.json()` in try/catch, throw `ApiError` on parse failure
- Move `Content-Type` header out of constructor defaults; only include when body is present
- Constrain `method` parameter to `'GET' | 'POST' | 'PUT' | 'PATCH' | 'DELETE'`
- Add `PATCH` method
- Fix JSDoc: remove false `PUBLIC_API_BASE_URL` claim, update class-level "implementations will be filled in"
- Remove trivial method-level JSDoc that restates method name
- Simplify `// Internal` section divider

**1b. Theme Store Hardening** (`theme.svelte.ts`)
- Wrap `localStorage.getItem()` in try/catch; on failure fall back to OS preference with `console.warn`
- Wrap `localStorage.setItem()` in try/catch inside the `$effect`; on failure log warning but keep DOM class toggle working
- Fix JSDoc on `initTheme()`: remove "inside a component's `$effect`" recommendation, simplify to "Call once in a top-level layout component"
- Fix effect comment: change "No reactive state is reassigned here" to "Runs whenever theme.current changes"
- Expand comment on theme object wrapper to explain Svelte 5 runes export limitation

**1c. Auth & Types Foundation** (`app.d.ts`, `hooks.server.ts`, `+layout.server.ts`, `login/+page.svelte`)
- Define concrete `AuthUser` and `AuthSession` interfaces in `app.d.ts` (id, email, name, etc.)
- Replace `Record<string, unknown> | null` with `AuthUser | null` and `AuthSession | null`
- Add comment noting these are placeholder types to be replaced with Better Auth types
- Create `src/hooks.server.ts` that initializes `event.locals.user = null` and `event.locals.session = null`
- Update `+layout.server.ts`: add `console.warn` when auth gate is bypassed, normalize `locals.user ?? null`
- Update `+layout.server.ts` JSDoc: note that hooks.server.ts must be created
- Update `login/+page.svelte`: disable submit button with "Sign in (coming soon)" label, or show inline message on submit

**1d. Config & Scaffold Polish** (`svelte.config.js`, `package.json`, `playwright.config.ts`, `+error.svelte`)
- Replace `@sveltejs/adapter-auto` with conditional Node/Static adapter based on `BUILD_TARGET` env var
- Add `@sveltejs/adapter-node` and `@sveltejs/adapter-static` to devDependencies
- Remove `@sveltejs/adapter-auto` from devDependencies
- Fix `package.json` scripts: replace `npm run` with `pnpm run` in the `test` script
- Fix `playwright.config.ts`: replace `npm run build && npm run preview` with `pnpm run build && pnpm run preview`
- Fix typo in `svelte.config.js` line 7: "execept" -> "except"
- Create `src/routes/+error.svelte` with branded error page matching the app's design language (Tailwind, dark mode, "Go home" link)

### Phase 2: Validation (Sequential -- after all Phase 1 tasks)

- Run `pnpm check` (svelte-check) to verify TypeScript correctness
- Run `pnpm lint` to verify ESLint + Prettier compliance
- Run `pnpm build` to verify the build succeeds
- Run `pnpm test:unit -- --run` to verify unit tests pass
- Manually inspect each modified file for consistency
- Verify discriminated union forces callers to check `ok` before accessing `data`
- Verify hooks.server.ts correctly initializes locals
- Verify adapter switching works with `BUILD_TARGET=desktop`

## Team Orchestration

- You operate as the team lead and orchestrate the team to execute the plan.
- You NEVER operate directly on the codebase. You use `Task` and `Task*` tools to deploy team members.
- All 4 Phase 1 tasks run in parallel (independent domains, no shared files).
- Phase 2 validation runs after all Phase 1 tasks complete.

### Team Members

- Specialist
  - Name: builder-api
  - Role: Overhaul the API client with error handling, discriminated response union, and comment fixes
  - Agent Type: frontend-specialist
  - Resume: true

- Specialist
  - Name: builder-theme
  - Role: Harden the theme store with localStorage error handling and fix JSDoc comments
  - Agent Type: frontend-specialist
  - Resume: true

- Specialist
  - Name: builder-auth
  - Role: Create hooks.server.ts, strengthen auth types, fix auth gate and login page
  - Agent Type: frontend-specialist
  - Resume: true

- Specialist
  - Name: builder-config
  - Role: Fix adapter pattern, package manager alignment, error page, and typos
  - Agent Type: frontend-specialist
  - Resume: true

- Quality Engineer (Validator)
  - Name: validator
  - Role: Validate all completed work against acceptance criteria (read-only inspection mode)
  - Agent Type: quality-engineer
  - Resume: false

## Step by Step Tasks

### 1. Overhaul API Client

- **Task ID**: overhaul-api-client
- **Depends On**: none
- **Assigned To**: builder-api
- **Agent Type**: frontend-specialist
- **Parallel**: true
- Create `ApiError` class extending `Error` with `url`, `method`, `status`, `responseBody` properties
- Rewrite `ApiResponse<T>` as discriminated union: `ApiSuccess<T> | ApiFailure`
- Wrap `fetch()` call in try/catch, throw `ApiError` with context on network failure
- Wrap `response.json()` in try/catch, throw `ApiError` on parse failure
- Remove `Content-Type` from constructor `defaultHeaders`; add it conditionally in `request()` only when `body !== undefined`
- Change `method` parameter type from `string` to `HttpMethod` literal union
- Add `patch<T>()` public method
- Fix `DEFAULT_BASE_URL` JSDoc: remove false `PUBLIC_API_BASE_URL` claim
- Update class-level JSDoc: remove "implementations will be filled in" -- methods are fully implemented
- Remove trivial single-line JSDoc on `get`, `post`, `put`, `delete` methods
- Simplify the `// Internal` section divider to a single line or remove entirely
- Ensure all changes preserve the existing public method signatures (except adding the discriminated union)

### 2. Harden Theme Store

- **Task ID**: harden-theme-store
- **Depends On**: none
- **Assigned To**: builder-theme
- **Agent Type**: frontend-specialist
- **Parallel**: true
- Wrap `localStorage.getItem(STORAGE_KEY)` in try/catch in `initTheme()`; on catch, fall back to OS preference and `console.warn('[theme] localStorage unavailable, using OS preference.')`
- Separate DOM class toggle from localStorage write in the `$effect`; wrap only `localStorage.setItem` in try/catch with `console.warn('[theme] Could not persist theme preference.')`
- Update `initTheme()` JSDoc: replace "Call this inside a component's `$effect` or top-level layout" with "Call once in a top-level layout component. Safe to call during SSR (returns early when window is unavailable)."
- Update effect comment from "No reactive state is reassigned here" to "Runs whenever theme.current changes: syncs the DOM class and persists to localStorage."
- Expand theme object comment: "Reactive state. Wrapped in an object because $state primitives cannot be directly exported; the .current property remains reactive across imports."

### 3. Fix Auth Types and Plumbing

- **Task ID**: fix-auth-plumbing
- **Depends On**: none
- **Assigned To**: builder-auth
- **Agent Type**: frontend-specialist
- **Parallel**: true
- In `app.d.ts`, define `AuthUser` interface with `{ id: string; email: string; name: string; createdAt: Date; updatedAt: Date }` and `AuthSession` interface with `{ id: string; userId: string; expiresAt: Date }`
- Replace `Record<string, unknown> | null` with `AuthUser | null` and `AuthSession | null` in `Locals`
- Add comment above Locals: "Placeholder types -- replace with Better Auth's User and Session types once auth integration is complete."
- Create `src/hooks.server.ts` with a `Handle` function that sets `event.locals.user = event.locals.user ?? null` and `event.locals.session = event.locals.session ?? null`, with TODO for Better Auth
- Update `+layout.server.ts`: add `if (!locals.user) console.warn('[auth] Auth gate disabled: serving (app) routes without authentication.')` and return `user: locals.user ?? null`
- Update `+layout.server.ts` JSDoc: change "will be populated by the server hooks in src/hooks.server.ts" to "Once Better Auth is wired, update src/hooks.server.ts to inject the user session into event.locals, then uncomment the guard below."
- Update `login/+page.svelte`: change submit button to disabled with "Sign in (coming soon)" text, add `opacity-60 cursor-not-allowed` classes, remove `onsubmit` handler since disabled button won't submit

### 4. Fix Config and Add Error Page

- **Task ID**: fix-config-scaffold
- **Depends On**: none
- **Assigned To**: builder-config
- **Agent Type**: frontend-specialist
- **Parallel**: true
- In `svelte.config.js`: replace `import adapter from '@sveltejs/adapter-auto'` with conditional import pattern using `process.env.BUILD_TARGET` to select between `@sveltejs/adapter-node` (default) and `@sveltejs/adapter-static` (when `BUILD_TARGET === 'desktop'`)
- Fix typo on line 7: "execept" -> "except"
- In `package.json`: replace `@sveltejs/adapter-auto` with `@sveltejs/adapter-node` and `@sveltejs/adapter-static` in devDependencies. Fix `test` script: `npm run` -> `pnpm run`
- In `playwright.config.ts`: change `npm run build && npm run preview` to `pnpm run build && pnpm run preview`
- Create `src/routes/+error.svelte` with a branded error page:
  - Use `$page` store for error status and message
  - Match app's Tailwind design (slate backgrounds, indigo accents, dark mode support)
  - Include error status code, message, and a "Go home" link back to `/`
  - Keep it simple -- no over-engineering
- Run `pnpm install` after package.json changes to update lockfile
- Remove the boilerplate SvelteKit comment from `src/lib/index.ts` ("place files you want to import through the $lib alias in this folder")

### 5. Validate All Changes

- **Task ID**: validate-all
- **Depends On**: overhaul-api-client, harden-theme-store, fix-auth-plumbing, fix-config-scaffold
- **Assigned To**: validator
- **Agent Type**: quality-engineer
- **Parallel**: false
- Run `pnpm check` to verify TypeScript type-checks pass
- Run `pnpm lint` to verify ESLint + Prettier compliance
- Run `pnpm build` to verify the project builds successfully
- Run `pnpm test:unit -- --run` to verify existing tests still pass
- Inspect `src/lib/api/client.ts`: verify `ApiResponse` is a discriminated union, `Content-Type` only sent with body, error handling wraps fetch and json parse
- Inspect `src/lib/stores/theme.svelte.ts`: verify localStorage calls are wrapped in try/catch
- Inspect `src/hooks.server.ts`: verify it exists and initializes locals to null
- Inspect `src/app.d.ts`: verify concrete AuthUser/AuthSession interfaces replace Record<string, unknown>
- Inspect `svelte.config.js`: verify dual adapter pattern with BUILD_TARGET switching
- Inspect `src/routes/+error.svelte`: verify it exists with proper styling
- Operate in validation mode: inspect and report only, do not modify files

## Acceptance Criteria

1. `pnpm check` passes with zero errors
2. `pnpm lint` passes with zero errors
3. `pnpm build` passes with zero errors
4. `pnpm test:unit -- --run` passes (existing tests still work)
5. `ApiResponse<T>` is a discriminated union that forces callers to check `ok` before accessing `data`
6. `ApiClient.request()` wraps `fetch()` and `response.json()` in try/catch with `ApiError`
7. GET and DELETE requests do not send `Content-Type` header
8. `localStorage` access in theme store is wrapped in try/catch (both read and write)
9. `App.Locals` uses concrete `AuthUser` and `AuthSession` interfaces (not `Record<string, unknown>`)
10. `src/hooks.server.ts` exists and initializes `locals.user` and `locals.session` to `null`
11. `svelte.config.js` uses conditional adapter selection based on `BUILD_TARGET`
12. `src/routes/+error.svelte` exists with branded error page
13. All `npm run` references replaced with `pnpm run` in scripts
14. No inaccurate JSDoc comments (PUBLIC_API_BASE_URL claim removed, "implementations will be filled in" updated)
15. Auth gate logs a warning when bypassed

## Validation Commands

Execute these commands to validate the task is complete:

- `cd apps/web && pnpm install` -- Install updated dependencies
- `cd apps/web && pnpm check` -- Verify TypeScript type-checking passes
- `cd apps/web && pnpm lint` -- Verify ESLint and Prettier compliance
- `cd apps/web && pnpm build` -- Verify production build succeeds
- `cd apps/web && pnpm test:unit -- --run` -- Verify unit tests pass
- `cd apps/web && BUILD_TARGET=desktop pnpm build` -- Verify Tauri/static build path works
- `grep -r "adapter-auto" apps/web/` -- Should return no results
- `grep -r "npm run" apps/web/package.json apps/web/playwright.config.ts` -- Should return no results
- `test -f apps/web/src/hooks.server.ts` -- Hooks file must exist
- `test -f apps/web/src/routes/+error.svelte` -- Error page must exist

## Notes

1. **No test files are being added in this plan.** The test-analyzer identified major coverage gaps (ApiClient, theme store, auth gate), but adding tests is a separate task from fixing the review findings. A follow-up plan should address test coverage.
2. **The `ALL_ENTITY_TYPES` typing issue** (typed as `readonly string[]` instead of `readonly EntityType[]`) is in the contracts codegen package, not the web scaffold. It should be addressed separately.
3. **Theme store encapsulation** (making `theme.current` read-only externally) was flagged as a suggestion but would change the public API. Deferred to avoid scope creep.
4. **The `@altair/contracts` workspace dependency** must remain in `package.json` dependencies. The adapter changes should not affect this.
5. All 4 builder tasks are fully independent -- no shared files between them. This is by design to enable safe parallel execution.
