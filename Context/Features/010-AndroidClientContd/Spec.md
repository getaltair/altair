# Feature 010: Android Client — Continued

| Field | Value |
|---|---|
| **Feature** | 010-AndroidClientContd |
| **Version** | 1.0 |
| **Status** | Draft |
| **Date** | 2026-04-16 |
| **Continues** | `Context/Features/009-AndroidClient/` |
| **Source Docs** | `docs/specs/09-PLAT-001-android.md`, `docs/specs/10-PLAN-001-v2.md` (Step 8), `docs/specs/06-state-machines.md`, `docs/specs/03-invariants.md`, `Context/Features/009-AndroidClient/Spec.md` |

---

## Overview

Feature 010 completes the Android client by delivering the remaining unimplemented portions of Feature 009: guidance detail navigation (Initiative and Epic detail screens), a sync status indicator, barcode scanning for inventory, a focus timer foreground service, sync debounce on rapid network transitions, and the post-PR-review test coverage gaps. All feature work is contained to `apps/android/`.

---

## Problem Statement

Feature 009 shipped the Android client foundation and most domain screens, but left several planned components incomplete. Users navigating to an initiative in the Guidance tab reach a dead end — there is no detail screen and no way to drill into epics or child quests. The sync status is invisible; users cannot tell whether their offline writes are pending or delivered. Inventory scanning requires manual barcode entry. The focus timer silently stops when the app is backgrounded with no user feedback. And approximately nine test classes required by post-PR-review tasks are absent, leaving auth, sync, and connector logic undertested.

---

## User Stories

**US-01 — Initiative and Epic browsing**
As a user, I can tap an initiative in the Guidance list to open its detail screen showing description and associated epics, and tap an epic to see its child quests, with a breadcrumb trail back up the hierarchy, so that I can navigate my goal structure from my phone.

**US-02 — Sync status visibility**
As a user, I can see a subtle indicator in the app bar when I have writes that have not yet synced to the server, so that I know my offline data is still pending and I am not confused about whether my changes were saved.

**US-03 — Barcode scanning**
As a user, I can open a camera overlay from the item creation screen, point it at a product barcode, and have the app either open an existing item's update-quantity flow or pre-fill the item creation form — even when offline — so that adding or updating inventory items from physical products is fast and error-free.

**US-04 — Focus timer background notification**
As a user, when I start a focus session and then background the app, I can see a persistent notification showing the timer countdown so that I know my session is still running without keeping the app in the foreground.

**US-05 — Focus timer completion notification**
As a user, I receive a local notification when my focus session timer completes, regardless of whether the app is in the foreground, so that I do not miss the end of a session.

**US-06 — Sync debounce on reconnect**
As a user, the app does not enqueue redundant sync jobs when my network transitions rapidly (e.g., WiFi handoffs), so that my battery is not drained by stacking sync workers.

---

## Requirements

### Must Have

**Guidance detail navigation**
- `InitiativeDetailScreen`: displays initiative title, description, status, and a list of associated epics. Accessible by tapping an initiative row in the existing initiative list.
- `EpicDetailScreen`: displays epic title, description, status, and a `LazyColumn` of child quests. Accessible by tapping an epic row in `InitiativeDetailScreen`.
- Breadcrumb navigation: back stack supports Today → Initiative List → Initiative Detail → Epic Detail → Quest Detail without tab resets.
- Screen routes `initiative/{id}` and `epic/{id}` added to `Screen.kt` and `NavGraph.kt`.
- `GuidanceViewModel` injected with `EpicDao` so epic data is available without a separate ViewModel.

**Offline sync indicator**
- A subtle Weathered Slate icon appears in the `TopAppBar` (or `MainScaffold`) when `PowerSyncDatabase.currentStatus` indicates pending uploads (`uploading > 0`) or the database has never completed an initial sync (`hasSynced == false`).
- The icon disappears once `uploading == 0` and `hasSynced == true`.
- No tooltip or label required — icon only.

**Sync debounce**
- `SyncCoordinator` tracks the timestamp of the most recent completed sync.
- Calls to `enqueueExpedited()` are no-ops if a sync completed within the preceding 5 minutes.
- The debounce window is a named constant, not a magic number.

