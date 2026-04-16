# Implementation Steps: Web Client

**Spec:** Context/Features/008-WebClient/Spec.md
**Tech:** Context/Features/008-WebClient/Tech.md

## Progress
- **Status:** Complete
- **Current task:** —
- **Last milestone:** Milestone 6 (Feature Complete) — 2026-04-15

## Team Orchestration

### Team Members

- **builder-web**
  - Role: Foundation, design system, PowerSync data layer, layout shell, cross-cutting routes
  - Agent Type: frontend-specialist
  - Resume: false

- **builder-guidance**
  - Role: Guidance domain repositories, utilities, and all guidance screens
  - Agent Type: frontend-specialist
  - Resume: false

- **builder-knowledge**
  - Role: Knowledge domain repositories, utilities, and all knowledge screens
  - Agent Type: frontend-specialist
  - Resume: false

- **builder-tracking**
  - Role: Tracking domain repositories, utilities, and all tracking screens
  - Agent Type: frontend-specialist
  - Resume: false

- **builder-tests**
  - Role: Unit tests, component tests, and Playwright E2E test suite
  - Agent Type: frontend-specialist
  - Resume: false

- **validator**
  - Role: Read-only quality and integration validation
  - Agent Type: quality-engineer
  - Resume: false

---

## Tasks

### Phase 1: Foundation

- [ ] S001: Install required dependencies
  - Run `bun add @powersync/web` and `bun add --dev @testing-library/svelte@^5 @playwright/test axe-core @axe-core/playwright` from `apps/web/`.
  - Run `npx playwright install chromium` to install the browser binary.
  - Verify `apps/web/package.json` reflects all new entries and `bun install` completes cleanly.
  - **Assigned:** builder-web
  - **Depends:** none
  - **Parallel:** false

- [ ] S002: Implement Ethereal Canvas design system in `apps/web/src/app.css`
  - Add Google Fonts `<link rel="preconnect">` and stylesheet links for Manrope and Plus Jakarta Sans to `apps/web/src/app.html` in the `<head>` before the `%sveltekit.head%` placeholder.
  - Replace the contents of `apps/web/src/app.css` with the full Ethereal Canvas token set as CSS custom properties on `:root`. Include all colour tokens, font-family tokens (`--font-display: 'Manrope'`, `--font-body: 'Plus Jakarta Sans'`), and motion values (`--motion-standard: 300ms cubic-bezier(0.4, 0, 0.2, 1)`) from `docs/specs/09-PLAT-002-web.md`. Add a minimal CSS reset after the token block.
  - No `#000000` anywhere in the file. Maximum-contrast text uses `--on-surface` (`#2a3435`).
  - Add dark mode custom properties in a `@media (prefers-color-scheme: dark)` block (values can be placeholders for now per the "Should Have" requirement).
  - **Assigned:** builder-web
  - **Depends:** S001
  - **Parallel:** false

- [ ] S002-T: Verify design system token correctness (parse `app.css` for presence of `#000000` — must be absent; assert `--font-display` and `--font-body` tokens are defined; assert `--motion-standard` is `300ms cubic-bezier`; assert Google Fonts links present in `app.html`) — maps to FA-017
  - **Assigned:** builder-web
  - **Depends:** S002
  - **Parallel:** false

- [ ] S003: Implement PowerSync data layer in `apps/web/src/lib/sync/`
  - Create `schema.ts`: define a `Table` for every synced Postgres table — `users`, `households`, `household_memberships`, `quests`, `routines`, `epics`, `initiatives`, `focus_sessions`, `daily_checkins`, `notes`, `note_snapshots`, `entity_relations`, `tags`, `entity_tags`, `tracking_items`, `tracking_item_events`, `tracking_locations`, `tracking_categories`, `shopping_lists`, `shopping_list_items`. Column names must exactly match the Postgres migration column names (snake_case). UUID columns → `Column.text()`, timestamps → `Column.text()`, booleans → `Column.integer()`. Export `AppSchema`.
  - Create `connector.ts`: implement `PowerSyncBackendConnector` with `fetchCredentials()` (calls `GET /api/auth/powersync-token`) and `uploadData()` (calls `database.getCrudBatch(100)` and maps each CRUD op to the corresponding server REST endpoint by table and operation type; calls `batch.complete()` on success).
  - Create `index.ts`: export `getSyncClient()` which returns a module-level `PowerSyncDatabase` singleton initialized with `AppSchema` and the connector. Guard initialization with `if (!browser) throw new Error('PowerSync requires a browser environment')`. Export `subscribeToStreams(client)` which connects the client and subscribes to the five sync streams (`user_data`, `household`, `guidance`, `knowledge`, `tracking`).
  - Add `@powersync/web` to `vite.config.ts` `optimizeDeps` if required by the SDK's Vite integration guide.
  - **Assigned:** builder-web
  - **Depends:** S001
  - **Parallel:** false

