# Plan: Step 3 - Web Client Scaffold (SvelteKit)

## Task Description

Complete the SvelteKit 2 web client scaffold for Altair. The project skeleton exists (package.json, svelte.config.js, vite.config.ts, Tailwind CSS, ESLint, Vitest, Playwright) but lacks the application shell, navigation, route stubs, contract wiring, dark mode, and responsive layout. This step finishes everything defined in Step 3 of the implementation plan.

## Objective

Deliver a fully scaffolded SvelteKit 2 web client with: a responsive sidebar-based app shell, route stubs for all domains (guidance, knowledge, tracking, settings), generated TypeScript contracts wired in, dark mode toggle, and an auth gate placeholder. The app should serve via `pnpm dev` and pass lint/type checks.

## Problem Statement

The SvelteKit project was created via `sv create` with Tailwind, ESLint, Vitest, and Playwright add-ons, but it currently renders only the default "Welcome to SvelteKit" page. The implementation plan requires a complete application shell with sidebar navigation, domain route stubs, contract type integration, dark mode, responsive layout, and an auth gate placeholder before feature work can begin.

## Solution Approach

Build the scaffold in three phases: (1) wire contracts and create foundational lib structure, (2) build the app shell layout with sidebar navigation, route stubs, and dark mode, (3) add the auth gate placeholder and validate everything compiles/lints cleanly. Use Svelte 5 runes throughout. Follow the ADHD-friendly UX research report principles (clean visual hierarchy, low cognitive load, progressive disclosure).

## Relevant Files

- `apps/web/package.json` - existing dependencies, may need `@altair/contracts` workspace dep added
- `apps/web/svelte.config.js` - SvelteKit config with runes mode
- `apps/web/vite.config.ts` - Vite config with Tailwind, devtools, test setup
- `apps/web/tsconfig.json` - TypeScript config
- `apps/web/src/app.html` - HTML shell, needs dark mode class support
- `apps/web/src/app.d.ts` - SvelteKit type definitions, needs auth locals stubs
- `apps/web/src/routes/+layout.svelte` - current bare layout, needs full app shell
- `apps/web/src/routes/layout.css` - Tailwind imports
- `apps/web/src/routes/+page.svelte` - current placeholder, becomes today/landing view
- `packages/contracts/package.json` - contract package definition (`@altair/contracts`)
- `packages/contracts/generated/typescript/index.ts` - generated TypeScript contract exports
- `docs/altair-implementation-plan.md` - Step 3 requirements and done criteria
- `docs/architecture/altair-architecture-spec.md` - Section 10.2 Web Client Architecture
- `docs/ui-ux-research-report.md` - ADHD-friendly design principles

### New Files

- `apps/web/src/lib/contracts/` - re-export barrel for generated contracts
- `apps/web/src/lib/api/client.ts` - HTTP client wrapper stub
- `apps/web/src/lib/stores/theme.svelte.ts` - dark mode state using Svelte 5 runes
- `apps/web/src/lib/components/layout/Sidebar.svelte` - navigation sidebar component
- `apps/web/src/lib/components/layout/AppShell.svelte` - main app shell layout
- `apps/web/src/lib/components/layout/ThemeToggle.svelte` - dark mode toggle button
- `apps/web/src/routes/guidance/+page.svelte` - guidance domain route stub
- `apps/web/src/routes/knowledge/+page.svelte` - knowledge domain route stub
- `apps/web/src/routes/tracking/+page.svelte` - tracking domain route stub
- `apps/web/src/routes/settings/+page.svelte` - settings route stub
- `apps/web/src/routes/login/+page.svelte` - login placeholder page
- `apps/web/src/routes/(app)/+layout.svelte` - authenticated layout group with app shell

## Implementation Phases

### Phase 1: Foundation

- Add `@altair/contracts` as workspace dependency to `apps/web/package.json`
- Create `src/lib/contracts/index.ts` barrel re-exporting from `@altair/contracts`
- Create `src/lib/api/client.ts` HTTP client wrapper stub
- Create `src/lib/stores/theme.svelte.ts` dark mode store using Svelte 5 runes
- Update `src/app.html` to support dark mode class on `<html>` element
- Update `src/app.d.ts` with auth stubs in `App.Locals`