**Test coverage — post-PR-review gaps**
All nine test classes specified in post-review tasks S026-T through S035-T must be written (or extended for tests where the file already exists):
- `AuthViewModelTest`: login success → `isAuthenticated = true`; login failure → error state; register failure → error state. Asserts `isAuthenticated` derives from `tokenPreferences.isLoggedInFlow`.
- `AltairPowerSyncConnectorTest`: `uploadData()` routes PUT/PATCH ops to `SyncApi.upsert()` and DELETE ops to `SyncApi.delete()`; `batch.complete(null)` called on success; NOT called when `SyncApi` throws.
- `FocusSessionScreenTest` (instrumented): `_isFinished` transitions to `true` via `onFinish()`; `recordSession()` is called on finish.
- `SyncCoordinatorTest` + `SyncWorkerTest`: `doWork()` returns `Result.retry()` on `IOException`, `Result.failure()` on `Exception`, rethrows `CancellationException`; `SyncCoordinator.triggerSync()` propagates exceptions from `powerSyncDatabase.connect()`.
- Unauthenticated redirect instrumented test: clearing tokens causes `MainActivity` to navigate to Login and clear the back stack.
- `TokenPreferencesTest`: `clearTokens()` nullifies both `accessToken` and `refreshToken`; `isLoggedInFlow` emits `false`.
- `TodayViewModel` Robolectric integration test (in-memory Room): `completeQuest()` does NOT update a `not_started` quest's status at the database level.
- `LoginScreenTest` extensions: assert loading spinner renders on `AuthUiState.Loading`; assert error message renders on `AuthUiState.Error` (extends existing `LoginScreenTest.kt`).
- `QuestDetailScreenTest` extension: renders only permitted status transitions using real `QuestDetailViewModel`, not a stub `validTransitions` map (extends existing `QuestDetailScreenTest.kt`).

**Barcode scanner (CameraX + ML Kit)**
- Full-screen camera overlay composable (`BarcodeScannerScreen`) launched from the item creation screen.
- CameraX `PreviewView` fills the screen; `ImageAnalysis` use case runs ML Kit `BarcodeScanning` analyzer on each frame.
- Visual scan frame: rounded-corner overlay border in Deep Muted Teal-Navy (`#446273`).
- On successful decode: query `TrackingItemDao` for an item with matching `barcode` column.
  - Match found: dismiss camera and open the update-quantity flow for that item.
  - No match: dismiss camera and navigate to item creation form with the barcode field pre-populated.
- Barcode lookup works fully offline against local Room data.
- Requires `CAMERA` runtime permission; permission rationale shown if denied.
- Target latency: barcode decode to UI response under 2 seconds (per `09-PLAT-001-android.md` performance target).

**Focus timer foreground service notification**
- `FocusTimerService extends LifecycleService`: started when the focus session screen is active and the app transitions to background.
- Foreground notification (persistent, non-dismissable while running) shows timer countdown updating every second.
- On `CountDownTimer.onFinish()`: cancel the foreground notification, fire a one-shot local notification ("Focus session complete").
- Stopped and notification cleared when the focus session screen is resumed or explicitly ended.
- On Android 13+ (API 33+), `POST_NOTIFICATIONS` runtime permission is requested before the service is started; on API 31–32, no runtime permission required.
- `FOREGROUND_SERVICE_MEDIA_PLAYBACK` or `FOREGROUND_SERVICE` manifest permission declared as appropriate.

---

### Won't Have (this feature)

- **FCM push notifications**: server-side push dispatch — deferred to Step 11.
- **Attachment upload/download**: binary file handling, camera attachment to notes/items — deferred to Step 10.
- **Voice note capture**: audio recording — deferred to P2.
- **Search**: full-text or semantic search wiring — deferred to Step 10.
- **Conflict resolution UI**: dedicated conflict view — deferred to Step 12.
- **Widgets**: Today widget, quick capture widget — deferred to P2.

---

## Testable Assertions

