# Feature 008: Web Client

| Field | Value |
|---|---|
| **Feature** | 008-WebClient |
| **Version** | 1.0 |
| **Status** | Draft |
| **Date** | 2026-04-15 |
| **Source Docs** | `docs/specs/10-PLAN-001-v2.md` (Step 9), `docs/specs/09-PLAT-002-web.md`, `docs/specs/07-user-flows.md`, `docs/specs/06-state-machines.md`, `docs/specs/03-invariants.md`, `docs/specs/04-architecture.md`, `DESIGN.md` |

---

## Overview

Feature 008 delivers the complete web client for Altair — a SvelteKit 2 / Svelte 5 single-page application with offline-first PowerSync sync, the Ethereal Canvas design system, and full CRUD coverage for the Guidance, Knowledge, and Tracking domains. It establishes the canonical client-side patterns (data layer, layout shell, reactive queries, CSRF protection) that the Android client will replicate in Feature 009.

---

## Problem Statement

The web app has only auth routes (login, register) and an empty shell. There is no design system, no domain UI, and no PowerSync integration. Users cannot create or manage quests, notes, or inventory items. Without this feature, the server's REST API and sync infrastructure are inaccessible to any browser-based user — and the patterns needed for Android development cannot be validated.

---

## User Stories

**US-01 — Layout and navigation**
As a user, I can navigate between the Today, Guidance, Knowledge, Tracking, Search, Settings, and Admin areas from a persistent sidebar so that I always know where I am and can move quickly between domains.

**US-02 — Daily start**
As a user, I can open the app each morning and see a greeting, any pending daily check-in, today's quests, and due routines on a single screen so that I can orient and start working immediately.

**US-03 — Guidance management**
As a user, I can create, view, update, and complete initiatives, epics, quests, and routines through the web UI, with quest status transitions restricted to only valid moves, so that my work is accurately tracked.

**US-04 — Focus sessions**
As a user, I can start a browser-based focus session timer from a quest detail screen so that I can work in focused intervals without switching to another app.

**US-05 — Knowledge editing**
As a user, I can write and edit notes, link notes to each other using `[[` notation, and see which other notes link back to mine so that my knowledge is connected and discoverable.

**US-06 — Tracking**
As a user, I can manage household inventory — browsing items with filters, logging consumption, creating shopping lists, and checking off items — so that stock levels are accurate and shopping is coordinated.

**US-07 — Offline writes**
As a user, I can create or update records while offline and have those changes automatically synchronise to the server when connectivity returns so that I am never blocked by a network outage.

**US-08 — Admin panel**
As an administrator, I can view the user list, household overview, and instance health from a protected admin panel so that I can operate and monitor my self-hosted instance.

**US-09 — Settings**
As a user, I can edit my profile (display name, email, password) and see placeholder notification preference controls so that my account details are current and preferences are accessible.

---

## Requirements

### Must Have

**Authentication and routing**
- All authenticated routes redirect to `/auth/login` when `event.locals.user` is null.
- The `/admin` route and its sub-routes redirect non-admin users (verified server-side per ADR-016 — `is_admin` is not in the JWT claims) to `/`.
- Logout clears session cookies and redirects to `/auth/login`.

**PowerSync integration**
- A `PowerSyncBackendConnector` provides `fetchCredentials()` (from the server JWT endpoint) and `uploadData()` (to the server REST API) so that PowerSync can authenticate and flush the outbox.
- The PowerSync client is initialised at app startup and auto-subscribes to the `user_data`, `household`, `guidance`, `knowledge`, and `tracking` sync streams on login.
- The local PowerSync schema column names must exactly mirror the corresponding PostgreSQL column names (invariant D-4).
- A repository layer wraps PowerSync reactive queries using `$derived` runes; components read from repositories, not directly from PowerSync.

**Layout and design system**
- The full Ethereal Canvas token set (all colours, fonts, motion values) from `docs/specs/09-PLAT-002-web.md` is implemented as CSS custom properties in `app.css`.
- Manrope is used exclusively for Display and Headline text; Plus Jakarta Sans for all Body, Label, and UI text.
- No pure black (`#000000`) appears anywhere; maximum-contrast text uses Midnight Charcoal (`#2a3435`).
- No explicit borders appear inside cards or between layout zones; hierarchy is achieved through tonal shift and spacing.
- Buttons are pill-shaped (`border-radius: 9999px`); cards use `border-radius: 1rem` (rounded-2xl).
- All state-change transitions use 300ms `cubic-bezier(0.4, 0, 0.2, 1)`.
- The sidebar collapses to a hamburger menu on viewports narrower than 768px.

**Today view (`/`)**
- Shows a Manrope Display greeting personalised with the user's `display_name`.
- Displays a daily check-in card when today's check-in has not yet been completed.
- Shows routines due today and quests due today (or with no due date) that are not complete, sourced from reactive PowerSync queries.

