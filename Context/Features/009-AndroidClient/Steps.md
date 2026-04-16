# Implementation Steps: Android Client

**Spec:** Context/Features/009-AndroidClient/Spec.md
**Tech:** Context/Features/009-AndroidClient/Tech.md

## Progress
- **Status:** Not Started
- **Current task:** —
- **Last milestone:** —

## Team Orchestration

### Team Members

- **builder-android**
  - Role: Foundation, data layer, navigation shell, auth flow, app-level infrastructure
  - Agent Type: mobile-specialist
  - Resume: false

- **builder-guidance**
  - Role: Guidance domain repositories, ViewModels, and all guidance + today screens
  - Agent Type: mobile-specialist
  - Resume: false

- **builder-knowledge**
  - Role: Knowledge domain repositories, ViewModels, and all knowledge screens
  - Agent Type: mobile-specialist
  - Resume: false

- **builder-tracking**
  - Role: Tracking domain repositories, ViewModels, and all tracking screens
  - Agent Type: mobile-specialist
  - Resume: false

- **builder-tests**
  - Role: Unit tests, ViewModel tests (Turbine), Compose UI tests, Koin module tests
  - Agent Type: mobile-specialist
  - Resume: false

- **validator**
  - Role: Read-only quality and integration validation
  - Agent Type: quality-engineer
  - Resume: false

---

## Tasks

### Phase 1: Foundation

- [ ] S001: Add all missing dependencies to `apps/android/gradle/libs.versions.toml` and `apps/android/app/build.gradle.kts`
  - Add version entries and library aliases for: Room 2.7.1, PowerSync (`com.powersync:core` + `com.powersync:integration-room`) v1.11.2, `androidx.sqlite:sqlite-bundled`, DataStore / security-crypto, WorkManager 2.10.1, OkHttp 4.12.0, Retrofit 2.11.0 + `converter-kotlinx-serialization`, kotlinx.serialization 1.8.0, kotlinx-datetime 0.6.0.
  - Add KSP plugin entry (for Room annotation processor): `com.google.devtools.ksp` version `2.3.20-2.0.21`.
  - Apply `kotlinx-serialization` and `ksp` Gradle plugins in `build.gradle.kts`.
  - Add `INTERNET` and `ACCESS_NETWORK_STATE` permissions to `AndroidManifest.xml`.
  - **IMPORTANT:** After adding, run `./gradlew :app:dependencies` to confirm all PowerSync artifacts resolve from Maven Central before proceeding. If `com.powersync:integration-room` does not resolve, verify the exact artifact ID from https://central.sonatype.com/artifact/com.powersync/integration-room and update accordingly. This verification blocks all Phase 2 tasks.
  - **Assigned:** builder-android
  - **Depends:** none
  - **Parallel:** false

- [ ] S002: Update design system — `Color.kt`, `Theme.kt`, `Type.kt`
  - **Color.kt:** Remove all default purple/pink tokens. Define `val`s for every Ethereal Canvas colour token from `docs/specs/09-PLAT-001-android.md` §Design Tokens (Pale Seafoam Mist, Midnight Charcoal, Deep Muted Teal-Navy, Sophisticated Terracotta, Weathered Slate, Ghost Border Ash, Soft Slate Haze, etc.). Build `LightColorScheme` and `DarkColorScheme` using `materialColorScheme()` with these values. No `Color(0xFF000000)` anywhere.
  - **Theme.kt:** Set `dynamicColor = false`. Apply `LightColorScheme` / `DarkColorScheme`. Export `AltairTheme` composable.
  - **Type.kt:** Bundle Manrope and Plus Jakarta Sans font files in `res/font/` (download from Google Fonts OFL release). Create `FontFamily` instances for each. Map `Typography.displayLarge` and `Typography.headlineLarge` to Manrope; map `Typography.bodyLarge`, `Typography.labelMedium`, `Typography.bodyMedium`, `Typography.labelSmall` to Plus Jakarta Sans.
  - **Assigned:** builder-android
  - **Depends:** S001
  - **Parallel:** false

- [ ] S002-T: Design system spot-check — maps to FA-016
  - Verify no `Color.Black` / `0xFF000000` appears in `Color.kt` or `Theme.kt`.
  - Verify `displayLarge` and `headlineLarge` use Manrope `FontFamily`.
  - Verify `LightColorScheme` has no pure black in any role.
  - **Assigned:** builder-android
  - **Depends:** S002
  - **Parallel:** false

