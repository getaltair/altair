# Implementation Steps: Android Client — Continued

**Spec:** Context/Features/010-AndroidClientContd/Spec.md
**Tech:** Context/Features/010-AndroidClientContd/Tech.md

## Progress
- **Status:** Complete
- **Current task:** --
- **Last milestone:** Phase 8 complete (all post-review tasks done 2026-04-16)

## Team Orchestration

### Team Members
- **builder-data**
  - Role: ViewModel, DAO, Service, and sync logic
  - Agent Type: backend-engineer
  - Resume: true
- **builder-ui**
  - Role: Jetpack Compose screens and navigation wiring
  - Agent Type: frontend-specialist
  - Resume: true
- **builder-test**
  - Role: All test class authoring
  - Agent Type: qa-kotlin
  - Resume: true
- **validator**
  - Role: Quality validation (read-only)
  - Agent Type: quality-engineer
  - Resume: false

---

## Tasks

### Phase 1: Foundation

- [ ] S001: Add CameraX and ML Kit dependencies to `apps/android/gradle/libs.versions.toml` and `apps/android/app/build.gradle.kts`
  Add versions: `camerax = "1.4.2"`, `mlkitBarcode = "17.3.0"`, `lifecycleService = "2.10.0"` (reuse existing `lifecycleRuntimeKtx` ref if version matches, else separate).
  Add library entries: `camerax-core`, `camerax-camera2`, `camerax-lifecycle`, `camerax-view` (group `androidx.camera`), `mlkit-barcode-scanning` (group `com.google.mlkit`, name `barcode-scanning`), `lifecycle-service` (group `androidx.lifecycle`, name `lifecycle-service`).
  Add to `app/build.gradle.kts` implementation block: all 4 CameraX libs + `mlkit-barcode-scanning` + `lifecycle-service`.
  - **Assigned:** builder-data
  - **Depends:** none
  - **Parallel:** true

- [ ] S002: Add `InitiativeDetail`, `EpicDetail`, and `BarcodeScanner` routes to `navigation/Screen.kt`
  - `object InitiativeDetail : Screen("today_graph/guidance/initiatives/{id}") { fun route(id: String) = "today_graph/guidance/initiatives/$id" }`
  - `object EpicDetail : Screen("today_graph/guidance/initiatives/{initiativeId}/epics/{id}") { fun route(initiativeId: String, id: String) = ... }`
  - `object BarcodeScanner : Screen("barcode_scanner")`
  - **Assigned:** builder-ui
  - **Depends:** none
  - **Parallel:** true

- [ ] S003: Fix timestamp bug in `GuidanceViewModel.kt`
  Line 85: replace `java.time.LocalDateTime.now().format(DateTimeFormatter.ISO_LOCAL_DATE_TIME)` with `Clock.System.now().toString()`. Add `import kotlinx.datetime.Clock`. Remove unused `java.time` imports.
  - **Assigned:** builder-data
  - **Depends:** none
  - **Parallel:** true

---

### Phase 2: Guidance Detail ViewModels

- [ ] S004: Create `InitiativeDetailViewModel` and `EpicDetailViewModel`
  **`ui/guidance/InitiativeDetailViewModel.kt`:**
  - Constructor: `SavedStateHandle`, `InitiativeDao`, `EpicDao`, `db: PowerSyncDatabase`
  - Read `initiativeId` from `SavedStateHandle["id"]`
  - `initiative: StateFlow<InitiativeEntity?>` from `initiativeDao.watchById(initiativeId)`
  - `epics: StateFlow<List<EpicEntity>>` from `epicDao.watchByInitiativeId(initiativeId)` — check if this query exists on `EpicDao`; add if missing

  **`ui/guidance/EpicDetailViewModel.kt`:**
  - Constructor: `SavedStateHandle`, `EpicDao`, `QuestDao`, `db: PowerSyncDatabase`
  - Read `epicId` from `SavedStateHandle["id"]`
  - `epic: StateFlow<EpicEntity?>` from `epicDao.watchById(epicId)`
  - `quests: StateFlow<List<QuestEntity>>` from `questDao.watchByEpicId(epicId)` — check if this query exists on `QuestDao`; add if missing

  Register both in `di/ViewModelModule.kt` using `viewModel { InitiativeDetailViewModel(get(), get(), get(), get()) }` pattern.
  - **Assigned:** builder-data
  - **Depends:** none
  - **Parallel:** true

