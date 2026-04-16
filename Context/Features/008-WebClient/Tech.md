# Tech Plan: Web Client

| Field | Value |
|---|---|
| **Feature** | 008-WebClient |
| **Version** | 1.0 |
| **Status** | Draft |
| **Date** | 2026-04-15 |
| **Spec** | Context/Features/008-WebClient/Spec.md |
| **Stacks involved** | SvelteKit 2 / Svelte 5 / TypeScript (web only) |
| **Source Docs** | `docs/specs/09-PLAT-002-web.md`, `docs/specs/10-PLAN-001-v2.md` §9, `docs/specs/06-state-machines.md`, `docs/specs/03-invariants.md`, `DESIGN.md`, all relevant ADRs |

---

## Architecture Overview

Feature 008 builds the complete web client on top of the existing SvelteKit scaffolding (`apps/web/`). Auth routes (login, register) with cookie-based JWT already work. Domain screens, the design system, and offline sync are all absent.

The architecture has three independent layers:

1. **Sync layer** (`src/lib/sync/`) — PowerSync browser SDK, initialized as a module singleton behind a `browser` guard. A `PowerSyncBackendConnector` handles credential refresh and CRUD uploads to the server REST API. A schema module mirrors the PostgreSQL column names for all synced tables.

2. **Repository layer** (`src/lib/repositories/`) — thin `.svelte.ts` modules that wrap PowerSync reactive queries with Svelte 5 `$state`/`$derived` runes. Components read only from repositories; no component imports PowerSync directly.

3. **UI layer** (`src/routes/`, `src/lib/components/`) — SvelteKit route tree with one `+layout.svelte` shell per major area (root, admin). All domain writes go through PowerSync local DB + outbox. Auth writes (login, logout, settings profile) remain form actions (`+page.server.ts`) as they always have.

The existing `hooks.server.ts` is extended minimally: the `checkOrigin` built-in in SvelteKit 2 satisfies CSRF (ADR-012); route guards are handled per-route via `+layout.server.ts` load functions, not the global hook.

---

## Key Decisions

### Decision 1: PowerSync initialization — module singleton vs. Svelte context provider

**Options considered:**
- **Svelte context provider** (`setContext` / `getContext` in root layout): the PowerSync instance is placed in Svelte context from `+layout.svelte`, and every component that needs it calls `getContext`. Clean dependency injection, but requires every repository to be inside the Svelte component tree and cannot be called from plain `.ts` modules.
- **Module-level singleton** (`src/lib/sync/index.ts` exports `getPowerSync()`): the module initializes the client once on first call, gated by `if (!browser) throw`. Repositories and utilities import the singleton directly. No context threading required.

**Chosen:** Module-level singleton with `browser` guard.

**Rationale:** SvelteKit 2 SSR runs `hooks.server.ts` and `+page.server.ts` load functions on Node, where `@powersync/web` cannot run (IndexedDB is unavailable). A module singleton initialized inside `if (browser)` is never touched during SSR. This is the pattern endorsed in the PowerSync JS SDK docs and avoids the context-threading overhead for repositories. The singleton is initialized (and subscribed to sync streams) from the root `+layout.svelte` via `$effect` after login is confirmed.

**Related ADRs:** ADR-012 (cookie-based session determines login state used to trigger initialization)

---

### Decision 2: Repository pattern — `.svelte.ts` with runes vs. plain TS observable wrappers

**Options considered:**
- **Plain TS modules returning observables**: repositories return RxJS or PowerSync `watch()` streams; components subscribe manually. Works but requires imperative subscription management (`$effect`, cleanup) in each component.
- **`.svelte.ts` rune modules**: repositories export `$state` / `$derived` values. Because `.svelte.ts` files participate in Svelte's rune transform, the returned values are live reactive objects. Components just reference `questRepository.todayQuests` with no subscription boilerplate.

**Chosen:** `.svelte.ts` rune modules.

**Rationale:** Matches the `.claude/rules/svelte.md` directive ("Stores use `.svelte.ts` extension for rune-based reactivity"). Components read a plain property; reactivity propagates automatically. The repository module owns the PowerSync reactive query (bridging the `watch()` async iterable to `$state` via `$effect`) and the `$derived` transform; the component owns nothing about the query. Keeps effects minimal in components.

