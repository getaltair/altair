# Feature 009: Android Client

| Field | Value |
|---|---|
| **Feature** | 009-AndroidClient |
| **Version** | 1.0 |
| **Status** | Draft |
| **Date** | 2026-04-16 |
| **Source Docs** | `docs/specs/10-PLAN-001-v2.md` (Step 8), `docs/specs/09-PLAT-001-android.md`, `docs/specs/03-invariants.md`, `docs/specs/04-architecture.md`, `docs/specs/06-state-machines.md`, `DESIGN.md` |

---

## Overview

Feature 009 delivers the complete Android client for Altair — a Kotlin / Jetpack Compose application with offline-first PowerSync sync, Material 3 / Ethereal Canvas design system, and full CRUD coverage for the Guidance, Knowledge, and Tracking domains. It replicates the data flow and offline-first patterns established by the web client (Feature 008), adapted for Android-native strengths: capture, mobile interaction, and background sync.

---

## Problem Statement

The Android project exists at `apps/android/` with a scaffold: contracts package, a minimal UI directory, and `MainActivity.kt`. There is no Room database, no PowerSync integration, no navigation graph, no domain screens, and no authentication flow. Users cannot access any Altair data from Android. This feature builds the complete Android client from scaffold to full-featured app.

---

## User Stories

**US-01 — Authentication**
As a user, I can log in and register on Android with my email and password so that my session is established and my tokens are stored securely on-device for future launches.

**US-02 — Today view**
As a user, I can open the app to a Today screen that shows my greeting, any incomplete daily check-in, due routines, and today's quests so that I can orient and start my day immediately.

**US-03 — Quick quest actions**
As a user, I can swipe right on an `in_progress` quest card in the Today view to complete it, with a smooth animation, so that quick completions require minimal taps.

**US-04 — Guidance management**
As a user, I can browse initiatives, epics, quests, and routines; apply valid status transitions to quests; and start focus session timers that work offline so that my goals are tracked accurately from my phone.

**US-05 — Focus sessions**
As a user, I can start a full-screen focus timer from a quest detail screen that runs via `CountDownTimer` and fires a completion event even when offline so that I can do focused work anywhere.

**US-06 — Knowledge capture**
As a user, I can quickly capture a text note and later edit it, link it to other notes, and see which other notes link back to it so that my knowledge is connected and mobile-accessible.

**US-07 — Tracking**
As a user, I can browse inventory, log consumption, create items, and check off shopping list items from my phone so that household resource tracking works wherever I am.

**US-08 — Offline writes**
As a user, I can create or update records while offline and have those changes automatically synchronise to the server via WorkManager when connectivity returns so that I am never blocked by a network outage.

**US-09 — Background sync**
As a user, my app syncs in the background on a 15-minute schedule and on connectivity changes so that data is fresh without requiring me to manually refresh.

**US-10 — Settings**
As a user, I can view and edit my display name and email, and log out, clearing all locally stored tokens so that my account information is current and I can securely sign out.

---

## Requirements

### Must Have

**Authentication and token storage**
- Login screen: email + password inputs on Pale Seafoam Mist background, pill-shaped primary CTA.
- Register screen: email, password, and display name inputs.
- On successful authentication, store the access token and refresh token in DataStore (encrypted) via `EncryptedSharedPreferences` or Jetpack Security DataStore.
- Auth interceptor (OkHttp or Ktor): attach JWT to all API requests; on 401, attempt token refresh before failing.
- On logout, clear all tokens from DataStore and navigate to the login screen.