**Guidance screens**
- Initiative list, initiative detail (epic/quest tree, inline quest creation), epic detail, quest detail, and routines list are all implemented.
- Quest detail exposes a focus session screen (`/guidance/quests/[id]/focus`) with a browser-based countdown timer, signature gradient progress ring, and background dimmed to Soft Slate Haze (`#cfddde`).
- Quest status transitions in the UI are restricted to those permitted by `docs/specs/06-state-machines.md` — invalid transitions are not presented as available actions.
- All writes go to PowerSync local DB + outbox (offline-first); no direct HTTP writes for domain data.

**Knowledge screens**
- Notes list (sortable by `updated_at`), note editor (wide content area + narrow metadata panel), and tag autocomplete are all implemented.
- Typing `[[` in the note editor triggers an inline search dropdown against local PowerSync data; selecting a result creates an `entity_relation` of type `note_link`.
- The backlinks section on the note detail derives its content from `entity_relations` at query time — it shows notes that link to the current note (invariant E-5). No separate backlink table is maintained.
- The snapshot history sidebar lists immutable note snapshots in view-only mode (invariant E-6).

**Tracking screens**
- Inventory view, item detail (with event timeline), item creation form, location CRUD, category CRUD, and shopping list view are all implemented.
- Consumption logging validates that a `quantity_change` would not reduce the item's quantity below zero before submitting (invariant E-7). The UI surfaces the error without page reload.
- Shopping list items have pill-shaped checkboxes; completed items are rendered at Ghost Border Ash opacity; linked item quantities are shown.
- Low-stock items display a Sophisticated Terracotta (`#9f403d`) quantity badge.

**Search shell**
- A `/search` route exists with a global search bar.
- The search result area shows a "Search not yet available" empty state.
- A local PowerSync query provides a basic text-filter fallback across entity titles.

**Admin panel**
- `/admin/users` lists users with name, email, status, and role.
- `/admin/households` lists households with member counts.
- `/admin/health` calls the `/health` server endpoint and displays DB connectivity, PowerSync status, and any available storage metrics.

**Settings**
- `/settings` allows editing display name, email, and password.
- Notification preference controls are visible but labelled as placeholder (wiring deferred to Feature 011).

**CSRF protection**
- SvelteKit's built-in `checkOrigin` (enabled by default in SvelteKit 2) rejects state-changing requests from mismatched origins with 403, satisfying invariant SEC-6 (ADR-012 supersedes ADR-009 — no custom CSRF token hook required).
- API proxy routes validate the `Origin` header.

**Testing**
- Vitest unit tests cover utility functions and reactive stores.
- `@testing-library/svelte` component tests cover: login flow, quest creation, note editing, and item creation.
- Playwright E2E test covers the full critical path: register → login → create quest → complete quest → logout.
- axe-core is integrated into the Playwright test suite and runs on every E2E test route.

### Should Have