**Implementation note:** Each repository is a module-level singleton (not instantiated per component). It is safe because Svelte 5 rune state in a `.svelte.ts` module is module-scoped reactive state, not per-component state.

---

### Decision 3: Design system delivery — `app.css` tokens + Tailwind CSS 4 utilities

**Options considered:**
- **Tailwind plugin with CSS custom properties**: define tokens in `tailwind.config.ts` so Tailwind generates `bg-primary`, `text-on-surface`, etc. Clean, but requires mapping the Ethereal Canvas palette into Tailwind's naming conventions and loses the direct custom-property contract that other code (e.g., JavaScript animation) may rely on.
- **Standalone `tokens.css`** imported from `app.css`: separates token definitions from reset and global styles. Adds an import hop.
- **All tokens in `app.css`**: CSS custom properties defined in `:root` at the top of `app.css`, followed by a minimal CSS reset. Tailwind CSS 4 (already installed via `@tailwindcss/vite`) handles spacing, flex, grid, and responsive utilities.

**Chosen:** All tokens in `app.css` + Tailwind CSS 4 for utilities.

**Rationale:** `app.css` is the natural single source of truth for global styles in SvelteKit. Tailwind CSS 4 is already wired into `vite.config.ts`. Using CSS custom properties in `app.css` means the full Ethereal Canvas token set (colours, font families, motion values) is always available as `var(--token-name)` in both scoped `<style>` blocks and inline style attributes. Tailwind provides utility classes for layout and spacing without duplication. No Tailwind-native colour mapping needed — components use `style="background: var(--surface-container)"` or a tiny helper utility class where appropriate.

**Font loading:** Manrope and Plus Jakarta Sans loaded via `<link rel="preconnect">` and Google Fonts `<link>` in `app.html`. Self-bundling deferred (P2).

---

### Decision 4: CSRF protection — SvelteKit built-in `checkOrigin`

**Options considered:**
- **Custom CSRF token hook**: inject a token into every rendered page, validate it on POST/PUT/DELETE form actions in a custom `handle` hook.
- **SvelteKit built-in `checkOrigin`**: enabled by default in SvelteKit 2 for all form actions. Rejects requests where `Origin` does not match the server's origin with 403.

**Chosen:** SvelteKit built-in `checkOrigin`.

**Rationale:** ADR-012 explicitly supersedes ADR-009 and documents that `checkOrigin` + `SameSite=Lax` cookies satisfy invariant SEC-6. No custom token machinery required. The `hooks.server.ts` file does not need a CSRF inject/validate step. This is not a deliberate omission — it is a resolved architectural decision. FA-015 tests the built-in behaviour (mismatched Origin → 403).

**Related ADRs:** ADR-012 (supersedes ADR-009)

---

### Decision 5: Admin route guard — `+layout.server.ts` server-side DB check

**Options considered:**
- **JWT claim check**: read `event.locals.user.is_admin` set by `hooks.server.ts`. Fast but incorrect — ADR-016 explicitly removes `is_admin` from JWT claims.
- **Server-side DB check in `+layout.server.ts`**: the `/admin/+layout.server.ts` load function calls the server's `/api/auth/me` endpoint (or directly queries the DB via a dedicated server endpoint) to verify admin status. Slower but authoritative.

**Chosen:** Server-side DB check via `/api/auth/me` response.

**Rationale:** ADR-016 mandates this approach. The `/api/auth/me` response already includes user profile data (fetched by `hooks.server.ts` logic). The `/admin/+layout.server.ts` load function calls `fetch('/api/auth/me', { headers: { Cookie: event.request.headers.get('Cookie') } })` to get the full user profile including `is_admin`, and redirects to `/` if false. This pattern is also used to expose `is_admin` to the layout component for conditional sidebar rendering (FA-012).

**Admin nav item visibility:** The root `+layout.server.ts` (or `+layout.ts`) also performs the `/api/auth/me` check and passes `isAdmin` as a page prop to the layout component so the Admin nav item can be conditionally rendered.

**Related ADRs:** ADR-016