- [ ] S004-T: Unit tests for `InitiativeDetailViewModel` and `EpicDetailViewModel`
  (initiative loads from DAO when initiativeId present; epics list populates from epicDao.watchByInitiativeId; epic loads from DAO when epicId present; quests list populates from questDao.watchByEpicId; null SavedStateHandle id emits null/empty state)
  - **Assigned:** builder-test
  - **Depends:** S004
  - **Parallel:** false

- [ ] S005: Create `InitiativeDetailScreen.kt` composable
  - Route: `Screen.InitiativeDetail` — receives `navBackStackEntry`, extracts `id` arg
  - Inject `InitiativeDetailViewModel` via `koinViewModel()`
  - Display: initiative title, description, status chip
  - `LazyColumn` of epic rows; each row is tappable → `navController.navigate(Screen.EpicDetail.route(initiativeId, epic.id))`
  - Back button / `TopAppBar` back navigation returns to initiative list
  - **Assigned:** builder-ui
  - **Depends:** S004
  - **Parallel:** false

- [ ] S006: Create `EpicDetailScreen.kt` composable
  - Route: `Screen.EpicDetail` — receives `navBackStackEntry`, extracts `initiativeId` and `id` args
  - Inject `EpicDetailViewModel` via `koinViewModel()`
  - Display: epic title, description, status chip
  - `LazyColumn` of quest rows with status chips; each row tappable → navigate to `Screen.QuestDetail.route(quest.id)`
  - Back button returns to `InitiativeDetailScreen`
  - **Assigned:** builder-ui
  - **Depends:** S004
  - **Parallel:** true

- [ ] S007: Wire `InitiativeDetailScreen` and `EpicDetailScreen` into `NavGraph.kt`
  Inside the `today_graph` nested navigation block, add:
  ```kotlin
  composable(Screen.InitiativeDetail.route) { backStackEntry ->
      InitiativeDetailScreen(navController, backStackEntry)
  }
  composable(Screen.EpicDetail.route) { backStackEntry ->
      EpicDetailScreen(navController, backStackEntry)
  }
  ```
  In the existing initiative list composable, wire row click to `navController.navigate(Screen.InitiativeDetail.route(initiative.id))`.
  - **Assigned:** builder-ui
  - **Depends:** S002, S005, S006
  - **Parallel:** false

🏁 MILESTONE M1: Guidance detail navigation complete — manually verify FA-020, FA-021, FA-022
  **Contracts:**
  - `apps/android/app/src/main/java/com/getaltair/altair/navigation/Screen.kt` — InitiativeDetail and EpicDetail routes declared
  - `apps/android/app/src/main/java/com/getaltair/altair/navigation/NavGraph.kt` — destinations registered in today_graph

---

### Phase 3: Sync Indicator and Debounce

- [ ] S008: Create `SyncStatusViewModel`
  **`ui/sync/SyncStatusViewModel.kt`:**
  - Constructor: `db: PowerSyncDatabase`
  - `isPending: StateFlow<Boolean>` derived from `db.currentStatus.asFlow().map { status -> status.uploading > 0 || !status.hasSynced }.stateIn(viewModelScope, SharingStarted.WhileSubscribed(5_000), false)`
  - Register in `di/ViewModelModule.kt`
  - **Assigned:** builder-data
  - **Depends:** none
  - **Parallel:** true