---

### Phase 2: Data Layer

- [ ] S003: Define all 20 Room entities in `data/local/entity/`
  - Create one `@Entity` data class per table. Column names must exactly match the PostgreSQL snake_case column names (invariant D-4). All UUIDs as `String`, all timestamps as `String?` (ISO-8601), all booleans as `Int` (0/1), nullable fields as `String?`.
  - Tables (entity class → table name): `UserEntity(users)`, `HouseholdEntity(households)`, `HouseholdMembershipEntity(household_memberships)`, `InitiativeEntity(initiatives)`, `EpicEntity(epics)`, `QuestEntity(quests)` [include `description`, `routine_id` — required by FA-013], `RoutineEntity(routines)`, `FocusSessionEntity(focus_sessions)`, `DailyCheckinEntity(daily_checkins)`, `NoteEntity(notes)`, `NoteSnapshotEntity(note_snapshots)`, `EntityRelationEntity(entity_relations)`, `TagEntity(tags)`, `EntityTagEntity(entity_tags)`, `TrackingItemEntity(tracking_items)`, `TrackingItemEventEntity(tracking_item_events)`, `TrackingLocationEntity(tracking_locations)`, `TrackingCategoryEntity(tracking_categories)`, `ShoppingListEntity(shopping_lists)`, `ShoppingListItemEntity(shopping_list_items)`.
  - `attachments` is NOT included — deferred to Step 10.
  - Reference `infra/migrations/` SQL files to verify each column name and nullability. Cross-check against `apps/web/src/lib/sync/schema.ts` for any column discrepancies.
  - **Assigned:** builder-android
  - **Depends:** S001
  - **Parallel:** true

- [ ] S003-T: Room entity column parity test — maps to FA-013
  - Write JUnit 5 tests that programmatically read the Room entity field annotations (`@ColumnInfo`) and assert they match the expected snake_case column names from the spec.
  - At minimum, assert `QuestEntity` defines columns: `title`, `description`, `status`, `priority`, `due_date`, `epic_id`, `initiative_id`, `routine_id`, `user_id`, `created_at`, `updated_at`, `deleted_at`.
  - **Assigned:** builder-tests
  - **Depends:** S003
  - **Parallel:** true

- [ ] S004: Define Room DAOs in `data/local/dao/`
  - One DAO interface per entity group: `UserDao`, `HouseholdDao`, `InitiativeDao`, `EpicDao`, `QuestDao`, `RoutineDao`, `FocusSessionDao`, `DailyCheckinDao`, `NoteDao`, `NoteSnapshotDao`, `EntityRelationDao`, `TagDao`, `TrackingItemDao`, `TrackingItemEventDao`, `TrackingLocationDao`, `TrackingCategoryDao`, `ShoppingListDao`, `ShoppingListItemDao`.
  - Each DAO must have: `Flow<List<T>> watchAll(userId)` (or `watchByHousehold` where appropriate), `Flow<T?> watchById(id)`, `suspend upsert(entity)` with `OnConflictStrategy.REPLACE`, `suspend delete(entity)`.
  - Note queries are filtered by `user_id` and `deleted_at IS NULL` unless the domain logic requires otherwise.
  - Backlinks query in `EntityRelationDao`: `watchBacklinksForNote(targetId)` — SELECT from `entity_relations` WHERE `target_id = :targetId` AND `relation_type = 'note_link'`. This is the reactive query for FA-010.
  - **Assigned:** builder-android
  - **Depends:** S003
  - **Parallel:** true

- [ ] S004-T: Room DAO in-memory database tests
  - Use `Room.inMemoryDatabaseBuilder` with KSP-generated schema.
  - Test `QuestDao.watchAll()` returns inserted quests via Turbine.
  - Test `EntityRelationDao.watchBacklinksForNote()` returns the expected backlink row.
  - Test `TrackingItemDao` upsert + watchById round-trip.
  - **Assigned:** builder-tests
  - **Depends:** S004
  - **Parallel:** true