---

### Decision 6: Note `[[` link trigger — textarea + positioned overlay

**Options considered:**
- **Rich-text editor (ProseMirror / TipTap)**: full editor with plugin support for link autocomplete. Powerful but heavy (~200KB+), significant Svelte 5 integration complexity, and overkill for Altair's Markdown note editing.
- **`contenteditable` div with manual cursor tracking**: allows styled inline chips but cursor position tracking cross-browser is error-prone.
- **Plain `<textarea>` + absolutely-positioned overlay**: detect `[[` in the textarea's value at the cursor position, render a dropdown overlay over the textarea, write the resolved link back into the textarea content. Lightweight, fully typed, no external dependency.

**Chosen:** Plain `<textarea>` + positioned overlay.

**Rationale:** Altair notes are stored as plain text/Markdown; rich-text formatting is out of scope for this feature. The `[[` trigger is a single interaction pattern: detect the `[[` prefix before the cursor, query PowerSync notes, show a dropdown, insert the selected note's title + ID. A small utility function (`detectLinkTrigger(value, cursorPos)`) handles the pattern. The overlay is a Svelte component positioned with `getBoundingClientRect()` on the textarea. No external editor library needed.

---

### Decision 7: Focus session timer — `setInterval` in component `$effect`

**Options considered:**
- **Web Worker**: the countdown runs in a worker; the main thread receives `postMessage` ticks. Survives tab backgrounding without throttling. More complex to wire.
- **`setInterval` in `$effect`**: straightforward, cleaned up on component destroy via the `$effect` return function. Throttled by browsers when the tab is backgrounded (typically to 1s tick rate, which is acceptable for a countdown timer).

**Chosen:** `setInterval` in `$effect` with 1-second tick.

**Rationale:** The focus session timer UI is a countdown measured in minutes (default 25m). Browser throttling of background tabs reduces tick frequency to ~1s — this does not affect correctness because the timer stores `startTime` (a `Date.now()` epoch) and computes `elapsed = Date.now() - startTime` on each tick. Even if ticks are delayed while backgrounded, the displayed time remains accurate. The Web Worker approach is a valid enhancement (P2) but adds complexity without observable accuracy improvement.

---

### Decision 8: Admin health route — server-side `load` function proxying to `/health`

**Options considered:**
- **Client-side fetch from `+page.svelte`**: the component fetches `/health` (proxied via a SvelteKit API route or directly) when the page mounts. No server involvement.
- **Server-side `load` function in `+page.server.ts`**: the load function calls the internal `/health` endpoint (same-process or via localhost) and returns the result. The page receives health data as a `data` prop.

**Chosen:** Server-side `load` function.

**Rationale:** The `/health` endpoint is an internal server resource. Making the browser call it directly requires CORS configuration on the server. A server-side `load` function fetches it from the SvelteKit server (same host), avoids CORS, and returns structured data to the component. The pattern matches the existing auth server actions (everything sensitive goes through server-side code, not client JS). The page can render immediately with the loaded health data and does not need a loading skeleton for the initial paint.

---

### Decision 9: `@testing-library/svelte` version — v5 for Svelte 5 compatibility

**Options considered:**
- **v4.x**: supports Svelte 4 and some Svelte 5 usage but lacks full rune support.
- **v5.x**: built specifically for Svelte 5 with rune-aware rendering.

**Chosen:** `@testing-library/svelte` v5.

**Rationale:** `apps/web/package.json` uses Svelte `^5.55.2`. v4 of `@testing-library/svelte` has known gaps with Svelte 5 runes (`$state`, `$derived` do not propagate correctly in the test environment). v5 ships with explicit Svelte 5 support. Playwright is used for E2E alongside it.

---

## Stack-Specific Details

### SvelteKit / Svelte 5 (`apps/web/`)

**New directories and files:**