### Phase 2: Core Implementation

- Build `Sidebar.svelte` with navigation links: Guidance, Knowledge, Tracking, Search (disabled), Settings
- Build `AppShell.svelte` combining sidebar + main content area
- Build `ThemeToggle.svelte` component
- Create route group `(app)` for authenticated routes with the app shell layout
- Create route stubs: `guidance/+page.svelte`, `knowledge/+page.svelte`, `tracking/+page.svelte`, `settings/+page.svelte`
- Create `login/+page.svelte` placeholder
- Update root `+layout.svelte` for global CSS and dark mode initialization
- Update `+page.svelte` to redirect to guidance (or serve as today view)
- Make layout responsive: collapsible sidebar on mobile, full sidebar on desktop

### Phase 3: Integration & Polish

- Verify all generated TypeScript contracts import without errors
- Verify `pnpm dev` serves the app
- Verify Tailwind CSS works with dark mode toggle
- Verify layout is responsive at mobile and desktop breakpoints
- Run `pnpm check` for type checking
- Run `pnpm lint` for linting
- Clean up demo routes if desired

## Team Orchestration

- You operate as the team lead and orchestrate the team to execute the plan.
- You're responsible for deploying the right team members with the right context to execute the plan.
- IMPORTANT: You NEVER operate directly on the codebase. You use `Task` and `Task*` tools to deploy team members to the building, validating, testing, deploying, and other tasks.
  - This is critical. Your job is to act as a high level director of the team, not a builder.
  - Your role is to validate all work is going well and make sure the team is on track to complete the plan.
  - You'll orchestrate this by using the Task* Tools to manage coordination between the team members.
  - Communication is paramount. You'll use the Task* Tools to communicate with the team members and ensure they're on track to complete the plan.
- Take note of the session id of each team member. This is how you'll reference them.

### Team Members

- Specialist
  - Name: builder-foundation
  - Role: Wire contracts, create lib structure, set up dark mode store, update app.d.ts and app.html
  - Agent Type: frontend-specialist
  - Resume: true

- Specialist
  - Name: builder-shell
  - Role: Build app shell layout, sidebar navigation, route stubs, responsive design, auth gate placeholder
  - Agent Type: frontend-specialist
  - Resume: true

- Quality Engineer (Validator)
  - Name: validator
  - Role: Validate completed work against acceptance criteria (read-only inspection mode)
  - Agent Type: quality-engineer
  - Resume: false

### Team Member Context

All builders MUST read and follow these instructions before writing any code:

1. **Svelte 5 Runes**: Use `$state`, `$derived`, `$effect`, `$props` everywhere. No legacy `let` reactivity or `$:` statements. No `createEventDispatcher`. Use callback props or Svelte 5 event handling.
2. **Svelte MCP**: Before writing any Svelte component, call `mcp__svelte__list-sections` to find relevant docs, then `mcp__svelte__get-documentation` to load them. After writing, call `mcp__svelte__svelte-autofixer` to validate.
3. **TypeScript strict mode**: All files must pass strict type checking.
4. **Tailwind CSS v4**: Use Tailwind v4 syntax. The CSS file at `src/routes/layout.css` already has `@import 'tailwindcss'` and plugins. Use utility classes directly.
5. **Contracts**: The `@altair/contracts` package exports enums: `EntityType`, `RelationType`, `RelationSource`, `RelationStatus`, `AttachmentState`, `SyncStream`. Import via the workspace package.
6. **Package manager**: This project uses `pnpm` (see `.npmrc` with `engine-strict=true`). Run install via `pnpm install` from `apps/web/`.
7. **No component library**: Build custom components with Tailwind utility classes. Follow ADHD-friendly design: clean visual hierarchy, generous whitespace, clear focus indicators.
8. **File naming**: Svelte components use PascalCase (e.g., `AppShell.svelte`). TypeScript files use camelCase (e.g., `theme.svelte.ts` for rune-based stores).
9. **Existing files to preserve**: Keep `src/lib/vitest-examples/` and `src/routes/demo/` intact (test examples).