**Room database and PowerSync integration**
- Define Room entities for all synced tables, with column names exactly matching the PostgreSQL schema (invariant D-4 — snake_case): `users`, `initiatives`, `tags`, `attachments`, `entity_relations`, `guidance_epics`, `guidance_quests`, `guidance_routines`, `guidance_focus_sessions`, `guidance_daily_checkins`, `knowledge_notes`, `knowledge_note_snapshots`, `tracking_locations`, `tracking_categories`, `tracking_items`, `tracking_item_events`, `tracking_shopping_lists`, `tracking_shopping_list_items`.
- Define Room DAOs for each entity group with `Flow<List<T>>` return types for queries and `suspend` functions for writes.
- Integrate the PowerSync Kotlin SDK with Room as the managed SQLite store.
- Implement `PowerSyncBackendConnector`: fetch JWT from Altair server (`/api/sync/token`), provide `fetchCredentials()` and `uploadData()` methods.
- Auto-subscribe to baseline sync streams on login: `user_data`, `household`, `guidance`, `knowledge`, `tracking`.

**Koin dependency injection**
- `DatabaseModule`: Room `AltairDatabase` instance, all DAOs.
- `RepositoryModule`: repository implementations wrapping DAOs + PowerSync.
- `ViewModelModule`: all ViewModels.
- `SyncModule`: PowerSync connector, sync coordinator.
- `PreferencesModule`: DataStore preferences.
- `NetworkModule`: HTTP client (OkHttp), Retrofit or Ktor API services for auth endpoints.
- All modules must pass `checkModules()` verification in tests.

**Navigation**
- Bottom navigation bar: **Today**, **Knowledge**, **Tracking**, **Settings** (4 tabs).
- Nested navigation (Navigation Component) for detail screens within each tab.
- Deep link handling: `altair://quest/{id}`, `altair://item/{id}`, `altair://checkin`.
- Default approach (pending OQ-001 resolution): Guidance detail screens (quest detail, focus session, initiative/epic browsing, routines) are accessible via Today-as-hub — nested within the Today tab's back stack. No dedicated Guidance bottom nav tab unless OQ-001 resolves to Option 2 (which requires an ADR).

**Today view**
- Greeting header (Manrope Display) with user's display name.
- Daily check-in card (energy + mood selectors) if not yet completed today.
- Due routines section — reactive `Flow` from Room/PowerSync.
- Today's quests list — filtered by today's due date or no due date and not yet completed.
- Swipe-right on `in_progress` quest card to complete: valid transition check (per `06-state-machines.md` — `in_progress → completed` only), 300ms cubic-bezier animation. Quests in `not_started` state show a "Start" action; swiping them advances to `in_progress`.
- Quick action FAB: New Quest, New Note.
- Empty state: "Nothing on the horizon" copy with create quest CTA.
- Entry point to full Guidance screens (initiative list, all quests, routines) via "All Guidance" action or navigation element on the Today screen.

**Guidance screens**
- Quest list: filterable by initiative, status, priority.
- Quest detail: title, status (valid transitions only, per state machine), priority, epic/initiative breadcrumb, tags, focus session history.
- Focus session: full-screen `CountDownTimer`-backed timer (Manrope Display), signature gradient progress ring, dim background to Soft Slate Haze, end button. Must function offline.
- Initiative list → initiative detail → epic list → epic detail → child quests (breadcrumb navigation).
- Routine list with frequency badges; mark-routine-done action.
- Daily check-in form (reachable from Today card and from Guidance section).
- All writes go via PowerSync outbox (offline-first).

**Knowledge screens**
- Quick note capture: minimal input surface accessible from Today FAB, expandable to full editor.
- Note list: sortable by `updated_at`, locally searchable via Room query.
- Note detail: title, content text editor (no rich text in v1), tags, backlinks display (E-5), snapshot history list (view-only, E-6).
- Note linking: search existing notes, create `entity_relation` (C-1, C-2).

**Tracking screens**
- Inventory list: search/filter bar, location and category filter chips, item cards with quantity badge, low-stock highlighting (Sophisticated Terracotta quantity badge).
- Item detail: name, quantity, location, category, event timeline (append-only from `tracking_item_events`), tags.
- Item creation form: name, quantity stepper, location dropdown (household-scoped), category dropdown (household-scoped), optional barcode field (manual text entry).
- Consumption logging: quantity selector, validation that consumption does not exceed current quantity (invariant E-7), creates `item_event`.
- Location and category management screens (household-scoped CRUD).
- Shopping list: checklist with pill-shaped checkboxes, linked item quantities shown, add from inventory or freeform, completed items dimmed to Ghost Border Ash opacity.