```
apps/web/src/
  lib/
    sync/
      schema.ts           — PowerSync table schema (mirrors Postgres columns)
      connector.ts        — PowerSyncBackendConnector implementation
      index.ts            — getSyncClient() singleton + subscribeToStreams()
    repositories/
      quest.svelte.ts     — todayQuests, questById, etc.
      routine.svelte.ts   — dueToday, all
      initiative.svelte.ts
      epic.svelte.ts
      note.svelte.ts      — byId, list, search, backlinksFor
      tag.svelte.ts
      item.svelte.ts      — list, byId, lowStock
      item-event.svelte.ts
      shopping-list.svelte.ts
    components/
      layout/
        Sidebar.svelte     — nav, responsive collapse
        TopBar.svelte
        Shell.svelte       — composes Sidebar + TopBar + main slot
      primitives/
        Button.svelte
        Card.svelte
        Input.svelte
        Badge.svelte
        Tag.svelte
    utils/
      quest-transitions.ts  — validNextStatuses(current): QuestStatus[]
      note-link-trigger.ts  — detectLinkTrigger(value, cursorPos)
      date.ts               — isToday, formatRelative
  routes/
    +layout.svelte          — root Shell, sync init, admin prop
    +layout.server.ts       — auth guard (redirect if no user), fetch isAdmin
    +page.svelte            — Today view
    +page.server.ts         — load today data (check-in status)
    guidance/
      initiatives/
        +page.svelte
        [id]/
          +page.svelte
          +page.server.ts
      epics/[id]/
        +page.svelte
      quests/[id]/
        +page.svelte
        focus/
          +page.svelte     — countdown timer
      routines/
        +page.svelte
    knowledge/
      +page.svelte
      [id]/
        +page.svelte       — note editor + backlinks + snapshots
    tracking/
      +page.svelte
      items/
        new/+page.svelte
        [id]/+page.svelte
      locations/+page.svelte
      categories/+page.svelte
      shopping-lists/+page.svelte
    search/
      +page.svelte         — placeholder + local filter
    settings/
      +page.svelte
      +page.server.ts      — profile update form action
    admin/
      +layout.server.ts    — is_admin check, redirect non-admins
      +layout.svelte
      users/+page.svelte
      households/+page.svelte
      health/
        +page.svelte
        +page.server.ts    — load() proxies to /health
```

**Files to modify:**

| File | Change |
|---|---|
| `src/app.css` | Add full Ethereal Canvas token set as CSS custom properties; minimal reset |
| `src/app.html` | Add Google Fonts `<link>` for Manrope + Plus Jakarta Sans |
| `src/hooks.server.ts` | Add auth redirect logic for non-authenticated requests (or handle in `+layout.server.ts` — see Integration Points) |
| `src/app.d.ts` | Add `isAdmin?: boolean` to `App.PageData` if passing via layout |

**Dependencies to install:**

```bash
bun add @powersync/web
bun add --dev @testing-library/svelte@^5 @playwright/test axe-core @axe-core/playwright
npx playwright install chromium
```

No additional production deps beyond `@powersync/web`. Tailwind CSS 4 and Vitest are already installed.

**Patterns to follow:**
- Runes: `.claude/rules/svelte.md` — `$state`, `$derived`, `$effect`, `$props`
- File naming: `.claude/rules/svelte.md` — `PascalCase.svelte`, `kebab-case.svelte.ts`
- Contract types: `$lib/contracts/` for all `entity_type` and `relation_type` values (invariant C-1, C-2)
- PowerSync schema: `.claude/rules/powersync.md` — column names must match Postgres exactly (invariant D-4)

---

## Integration Points

### PowerSync connector → server REST API

The `PowerSyncBackendConnector` in `src/lib/sync/connector.ts` implements two methods:

```typescript
// fetchCredentials(): called by PowerSync SDK to get a token for the sync service
async fetchCredentials(): Promise<PowerSyncCredentials> {
  const res = await fetch('/api/auth/powersync-token'); // SvelteKit API route proxying server JWT
  // ...
}

// uploadData(): called by PowerSync to flush the local outbox to the server
async uploadData(database: AbstractPowerSyncDatabase): Promise<void> {
  const batch = await database.getCrudBatch(100);
  for (const op of batch.crud) {
    // map op.table + op.op to the appropriate server REST endpoint
    // POST /api/guidance/quests, PATCH /api/knowledge/notes/:id, etc.
  }
  await batch.complete();
}
```

