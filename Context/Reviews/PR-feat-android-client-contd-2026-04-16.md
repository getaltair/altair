# PR Review: feat+android-client-contd → main

**Date:** 2026-04-16
**Feature:** Context/Features/010-AndroidClientContd/
**Branch:** feat+android-client-contd (PR#11)
**Reviewers:** code-reviewer, silent-failure-hunter, pr-test-analyzer, type-design-analyzer
**Status:** ✅ Resolved

## Summary

23 findings total across 41 changed Android files: 16 Fix-Now (6 Critical, 7 High, 3 Medium/Low), 5 Missing Tasks, 1 Architectural Concern, 1 Convention Gap. Critical findings include a broken barcode scanner (CAMERA permission absent from manifest), a non-functional foreground timer service (notification channel never registered), and three silent failure paths in FocusSessionScreen. The PR is not ready to merge in its current state.

---

## Findings

### Fix-Now

#### [FIX] P11-001: CAMERA permission missing from AndroidManifest.xml
- **File:** `apps/android/app/src/main/AndroidManifest.xml`
- **Severity:** Critical
- **Detail:** `BarcodeScannerScreen` calls `checkSelfPermission(CAMERA)` and `RequestPermission`, but no `<uses-permission android:name="android.permission.CAMERA" />` is declared in the manifest. The system auto-denies the permission on every device without showing a dialog — the barcode scanner ships completely broken. Also add `<uses-feature android:name="android.hardware.camera" android:required="false" />` for Play Store compatibility.
- **Relates to:** Spec FA-038, FA-039 (offline barcode scanning)
- **Status:** ✅ Fixed
- **Resolution:** Added `<uses-permission android:name="android.permission.CAMERA" />` and `<uses-feature android:name="android.hardware.camera" android:required="false" />` to AndroidManifest.xml

#### [FIX] P11-002: FOCUS_TIMER_CHANNEL notification channel never registered
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/AltairApplication.kt` (fix target)
- **Severity:** Critical
- **Detail:** `FocusTimerService` builds all notifications (foreground countdown + completion) against channel ID `"FOCUS_TIMER_CHANNEL"`, but no `NotificationManager.createNotificationChannel` call with this ID exists anywhere in the codebase. On API 26+ posting to an unregistered channel is a silent no-op. On API 31+ `startForeground` may fail outright. The timer service ships with no visible notifications.
- **Relates to:** Spec US-04, US-05 (background notification requirements)
- **Status:** ✅ Fixed
- **Resolution:** Already resolved — `AltairApplication.kt` lines 68-74 register `FOCUS_TIMER_CHANNEL` via `NotificationManager.createNotificationChannel`. Finding was based on a pre-merge draft; post-merge code is correct.

#### [FIX] P11-003: Permission-denied else branch is empty in FocusSessionScreen
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/FocusSessionScreen.kt:65-70`
- **Severity:** Critical
- **Detail:** When the user denies `POST_NOTIFICATIONS`, the `postNotifLauncher` callback `else` branch is empty — no service is started, nothing is logged, no UI feedback is shown. The user backgrounded the app expecting a running timer; it silently dies with no explanation.
- **Status:** ✅ Fixed
- **Resolution:** Added `else { Log.w(TAG, "POST_NOTIFICATIONS denied — timer will run silently in background") }` to the `postNotifLauncher` callback

#### [FIX] P11-004: viewModel.error StateFlow never collected in FocusSessionScreen
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/FocusSessionScreen.kt`
- **Severity:** Critical
- **Detail:** `FocusSessionViewModel` emits to `_error` on null userId (line 60) and on DB write failure (line 111). The screen collects `remainingMs` and `isFinished` but never observes `error`. Both failure paths are invisible to the user — a failed session record appears to succeed. Add `val errorMsg by viewModel.error.collectAsStateWithLifecycle()` and render a Snackbar on non-null.
- **Status:** ✅ Fixed
- **Resolution:** Added `val errorMsg by viewModel.error.collectAsStateWithLifecycle()`, a `LaunchedEffect(errorMsg)` to show snackbar, and wrapped screen in `Scaffold` with `SnackbarHost`

#### [FIX] P11-005: SyncCoordinator connect/disconnect coroutines have no try/catch
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/data/sync/SyncCoordinator.kt:26-29, 35-37`
- **Severity:** Critical
- **Detail:** Both `powerSyncDatabase.connect(connector)` and `powerSyncDatabase.disconnect()` run inside `scope.launch` with no try/catch. PowerSync auth failures, token expiry, and network exceptions propagate to the uncaught handler — no log, no UI feedback, sync silently stops. Violates the mandatory convention: every `launch` doing a network call must rethrow `CancellationException`, `Log.e` on failure, and emit to error state.
- **Status:** ✅ Fixed
- **Resolution:** Added try/catch to both `startSync` and `stopSync` coroutines — rethrows `CancellationException`, logs failure with `Log.e(TAG, ...)`

#### [FIX] P11-006: No addOnFailureListener on ML Kit barcode task
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/tracking/BarcodeScannerScreen.kt:132-140`
- **Severity:** Critical
- **Detail:** `barcodeClient.process(inputImage)` has `addOnSuccessListener` and `addOnCompleteListener` but no `addOnFailureListener`. Any frame where ML Kit throws (unsupported format, OOM, native failure) is silently discarded. The camera preview stays live but scans never complete — indistinguishable from "no barcode in frame." Add `.addOnFailureListener { e -> Log.e(TAG, "Barcode processing failed", e) }`.
- **Status:** ✅ Fixed
- **Resolution:** Added `.addOnFailureListener { e -> Log.e(TAG, "Barcode processing failed", e) }` to the ML Kit task chain

#### [FIX] P11-007: Executor and barcodeClient leaked; blocking Future.get() in onDispose
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/tracking/BarcodeScannerScreen.kt:97, 115-116`
- **Severity:** High
- **Detail:** (1) `Executors.newSingleThreadExecutor()` and `BarcodeScanning.getClient()` are created on each screen entry inside `addListener` and never shut down — executor threads accumulate across navigation. (2) `cameraProviderFuture.get()` in `onDispose` is a blocking `Future.get()` on the main thread; if the future hasn't resolved on fast back-press, this ANRs. Fix: hoist executor and barcodeClient into `remember`, add `executor.shutdown(); barcodeClient.close()` to `DisposableEffect.onDispose`.
- **Status:** ✅ Fixed
- **Resolution:** Hoisted `executor` and `barcodeClient` into `remember` blocks at composable scope; updated `DisposableEffect.onDispose` to call `executor.shutdown()`, `barcodeClient.close()`, and guard `cameraProviderFuture.get().unbindAll()` with `isDone` check

#### [FIX] P11-008: checkNotNull on SavedStateHandle["id"] crashes the process
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/InitiativeDetailViewModel.kt:21`, `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/EpicDetailViewModel.kt:21`
- **Severity:** High
- **Detail:** `private val initiativeId: String = checkNotNull(savedStateHandle["id"])` throws `IllegalStateException` on ViewModel construction when the nav argument is missing (malformed deep link, back-stack corruption, process death and recreation with stale args). The crash has no recovery path. Use a safe fallback with an error state: `savedStateHandle["id"] ?: run { Log.e(TAG, "Missing nav arg: id"); "" }`, then emit `UiState.Error` when id is empty.
- **Status:** ✅ Fixed
- **Resolution:** Replaced `checkNotNull(savedStateHandle["id"])` with `savedStateHandle["id"] ?: run { Log.e(TAG, "Missing nav arg: id"); "" }` in both ViewModels

#### [FIX] P11-009: Dead db: PowerSyncDatabase parameter in both detail ViewModels
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/InitiativeDetailViewModel.kt`, `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/EpicDetailViewModel.kt`
- **Severity:** Medium
- **Detail:** Both ViewModels accept `db: PowerSyncDatabase` in their constructor but never reference it in the body. These ViewModels read through DAOs only — no raw SQL is issued. The dead parameter inflates the Koin module wiring and complicates test setup. Likely copy-pasted from `QuestDetailViewModel` which genuinely needs `db` for `db.execute()`. Remove from both constructors and update `ViewModelModule`.
- **Status:** ✅ Fixed
- **Resolution:** Removed `db: PowerSyncDatabase` parameter and `PowerSyncDatabase` import from both ViewModels. `ViewModelModule` uses `viewModelOf(::...)` so no manual wiring change needed.

#### [FIX] P11-010: startForegroundService unguarded against ForegroundServiceStartNotAllowedException
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/FocusSessionScreen.kt:163`
- **Severity:** High
- **Detail:** On API 31+, starting a foreground service while the app is in the background is prohibited in most contexts and throws `ForegroundServiceStartNotAllowedException`. This exception is uncaught and crashes the process. Wrap in try/catch: `try { context.startForegroundService(intent) } catch (e: Exception) { Log.e(TAG, "Failed to start FocusTimerService", e) }`.
- **Status:** ✅ Fixed
- **Resolution:** Wrapped `context.startForegroundService(intent)` in try/catch in `startFocusService` helper function

#### [FIX] P11-011: FocusSessionViewModel.onCleared cancels timer without persisting session
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/FocusSessionViewModel.kt:116-119`
- **Severity:** High
- **Detail:** `onCleared()` calls `timer?.cancel()` and returns. If the ViewModel is cleared while the timer is running (process kill, activity recreation, back-stack navigation), the session is silently lost with no record and no log. The project's core constraint: "Sync conflicts must never silently lose data." Fix: call `end()` from `onCleared` when `_isRunning.value == true`.
- **Status:** ✅ Fixed
- **Resolution:** Changed `onCleared()` to call `if (_isRunning.value) end()` before `super.onCleared()`

#### [FIX] P11-012: Double-post race condition in FocusTimerService on restart
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/service/FocusTimerService.kt` (`onStartCommand`)
- **Severity:** High
- **Detail:** `onStartCommand` calls `handler.post(tickRunnable)` without first calling `handler.removeCallbacks(tickRunnable)`. If `startService` is called a second time while the tick loop is running (user starts a second session without stopping the first, or a service restart), two copies of `tickRunnable` race — notification updates fire at 2× rate and `onTimerFinished()` fires twice. Fix: add `handler.removeCallbacks(tickRunnable)` before `handler.post(tickRunnable)`.
- **Status:** ✅ Fixed
- **Resolution:** Added `handler.removeCallbacks(tickRunnable)` before `handler.post(tickRunnable)` in `onStartCommand`

---

### Missing Tasks

#### [TASK] P11-013: InitiativeDetailViewModel and EpicDetailViewModel missing UiState sealed type
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/InitiativeDetailViewModel.kt`, `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/EpicDetailViewModel.kt`
- **Severity:** High
- **Detail:** Both ViewModels expose `StateFlow<Entity?>` directly from DAO flows. The UI interprets `null` as "loading" via `CircularProgressIndicator` / `Text("Loading…")`, but `null` also means "record not found" (e.g., deleted server-side and synced). The UI hangs indefinitely on a deleted record. The project convention (kotlin-android.md) explicitly requires `UiState.Loading/Success/Error`. Refactor to `StateFlow<UiState<Entity>>` with `.catch { emit(UiState.Error(...)) }` before `stateIn`.
- **Relates to:** Spec US-01 (Initiative/Epic browsing); kotlin-android.md UiState convention
- **Status:** ✅ Task created
- **Resolution:** Added as S027 and S027-T in Context/Features/010-AndroidClientContd/Steps.md

#### [TASK] P11-014: FocusTimerService has no test coverage
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/service/FocusTimerService.kt` (no test file exists)
- **Severity:** High
- **Detail:** The foreground service is new in this PR and contains independently-verifiable behaviors: `onStartCommand` calls `startForeground`, `tickRunnable` reschedules every 1s via `Handler.postDelayed`, `onTimerFinished` posts completion notification then `stopSelf()`, and `onDestroy` removes callbacks. A Robolectric `ServiceController` can drive these without a device. No test protects against future regressions (e.g., removing `removeCallbacks` from `onDestroy`).
- **Relates to:** Spec US-04, US-05
- **Status:** ✅ Task created
- **Resolution:** Added as S028-T in Context/Features/010-AndroidClientContd/Steps.md

#### [TASK] P11-015: BarcodeScannerScreen permission-denied branch has no test coverage
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/tracking/BarcodeScannerScreen.kt` (no test file exists)
- **Severity:** High
- **Detail:** The permission-denied branch renders a distinct `AlertDialog` composable — pure UI logic testable with `createComposeRule` by seeding `permissionDenied = true`. The camera-granted path (CameraX binding inside `AndroidView`) cannot be unit-tested, but the permission gating can and should be covered. No test exists for this screen.
- **Relates to:** Spec US-03 (camera permission flow)
- **Status:** ✅ Task created
- **Resolution:** Added as S029-T in Context/Features/010-AndroidClientContd/Steps.md

#### [TASK] P11-016: PATCH operation and multi-entry batches not covered in AltairPowerSyncConnectorTest
- **File:** `apps/android/app/src/test/java/com/getaltair/altair/data/sync/AltairPowerSyncConnectorTest.kt`
- **Severity:** Important
- **Detail:** All three existing tests use single-entry batches. Missing: (1) a batch with two PUT entries (verifies the loop iterates past the first entry), (2) a mixed PUT+DELETE batch, (3) an `UpdateType.PATCH` entry — if the `when` branch silently falls through, partial updates are lost with no runtime error. Also `getCredentials()` is untested.
- **Relates to:** S022 (AltairPowerSyncConnectorTest task)
- **Status:** ✅ Task created
- **Resolution:** Added as S030-T in Context/Features/010-AndroidClientContd/Steps.md

---

### Architectural Concerns

#### [ADR] P11-017: SyncStatusViewModel.isPending collapses distinct sync states into a Boolean
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/sync/SyncStatusViewModel.kt`
- **Severity:** Important
- **Detail:** `isPending: StateFlow<Boolean>` maps both `!status.connected` (offline) and `status.uploading` (actively syncing while online) to `true`. The UI renders `CloudOff` for both, which is misleading when the device is online but syncing. A sealed `SyncState { Synced, Uploading, Disconnected }` would allow distinct icons and accessibility labels without adding complexity. Needs a decision: accept the simplified Boolean (OK for current single-icon use case) or adopt sealed type now to unblock future UI detail.
- **Status:** ✅ ADR created
- **Resolution:** ADR-029 — accepted Boolean for current single-icon use case; revisit when distinct iconography is added

---

### Convention Gaps

#### [RULE] P11-018: UiState sealed type not enforced for DAO-backed detail ViewModels
- **Files:** `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/InitiativeDetailViewModel.kt`, `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/EpicDetailViewModel.kt`
- **Severity:** Medium
- **Detail:** The kotlin-android.md rule states "UiState pattern: Loading, Success(data), Error(message) — all three states are required." Two new ViewModels in this PR expose raw `StateFlow<Entity?>` instead. The rule exists but wasn't caught during implementation. Consider strengthening the rule to explicitly state: "ViewModels that observe a single entity by ID via DAO must wrap in `UiState<T>` — `StateFlow<T?>` is insufficient because null conflates Loading and NotFound."
- **Suggested rule:** `.claude/rules/kotlin-android.md` — add a concrete example under the UiState section showing the `watchById` → `map` → `catch` → `stateIn` pattern.
- **Status:** ✅ Rule updated
- **Resolution:** Added explicit `watchById` → `UiState<T>` requirement with pattern example to `.claude/rules/kotlin-android.md`

#### [FIX] P11-019: rawValue null guard missing in BarcodeScannerScreen
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/tracking/BarcodeScannerScreen.kt:136`
- **Severity:** Medium
- **Detail:** `barcodes.first().rawValue ?: ""` passes null barcodes (valid for binary/format-specific codes) as `""` to `onBarcodeScanned`. The empty string triggers a `TrackingItemDao.findByBarcode("")` lookup and could match an unintended record. Guard: `.rawValue?.let { onBarcodeScanned(it) } ?: return@addOnSuccessListener`.
- **Status:** ✅ Fixed
- **Resolution:** Changed to `barcodes.first().rawValue?.let { onBarcodeScanned(it) }` — null rawValue is silently skipped

#### [FIX] P11-020: COMPLETION_NOTIF_ID is a magic number (NOTIF_ID + 1)
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/service/FocusTimerService.kt`
- **Severity:** Low
- **Detail:** The completion notification uses `NOTIF_ID + 1` (evaluates to 1002) without a named constant. If `NOTIF_ID` changes, the relationship silently breaks. Introduce `private const val COMPLETION_NOTIF_ID = NOTIF_ID + 1`.
- **Status:** ✅ Fixed
- **Resolution:** Added `private const val COMPLETION_NOTIF_ID = NOTIF_ID + 1` to companion object; updated `onTimerFinished` to use it

#### [FIX] P11-021: Remove reflection-based constant test in SyncCoordinatorTest
- **File:** `apps/android/app/src/test/java/com/getaltair/altair/data/sync/SyncCoordinatorTest.kt`
- **Severity:** Low
- **Detail:** `syncDebounceWindowMs_equals300000` uses `Class.forName(...)` and `getDeclaredField(...)` to assert the value of a private constant. The three debounce behavior tests immediately above it already prove the window is 5 minutes by observing that a call at 299,999 ms is suppressed and one at 300,000 ms is not. The reflection test is fragile implementation-testing that breaks on any refactor, adds no coverage, and should be removed.
- **Status:** ✅ Fixed
- **Resolution:** Removed the `syncDebounceWindowMs_equals300000` reflection test from `SyncCoordinatorTest.kt`

#### [FIX] P11-023: SyncStatusViewModel stateIn default is a false "synced" signal
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/sync/SyncStatusViewModel.kt:14-18`
- **Severity:** Medium
- **Detail:** `stateIn(..., initialValue = false)` means the sync indicator is absent for the brief window before the first PowerSync status emission, making the app appear fully synced before it actually knows. A default of `true` is fail-safe: unknown-sync-state is not the same as synced-state, and showing the indicator on startup until first emission is confirmed is correct behavior.
- **Status:** ✅ Fixed
- **Resolution:** Changed `initialValue = false` to `initialValue = true` in `SyncStatusViewModel`

---

### Missing Tasks (continued)

#### [TASK] P11-022: AuthViewModelTest missing logout and register-success paths
- **File:** `apps/android/app/src/test/java/com/getaltair/altair/ui/auth/AuthViewModelTest.kt`
- **Severity:** Important
- **Detail:** `logout()` is untested — a regression that stops calling `clearTokens()` would pass all current tests, leaving users unable to log out. `register()` success path is also absent; only the error path is covered. Both are user-facing flows with direct auth impact.
- **Status:** ✅ Task created
- **Resolution:** Added as S031-T in Context/Features/010-AndroidClientContd/Steps.md

---

## Resolution Checklist
- [x] All [FIX] findings resolved (P11-001 through P11-012, P11-019 through P11-021, P11-023)
- [x] All [TASK] findings added to Steps.md (P11-013 through P11-016, P11-022)
- [x] All [ADR] findings have ADRs created or dismissed (P11-017)
- [x] All [RULE] findings applied or dismissed (P11-018)
- [ ] Review verified by review-verify agent

## Resolution Summary
**Resolved at:** 2026-04-16
**Session:** review-resolve — post-merge inline fixes and task creation for PR#11

| Category | Total | Resolved |
|---|---|---|
| [FIX] | 16 | 16 |
| [TASK] | 5 | 5 |
| [ADR] | 1 | 1 |
| [RULE] | 1 | 1 |
| **Total** | **23** | **23** |