- [ ] S008-T: Unit test `SyncStatusViewModel`
  (isPending emits true when uploading > 0; isPending emits true when hasSynced == false; isPending emits false when uploading == 0 and hasSynced == true; initial value is false)
  - **Assigned:** builder-test
  - **Depends:** S008
  - **Parallel:** false

- [ ] S009: Add `TopAppBar` with sync indicator to `MainScaffold.kt`
  - Inject `SyncStatusViewModel` via `koinViewModel()` inside `MainScaffold`
  - Collect `isPending` as `State` via `collectAsState()`
  - Add `topBar` slot to `Scaffold` with a `TopAppBar` containing the app name and — when `isPending == true` — an `Icon(Icons.Default.CloudOff)` tinted `Color(0xFF8A9EA2)` (Weathered Slate)
  - When `isPending == false`, icon is not rendered (no placeholder space)
  - **Assigned:** builder-ui
  - **Depends:** S008
  - **Parallel:** true

- [ ] S010: Add sync debounce to `SyncCoordinator.kt`
  - Add `private const val SYNC_DEBOUNCE_WINDOW_MS = 5 * 60 * 1_000L` at file top
  - Add `private var lastSyncTime: Long = 0L` field
  - In `enqueueExpedited()`: add guard at top — `if (System.currentTimeMillis() - lastSyncTime < SYNC_DEBOUNCE_WINDOW_MS) return`
  - After `workManager.enqueue(...)` call: `lastSyncTime = System.currentTimeMillis()`
  - **Assigned:** builder-data
  - **Depends:** none
  - **Parallel:** true

- [ ] S010-T: Unit test `SyncCoordinator` debounce behaviour
  (enqueueExpedited enqueues work when lastSyncTime is 0; enqueueExpedited is a no-op when called within 5 minutes of last sync; enqueueExpedited enqueues again after window expires; SYNC_DEBOUNCE_WINDOW_MS constant equals 300_000L)
  - **Assigned:** builder-test
  - **Depends:** S010
  - **Parallel:** false

🏁 MILESTONE M2: Sync indicator and debounce complete — manually verify FA-023, FA-024; unit-verify FA-025 via S010-T

---

### Phase 4: Barcode Scanner

- [ ] S011: Add `findByBarcode` query to `TrackingItemDao.kt`
  ```kotlin
  @Query("SELECT * FROM tracking_items WHERE barcode = :barcode AND deleted_at IS NULL LIMIT 1")
  suspend fun findByBarcode(barcode: String): TrackingItemEntity?
  ```
  Note: `TrackingItemEntity` already has `barcode: String?` — no Room migration required.
  - **Assigned:** builder-data
  - **Depends:** none
  - **Parallel:** true

- [ ] S011-T: Unit test `TrackingItemDao.findByBarcode`
  (returns matching entity when barcode exists in DB; returns null when no match; returns null when barcode matches a soft-deleted item; returns null for empty string barcode)
  Test uses in-memory Room database.
  - **Assigned:** builder-test
  - **Depends:** S011
  - **Parallel:** false

- [ ] S012: Create `BarcodeScannerScreen.kt` composable
  - Full-screen `AndroidView { PreviewView }` as background
  - `Canvas` overlay: centered scan frame with rounded-corner border, stroke color `Color(0xFF446273)` (Deep Muted Teal-Navy)
  - Camera permission: `rememberLauncherForActivityResult(RequestPermission(CAMERA))` — if denied, show rationale dialog; if granted, bind CameraX use cases
  - `ImageAnalysis` use case bound to `LocalLifecycleOwner.current`; analyzer calls `BarcodeScanning.getClient().process(imageProxy.image)` for each frame
  - On successful decode: call `onBarcodeScanned(barcode: String)` callback (passed in)
  - Parameters: `onBarcodeScanned: (String) -> Unit`, `onDismiss: () -> Unit`, `modifier: Modifier = Modifier`
  - **Assigned:** builder-ui
  - **Depends:** S001
  - **Parallel:** true

