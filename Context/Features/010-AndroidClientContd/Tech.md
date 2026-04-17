# Tech Plan: Android Client — Continued

**Spec:** Context/Features/010-AndroidClientContd/Spec.md
**Stacks involved:** Kotlin / Jetpack Compose / MVVM / Koin / Room / PowerSync / WorkManager / CameraX / ML Kit

---

## Architecture Overview

Feature 010 extends the existing Android client (`apps/android/`) by filling six unimplemented areas from Feature 009. No new architectural layers are introduced — all work follows the established Screen → ViewModel → DAO/PowerSync pattern already present in the codebase. New dependencies (CameraX, ML Kit) integrate at the composable layer. The foreground service is the only component that operates outside the Compose lifecycle; it is a standard Android Service and does not require new architectural patterns.

The nine missing test classes are pure additions — no production code changes are needed to enable them, only the test files themselves.

---

## Key Decisions

### Decision 1: EpicDao Injection Scope (OQ-001)

**Options considered:**
- Option A: Add `EpicDao` to `GuidanceViewModel` — 5-arg constructor, keeps all guidance data in one VM
- Option B: Dedicated `InitiativeDetailViewModel(initiativeId, initiativeDao, epicDao, db)` and `EpicDetailViewModel(epicId, epicDao, questDao, db)` — matches the existing `QuestDetailViewModel(SavedStateHandle, ...)` pattern
- Option C: `GuidanceRepository` abstraction — reduces constructor arity but introduces a new layer not used elsewhere in the project

**Chosen:** Option B — dedicated detail ViewModels with `SavedStateHandle`

**Rationale:** `QuestDetailViewModel` already follows this pattern (reads ID from `SavedStateHandle`). Option A would exceed the project's implicit limit of 4 DAO args and couples data for the list screen with data for two different detail screens. Option C adds a layer that no other guidance domain component uses, which violates the simplicity-first rule for single-use abstractions.

**Related ADRs:** ADR-022 (Kotlin/Koin DI approach)

---

### Decision 2: Initiative/Epic Navigation Routes

**Options considered:**
- Option A: Add routes inside the existing `today_graph` nested graph, consistent with how `QuestDetail` (`quest/{id}`) already lives in `today_graph`
- Option B: Top-level routes outside any graph

**Chosen:** Option A — routes added inside `today_graph`

**Rationale:** `QuestDetail` and `DailyCheckin` are already nested inside `today_graph`. Initiative and epic detail are reached by drilling from the Today tab, so placing their routes there preserves the expected back-stack behavior (back from Epic Detail → Initiative Detail → Initiative list → Today tab) without a tab reset.

**Files to change:**
- `navigation/Screen.kt` — add `InitiativeDetail("today_graph/guidance/initiatives/{id}")` and `EpicDetail("today_graph/guidance/initiatives/{initiativeId}/epics/{id}")` objects
- `navigation/NavGraph.kt` — add composable destinations for both routes inside `today_graph`

---

### Decision 3: Sync Status Indicator

**Options considered:**
- Option A: Observe `PowerSyncDatabase.currentStatus.asFlow()` directly inside `MainScaffold` composable
- Option B: `SyncStatusViewModel` that wraps the flow and exposes `isPending: StateFlow<Boolean>` — obtained via `koinViewModel()` in `MainScaffold`

**Chosen:** Option B — dedicated `SyncStatusViewModel`

**Rationale:** Option A couples network state observation to the composable function and makes the logic untestable. Option B follows the project's ViewModel-owns-state pattern and can be unit-tested with a mock `PowerSyncDatabase`.

**Implementation notes:**
- `isPending` = `uploading > 0 || !hasSynced`
- Icon: `Icons.Default.CloudOff` (or `SyncProblem`) styled with `WeatheredSlate` (`#8a9ea2`) tint
- Icon placed in `TopAppBar` — `MainScaffold` must be updated to include a `TopAppBar`; current version has no `topBar` slot populated

---

### Decision 4: Sync Debounce (OQ-002)

**Options considered:**
- Option A: In-memory `lastSyncTime: Long` in `SyncCoordinator` with named constant
- Option B: Persist `lastSyncTime` in `DataStore`

**Chosen:** Option A — in-memory

**Rationale:** WorkManager's `ExistingWorkPolicy.REPLACE` already deduplicates concurrent enqueue calls. The 5-minute debounce window only matters within a single process lifetime (rapid WiFi handoffs). On cold start, a sync is desirable, not harmful. Persisting to `DataStore` adds async complexity and a new dependency with no observable user benefit.

**Constant:** `SYNC_DEBOUNCE_WINDOW_MS = 5 * 60 * 1_000L` declared at top of `SyncCoordinator.kt`

---

### Decision 5: Barcode Scanner — CameraX + ML Kit (OQ-003)

**Options considered for ML Kit variant:**
- Bundled (`com.google.mlkit:barcode-scanning:17.3.0`) — model ships with APK; works offline at install with no Play Services requirement
- Unbundled (`com.google.android.gms:play-services-mlkit-barcode-scanning`) — smaller APK, but requires Google Play Services and may download the model on first use