- [ ] S005: Implement PowerSync data layer
  - Create `AltairDatabase` (`@Database` class) listing all 20 entities, version 1. Use `RoomConnectionPool.buildSupportSQLiteOpenHelper()` as the Room `openHelperFactory` so PowerSync's internal views do not fail Room schema validation.
  - Create `AltairPowerSyncConnector` implementing `PowerSyncBackendConnector`: `fetchCredentials()` calls `GET /api/auth/powersync-token` via the Retrofit `AuthApi`; `uploadData()` iterates the CRUD batch, calls the REST API per `UpdateType`, calls `batch.complete()` only when all entries succeed, rethrows on failure.
  - Define the PowerSync schema (column definitions mirroring Room entities) — required by the SDK for CRUD tracking. This schema is separate from the Room entity definitions.
  - Create `SyncCoordinator`: `startSync()` calls `powerSyncDatabase.connect(connector)` and subscribes to the five streams (`user_data`, `household`, `guidance`, `knowledge`, `tracking`); `stopSync()` calls `powerSyncDatabase.disconnect()`.
  - **Assigned:** builder-android
  - **Depends:** S003, S004
  - **Parallel:** false

- [ ] S006: Implement Koin DI modules in `di/`
  - `DatabaseModule`: `AltairDatabase` singleton (Room + RoomConnectionPool), all DAOs derived from it.
  - `SyncModule`: `PowerSyncDatabase` (from RoomConnectionPool), `AltairPowerSyncConnector`, `SyncCoordinator`.
  - `PreferencesModule`: `EncryptedSharedPreferences` instance (Jetpack Security Crypto); expose `TokenPreferences` wrapper.
  - `NetworkModule`: `OkHttpClient` singleton with `AuthInterceptor` and `AuthAuthenticator`; `Retrofit` instance with `kotlinx.serialization` converter; `AuthApi` and `SyncApi` service interfaces.
  - `RepositoryModule`: all repository implementations (`QuestRepositoryImpl`, `NoteRepositoryImpl`, `TrackingItemRepositoryImpl`, etc.), each taking a DAO + `PowerSyncDatabase` from Koin.
  - `ViewModelModule`: all ViewModels declared with `viewModelOf { }`.
  - Register all modules in `AltairApplication.onCreate()` via `startKoin { modules(...) }`.
  - **Assigned:** builder-android
  - **Depends:** S005
  - **Parallel:** false

- [ ] S006-T: Koin `checkModules()` test — maps to FA-014
  - Create `KoinModulesTest` in the test source set.
  - Call `checkModules { modules(databaseModule, syncModule, preferencesModule, networkModule, repositoryModule, viewModelModule) }`.
  - Test must pass with zero missing or circular dependency errors.
  - **Assigned:** builder-tests
  - **Depends:** S006
  - **Parallel:** false