| ID | Assertion | Verification |
|---|---|---|
| FA-020 | Tapping an initiative row in the initiative list navigates to `InitiativeDetailScreen` showing the initiative's title and a list of its epics. | Manual: tap initiative row, assert detail screen renders with title and epic list. |
| FA-021 | Tapping an epic row in `InitiativeDetailScreen` navigates to `EpicDetailScreen` showing the epic's title and its child quests. | Manual: tap epic row from initiative detail, assert epic detail renders with quest list. |
| FA-022 | From Epic Detail, the back stack navigates: Epic Detail → Initiative Detail → Initiative List → Today (no tab reset). | Manual: press back repeatedly from epic detail, assert correct screen order. |
| FA-023 | A Weathered Slate sync-pending icon is visible in the app bar after performing a local write while offline. | Manual: disable network, create a quest, assert icon is present in app bar. |
| FA-024 | The sync-pending icon disappears after network connectivity is restored and PowerSync upload count returns to zero. | Manual: re-enable network, wait for sync completion, assert icon is no longer visible. |
| FA-025 | `SyncCoordinator.enqueueExpedited()` does NOT enqueue a `OneTimeWorkRequest` when called within 5 minutes of a completed sync. | Unit test: set `lastSyncTime` to `now - 4 minutes`, call `enqueueExpedited()`, assert `WorkManager` work count does not increase. |
| FA-026 | `AuthViewModelTest.login_success_setsIsAuthenticatedTrue` passes: after mock `AuthRepository` returns success, `AuthViewModel.isAuthenticated` emits `true`. | Unit test. |
| FA-027 | `AuthViewModelTest.login_failure_emitsErrorState` passes: after mock `AuthRepository` throws, `AuthViewModel.uiState` emits `AuthUiState.Error`. | Unit test. |
| FA-028 | `AltairPowerSyncConnectorTest`: `uploadData()` calls `SyncApi.upsert()` for a PUT op and does NOT call `SyncApi.delete()`; calls `batch.complete(null)` exactly once on success. | Unit test with mock `SyncApi`. |
| FA-029 | `AltairPowerSyncConnectorTest`: when `SyncApi.upsert()` throws, `batch.complete(null)` is NOT called. | Unit test with mock `SyncApi` configured to throw. |
| FA-030 | `SyncWorkerTest`: `doWork()` returns `Result.retry()` when `SyncCoordinator.triggerSync()` throws `IOException`. | Unit test. |
| FA-031 | `SyncWorkerTest`: `doWork()` rethrows `CancellationException` without wrapping. | Unit test. |
| FA-032 | `TokenPreferencesTest`: after `clearTokens()`, both `accessToken` and `refreshToken` are `null`; `isLoggedInFlow` emits `false`. | Unit test with fake `SharedPreferences`. |
| FA-033 | Unauthenticated redirect: after tokens are cleared programmatically, `MainActivity` navigates to the Login screen and the back stack does not contain any protected screen. | Instrumented test. |
| FA-034 | `TodayViewModel` Robolectric test: calling `completeQuest(questId)` on a quest with status `not_started` does not change the quest's status in the in-memory Room database. | Robolectric + in-memory Room test. |
| FA-035 | `LoginScreenTest`: when `AuthUiState` is `Loading`, a progress indicator is present in the composition. | Compose UI test. |
| FA-036 | `LoginScreenTest`: when `AuthUiState` is `Error("…")`, an error message string is present in the composition. | Compose UI test. |
| FA-037 | `QuestDetailScreenTest`: using real `QuestDetailViewModel`, a quest with status `not_started` renders a "Start" action and does NOT render a "Complete" action. | Compose UI test with real ViewModel. |
| FA-038 | `BarcodeScannerScreen` opens from the item creation form; on a mock ML Kit scan result matching an existing `TrackingItemEntity.barcode`, the update-quantity flow is presented. | Instrumented test with mock `BarcodeScanning` analyzer. |
| FA-039 | On a mock ML Kit scan result with no matching `barcode` in Room, the item creation form is presented with the barcode field pre-populated. | Instrumented test. |
| FA-040 | `FocusTimerService` posts a foreground notification when the app is backgrounded during an active focus session; the notification is dismissed when the session ends. | Manual: start focus session, background app, assert notification visible; end session, assert notification gone. |
| FA-041 | On API 33+, `POST_NOTIFICATIONS` permission is requested before starting `FocusTimerService`; if denied, the service does not crash and the focus session continues without a notification. | Manual on API 33 emulator. |

---

## Open Questions

- [ ] **OQ-001 — EpicDao injection scope**: `GuidanceViewModel` currently takes `InitiativeDao`, `QuestDao`, `RoutineDao`, and `db: PowerSyncDatabase`. Adding `EpicDao` grows the constructor further. Determine during Tech phase whether a dedicated `EpicViewModel` + `InitiativeViewModel` pair is preferable to consolidating into the existing `GuidanceViewModel`, or whether a `GuidanceRepository` abstraction should be introduced to reduce constructor arity.
- [ ] **OQ-002 — Debounce persistence across restarts**: The 5-minute sync debounce window should survive process death (e.g., store `lastSyncTime` in `DataStore` rather than in-memory). Evaluate during Tech phase whether in-memory is sufficient given WorkManager's own deduplication, or whether persistence is needed to prevent double-sync on cold start.
- [ ] **OQ-003 — ML Kit dependency variant**: ML Kit `BarcodeScanning` ships as both a bundled variant (larger APK, works fully offline at install) and an unbundled variant (smaller APK, may download models on first use via Google Play). The bundled variant is preferred for offline reliability per the platform spec. Confirm artifact ID during Tech phase.

---

## Dependencies

| Dependency | Status | Notes |
|---|---|---|
| Feature 009: Android Client | Complete (partial) | Foundation, auth, Room, PowerSync, navigation shell, Today, Knowledge, Tracking, basic Guidance screens all present |
| `apps/android/` codebase | Present | `GuidanceViewModel`, `NavGraph`, `Screen`, `MainScaffold`, `SyncCoordinator`, `AltairPowerSyncConnector` all exist and compile |
| CameraX (`androidx.camera`) | Not yet added | Requires dependency addition to `libs.versions.toml` |
| ML Kit Barcode Scanning (`com.google.mlkit:barcode-scanning`) | Not yet added | Requires dependency addition |
| `docs/specs/06-state-machines.md` | Current | Quest state machine required for QuestDetailScreenTest assertions |
| `docs/specs/03-invariants.md` | Current | E-7 (consumption validation) implicitly covered by existing tests; no new invariants introduced |

---

## Revision History

| Date | Change | ADR |
|---|---|---|
| 2026-04-16 | Initial spec | — |
| 2026-04-16 | Barcode scanner and focus timer foreground service promoted to Must Have | — |