**Chosen:** Bundled `com.google.mlkit:barcode-scanning:17.3.0`

**Rationale:** Spec requires offline reliability (FA-038, FA-039). The bundled variant guarantees the model is present at install. APK size increase (~3 MB) is acceptable per spec; the platform spec does not impose an APK size budget.

**Architecture:**
- `BarcodeScannerScreen` composable: full-screen `AndroidView` wrapping `PreviewView` (CameraX) + `Canvas` overlay for the rounded-corner scan frame in Deep Muted Teal-Navy (`#446273`)
- `ImageAnalysis` use case feeds each frame to `BarcodeScanning.getClient()` analyzer
- On successful decode: coroutine lookup against `TrackingItemDao.findByBarcode(barcode: String)` — if found, navigate to update-quantity flow; if not found, navigate to item creation form with `barcode` pre-populated
- Camera permission: use `rememberLauncherForActivityResult(RequestPermission)` in `BarcodeScannerScreen`; show rationale dialog if permission denied
- Camera lifecycle owner: `LocalLifecycleOwner.current`

**New dependencies (all added to `libs.versions.toml`):**
```
camerax = "1.4.2"
mlkitBarcode = "17.3.0"
lifecycleService = "2.10.0"  # for LifecycleService

camerax-core = { group = "androidx.camera", name = "camera-core", version.ref = "camerax" }
camerax-camera2 = { group = "androidx.camera", name = "camera-camera2", version.ref = "camerax" }
camerax-lifecycle = { group = "androidx.camera", name = "camera-lifecycle", version.ref = "camerax" }
camerax-view = { group = "androidx.camera", name = "camera-view", version.ref = "camerax" }
mlkit-barcode-scanning = { group = "com.google.mlkit", name = "barcode-scanning", version.ref = "mlkitBarcode" }
lifecycle-service = { group = "androidx.lifecycle", name = "lifecycle-service", version.ref = "lifecycleService" }
```

---

### Decision 6: Focus Timer Foreground Service

**Service class:** `FocusTimerService : LifecycleService`

**Foreground service type (API 34+):**
Android 14 (API 34) requires `android:foregroundServiceType` to be declared in the manifest for all foreground services. A productivity countdown timer does not qualify as `mediaPlayback`, `location`, `camera`, `microphone`, `dataSync`, or `connectedDevice`. The correct type is `specialUse`, which permits arbitrary background work and is appropriate for use cases that don't fit predefined categories. `shortService` is explicitly excluded — it caps at ~3 minutes and would terminate mid-session.

Manifest declaration:
```xml
<service
    android:name=".service.FocusTimerService"
    android:foregroundServiceType="specialUse"
    android:description="@string/focus_timer_service_description"
    android:exported="false" />
<uses-permission android:name="android.permission.FOREGROUND_SERVICE" />
<uses-permission android:name="android.permission.FOREGROUND_SERVICE_SPECIAL_USE" />
```

**Timer design — single source of truth:**
Store `endTimeEpochMs: Long` once when the session starts (passed via `Intent` extra). Both `FocusSessionViewModel` and `FocusTimerService` derive remaining time by computing `endTimeEpochMs - System.currentTimeMillis()` independently. This eliminates timer drift between the ViewModel and service, and prevents the double-`recordSession()` risk that would arise from two synchronized `CountDownTimer` instances.

**Service lifecycle:**
- Tied to `ProcessLifecycleOwner` — started when the app transitions to background during an active focus session, stopped when the app returns to foreground or the session ends
- `FocusSessionScreen` registers a `LifecycleEventObserver` on `ProcessLifecycleOwner`; on `ON_STOP`, starts the service with `endTimeEpochMs`; on `ON_START`, stops the service
- Service calls `startForeground()` with a persistent non-dismissable notification; updates the notification every second via `Handler(Looper.getMainLooper()).postDelayed`
- On `CountDownTimer.onFinish()`: cancel foreground notification, post one-shot local completion notification, stop self

**POST_NOTIFICATIONS permission (API 33+):**
- Request `POST_NOTIFICATIONS` at runtime before starting the service on API 33+
- If denied: service does not start, focus session continues without notification (no crash)
- On API 31–32: no runtime permission needed; foreground notification posts unconditionally

**Notification channel:** Create `FOCUS_TIMER_CHANNEL` in `AltairApplication.onCreate()` with `IMPORTANCE_LOW` (silent, no sound)

---

## Stack-Specific Details

### Kotlin / Jetpack Compose

**Files to create:**
- `ui/guidance/InitiativeDetailScreen.kt`
- `ui/guidance/EpicDetailScreen.kt`
- `ui/guidance/InitiativeDetailViewModel.kt`
- `ui/guidance/EpicDetailViewModel.kt`
- `ui/sync/SyncStatusViewModel.kt`
- `ui/tracking/BarcodeScannerScreen.kt`
- `service/FocusTimerService.kt`