- [ ] S003-T: Test PowerSync schema column parity and connector interface (schema column names match migration SQL for quests, notes, tracking_items — assert no mismatches; connector exports `fetchCredentials` and `uploadData` functions; `getSyncClient()` throws when called outside browser context; `uploadData` calls `batch.complete()` on success) — maps to FA-016
  - **Assigned:** builder-web
  - **Depends:** S003
  - **Parallel:** false

- [ ] S004: Implement root layout shell and auth guard
  - Create `apps/web/src/lib/components/layout/Sidebar.svelte`: left sidebar with nav links for Today, Guidance, Knowledge, Tracking, Search, Settings, and Admin (Admin link only rendered when `isAdmin` prop is true). Active route highlighted with pill-shaped background on Pale Seafoam Mist. Collapses to hamburger toggle on viewports < 768px using CSS media query and `$state` toggle. No visible borders between nav items or between sidebar and content area.
  - Create `apps/web/src/lib/components/layout/TopBar.svelte`: top bar on Foggy Canvas White.
  - Create `apps/web/src/lib/components/layout/Shell.svelte`: composes `Sidebar` + `TopBar` + `{@render children()}` slot.
  - Update `apps/web/src/routes/+layout.server.ts`: redirect to `/auth/login` if `event.locals.user` is null (FA-001). Call `fetch('/api/auth/me')` to get the full user profile; pass `isAdmin: profile.is_admin ?? false` as layout data.
  - Update `apps/web/src/routes/+layout.svelte`: use `$props()` to receive layout data. In a `$effect`, call `subscribeToStreams(getSyncClient())` after confirming the user is logged in. Render `<Shell isAdmin={data.isAdmin}>`.
  - Update `apps/web/src/app.d.ts`: add `isAdmin?: boolean` to `App.PageData`.
  - **Assigned:** builder-web
  - **Depends:** S003
  - **Parallel:** false

- [ ] S004-T: Test layout shell auth guard and sidebar behavior (unauthenticated request to `/` redirects to `/auth/login`; sidebar renders Admin nav link when `isAdmin=true`; Admin nav link absent when `isAdmin=false`; hamburger visible at 767px viewport, sidebar visible at 768px) — maps to FA-001, FA-012, FA-018
  - **Assigned:** builder-web
  - **Depends:** S004
  - **Parallel:** false

- [ ] S005: Implement primitive component library in `apps/web/src/lib/components/primitives/`
  - `Button.svelte`: accepts `variant` prop (`primary` | `secondary` | `ghost`), `disabled`, and `onclick`. All buttons are pill-shaped (`border-radius: 9999px`). Uses CSS custom property tokens for colour. All state-change transitions use `var(--motion-standard)`.
  - `Card.svelte`: `border-radius: 1rem`. No explicit borders inside. Tonal background using `var(--surface-container)`. Accepts `{@render children()}`.
  - `Input.svelte`: text input with label, error state, and helper text slots. Uses `$bindable()` for value.
  - `Badge.svelte`: small coloured pill, accepts `color` and `label` props.
  - `Tag.svelte`: chip display for entity tags, accepts `label` and optional `onremove`.
  - All components use `$props()` and Svelte 5 rune patterns per `.claude/rules/svelte.md`.
  - **Assigned:** builder-web
  - **Depends:** S002
  - **Parallel:** false

---