**Background sync**
- `SyncWorker` (WorkManager `PeriodicWorkRequest`): 15-minute interval, `NetworkType.CONNECTED` constraint.
- Event-driven expedited sync (`OneTimeWorkRequest`) on app foreground resume, connectivity change (`NetworkCallback`), and after any local write.
- Offline indicator: subtle Weathered Slate icon when mutations are queued but unsynced.

**Design system**
- Material 3 color scheme mapped to Ethereal Canvas tokens (verified in `Color.kt` and `Theme.kt`).
- Manrope font for `Typography.displayLarge` and `Typography.headlineLarge`; Plus Jakarta Sans for `Typography.bodyLarge` and `Typography.labelMedium`.
- No pure black (`#000000`) anywhere; Midnight Charcoal (`#2a3435`) for maximum-contrast text.
- Cards: `ElevatedCard` with `RoundedCornerShape(16.dp)`, no explicit elevation shadow for static cards.
- Buttons: pill-shaped via `RoundedCornerShape(50)` with primary color.
- Inputs: filled style with `surfaceContainerLow` color, no outline border.
- No dividers inside cards — spacing-only separation between sections.

**Testing**
- JUnit 5 + Turbine unit tests for ViewModels and Use Cases (Flow testing).
- Room DAO tests with in-memory database.
- Compose UI tests (semantics-based) for key screens: login, today view, quest detail, item creation.
- Koin module integration tests via `checkModules()`.
- App builds and runs on API 26+ emulator.

---

### Should Have

**Barcode scanner (P1 — build if time permits)**
- Full-screen camera overlay using CameraX + ML Kit barcode scanning.
- Scan frame with Deep Muted Teal-Navy border.
- Result slide-up card: match existing item in Room (update quantity flow) or pre-fill item creation form.
- Barcode value stored on item record.
- Lookup works offline against local Room data.

**Focus timer client-local notification (P1)**
- When a focus session is running and the app is backgrounded, post a client-local foreground service notification showing the timer countdown.
- On timer completion, fire a local notification: "Focus session complete."
- No FCM, SSE, or server-side scheduling involved (local only).
- Requires `POST_NOTIFICATIONS` runtime permission (Android 13+).

---

### Won't Have (this feature)

- **FCM push notifications**: server-side push dispatch, notification scheduler, SSE stream — deferred to Step 11.
- **Attachment upload/download**: binary file handling, camera capture to note/item, attachment UI — deferred to Step 10.
- **Search**: full-text and semantic search wiring — deferred to Step 10.
- **Widgets**: Today widget, quick-capture widget — deferred to P2.
- **Voice notes**: audio capture and playback — deferred to P2.
- **Share intents**: receiving text/images from other apps — deferred to P2.
- **Conflict resolution UI**: dedicated conflict view — deferred to Step 12.

---

## Testable Assertions

These assertions define the observable behaviors that verify this feature is complete. Each maps to one or more implementation tasks in Steps.md.