- Keyboard shortcut support for primary create actions (new quest, new note).
- Debounced local search on the notes list and inventory view (without waiting for Step 10's server search).
- Connection status indicator (subtle offline badge) when PowerSync is not syncing.
- Dark mode CSS custom properties defined (even if not toggled by the UI in this feature).

### Won't Have (this iteration)

- Server-side rendering for domain screens (auth routes are SSR; domain screens use SPA navigation).
- Server-side full-text or semantic search — deferred to Feature 010.
- File attachment upload/download — deferred to Feature 010.
- Notification delivery (SSE stream, in-app bell) — deferred to Feature 011.
- Android client — deferred to Feature 009.
- Desktop (Tauri) client — deferred to v2 (ADR-001).
- Web Push notifications — P2 per `docs/specs/09-PLAT-002-web.md`.
- Graph visualisation of note relationships — P2.
- Bulk operations (multi-select, batch edit) — P1, deferred.
- Drag-and-drop epic reordering — referenced in platform spec but deferred to polish phase (Feature 012).

---

## Testable Assertions

| ID | Assertion | Verification |
|---|---|---|
| FA-001 | An unauthenticated request to any domain route (e.g. `/`) is redirected to `/auth/login`. | E2E: load `/` without a session cookie, assert redirect to `/auth/login`. |
| FA-002 | A non-admin user navigating to `/admin` is redirected to `/` (admin status verified server-side per ADR-016, not from JWT claims). | E2E: log in as a non-admin user, navigate to `/admin`, assert redirect to `/`. |
| FA-003 | After login, PowerSync completes an initial sync and data from all five baseline streams is visible in IndexedDB. | Integration test: login, wait for sync completion event, query IndexedDB for synced tables from each stream, assert rows present. |
| FA-004 | A write performed while PowerSync is disconnected is queued in the outbox; after reconnection, the change appears on the server. | Integration test: disconnect PowerSync, create a quest, reconnect, query server API, assert quest is present. |
| FA-005 | Quest status transitions shown in the UI are restricted to valid moves per the state machine. Attempting to apply an invalid transition is not possible via the UI. | Component test: render quest detail in each status, assert only valid transition buttons are rendered. |
| FA-006 | Selecting a note from the `[[` search dropdown creates an `entity_relation` row of type `note_link` in the local PowerSync DB. | Component/integration test: type `[[`, select a note, assert `entity_relations` table contains the new row with correct `relation_type`. |
| FA-007 | The backlinks section of a note shows all notes that contain an `entity_relation` pointing to it, derived at query time. | Integration test: create note A linking to note B; open note B; assert note A appears in the backlinks section. |
| FA-008 | The focus session screen dims surrounding UI to Soft Slate Haze (`#cfddde`) when the timer is running. | Component test: start timer, assert background-color computed style matches `#cfddde`. |
| FA-009 | A consumption event that would make item quantity negative is blocked in the UI with an error message before any outbox entry is created. | Component test: set item quantity to 3, attempt to log consumption of 5, assert error message displayed and outbox unchanged. |
| FA-010 | All shopping list items render pill-shaped checkboxes; checking an item dims it to Ghost Border Ash opacity. | Component test: render shopping list, assert checkbox border-radius is `9999px`; check an item, assert opacity changes. |
| FA-011 | The admin health view calls the `/health` server endpoint and displays DB connectivity status. | E2E/integration test: navigate to `/admin/health`, assert page contains connectivity status text from the `/health` response. |
| FA-012 | Non-admin users do not see the Admin nav item in the sidebar (admin status derived from server-side check, not JWT). | Component test: render layout with a non-admin user context prop, assert Admin link is absent from navigation. |
| FA-013 | The Playwright E2E critical-path test passes: register → login → create quest → mark quest complete → logout. | CI: run `playwright test` against a running local stack, assert exit 0. |
| FA-014 | axe-core reports zero critical or serious WCAG AA violations on all routes exercised by Playwright E2E tests. | CI: axe-core assertions in Playwright suite, assert no violations at critical or serious level. |
| FA-015 | A state-changing form action submitted from a different origin is rejected with 403 by SvelteKit's built-in `checkOrigin` (invariant SEC-6, ADR-012). | Integration test: submit a form action with a mismatched `Origin` header, assert 403 response. |
| FA-016 | The PowerSync schema defines column names that exactly match the corresponding PostgreSQL column names (snake_case) for every synced table. | Code review + unit test: compare PowerSync schema column names against migration column names for each table, assert no mismatches. |
| FA-017 | The design system CSS defines no colour value of `#000000`; all text uses `--on-surface` (`#2a3435`) or lighter. | Unit test or static analysis: parse `app.css`, assert `#000000` is absent, assert text colour values are within the defined palette. |
| FA-018 | The sidebar navigation collapses to a hamburger toggle on a 767px-wide viewport and expands on a 768px-wide viewport. | E2E test: resize browser to 767px, assert sidebar is hidden and hamburger is visible; resize to 768px, assert sidebar is visible. |
| FA-019 | The `/search` route shows a "Search not yet available" empty state with a local text-filter fallback input. | E2E: navigate to `/search`, assert empty-state message present and local filter input is present. |
| FA-020 | All domain screens (Guidance, Knowledge, Tracking) support create, read, update, and delete via the UI, with changes persisting to the local PowerSync DB. | E2E: exercise CRUD for at least one entity per domain, assert changes visible after page navigation. |

---

## Open Questions

- [ ] Should note snapshot creation be triggered automatically on every save, or only on an explicit "snapshot" user action? (Deferred from the knowledge domain spec — needs resolution before the note editor is finalised.)
- [ ] Does the daily check-in card use a modal overlay or an inline card on the Today view when check-in is available? `07-user-flows.md` UF-02 describes it as a card, but the plan leaves the exact treatment open.
- [ ] Should the admin invite flow create a user directly, or send an email invitation? Email delivery is not configured in the server (Step 11), so direct creation may be the only in-scope option.

---

## Dependencies

| Dependency | Status | Notes |
|---|---|---|
| Features 001–007 | Complete | Server API endpoints for all three domains operational. PowerSync sync rules, sync engine, and Docker Compose stack are running. |
| `@powersync/web` SDK | External library | Must be installed and compatible with SvelteKit 2 / Svelte 5. |
| Manrope + Plus Jakarta Sans | External fonts | Load from Google Fonts or bundle locally. |
| PowerSync service (`localhost:8082` dev) | Infrastructure | Must be reachable from the browser during development. |
| `$lib/contracts/` package | Present | Entity type and relation type constants; must be used for all `entity_type` and `relation_type` values (invariant C-1, C-2). |
| `apps/web/src/hooks.server.ts` | Present (partial) | JWT decode → `event.locals.user` implemented. CSRF hook enforcement added in this feature. |