🏁 MILESTONE 1: Foundation complete — verify FA-001, FA-012, FA-016, FA-017, FA-018
  **Contracts:**
  - `apps/web/src/lib/sync/schema.ts` — PowerSync AppSchema; all repositories import and query against this schema
  - `apps/web/src/lib/sync/connector.ts` — PowerSyncBackendConnector; root layout initializes sync via this connector
  - `apps/web/src/lib/sync/index.ts` — getSyncClient() singleton and subscribeToStreams(); all repositories call getSyncClient()
  - `apps/web/src/app.css` — full Ethereal Canvas token set; all components reference these CSS custom properties
  - `apps/web/src/lib/components/layout/Shell.svelte` — root layout shell; all routes render inside this
  - `apps/web/src/lib/components/primitives/` — Button, Card, Input, Badge, Tag; all domain screens use these

---

### Phase 2: Repositories and Utilities

Tasks in this phase are independent of each other and run in parallel.

- [ ] S006: Implement guidance repositories and utilities
  - Create `apps/web/src/lib/utils/quest-transitions.ts`: export `validNextStatuses(current: QuestStatus): QuestStatus[]` using the state machine from `docs/specs/06-state-machines.md`. Valid moves: `backlog → active`, `active → paused | completed | cancelled`, `paused → active | cancelled`. Returns empty array for terminal states (`completed`, `cancelled`). Use quest status constants from `$lib/contracts/`.
  - Create `apps/web/src/lib/utils/date.ts`: export `isToday(dateStr: string): boolean` and `formatRelative(dateStr: string): string`.
  - Create `apps/web/src/lib/repositories/quest.svelte.ts`: module-level `$state` arrays backed by PowerSync `watch()` queries via `$effect`. Export `todayQuests` (not completed, due today or no due date), `questById(id)`, `questsByEpic(epicId)`, `allQuests`.
  - Create `apps/web/src/lib/repositories/routine.svelte.ts`: export `dueToday`, `allRoutines`.
  - Create `apps/web/src/lib/repositories/initiative.svelte.ts`: export `allInitiatives`, `initiativeById(id)`.
  - Create `apps/web/src/lib/repositories/epic.svelte.ts`: export `epicsByInitiative(initiativeId)`, `epicById(id)`.
  - **Assigned:** builder-guidance
  - **Depends:** S004
  - **Parallel:** true

- [ ] S006-T: Test quest state machine transitions (all valid next statuses for each status; `validNextStatuses('completed')` returns empty array; `validNextStatuses('cancelled')` returns empty array; `validNextStatuses('backlog')` returns `['active']`; constants imported from `$lib/contracts/`) — maps to FA-005
  - **Assigned:** builder-guidance
  - **Depends:** S006
  - **Parallel:** false

- [ ] S007: Implement knowledge repositories and utilities
  - Create `apps/web/src/lib/utils/note-link-trigger.ts`: export `detectLinkTrigger(value: string, cursorPos: number): { query: string; triggerStart: number } | null`. Returns the search query (text after `[[`) and the index of `[[` if the cursor is immediately after a `[[` prefix with no intervening whitespace. Returns null otherwise.
  - Create `apps/web/src/lib/repositories/note.svelte.ts`: export `allNotes` (sorted by `updated_at` desc), `noteById(id)`, `searchNotes(query: string)` (local text filter across title and content), `backlinksFor(noteId: string)` (queries `entity_relations` where `target_entity_id = noteId` and `relation_type = NOTE_LINK`).
  - Create `apps/web/src/lib/repositories/tag.svelte.ts`: export `allTags`, `tagsForEntity(entityId: string, entityType: string)`.
  - **Assigned:** builder-knowledge
  - **Depends:** S004
  - **Parallel:** true

- [ ] S007-T: Test note link trigger utility (`detectLinkTrigger('hello [[w', 9)` returns `{query:'w', triggerStart:6}`; no trigger mid-word returns null; cursor before `[[` returns null; empty query after `[[` returns `{query:'', ...}`; multiple brackets handled correctly)
  - **Assigned:** builder-knowledge
  - **Depends:** S007
  - **Parallel:** false