**Files to modify:**
- `navigation/Screen.kt` — add `InitiativeDetail`, `EpicDetail`
- `navigation/NavGraph.kt` — add composable destinations; pass `initiativeId` to `EpicDetail` route
- `ui/MainScaffold.kt` — add `TopAppBar` with `SyncStatusViewModel` icon
- `data/sync/SyncCoordinator.kt` — add `lastSyncTime`, `SYNC_DEBOUNCE_WINDOW_MS`, debounce guard in `enqueueExpedited()`
- `di/ViewModelModule.kt` — add `InitiativeDetailViewModel`, `EpicDetailViewModel`, `SyncStatusViewModel` definitions
- `AndroidManifest.xml` — declare `FocusTimerService`, `FOREGROUND_SERVICE`, `FOREGROUND_SERVICE_SPECIAL_USE` permissions
- `AltairApplication.kt` — create notification channel
- `apps/android/gradle/libs.versions.toml` — add CameraX, ML Kit, lifecycle-service

**Also fix (identified during reading):**
- `GuidanceViewModel.kt` line 85: replace `java.time.LocalDateTime.now()` with `Clock.System.now().toString()` — violates `kotlin-android.md` rule on timestamp generation

### Test Infrastructure

| Test class | Framework | Notes |
|---|---|---|
| `AuthViewModelTest` | JUnit 5 + Turbine + MockK | Unit — mock `AuthRepository`, `TokenPreferences` |
| `AltairPowerSyncConnectorTest` | JUnit 5 + MockK | Unit — mock `SyncApi`, `UploadCrudEntryBatch` |
| `FocusSessionScreenTest` | AndroidX Test (instrumented) | Instrumented — state transition + `recordSession()` |
| `SyncCoordinatorTest` | JUnit 5 + MockK | Unit — mock `WorkManager`, `PowerSyncDatabase` |
| `SyncWorkerTest` | JUnit 5 + MockK | Unit — mock `SyncCoordinator`, inject via Koin |
| `TokenPreferencesTest` | JUnit 5 | Unit — fake `SharedPreferences` |
| Unauthenticated redirect | AndroidX Test (instrumented) | Instrumented — clear tokens, assert `MainActivity` nav state |
| `TodayViewModelTest` | Robolectric + in-memory Room | Robolectric already in `libs.versions.toml` (4.14.1) |
| `LoginScreenTest` (extend) | Compose UI test | Extend existing file — add Loading and Error state cases |
| `QuestDetailScreenTest` (extend) | Compose UI test | Extend existing file — use real `QuestDetailViewModel` |

Robolectric 4.14.1 is already in the version catalog. No new test framework additions required.

---

## Integration Points

**BarcodeScannerScreen → TrackingItemDao:** `TrackingItemDao` must expose `suspend fun findByBarcode(barcode: String): TrackingItemEntity?`. Check whether the `barcode` column exists on `TrackingItemEntity`; add it if missing (new Room migration required).

**FocusTimerService ↔ FocusSessionScreen:** Communication via `Intent` extras only — `endTimeEpochMs: Long` passed on start. No `Binder` binding required; the ViewModel already owns session state.

**SyncStatusViewModel → MainScaffold:** `koinViewModel<SyncStatusViewModel>()` called inside `MainScaffold`. Requires `SyncStatusViewModel` registered in `ViewModelModule`.

---

## Risks & Unknowns

- **Risk:** `TrackingItemEntity` may not have a `barcode` column.
  - **Mitigation:** Check `TrackingItemEntity.kt` before implementing `BarcodeScannerScreen`. If absent, add a nullable `barcode: String?` column and a Room migration.

- **Risk:** `specialUse` foreground service type may require Google Play justification for production app listing.
  - **Mitigation:** This is a development-phase concern only. Document the justification string in `@string/focus_timer_service_description`. For now, any string describing "countdown timer for focus sessions" satisfies the manifest requirement.

- **Risk:** CameraX `ImageAnalysis` on older/lower-end devices may not meet the 2-second decode target.
  - **Mitigation:** ML Kit barcode scanning on the bundled model is fast enough on API 26+ hardware in practice. The 2-second target from the platform spec is achievable; no special optimization needed beyond default `ImageAnalysis` configuration.

- **Unknown:** CameraX stable version — `1.4.2` is used above; verify this is still current stable at implementation time (Step S001).

---

## Testing Strategy

- Unit tests (JUnit 5 + Turbine + MockK): `AuthViewModelTest`, `AltairPowerSyncConnectorTest`, `SyncCoordinatorTest`, `SyncWorkerTest`, `TokenPreferencesTest`, `SyncStatusViewModel` — no Android context needed
- Robolectric: `TodayViewModelTest` — in-memory Room database, no emulator needed
- Compose UI tests: `LoginScreenTest` extensions, `QuestDetailScreenTest` extensions — use `createComposeRule()`
- Instrumented: `FocusSessionScreenTest`, unauthenticated redirect test — require emulator or device
- Manual: FA-020 through FA-025 (navigation back stack, sync indicator visibility), FA-040, FA-041 (foreground service notification on background, API 33 permission flow)