## Step by Step Tasks

- IMPORTANT: Execute every step in order, top to bottom. Each task maps directly to a `TaskCreate` call.
- Before you start, run `TaskCreate` to create the initial task list that all team members can see and execute.

### 1. Wire Contracts and Create Lib Foundation

- **Task ID**: wire-contracts
- **Depends On**: none
- **Assigned To**: builder-foundation
- **Agent Type**: frontend-specialist
- **Parallel**: true
- Add `"@altair/contracts": "workspace:*"` to `apps/web/package.json` dependencies and run `pnpm install` from `apps/web/`
- Create `src/lib/contracts/index.ts` that re-exports everything from `@altair/contracts`
- Create `src/lib/api/client.ts` with a stub HTTP client class (base URL from env, typed fetch wrapper, no implementation needed yet)
- Verify contract imports resolve by checking TypeScript can see the types

### 2. Set Up Dark Mode and App Types

- **Task ID**: dark-mode-setup
- **Depends On**: none
- **Assigned To**: builder-foundation
- **Agent Type**: frontend-specialist
- **Parallel**: true (can run alongside wire-contracts since same agent with resume)
- Create `src/lib/stores/theme.svelte.ts` using Svelte 5 runes: `$state` for current theme ('light' | 'dark'), `$effect` to sync to `document.documentElement.classList` and `localStorage`, function to toggle
- Update `src/app.html`: add `class=""` to `<html>` tag (dark mode class will be set by JS), add inline script in `<head>` to prevent FOUC by reading localStorage before paint
- Update `src/app.d.ts`: add `user` and `session` stubs to `App.Locals` interface (typed as `unknown | null` for now)
- Add dark mode Tailwind config if needed (Tailwind v4 uses `@custom-variant dark (&:where(.dark *))` or class-based)

### 3. Build App Shell and Sidebar Navigation

- **Task ID**: build-app-shell
- **Depends On**: wire-contracts, dark-mode-setup
- **Assigned To**: builder-shell
- **Agent Type**: frontend-specialist
- **Parallel**: false
- Use the Svelte MCP tools to load relevant SvelteKit layout and component documentation before building
- Create `src/lib/components/layout/Sidebar.svelte`:
  - Navigation links: Guidance (compass/target icon), Knowledge (book/notes icon), Tracking (package/list icon), Search (disabled/coming soon), Settings (gear icon)
  - Use simple SVG icons or Unicode symbols (no icon library dependency)
  - Active route highlighting using `$page.url.pathname`
  - Collapsible on mobile (hamburger toggle), always visible on desktop (lg: breakpoint)
  - Theme toggle integrated at bottom of sidebar
- Create `src/lib/components/layout/ThemeToggle.svelte`:
  - Button that calls the theme toggle function from the store
  - Sun/moon icon swap based on current theme
- Create `src/lib/components/layout/AppShell.svelte`:
  - Combines Sidebar + main content slot
  - Responsive: sidebar overlay on mobile, fixed sidebar on desktop
  - Main content area with proper padding and max-width constraints
- Create route group `src/routes/(app)/+layout.svelte` that renders AppShell and wraps `{@render children()}`
- Move or update root `+layout.svelte` to handle global CSS import and dark mode init only

### 4. Create Domain Route Stubs

- **Task ID**: create-route-stubs
- **Depends On**: build-app-shell
- **Assigned To**: builder-shell
- **Agent Type**: frontend-specialist
- **Parallel**: false
- Create `src/routes/(app)/guidance/+page.svelte` - placeholder with heading "Guidance" and brief description
- Create `src/routes/(app)/knowledge/+page.svelte` - placeholder with heading "Knowledge" and brief description
- Create `src/routes/(app)/tracking/+page.svelte` - placeholder with heading "Tracking" and brief description
- Create `src/routes/(app)/settings/+page.svelte` - placeholder with heading "Settings" and brief description
- Create `src/routes/(app)/+page.svelte` - landing/today view that replaces root page (or redirect to guidance)
- Create `src/routes/login/+page.svelte` - simple login placeholder with "Sign in" heading (outside app group, no sidebar)
- Each stub should import and display its relevant `EntityType` values from contracts to prove wiring works
- Use the Svelte MCP autofixer on each component