- [ ] S007: Implement auth layer
  - `AuthApi` (Retrofit): `POST /api/auth/login`, `POST /api/auth/register`, `POST /api/auth/refresh`.
  - `TokenPreferences`: wrapper around `EncryptedSharedPreferences` exposing `accessToken`, `refreshToken` as nullable `String` properties; `clearTokens()` method.
  - `AuthInterceptor`: attaches `Authorization: Bearer <accessToken>` to every request when token is present.
  - `AuthAuthenticator`: on 401, calls `authApi.refresh(refreshToken)` synchronously (runBlocking), stores new tokens, retries request. Returns null and calls `prefs.clearTokens()` if refresh fails (triggers re-login).
  - `AuthRepository`: `login()`, `register()`, `logout()` (clears tokens + stops sync).
  - Annotate `contracts/Dtos.kt` data classes with `@Serializable` from kotlinx.serialization (required for Retrofit's `converter-kotlinx-serialization`).
  - **Assigned:** builder-android
  - **Depends:** S006
  - **Parallel:** false

---

### Milestone 1 — Data Layer Complete

**Gate: all of the following must hold before Phase 3 begins**
- S006-T Koin `checkModules()` passes (FA-014)
- S003-T column parity test passes (FA-013)
- S004-T Room DAO in-memory tests pass
- `./gradlew :app:assembleDebug` succeeds with no errors (FA-015 prerequisite)

---

### Phase 3: Navigation Shell and Auth Screens

- [ ] S008: Implement Navigation Compose graph + `MainScaffold`
  - Create `NavGraph.kt` with the full nav graph (see Tech.md Navigation Architecture section).
  - `MainScaffold`: `Scaffold` with `NavigationBar` (4 items: Today, Knowledge, Tracking, Settings). Each tab has its own nested `NavHost` sub-graph.
  - Deep link declarations: `altair://quest/{id}` → quest detail, `altair://item/{id}` → item detail, `altair://checkin` → daily check-in.
  - `MainActivity` sets `NavHost` as content; checks `TokenPreferences` on startup to determine start destination (auth graph if no token, main graph if token present).
  - **Assigned:** builder-android
  - **Depends:** S007
  - **Parallel:** false

- [ ] S009: Implement Login and Register screens
  - `LoginScreen`: email + password inputs on Pale Seafoam Mist background, pill-shaped CTA (`RoundedCornerShape(50)`), Material 3 filled text fields (`surfaceContainerLow`), no outline border, error state for invalid credentials. Calls `AuthViewModel.login()`.
  - `RegisterScreen`: email, password, display name inputs. Same styling conventions.
  - `AuthViewModel`: `UiState` sealed class (`Loading`, `Success`, `Error`). Calls `AuthRepository`; on success stores tokens via `TokenPreferences`, triggers initial PowerSync stream subscription.
  - **Assigned:** builder-android
  - **Depends:** S008
  - **Parallel:** false

- [ ] S010: App startup sequence — PowerSync init and session guard
  - In `AltairApplication.onCreate()`: initialize Koin, start WorkManager periodic sync registration.
  - In `MainActivity.onCreate()`: after Koin init, check `TokenPreferences.accessToken`; if present, call `SyncCoordinator.startSync()` before composing the main graph.
  - Reactive auth guard: `AuthViewModel` exposes `isAuthenticated: StateFlow<Boolean>`. `NavGraph` observes this and redirects to auth graph when token is cleared (logout, 401-refresh failure).
  - **Assigned:** builder-android
  - **Depends:** S009
  - **Parallel:** false

---

### Milestone 2 — Auth + Navigation Working

**Gate: all of the following must hold before Phase 4 begins**
- FA-001 (DataStore token persistence after login): manually verifiable via ADB DataStore dump
- FA-012 (logout clears tokens, redirect to login): verifiable via `TokenPreferences.accessToken == null` post-logout
- FA-015: `./gradlew :app:assembleDebug` targets API 31, no crash on launch in emulator
- FA-019 prerequisite: `AuthAuthenticator` and `AuthInterceptor` implementations exist and compile (S007)
- FA-017 deep link routing: manually verifiable via ADB deep link intent

---

### Phase 4: Domain Screens (Parallel)

Phase 4 tasks can be assigned and executed in parallel across builder-guidance, builder-knowledge, and builder-tracking. Each builder works within their own package (`guidance/`, `knowledge/`, `tracking/`). Navigation destinations are pre-declared in S008; builders add Composables to the already-registered routes.

- [ ] S011: Today view (`TodayGraph` entry screen)
  - `TodayScreen`: Manrope Display greeting (`Typography.displayLarge`) with `user.displayName`.
  - Daily check-in card: shown if today's `daily_checkins` row is absent or incomplete for the current user. Energy (1–5) + mood (1–5) selectors. Submits via `PowerSyncDatabase.execute()`.
  - Due routines section: `Flow` from `RoutineDao.watchDueToday(userId)`.
  - Today's quests list: `Flow` from `QuestDao.watchTodayQuests(userId)` (due_date = today OR due_date IS NULL, status NOT IN (`completed`, `cancelled`, `deferred`)).
  - Quest cards: `ElevatedCard` with `RoundedCornerShape(16.dp)`, no explicit elevation shadow. Each card shows title, status badge, priority indicator.
  - Swipe-right on `in_progress` quest card → complete (`in_progress → completed` only, per state machine; cards in other states do not trigger completion). 300ms `cubic-bezier(0.4, 0, 0.2, 1)` animation. Cards in `not_started` show a "Start" button (tapping transitions to `in_progress` then recomposes).
  - Quick action FAB: "New Quest" navigates to quest creation; "New Note" navigates to note capture.
  - "All Guidance" entry: chip or button navigating to `TodayGraph/guidance` (initiative list + quest list + routines).
  - Empty state: "Nothing on the horizon" copy with create quest CTA.
  - **Assigned:** builder-guidance
  - **Depends:** S010
  - **Parallel:** true

- [ ] S012: Guidance screens
  - Quest list screen: filterable `LazyColumn` by initiative, status, priority. Data from `QuestDao.watchAll(userId)`. Navigates to quest detail.
  - Quest detail screen: title, `description`, status (only valid transitions shown — `not_started` → Start, `in_progress` → Complete/Defer/Cancel; no direct `not_started → completed`), priority, epic/initiative breadcrumb, tags, focus session history. All status writes via `PowerSyncDatabase.execute(UPDATE quests SET status=... WHERE id=...)`.
  - Focus session screen (`/today/guidance/quests/{id}/focus`): full-screen `CountDownTimer`-backed timer (default 25 min). Manrope Display timer text. Signature gradient progress ring (arc drawn via `Canvas`). Background dims to Soft Slate Haze (`#cfddde → Color(0xFFCFDDDE)`). End button. On finish: creates `focus_sessions` row via `PowerSyncDatabase.execute()`. Must function offline.
  - Initiative list → initiative detail → epic list → epic detail → child quests (breadcrumb navigation within `TodayGraph`).
  - Routine list: frequency badges (daily/weekly/monthly). Mark-done action.
  - Daily check-in form (reachable from Today card and from routines/guidance area).
  - **Assigned:** builder-guidance
  - **Depends:** S010
  - **Parallel:** true

- [ ] S013: Knowledge screens
  - Quick note capture: minimal `TextField` accessible from Today FAB; expands to full editor.
  - Note list: `LazyColumn` sorted by `updated_at` DESC. Locally searchable via `NoteDao.watchSearch(query)` Room `LIKE` query.
  - Note detail: title, content `TextField` (no rich text), tags, backlinks section (from `EntityRelationDao.watchBacklinksForNote(noteId)` — shows notes that link TO this note, per invariant E-5), snapshot history list (view-only, from `NoteSnapshotDao.watchByNoteId(noteId)`).
  - Note linking: typing `[[` in the content field triggers an inline search `DropdownMenu` against `NoteDao.watchSearch(query)`. Selecting a result creates an `entity_relation` row via `PowerSyncDatabase.execute()` with `relation_type = "note_link"` (use `$lib/contracts` equivalent — `EntityRelationType.NOTE_LINK` from `contracts/Dtos.kt`).
  - **Assigned:** builder-knowledge
  - **Depends:** S010
  - **Parallel:** true

- [ ] S014: Tracking screens
  - Inventory list: search/filter bar, location + category `FilterChip` row, `LazyColumn` of item cards. Low-stock items show Sophisticated Terracotta (`Color(0xFF9F403D)`) quantity badge. Data from `TrackingItemDao.watchAll(householdId)`.
  - Item detail: name, quantity, location, category, `LazyColumn` event timeline from `TrackingItemEventDao.watchByItemId(itemId)`, tags.
  - Item creation form: name, quantity stepper, location dropdown, category dropdown, optional barcode text field. Creates item via `PowerSyncDatabase.execute()`.
  - Consumption logging: quantity selector. Validates that `consumptionAmount > 0` and `consumptionAmount <= currentQuantity` (invariant E-7). If invalid: surface error in UI, do NOT call `execute()`. If valid: insert `tracking_item_events` row and update `tracking_items.quantity`.
  - Location CRUD screen (household-scoped).
  - Category CRUD screen (household-scoped).
  - Shopping list: `LazyColumn` with pill-shaped checkboxes (`RoundedCornerShape(50)`). Completed items rendered at Ghost Border Ash opacity (`0.38f`). Add from inventory or freeform. Linked item quantity shown beside item name.
  - **Assigned:** builder-tracking
  - **Depends:** S010
  - **Parallel:** true

- [ ] S015: Settings screen
  - Display name and email edit fields. Save via `PowerSyncDatabase.execute(UPDATE users ...)` + `PATCH /api/auth/me` via Retrofit.
  - Logout button: calls `AuthRepository.logout()` (clears tokens, stops PowerSync, navigates to login).
  - **Assigned:** builder-android
  - **Depends:** S010
  - **Parallel:** true

---

### Milestone 3 — Domain Screens Complete

**Gate: all of the following must hold before Phase 5 begins**
- FA-004 (quest state transitions — only valid actions shown): verified by S020-T ViewModel test
- FA-005 (offline write → PowerSync outbox): manually verifiable by disabling network, creating quest, checking PowerSync pending count > 0
- FA-006 (swipe-to-complete): verified by S021-T Compose UI test
- FA-007 (CountDownTimer offline): verified by S020-T unit test
- FA-008 (consumption E-7 block): verified by S020-T unit test
- FA-010 (backlinks E-5): verified by S020-T ViewModel test
- FA-011 (Today-as-hub navigation): verified by S021-T Compose UI test

---

### Phase 5: Background Sync

- [ ] S016: WorkManager sync workers
  - `SyncWorker extends CoroutineWorker`: calls `SyncCoordinator.triggerSync()`. Returns `Result.success()` on completion, `Result.retry()` on transient failure.
  - Register periodic work in `AltairApplication.onCreate()`:
    ```kotlin
    WorkManager.getInstance(this).enqueueUniquePeriodicWork(
        "sync_periodic",
        ExistingPeriodicWorkPolicy.KEEP,
        PeriodicWorkRequestBuilder<SyncWorker>(15, TimeUnit.MINUTES)
            .setConstraints(Constraints(requiredNetworkType = NetworkType.CONNECTED))
            .addTag("sync_periodic")
            .build()
    )
    ```
  - `SyncCoordinator.enqueueExpedited()` enqueues a `OneTimeWorkRequest` tagged `"sync_expedited"` using `setExpedited(OutOfQuotaPolicy.RUN_AS_NON_EXPEDITED_WORK_REQUEST)`.
  - Register a `ConnectivityManager.NetworkCallback` in `AltairApplication`: `onAvailable()` calls `SyncCoordinator.enqueueExpedited()`.
  - After each local write in repository implementations, call `SyncCoordinator.enqueueExpedited()`.
  - **Assigned:** builder-android
  - **Depends:** S006
  - **Parallel:** false

- [ ] S017: Offline indicator
  - Observe `PowerSyncDatabase.currentStatus.asFlow()` in `TodayViewModel` (or a shared `SyncStatusViewModel`).
  - Show a subtle Weathered Slate (`Color(0xFF8A9FA0)` — verify exact token from `docs/specs/09-PLAT-001-android.md`) `Icon` in the `TopAppBar` when `status.hasSynced == false || status.uploading > 0`.
  - **Assigned:** builder-android
  - **Depends:** S016
  - **Parallel:** false

---

### Milestone 4 — Sync Complete

**Gate: all of the following must hold before Phase 6 begins**
- FA-002 (Room tables populated after initial sync): manually verifiable post-login on populated server
- FA-003 (WorkManager periodic job enqueued): `WorkManager.getWorkInfosByTag("sync_periodic")` returns at least one ENQUEUED/RUNNING job
- FA-018 (expedited sync enqueued after local write): observable via `WorkManager.getWorkInfosByTag("sync_expedited")` after offline write

---

### Phase 6: Testing

- [ ] S018-T: Koin `checkModules()` full test (also listed under S006-T — re-verify with all modules present)
  - Maps to FA-014.
  - **Assigned:** builder-tests
  - **Depends:** S016
  - **Parallel:** true

- [ ] S019-T: Room DAO in-memory database tests (extended)
  - Extend S004-T: add tests for `QuestDao` Flow emission on upsert (Turbine), `EntityRelationDao` backlinks query, `TrackingItemDao` consumption quantity invariant (DAO-level check).
  - Use `Room.inMemoryDatabaseBuilder` with all 20 entities.
  - **Assigned:** builder-tests
  - **Depends:** S016
  - **Parallel:** true

- [ ] S020-T: ViewModel and Use Case unit tests (JUnit 5 + Turbine)
  - `TodayViewModelTest`: today quests filtering, daily check-in state, swipe-complete only for `in_progress` quests (FA-006).
  - `QuestDetailViewModelTest`: status transition options — assert only valid transitions are exposed by ViewModel; assert `not_started → completed` is NOT presented (FA-004).
  - `FocusSessionViewModelTest`: `CountDownTimer` fires `onFinish()` after duration with `TestCoroutineDispatcher`; verify offline (no network mock) (FA-007).
  - `TrackingItemViewModelTest`: consumption validation — assert `execute()` is NOT called when `consumptionAmount > currentQuantity`; assert error state is set (FA-008).
  - `NoteDetailViewModelTest`: backlinks section populated from `EntityRelationDao.watchBacklinksForNote()` (FA-010).
  - `AuthAuthenticatorTest`: mock `AuthApi.refresh()` returning new tokens; assert request is retried with new Authorization header; assert no login-screen navigation (FA-019).
  - **Assigned:** builder-tests
  - **Depends:** S015
  - **Parallel:** true

- [ ] S021-T: Compose UI tests (semantics-based)
  - `LoginScreenTest`: fill email + password, click submit, assert navigation to Today screen (using mock `AuthRepository`).
  - `TodayScreenTest`: render with quests fixture; assert swipe-right on `in_progress` card triggers complete action; assert `not_started` card does not (FA-006); assert "All Guidance" navigation element is present (FA-011).
  - `QuestDetailScreenTest`: render with `not_started` quest; assert "Complete" action is NOT present; assert only "Start" is shown (FA-004).
  - `ItemCreationFormTest`: fill required fields; assert `PowerSyncDatabase.execute()` is called once on submit.
  - **Assigned:** builder-tests
  - **Depends:** S015
  - **Parallel:** true

- [ ] S022-T: Build and emulator smoke test — maps to FA-015
  - `./gradlew :app:assembleDebug` must complete with no errors targeting API 31.
  - Launch Today screen on API 31 emulator — no crash on launch.
  - **Assigned:** validator
  - **Depends:** S021-T
  - **Parallel:** false

---

### Milestone 5 — Tests Passing (Feature Complete)

**Gate: all of the following must hold**
- S018-T: Koin `checkModules()` passes (FA-014)
- S019-T: Room DAO tests pass
- S020-T: All ViewModel + Use Case tests pass
- S021-T: All Compose UI tests pass
- S022-T: `assembleDebug` succeeds, Today screen launches on API 31 emulator (FA-015)

---

### Phase 7: P1 Should Have (Build If Time Permits)

- [ ] S023: Barcode scanner (P1 — CameraX + ML Kit)
  - Full-screen camera overlay with CameraX `PreviewView`. ML Kit `BarcodeScanning` analyzer via `ImageAnalysis`.
  - Scan frame with Deep Muted Teal-Navy border.
  - On scan result: query `TrackingItemDao` for existing item by `barcode` column. If found: open update-quantity flow. If not found: navigate to item creation form with barcode pre-filled.
  - Lookup works offline against local Room data.
  - **Assigned:** builder-tracking
  - **Depends:** Milestone 5
  - **Parallel:** true

- [ ] S024: Focus timer foreground service notification (P1)
  - `FocusTimerService extends LifecycleService`: starts a foreground notification when a focus session screen is active and app is backgrounded.
  - Notification shows timer countdown. On `CountDownTimer.onFinish()`: fire a completion local notification via `NotificationManager`.
  - Requires `POST_NOTIFICATIONS` runtime permission request (Android 13+, API 33+; no-op on API 31–32).
  - **Assigned:** builder-guidance
  - **Depends:** Milestone 5
  - **Parallel:** true

---

### Documentation

- [ ] S025-D: Update planning artifacts on feature complete
  - Update this `Steps.md`: mark all completed tasks, set status to Complete, record final milestone date.
  - Add ADR-022, ADR-023 links to `Context/Decisions/` index if one exists.
  - **Assigned:** builder-android
  - **Depends:** Milestone 5
  - **Parallel:** false

---

## FA → Task Traceability

| FA | Assertion | Task |
|---|---|---|
| FA-001 | DataStore token persistence after login | S007, S009 |
| FA-002 | Room tables populated after initial sync | S005, S010 |
| FA-003 | WorkManager periodic job enqueued after startup | S016 |
| FA-004 | Quest detail shows only valid status transitions | S012, S020-T, S021-T |
| FA-005 | Offline write → PowerSync outbox; clears after reconnect | S005, S016 |
| FA-006 | Swipe-to-complete only for `in_progress` quests | S011, S020-T, S021-T |
| FA-007 | CountDownTimer fires `onFinish()` offline | S012, S020-T |
| FA-008 | Consumption E-7 block before any write | S014, S020-T |
| FA-009 | Shopping list offline write persists across restart | S014, S016 |
| FA-010 | Backlinks section E-5 from `entity_relations` | S013, S020-T |
| FA-011 | Today-as-hub navigation to all Guidance screens | S011, S021-T |
| FA-012 | Logout clears tokens, redirects to login | S007, S009 |
| FA-013 | `QuestEntity` defines all required columns in snake_case | S003, S003-T |
| FA-014 | Koin `checkModules()` passes | S006, S018-T |
| FA-015 | Builds on API 31, Today screen launches without crash | S001, S022-T |
| FA-016 | Design system: no `Color.Black`, correct shapes, Manrope headings | S002, S002-T |
| FA-017 | Deep link `altair://quest/{id}` opens quest detail | S008 |
| FA-018 | Expedited sync enqueued after local write | S016 |
| FA-019 | Auth interceptor refreshes token on 401 and retries | S007, S020-T |