| ID | Assertion |
|---|---|
| FA-001 | After successful login, DataStore contains a non-empty access token and refresh token that persist across app restart. |
| FA-002 | After login and initial sync on a populated server instance, all Room tables for the logged-in user contain at least one row (quests, notes, tracking_items). |
| FA-003 | WorkManager's `getWorkInfosByTag("sync_periodic")` returns at least one `ENQUEUED` or `RUNNING` job after app startup. |
| FA-004 | On the quest detail screen, only valid status transitions (per `06-state-machines.md`) are shown as available actions; invalid transitions are absent from the UI. |
| FA-005 | After disabling network and performing a local write (create quest), PowerSync's pending upload count is greater than zero; after re-enabling network, the count returns to zero within 30 seconds. |
| FA-006 | Swiping right on a quest card in the Today view that is in `not_started` or `in_progress` status transitions it to `completed`; the Room row reflects the new status. |
| FA-007 | A `CountDownTimer` for a 5-second focus session fires `onFinish()` after 5 seconds with no network connectivity (verified in a unit test with a test dispatcher). |
| FA-008 | Attempting to log a consumption quantity exceeding the current item quantity is rejected before any write — the Room database row quantity is unchanged. |
| FA-009 | After checking off a shopping list item while offline and restarting the app, the item remains checked (the write persisted to Room before sync). |
| FA-010 | On a note detail screen for note A, the backlinks section lists note B when note B has an `entity_relation` referencing note A (invariant E-5). |
| FA-011 | From the Today view, a user can navigate to the full quest list (showing quests not due today), the initiative list, and the routine list without switching bottom nav tabs. |
| FA-012 | After logout, DataStore contains no access or refresh tokens; navigating back to a protected screen redirects to the login screen. |
| FA-013 | The Room entity for `guidance_quests` defines columns `title`, `description`, `status`, `priority`, `due_date`, `epic_id`, `initiative_id`, `user_id`, `created_at`, `updated_at`, `deleted_at` — all in snake_case, matching the Postgres migration. |
| FA-014 | All Koin modules pass `checkModules()` with no missing dependency or circular dependency errors. |
| FA-015 | The app builds without errors targeting API 31 (`minSdk 31` — matches existing `build.gradle.kts`) and runs the Today view on an API 31 emulator with no crash on launch. |
| FA-016 | Design system spot-check: no `Color.Black` / `#000000` appears in the rendered UI; quest cards use `RoundedCornerShape(16.dp)`; primary buttons use `RoundedCornerShape(50)`; headings render in Manrope. |
| FA-017 | Deep link `altair://quest/{id}` opens the quest detail screen for the specified quest when the app is already running. |
| FA-018 | After a local write while offline, a `SyncWorker` `OneTimeWorkRequest` is enqueued (observable via `WorkManager.getInstance().getWorkInfosByTag("sync_expedited")`). |
| FA-019 | When the server returns a 401 on an authenticated request and a valid refresh token is stored, the auth interceptor refreshes the token and retries the original request successfully — without user-visible error or navigation to the login screen. |

---

## Open Questions

**OQ-001 — Guidance navigation entry point** ✓ Resolved
Guidance screens (quest list, initiative/epic browsing, routines) are accessed via Today-as-hub: "All Guidance" section links on the Today screen, nested within the Today tab's back stack. Bottom nav remains 4 items: Today, Knowledge, Tracking, Settings.

**OQ-002 — DataStore encryption implementation** ✓ Resolved
Use `EncryptedSharedPreferences` (Jetpack Security Crypto) for token storage. The deprecated-API warning is acceptable; the library remains functional across API 26–36 and is the lowest-friction path.

---

## Dependencies

| Dependency | Status | Notes |
|---|---|---|
| Feature 001–007: Server + Sync Engine | Complete | Server REST API, PowerSync sync rules, and all domain endpoints are operational |
| Feature 008: Web Client | Complete | Establishes reference patterns for PowerSync connector and repository layer |
| `apps/android/` scaffold | Present | Contracts package, `ui/`, `MainActivity.kt` exist; no Room, no navigation, no screens |
| `docs/specs/09-PLAT-001-android.md` | Current | Android platform architecture spec |
| `docs/specs/06-state-machines.md` | Current | Quest and routine state machines; required for valid-transition enforcement |
| `docs/specs/03-invariants.md` | Current | D-4, E-2, E-5, E-6, E-7, E-9, SEC-2, SEC-5 are directly enforced by this feature |
| PowerSync Kotlin SDK | Available | Kotlin SDK integrates with Room as managed SQLite store |
| Material 3 theme | Present | `Color.kt` and `Theme.kt` exist in scaffold with Ethereal Canvas mapping started |