### 5. Wire Auth Gate Placeholder

- **Task ID**: auth-gate
- **Depends On**: create-route-stubs
- **Assigned To**: builder-shell
- **Agent Type**: frontend-specialist
- **Parallel**: false
- Add a `+layout.server.ts` to the `(app)` route group that checks `event.locals.user` and redirects to `/login` if null (placeholder logic, always passes for now since auth isn't wired)
- Add a comment in the layout server file indicating where Better Auth session check will go
- Ensure the login page works standalone without the app shell

### 6. Validate All Acceptance Criteria

- **Task ID**: validate-all
- **Depends On**: wire-contracts, dark-mode-setup, build-app-shell, create-route-stubs, auth-gate
- **Assigned To**: validator
- **Agent Type**: quality-engineer
- **Parallel**: false
- Run `pnpm dev` and verify the app serves without errors
- Run `pnpm check` and verify no TypeScript errors
- Run `pnpm lint` and verify no lint errors
- Verify app shell renders with navigation sidebar containing: Guidance, Knowledge, Tracking, Settings
- Verify all domain route stubs exist and render placeholder content
- Verify generated TypeScript contracts import without errors (check contract re-exports in components)
- Verify dark mode toggle switches between light and dark themes
- Verify layout is responsive: sidebar collapses on mobile, visible on desktop
- Verify login page renders outside app shell
- Verify Svelte 5 runes are used throughout (no legacy syntax)
- Operate in validation mode: inspect and report only, do not modify files

## Acceptance Criteria

- `pnpm dev` serves the SvelteKit app without errors
- App shell renders with navigation sidebar containing links to Guidance, Knowledge, Tracking, and Settings
- All domain route stubs (guidance, knowledge, tracking, settings) exist and render placeholder content
- Generated TypeScript contracts from `@altair/contracts` import without errors in the web app
- Tailwind CSS works with dark mode toggle (class-based switching)
- Layout is responsive at mobile (< 1024px: collapsible sidebar) and desktop (>= 1024px: fixed sidebar) breakpoints
- Login placeholder page exists outside the app shell
- Auth gate placeholder exists in the (app) layout group
- `pnpm check` passes with no TypeScript errors
- `pnpm lint` passes with no lint errors
- All components use Svelte 5 runes syntax

## Validation Commands

Execute these commands to validate the task is complete:

- `cd apps/web && pnpm install` - Install dependencies including workspace contract package
- `cd apps/web && pnpm check` - Verify TypeScript compilation passes
- `cd apps/web && pnpm lint` - Verify ESLint passes
- `cd apps/web && pnpm build` - Verify production build succeeds
- `cd apps/web && pnpm dev` - Verify dev server starts (manual check)

## Notes

- The `@altair/contracts` package uses `"exports": { ".": "./generated/typescript/index.ts" }` with raw `.ts` files. This works with pnpm workspaces since Vite/SvelteKit will handle the TypeScript compilation.
- Tailwind CSS v4 is already configured via `@tailwindcss/vite` plugin. Dark mode with class strategy may need a `@custom-variant` directive in the CSS.
- The `src/routes/demo/` and `src/lib/vitest-examples/` directories contain scaffolding examples and should be preserved.
- PowerSync SDK is mentioned in Step 3's dependency list but the plan says "installed, not configured yet." Do NOT install or configure PowerSync in this step. That happens in Step 7.
- The root `+page.svelte` currently lives at `src/routes/+page.svelte`. With the `(app)` route group, the main landing page moves to `src/routes/(app)/+page.svelte`. The root page can redirect to `/(app)` or be removed.