- [ ] S008: Implement tracking repositories
  - Create `apps/web/src/lib/repositories/item.svelte.ts`: export `allItems(filters?: {categoryId?: string; locationId?: string})`, `itemById(id)`, `lowStockItems` (items where current quantity below threshold).
  - Create `apps/web/src/lib/repositories/item-event.svelte.ts`: export `eventsForItem(itemId: string)` (ordered by `created_at` asc).
  - Create `apps/web/src/lib/repositories/shopping-list.svelte.ts`: export `allShoppingLists`, `shoppingListById(id)`, `itemsForShoppingList(listId: string)`.
  - Each repository bridges PowerSync `watch()` async iterable to `$state` via `$effect`. Use `$lib/contracts/` for `EntityType` and `RelationType` constants (invariant C-1, C-2).
  - **Assigned:** builder-tracking
  - **Depends:** S004
  - **Parallel:** true

---

🏁 MILESTONE 2: Repositories and utilities complete — all domain data layers ready for consumption by screens
  **Contracts:**
  - `apps/web/src/lib/utils/quest-transitions.ts` — validNextStatuses(); guidance screens use this for transition button rendering
  - `apps/web/src/lib/utils/note-link-trigger.ts` — detectLinkTrigger(); note editor uses this to detect [[ trigger
  - `apps/web/src/lib/repositories/quest.svelte.ts` — todayQuests, questById, questsByEpic
  - `apps/web/src/lib/repositories/note.svelte.ts` — noteById, backlinksFor, searchNotes
  - `apps/web/src/lib/repositories/item.svelte.ts` — allItems, itemById, lowStockItems
  - `apps/web/src/lib/repositories/shopping-list.svelte.ts` — allShoppingLists, itemsForShoppingList

---

### Phase 3: Domain Screens

Tasks S009–S013 are independent of each other and run in parallel.

- [ ] S009: Implement Today view at `apps/web/src/routes/`
  - `+page.server.ts`: load function fetches daily check-in status for today (calls server or queries PowerSync to determine if today's check-in exists). Returns `{ hasCheckedIn: boolean }`.
  - `+page.svelte`: display Manrope Display greeting personalised with `data.user.display_name` ("Good morning, {name}"). Render daily check-in card inline (as a card, per UF-02) if `!data.hasCheckedIn`. Render "Due Routines" section from `dueToday` repository. Render "Today's Quests" list from `todayQuests` repository. Include quick-action buttons "New Quest" (keyboard shortcut `n q`) and "New Note" (keyboard shortcut `n n`) using the `Button` primitive. Empty state per UF-02 when sections have no data.
  - **Assigned:** builder-guidance
  - **Depends:** S006
  - **Parallel:** true

- [ ] S010: Implement guidance screens at `apps/web/src/routes/guidance/`
  - `initiatives/+page.svelte`: list all initiatives with status filter chips. Uses `allInitiatives` repository.
  - `initiatives/[id]/+page.svelte` + `+page.server.ts`: initiative detail with epic/quest tree. Inline quest creation form that writes to PowerSync local DB (not a server form action). Uses `initiativeById`, `epicsByInitiative`, `questsByEpic`.
  - `epics/[id]/+page.svelte`: epic detail with child quests, breadcrumb nav back to initiative.
  - `quests/[id]/+page.svelte`: quest detail showing title, description, status, tags, focus session history. Status transition buttons rendered only for valid next statuses per `validNextStatuses()` (FA-005). All status writes go to PowerSync local DB + outbox via `getSyncClient().execute(...)`.
  - `routines/+page.svelte`: routines list with frequency badges.
  - No direct HTTP writes for domain data — all mutations via PowerSync.
  - **Assigned:** builder-guidance
  - **Depends:** S006
  - **Parallel:** true

- [ ] S010-T: Test guidance screen quest transition UI (render `QuestDetail` with `status='active'`; assert only `paused`, `completed`, `cancelled` buttons are present; assert `backlog` button is absent; render with `status='completed'`; assert no transition buttons rendered) — maps to FA-005
  - **Assigned:** builder-guidance
  - **Depends:** S010
  - **Parallel:** false

- [ ] S011: Implement focus session screen at `apps/web/src/routes/guidance/quests/[id]/focus/`
  - `+page.svelte`: countdown timer initialized to 25 minutes. Uses `setInterval` (1s tick) inside `$effect`; cleanup on destroy via `$effect` return. Stores `startTime` as `Date.now()` epoch and computes `elapsed = Date.now() - startTime` on each tick for accuracy under tab throttling. Renders a gradient progress ring SVG (signature Ethereal Canvas styling). While timer is running, sets the page background to Soft Slate Haze (`#cfddde`) — background returns to default on pause/complete. "Start", "Pause", and "Stop" controls using `Button` primitive. On session complete, writes a `focus_sessions` row to PowerSync local DB.
  - **Assigned:** builder-guidance
  - **Depends:** S006
  - **Parallel:** true

- [ ] S011-T: Test focus session screen (start timer; assert `background-color` computed style is `#cfddde`; pause timer; assert background reverts; timer uses `startTime` epoch not tick count for elapsed calculation; session write calls `powerSync.execute` on complete) — maps to FA-008
  - **Assigned:** builder-guidance
  - **Depends:** S011
  - **Parallel:** false

- [ ] S012: Implement knowledge screens at `apps/web/src/routes/knowledge/`
  - `+page.svelte`: notes list sorted by `updated_at` desc using `allNotes` repository. Local search input uses `searchNotes(query)` with debounce (300ms). Notes displayed as cards with title and `updated_at`.
  - `[id]/+page.svelte`: two-column layout — wide content area with `<textarea>` note editor, narrow metadata panel with tags (chip input with autocomplete from `allTags`) and snapshot history sidebar (immutable list of `note_snapshots` in view-only mode, invariant E-6). Backlinks section below the editor derives from `backlinksFor(noteId)` at render time (invariant E-5) — displays linked note titles as chips.
  - `[[` trigger: on `input` event, call `detectLinkTrigger(value, selectionStart)`. If trigger detected, show an absolutely-positioned dropdown overlay (Svelte component) containing filtered results from `searchNotes(query)`. On selection, insert `[[Note Title|noteId]]` into the textarea and write to `entity_relations` via `getSyncClient().execute(INSERT ...)` using `EntityType.NOTE` and `RelationType.NOTE_LINK` from `$lib/contracts/`.
  - **Assigned:** builder-knowledge
  - **Depends:** S007
  - **Parallel:** true

- [ ] S012-T: Test knowledge screen note linking and backlinks (type `[[` in textarea; assert dropdown appears; select a note; assert `entity_relations` INSERT called with correct `relation_type`; render note B editor; assert note A appears in backlinks section when entity_relation exists) — maps to FA-006, FA-007
  - **Assigned:** builder-knowledge
  - **Depends:** S012
  - **Parallel:** false

- [ ] S013: Implement tracking screens at `apps/web/src/routes/tracking/`
  - `+page.svelte`: inventory list with table/card toggle view. Filter controls for location and category (dropdowns). Low-stock items display a Sophisticated Terracotta (`#9f403d`) quantity badge using `Badge` primitive. Uses `allItems` and `lowStockItems` repositories.
  - `items/[id]/+page.svelte`: item detail showing name, location, category, current quantity, and event timeline from `eventsForItem`. Consumption logging form: quantity selector, validates that `current_qty + quantity_delta >= 0` before writing to PowerSync outbox (invariant E-7). If validation fails, display inline error message without page reload; no outbox entry created (FA-009).
  - `items/new/+page.svelte`: item creation form. Writes to PowerSync local DB on submit.
  - `locations/+page.svelte`: location list with create/edit/delete (soft-delete) via PowerSync writes.
  - `categories/+page.svelte`: category list with create/edit/delete via PowerSync writes.
  - `shopping-lists/+page.svelte`: shopping list view. List items rendered with pill-shaped checkboxes (`border-radius: 9999px`). Checking an item transitions status to `purchased`, dims item to Ghost Border Ash opacity. Unchecking transitions back to `pending`. Linked inventory item quantity displayed inline. Uses `allShoppingLists`, `itemsForShoppingList` repositories.
  - **Assigned:** builder-tracking
  - **Depends:** S008
  - **Parallel:** true

- [ ] S013-T: Test tracking screen quantity validation and shopping list rendering (render `ItemDetail` with `currentQty=3`; attempt consumption of 5; assert error message displayed; assert PowerSync execute NOT called; render `ShoppingList`; assert checkbox border-radius is `9999px`; check item; assert item rendered at Ghost Border Ash opacity) — maps to FA-009, FA-010
  - **Assigned:** builder-tracking
  - **Depends:** S013
  - **Parallel:** false

---

🏁 MILESTONE 3: Domain screens complete — verify FA-005, FA-006, FA-007, FA-008, FA-009, FA-010
  **Contracts:**
  - `apps/web/src/routes/guidance/quests/[id]/+page.svelte` — quest detail with status transitions; E2E tests use this route
  - `apps/web/src/routes/knowledge/[id]/+page.svelte` — note editor with [[ trigger; E2E tests use this route
  - `apps/web/src/routes/tracking/+page.svelte` — inventory list; E2E tests use this route

---

### Phase 4: Cross-Cutting Routes

- [ ] S014: Verify and resolve `is_admin` availability on `/api/auth/me`
  - Read `apps/server/server/src/auth/handlers.rs` me handler. Inspect the `UserProfile` response struct — confirm whether `is_admin: bool` is included in the response.
  - If `is_admin` is present: document the field path and proceed. No code changes needed.
  - If `is_admin` is absent: add `is_admin: bool` to the `UserProfile` struct in `apps/server/server/src/auth/models.rs` and include it in the me handler's DB query (`SELECT ..., is_admin FROM users WHERE id = $1`). Run `cargo clippy` to confirm no regressions.
  - Update Tech.md "Risks & Unknowns" — mark the `is_admin` unknown as resolved.
  - **Assigned:** builder-web
  - **Depends:** S004
  - **Parallel:** false

- [ ] S015: Implement admin panel at `apps/web/src/routes/admin/`
  - `+layout.server.ts`: if `!locals.user` redirect to `/auth/login`. Call `fetch('/api/auth/me')` and check `profile.is_admin`; if false redirect to `/` (FA-002). Return `{ isAdmin: true }`. This guard runs on all `/admin/*` routes.
  - `+layout.svelte`: admin layout shell. Renders admin sub-navigation (Users, Households, Health).
  - `users/+page.svelte`: fetch and display user list with name, email, status, and role columns. Direct invite creates user (not email invitation, per Open Question resolution). Deactivate action available per row.
  - `households/+page.svelte`: list households with member counts.
  - `health/+page.server.ts`: load function fetches the server's `/health` endpoint server-side (same host, no CORS). Returns `{ db: string; powersync: string; storage?: string }`.
  - `health/+page.svelte`: displays DB connectivity status, PowerSync status, and any storage metrics from load data (FA-011).
  - **Assigned:** builder-web
  - **Depends:** S014
  - **Parallel:** false

- [ ] S015-T: Test admin guard and health view (non-admin user navigating to `/admin` redirects to `/`; unauthenticated navigating to `/admin` redirects to `/auth/login`; `health/+page.server.ts` load function returns data from `/health` response including DB connectivity status) — maps to FA-002, FA-011
  - **Assigned:** builder-web
  - **Depends:** S015
  - **Parallel:** false

- [ ] S016: Implement settings at `apps/web/src/routes/settings/`
  - `+page.server.ts`: load function returns current user profile (`display_name`, `email`). Form actions for `updateProfile` (display name + email) and `changePassword` (old + new password). Both call server REST endpoints via `fetch` in form actions (auth mutations, not PowerSync). Logout action clears session cookie via existing server endpoint and redirects to `/auth/login`.
  - `+page.svelte`: profile edit form using `Input` primitive. Password change section. Notification preferences section visible but labelled as "Coming soon" (placeholder per Spec Won't Have).
  - **Assigned:** builder-web
  - **Depends:** S004
  - **Parallel:** false

- [ ] S017: Implement search shell at `apps/web/src/routes/search/`
  - `+page.svelte`: global search bar `Input`. "Search not yet available" empty-state message. Local filter input that uses `searchNotes(query)` from the note repository as a basic text-filter fallback across note titles (FA-019). The local filter result list appears below the empty-state message when a query is typed.
  - **Assigned:** builder-web
  - **Depends:** S007
  - **Parallel:** false

---

🏁 MILESTONE 4: All routes complete — verify FA-002, FA-011, FA-019
  **Contracts:**
  - `apps/web/src/routes/admin/+layout.server.ts` — admin guard pattern; E2E tests verify redirect behavior
  - `apps/web/src/routes/` — complete route tree; Playwright E2E suite covers all routes in Milestone 5

---

### Phase 5: Tests

Tasks S018–S020 are independent and run in parallel. S021–S023 depend on S020.

- [ ] S018: Write unit tests for utilities and static analysis checks
  - `apps/web/src/lib/utils/quest-transitions.spec.ts`: exhaustive test of `validNextStatuses` for all six statuses — valid transitions return correct arrays, invalid/terminal statuses return empty arrays. Maps to FA-005.
  - `apps/web/src/lib/utils/note-link-trigger.spec.ts`: edge case coverage for `detectLinkTrigger` — trigger at end of line, no trigger mid-word, empty query string, multiple `[[` in value, cursor before trigger.
  - `apps/web/src/lib/utils/date.spec.ts`: `isToday` and `formatRelative` correctness.
  - `apps/web/src/lib/sync/schema.spec.ts`: schema column parity test — parse `schema.ts` exports and verify column name matches migration SQL files in `infra/migrations/` for at least quests, notes, tracking_items tables. No column mismatch allowed. Maps to FA-016.
  - `apps/web/src/app.css.spec.ts` (or a Vitest inline test): read `app.css` contents, assert `#000000` is absent, assert `--on-surface` is defined with value `#2a3435` or equivalent, assert `--font-display` and `--font-body` tokens are present. Maps to FA-017.
  - **Assigned:** builder-tests
  - **Depends:** S006, S007, S003
  - **Parallel:** true

- [ ] S019: Write component tests using `@testing-library/svelte` v5
  - `QuestDetail.spec.ts`: mount quest detail component for each of the six statuses; assert only valid transition buttons are rendered per `validNextStatuses()`; assert invalid buttons are absent. Maps to FA-005.
  - `FocusSession.spec.ts`: start the timer; assert page background-color computed style is `#cfddde`; pause; assert background returns to default. Maps to FA-008.
  - `ShoppingList.spec.ts`: render list; assert checkbox border-radius is `9999px`; check item; assert opacity class applied (Ghost Border Ash). Maps to FA-010.
  - `ItemDetail.spec.ts`: mount with `currentQty=3`; submit consumption of 5; assert error message visible; assert PowerSync `execute` was not called. Maps to FA-009.
  - `Sidebar.spec.ts`: render with `isAdmin=false`; assert Admin link absent; render with `isAdmin=true`; assert Admin link present. Maps to FA-012.
  - Login flow test (existing auth route coverage): mount login page; submit valid credentials; assert redirect occurs.
  - **Assigned:** builder-tests
  - **Depends:** S010, S011, S013, S004
  - **Parallel:** true

- [ ] S020: Configure Playwright and axe-core integration
  - Create `apps/web/playwright.config.ts`: configure `baseURL`, test directory (`src/**/*.e2e.ts`), Chromium browser, and local dev server startup command (`bun dev`).
  - Create `apps/web/src/lib/test-utils/axe-helper.ts`: helper that runs `@axe-core/playwright` on the current page and asserts zero critical or serious WCAG AA violations (FA-014). To be called at the end of every E2E test.
  - Add `"test:e2e": "playwright test"` to `apps/web/package.json` scripts.
  - **Assigned:** builder-tests
  - **Depends:** S001
  - **Parallel:** true

- [ ] S021: Write Playwright E2E critical-path test
  - `apps/web/src/routes/auth/auth-flow.e2e.ts`: test the full critical path: register new user → login → navigate to `/guidance/initiatives` → create new quest → navigate to quest detail → mark quest complete → assert status is `completed` → logout → assert redirect to `/auth/login`. Call `axe-helper` on each route visited. Maps to FA-013, FA-014.
  - **Assigned:** builder-tests
  - **Depends:** S020
  - **Parallel:** false

- [ ] S022: Write Playwright E2E integration tests (PowerSync sync, offline, CSRF)
  - `apps/web/src/lib/sync/sync-integration.e2e.ts`:
    - FA-003: login, wait for PowerSync sync complete event, query IndexedDB for synced tables, assert rows present in each domain.
    - FA-004: intercept network to simulate offline, create a quest, restore network, query server API, assert quest present.
    - FA-006: open note editor, type `[[`, select note from dropdown, assert `entity_relations` row in IndexedDB.
    - FA-007: navigate to linked note, assert backlinks section shows source note.
  - `apps/web/src/routes/auth/csrf.e2e.ts`:
    - FA-015: submit a form action with a mismatched `Origin` header via `fetch` with `mode: 'cors'` override; assert 403 response.
  - **Assigned:** builder-tests
  - **Depends:** S020
  - **Parallel:** false

- [ ] S023: Write Playwright E2E accessibility, responsive, and CRUD tests
  - `apps/web/src/routes/ui.e2e.ts`:
    - FA-014: run axe-core on Today, /guidance, /knowledge, /tracking, /search, /settings, /admin/health.
    - FA-018: resize to 767px, assert sidebar hidden and hamburger visible; resize to 768px, assert sidebar visible.
    - FA-019: navigate to `/search`, assert "Search not yet available" message present, assert local filter input exists.
    - FA-020: create one entity per domain (quest, note, item), update it, assert change visible after navigation, delete/soft-delete, assert gone from list.
  - Call `axe-helper` after every route navigation. Maps to FA-014, FA-018, FA-019, FA-020.
  - **Assigned:** builder-tests
  - **Depends:** S020
  - **Parallel:** false

---

🏁 MILESTONE 5: Testing complete — verify FA-001 through FA-020 via test suite; all unit, component, and E2E tests pass

---

### Phase 6: Documentation and Validation

- [ ] S024-D: Update `CLAUDE.md` active work section
  - Change Feature 007 Tracking Domain status to reflect server implementation complete (pending web client use in this feature).
  - Add Feature 008 Web Client as in-progress with status "Domain screens, admin, and testing complete."
  - **Assigned:** builder-web
  - **Depends:** S023
  - **Parallel:** false

- [ ] S025: Full validation pass
  - Read all new files in `apps/web/src/lib/sync/`, `apps/web/src/lib/repositories/`, `apps/web/src/lib/components/`, and `apps/web/src/routes/`.
  - Verify: no component imports PowerSync directly (all go through repositories); no `#000000` in any `.svelte` or `.css` file; all domain writes use `getSyncClient().execute()` or PowerSync methods (no direct `fetch` for domain data); `$lib/contracts/` constants used for all `entity_type` and `relation_type` values; `validNextStatuses()` used for all quest transition UI rendering.
  - Run `bun run check` (SvelteKit type check), `bun run test` (Vitest), `bun run test:e2e` (Playwright) and confirm exit 0.
  - **Assigned:** validator
  - **Depends:** S024-D, S021, S022, S023
  - **Parallel:** false

---

🏁 MILESTONE 6: Feature complete — all FA-001–FA-020 verified; all tests passing; no TODOs remaining; drift check against Spec.md requirements confirmed

---

## Acceptance Criteria

- [ ] All 20 testable assertions (FA-001–FA-020) verified
- [ ] `bun run check` passes (SvelteKit type check, zero TS errors)
- [ ] `bun run test` passes (Vitest unit + component tests)
- [ ] `bun run test:e2e` passes (Playwright E2E, including axe-core assertions)
- [ ] No component imports `@powersync/web` directly (all domain data via repositories)
- [ ] No `#000000` anywhere in the web app source
- [ ] All domain writes go through PowerSync local DB (no direct `fetch` for Guidance/Knowledge/Tracking CRUD)
- [ ] All `entity_type` and `relation_type` values sourced from `$lib/contracts/`
- [ ] Admin guard verified server-side (not from JWT claims)
- [ ] No TODO/FIXME stubs remaining

## Validation Commands

```bash
# From apps/web/
bun run check          # SvelteKit type check
bun run test           # Vitest unit and component tests
bun run test:e2e       # Playwright E2E (requires local stack running)

# Install browsers if not already done:
npx playwright install chromium
```