- [ ] S013: Wire `BarcodeScannerScreen` into item creation navigation
  - Add `composable(Screen.BarcodeScanner.route)` in `NavGraph.kt`
  - In the item creation screen composable, add a scan icon/button that navigates to `Screen.BarcodeScanner`
  - Pass `onBarcodeScanned` callback: query `TrackingItemDao.findByBarcode(barcode)` in a coroutine; if found → pop back stack and navigate to update-quantity flow for that item; if not found → pop back stack and return to item creation form with barcode pre-populated via `savedStateHandle` or navigation argument
  - `onDismiss` → `navController.popBackStack()`
  - **Assigned:** builder-ui
  - **Depends:** S002, S011, S012
  - **Parallel:** false

🏁 MILESTONE M3: Barcode scanner complete — manually verify FA-038, FA-039

---

### Phase 5: Focus Timer Foreground Service

- [ ] S014: Create notification channel and manifest declarations
  - In `AltairApplication.onCreate()`: create `NotificationChannel("FOCUS_TIMER_CHANNEL", "Focus Timer", NotificationManager.IMPORTANCE_LOW)` — call only on API 26+ (already required by minSdk 31, so unconditional is fine)
  - In `AndroidManifest.xml`:
    - Add `<uses-permission android:name="android.permission.FOREGROUND_SERVICE" />`
    - Add `<uses-permission android:name="android.permission.FOREGROUND_SERVICE_SPECIAL_USE" />`
    - Add `<uses-permission android:name="android.permission.POST_NOTIFICATIONS" />`
    - Add `<service android:name=".service.FocusTimerService" android:foregroundServiceType="specialUse" android:description="@string/focus_timer_service_description" android:exported="false" />`
  - In `res/values/strings.xml`: add `<string name="focus_timer_service_description">Countdown timer for focus sessions</string>`
  - **Assigned:** builder-data
  - **Depends:** S001
  - **Parallel:** true

- [ ] S015: Create `FocusTimerService : LifecycleService`
  **`service/FocusTimerService.kt`:**
  - `companion object { const val EXTRA_END_TIME_EPOCH_MS = "end_time_epoch_ms" }`
  - `onStartCommand`: extract `endTimeEpochMs` from intent; call `startForeground(NOTIF_ID, buildNotification(remainingMs))` with `foregroundServiceType = ServiceInfo.FOREGROUND_SERVICE_TYPE_SPECIAL_USE` on API 34+
  - `Handler(Looper.getMainLooper())` posts a `Runnable` every 1_000ms: compute `remaining = endTimeEpochMs - System.currentTimeMillis()`; if `remaining <= 0` → `onTimerFinished()`; else update notification
  - `onTimerFinished()`: cancel foreground notification via `NotificationManagerCompat.cancel(NOTIF_ID)`; post one-shot completion notification with title "Focus session complete"; call `stopSelf()`
  - `onDestroy()`: remove handler callbacks; cancel foreground notification
  - `buildNotification(remainingMs: Long)`: builds a non-dismissable `NotificationCompat.Builder` notification on `FOCUS_TIMER_CHANNEL` with formatted countdown (mm:ss); `PRIORITY_LOW`, `FLAG_ONGOING_EVENT`, `FLAG_NO_CLEAR`
  - **Assigned:** builder-data
  - **Depends:** S014
  - **Parallel:** false