The `uploadData` function maps PowerSync CRUD operations (`PUT`, `PATCH`, `DELETE`) to the server REST API using the same auth cookie (set by the browser automatically). The outbox flush runs whenever connectivity returns.

### Admin guard → server `/api/auth/me`

`/admin/+layout.server.ts`:

```typescript
export const load: LayoutServerLoad = async ({ fetch, locals, url }) => {
  if (!locals.user) redirect(302, '/auth/login');
  const res = await fetch('/api/auth/me');
  const profile = await res.json();
  if (!profile.is_admin) redirect(302, '/');
  return { isAdmin: true };
};
```

The root `+layout.server.ts` similarly fetches `/api/auth/me` and passes `isAdmin` as layout data for conditional sidebar nav rendering.

### PowerSync schema → Postgres migrations

The schema in `src/lib/sync/schema.ts` must exactly mirror the column names in the Postgres migrations under `infra/migrations/`. The PowerSync table list covers: `users`, `households`, `household_memberships`, `quests`, `routines`, `epics`, `initiatives`, `focus_sessions`, `daily_checkins`, `notes`, `note_snapshots`, `entity_relations`, `tags`, `entity_tags`, `tracking_items`, `tracking_item_events`, `tracking_locations`, `tracking_categories`, `shopping_lists`, `shopping_list_items`.

Sync rule streams referenced in `infra/compose/sync_rules.yaml`:
- `user_data` — users, households, memberships
- `household` — household-scoped shared data
- `guidance` — quests, routines, epics, initiatives, focus sessions, daily check-ins
- `knowledge` — notes, snapshots, entity_relations, tags, entity_tags
- `tracking` — items, events, locations, categories, shopping lists

### Note linking → `entity_relations` table

When a user selects a note from the `[[` dropdown, the note editor calls:

```typescript
powerSync.execute(
  `INSERT INTO entity_relations (id, source_entity_type, source_entity_id, target_entity_type, target_entity_id, relation_type, user_id, created_at, updated_at)
   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)`,
  [uuid(), EntityType.NOTE, currentNoteId, EntityType.NOTE, selectedNoteId, RelationType.NOTE_LINK, userId, now, now]
);
```

The `EntityType` and `RelationType` constants come from `$lib/contracts/` (invariant C-1, C-2). The backlinks query derives results at read time from `entity_relations` (invariant E-5) — no separate backlinks table.

---

## PowerSync Schema Snapshot

All column names must match Postgres exactly. Representative examples:

```typescript
// src/lib/sync/schema.ts
import { Schema, Table, Column, ColumnType } from '@powersync/web';

const quests = new Table({
  id: Column.text(),
  title: Column.text(),
  status: Column.text(),
  priority: Column.text(),
  description: Column.text(),
  due_date: Column.text(),
  epic_id: Column.text(),
  initiative_id: Column.text(),
  user_id: Column.text(),
  household_id: Column.text(),
  created_at: Column.text(),
  updated_at: Column.text(),
  deleted_at: Column.text(),
});

// ... one Table per synced Postgres table
export const AppSchema = new Schema({ quests, /* ... */ });
```

UUID columns map to `Column.text()`. Timestamps map to `Column.text()` (ISO-8601 strings). Booleans map to `Column.integer()` (0/1).

---

## Risks & Unknowns

- **Risk:** `@powersync/web` requires a Web Worker for its OPFS SQLite backend. SvelteKit's Vite config may need `optimizeDeps.exclude` or a `worker` Vite config tweak to bundle the worker correctly.
  - **Mitigation:** Follow the `@powersync/web` Vite configuration guide. Add `@powersync/web` to `vite.config.ts`'s `optimizeDeps.include` or exclude list per SDK docs. Validate during initial S901 task.

- **Risk:** PowerSync `watch()` async iterable API shape may differ across SDK versions. The repository `.svelte.ts` pattern bridges `watch()` to `$state` via `$effect`; if the iterable emits differently than expected, the bridge may need adjustment.
  - **Mitigation:** Inspect the installed `@powersync/web` API surface in S901. Confirm `watch()` yields result rows on each change. Wrap in a standard `for await...of` loop inside `$effect`. The contract to components (a `$derived` value) does not change.