- [ ] S016: Modify `FocusSessionScreen.kt` — service lifecycle + POST_NOTIFICATIONS permission
  - Add `DisposableEffect(Unit)` that registers a `LifecycleEventObserver` on `ProcessLifecycleOwner.get()` (or `LocalLifecycleOwner`'s process lifecycle)
    - `ON_STOP`: if session is running (`isRunning == true`), compute `endTimeEpochMs = System.currentTimeMillis() + remainingMs.value`, then on API 33+: check `POST_NOTIFICATIONS` permission; if granted start service; if not granted, request permission first via `rememberLauncherForActivityResult(RequestPermission(POST_NOTIFICATIONS))` — start service only after grant; on API < 33: start service unconditionally
    - `ON_START`: call `stopService(Intent(context, FocusTimerService::class.java))`
  - On explicit session end (user taps "End"): also stop service
  - `onDispose` removes the lifecycle observer
  - **Assigned:** builder-ui
  - **Depends:** S015
  - **Parallel:** false

🏁 MILESTONE M4: Focus timer foreground service complete — manually verify FA-040, FA-041 on emulator

---

### Phase 6: Post-Review Test Coverage

All tasks in this phase write new test files or extend existing ones. They can run in parallel after their respective production code is in place.

- [ ] S017: Write `AuthViewModelTest`
  **`test/ui/auth/AuthViewModelTest.kt`** (new file, JUnit 5 + Turbine + MockK):
  - `login_success_setsIsAuthenticatedTrue`: mock `AuthRepository.login()` returns success; assert `AuthViewModel.isAuthenticated` emits `true` via Turbine
  - `login_failure_emitsErrorState`: mock `AuthRepository.login()` throws; assert `uiState` emits `AuthUiState.Error`
  - `register_failure_emitsErrorState`: mock `AuthRepository.register()` throws; assert `uiState` emits `AuthUiState.Error`
  - `isAuthenticated_derivesFromTokenPreferencesFlow`: mock `tokenPreferences.isLoggedInFlow` emitting `false` then `true`; assert `isAuthenticated` mirrors the flow
  Covers FA-026, FA-027.
  - **Assigned:** builder-test
  - **Depends:** none
  - **Parallel:** true

- [ ] S018: Write `AltairPowerSyncConnectorTest`
  **`test/data/sync/AltairPowerSyncConnectorTest.kt`** (new file, JUnit 5 + MockK):
  - `uploadData_putOp_callsUpsertNotDelete`: mock `SyncApi.upsert()` succeeds; pass a PUT `CrudEntry`; assert `SyncApi.upsert()` called once, `SyncApi.delete()` not called; assert `batch.complete(null)` called once
  - `uploadData_deleteOp_callsDeleteNotUpsert`: pass a DELETE `CrudEntry`; assert `SyncApi.delete()` called once, `SyncApi.upsert()` not called
  - `uploadData_upsertThrows_batchCompleteNotCalled`: mock `SyncApi.upsert()` throws; assert `batch.complete(null)` NOT called
  Covers FA-028, FA-029.
  - **Assigned:** builder-test
  - **Depends:** none
  - **Parallel:** true

- [ ] S019: Write `SyncWorkerTest`
  **`test/data/sync/SyncWorkerTest.kt`** (new file, JUnit 5 + MockK):
  - `doWork_ioException_returnsRetry`: mock `SyncCoordinator.triggerSync()` throws `IOException`; assert `doWork()` returns `Result.retry()`
  - `doWork_genericException_returnsFailure`: mock throws `RuntimeException`; assert returns `Result.failure()`
  - `doWork_cancellationException_rethrows`: mock throws `CancellationException`; assert `CancellationException` propagates (not wrapped)
  Covers FA-030, FA-031.
  - **Assigned:** builder-test
  - **Depends:** none
  - **Parallel:** true

- [ ] S020: Write `TokenPreferencesTest`
  **`test/data/preferences/TokenPreferencesTest.kt`** (new file, JUnit 5):
  - Use a fake/in-memory `SharedPreferences` (via `AndroidTestUtils` or `mockk`)
  - `clearTokens_nullifiesAccessToken`: call `clearTokens()`; assert `accessToken` is `null`
  - `clearTokens_nullifiesRefreshToken`: assert `refreshToken` is `null`
  - `isLoggedInFlow_emitsFalseAfterClearTokens`: collect `isLoggedInFlow` via Turbine; call `clearTokens()`; assert `false` emitted
  Covers FA-032.
  - **Assigned:** builder-test
  - **Depends:** none
  - **Parallel:** true

- [ ] S021: Write `TodayViewModelTest` (Robolectric + in-memory Room)
  **`test/ui/today/TodayViewModelTest.kt`** (new file, Robolectric + in-memory Room):
  - Annotate with `@RunWith(RobolectricTestRunner::class)`
  - Build in-memory `AltairDatabase` via `Room.inMemoryDatabaseBuilder()`
  - Insert a `QuestEntity` with `status = "not_started"`
  - Call `TodayViewModel.completeQuest(questId)`
  - Assert the quest's `status` in the in-memory DB remains `"not_started"`
  Covers FA-034.
  - **Assigned:** builder-test
  - **Depends:** none
  - **Parallel:** true

- [ ] S022: Write `FocusSessionScreenTest` (instrumented)
  **`androidTest/ui/guidance/FocusSessionScreenTest.kt`** (new file, AndroidX Test):
  - `isFinished_transitionsTrue_viaOnFinish`: advance the ViewModel's `CountDownTimer` to completion (or mock it); assert `_isFinished` emits `true`
  - `recordSession_calledOnFinish`: verify `db.execute(INSERT focus_sessions ...)` is invoked after `onFinish()`
  Covers FA-028 (screen-level focus session state, not the connector test).
  - **Assigned:** builder-test
  - **Depends:** S016
  - **Parallel:** false

- [ ] S023: Write unauthenticated redirect instrumented test
  **`androidTest/MainActivityTest.kt`** (new file or extend existing, AndroidX Test):
  - Launch `MainActivity`; log in to reach a protected screen
  - Programmatically clear tokens via `TokenPreferences.clearTokens()`
  - Assert `MainActivity` navigates to the Login screen
  - Assert the back stack does not contain any protected screen (back press from Login exits the app)
  Covers FA-033.
  - **Assigned:** builder-test
  - **Depends:** S020
  - **Parallel:** false

- [ ] S024: Extend `LoginScreenTest.kt` with loading and error state assertions
  Locate existing `LoginScreenTest.kt`; add two test cases:
  - `loginScreen_loadingState_showsProgressIndicator`: set `AuthUiState.Loading`; assert `onNodeWithTag("loading_indicator")` or `CircularProgressIndicator` semantic is present
  - `loginScreen_errorState_showsErrorMessage`: set `AuthUiState.Error("Invalid credentials")`; assert error text node is present
  Covers FA-035, FA-036.
  - **Assigned:** builder-test
  - **Depends:** none
  - **Parallel:** true

- [ ] S025: Extend `QuestDetailScreenTest.kt` with real ViewModel state machine assertions
  Locate existing `QuestDetailScreenTest.kt`; add or replace stub ViewModel with a real `QuestDetailViewModel` backed by in-memory Room:
  - `questDetail_notStarted_showsStartAction`: insert a `not_started` quest; assert "Start" action is rendered
  - `questDetail_notStarted_doesNotShowCompleteAction`: assert "Complete" action is NOT rendered for `not_started` quest
  Covers FA-037.
  - **Assigned:** builder-test
  - **Depends:** none
  - **Parallel:** true

🏁 MILESTONE M5: All post-review tests written — run unit test suite (`./gradlew :app:test`) to verify FA-026 through FA-037; instrumented tests (S022, S023) require emulator

---

### Phase 7: Validation

- [ ] S026: Full drift check against spec assertions
  Read-only inspection of all changed and created files. Verify:
  - All 22 testable assertions FA-020 through FA-041 are addressed by production code or tests
  - No `TODO` or `FIXME` stubs remain in any created file
  - All `StateFlow` exposed as read-only (no public `MutableStateFlow`)
  - All `viewModelScope.launch` blocks in new ViewModels catch `CancellationException` and rethrow
  - `Clock.System.now().toString()` used for all timestamps — no `java.time` usage in new/modified files
  - `SYNC_DEBOUNCE_WINDOW_MS` is a named constant, not a magic number
  - `FocusTimerService` manifest declaration includes `foregroundServiceType="specialUse"`
  - **Assigned:** validator
  - **Depends:** S004-T, S007, S008-T, S009, S010-T, S013, S016, S017, S018, S019, S020, S021, S022, S023, S024, S025
  - **Parallel:** false

🏁 MILESTONE FINAL: Feature 010 complete — all assertions addressed, all tests written, no stubs remaining

---

### Phase 8: Post-Review Fixes (from PR#11 review)

- [ ] S027: Refactor `InitiativeDetailViewModel` and `EpicDetailViewModel` to expose `StateFlow<UiState<T>>`
  Replace `StateFlow<Entity?>` with `StateFlow<UiState<Entity>>` in both ViewModels. Apply `.map { UiState.Success(it) }.catch { emit(UiState.Error(...)) }` before `stateIn`. Update corresponding screens to handle `Loading`, `Success`, and `Error` states. `null` from DAO currently conflates Loading and NotFound — this makes deleted-record hang impossible.
  - **Relates to:** P11-013, kotlin-android.md UiState convention, Spec US-01

- [ ] S027-T: Unit test `InitiativeDetailViewModel` and `EpicDetailViewModel` UiState transitions
  Use in-memory Room. Test: `Loading` on init, `Success` when entity present, `Error` on DAO exception, and graceful behavior when id is empty string (not-found path).
  - **Relates to:** P11-013

- [ ] S028-T: Write Robolectric unit tests for `FocusTimerService`
  Use `ServiceController` to drive: `onStartCommand` calls `startForeground`; `tickRunnable` reschedules every 1s via `Handler.postDelayed`; `onTimerFinished` posts completion notification then `stopSelf()`; `onDestroy` removes callbacks.
  - **Relates to:** P11-014, Spec US-04, US-05

- [ ] S029-T: Write Compose UI test for `BarcodeScannerScreen` permission-denied branch
  Use `createComposeRule`, seed `permissionDenied = true`. Assert `AlertDialog` with "Camera permission required" title is shown. Covers FA-038/FA-039 permission gating.
  - **Relates to:** P11-015, Spec US-03

- [ ] S030-T: Extend `AltairPowerSyncConnectorTest` with PATCH and multi-entry batch coverage
  Add: (1) two-entry PUT batch (verifies loop iterates past first), (2) mixed PUT+DELETE batch, (3) `UpdateType.PATCH` entry — verify no silent fall-through. Also add `getCredentials()` test.
  - **Relates to:** P11-016

- [ ] S031-T: Extend `AuthViewModelTest` with logout and register-success paths
  Add: `logout_clearsTokens` (verify `clearTokens()` called); `register_success_setsAuthenticated` (verify UiState transitions to authenticated). Both are user-facing flows with direct auth impact.
  - **Relates to:** P11-022

---

## Acceptance Criteria
- [ ] FA-020 through FA-041: all testable assertions addressed
- [ ] `./gradlew :app:test` passes (unit + Robolectric tests)
- [ ] No `TODO`/`FIXME` stubs in any new or modified file
- [ ] `Clock.System.now()` used for all timestamps; no `java.time` usage
- [ ] `SYNC_DEBOUNCE_WINDOW_MS` named constant present in `SyncCoordinator.kt`
- [ ] `FocusTimerService` declared in manifest with `foregroundServiceType="specialUse"`
- [ ] Instrumented tests (FA-033, FA-038, FA-039, FA-040, FA-041) documented for manual/emulator verification

## Validation Commands
```bash
# Unit + Robolectric tests
./gradlew :app:test

# Instrumented tests (requires connected emulator/device)
./gradlew :app:connectedAndroidTest

# Compile check only
./gradlew :app:assembleDebug
```