- **Risk:** `@testing-library/svelte` v5 was released recently; there may be rough edges with Vitest 4.x happy-dom.
  - **Mitigation:** Pin to a known stable version at install time. Component tests cover discrete, small components first (Button, Input) before complex ones (NoteEditor). If integration issues arise, fall back to Playwright for component-level assertions.

- **Risk:** The admin panel's `/api/auth/me` call adds a server round-trip to every `/admin/*` page load.
  - **Mitigation:** Acceptable for an admin panel (low traffic, not latency-sensitive). Cache the result in the root layout data (SvelteKit layout data is available to all child layouts without refetching). Add `Cache-Control: private, max-age=60` to the `/api/auth/me` response if needed.

- **Unknown:** Does the server currently expose `is_admin` on the `/api/auth/me` response?
  - **Resolution (S014 — resolved):** Confirmed present. `UserProfile` in `apps/server/server/src/auth/models.rs` includes `is_admin: bool` (line 71). The `me` handler in `apps/server/server/src/auth/handlers.rs` queries `SELECT id, email, display_name, is_admin FROM users WHERE id = $1` and maps the field directly onto the response struct (lines 273–285). No server changes needed. The field path in the JSON response is `profile.is_admin` (boolean).

- **Unknown:** Does `infra/compose/sync_rules.yaml` include the `tracking` stream for all six tracking tables? Feature 007 is still in progress.
  - **Resolution:** Verify sync rules include tracking tables before S901 (data layer). If tracking tables are absent from sync rules, add them as part of the data layer task or coordinate with the tracking domain completion. The PowerSync schema in this feature must include them.

---

## Testing Strategy

- **Unit tests** (Vitest, co-located `*.spec.ts`):
  - `quest-transitions.ts` — all valid and invalid `validNextStatuses` results
  - `note-link-trigger.ts` — `detectLinkTrigger` edge cases (no trigger, mid-word, multiple brackets)
  - `date.ts` — `isToday`, `formatRelative`
  - PowerSync schema — column name parity test: parse `schema.ts` and verify every column name matches the migration SQL (FA-016)
  - `app.css` static analysis — parse for `#000000` absence, assert text colours within palette (FA-017)

- **Component tests** (`@testing-library/svelte` v5, co-located `*.spec.ts`):
  - Login flow (existing auth route)
  - `QuestDetail` — renders only valid status transition buttons (FA-005)
  - `FocusSession` — background dims to `#cfddde` when timer running (FA-008)
  - `ShoppingList` — pill-shaped checkboxes, completed item opacity (FA-010)
  - `ItemDetail` consumption logging — blocks negative quantity, no outbox entry (FA-009)
  - Layout sidebar — Admin nav absent for non-admin user prop (FA-012)

- **E2E tests** (Playwright, `*.e2e.ts`):
  - Unauthenticated redirect (FA-001)
  - Non-admin `/admin` redirect (FA-002)
  - Sidebar responsive collapse at 767px/768px (FA-018)
  - Search empty state (FA-019)
  - Critical path: register → login → create quest → complete quest → logout (FA-013)
  - CRUD for one entity per domain (FA-020)
  - axe-core on every route visited (FA-014)

- **Integration tests** (Playwright against running local stack):
  - PowerSync initial sync + IndexedDB population (FA-003)
  - Offline write → reconnect → server-side assertion (FA-004)
  - `[[` link creates `entity_relations` row (FA-006)
  - Backlinks derived at query time (FA-007)
  - Admin health view (FA-011)
  - CSRF rejection on mismatched Origin (FA-015)

---

## ADRs

No new ADRs required. All decisions are governed by or consistent with:

- **ADR-003** — LWW conflict resolution; PowerSync outbox flushes follow this
- **ADR-012** — Built-in auth + JWT + SameSite=Lax; checkOrigin satisfies SEC-6; supersedes ADR-009
- **ADR-016** — `is_admin` not in JWT claims; admin guard uses server-side DB check
- **ADR-007** — Monorepo structure; web app lives at `apps/web/`

---

## Revision History

| Date | Change |
|---|---|
| 2026-04-15 | Initial draft |
