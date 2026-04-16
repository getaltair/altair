# PR Review: feat/android-client → main

**Date:** 2026-04-16
**Feature:** Context/Features/009-AndroidClient/
**Branch:** feat/android-client
**PR:** #10
**Reviewers:** code-reviewer, pr-test-analyzer, silent-failure-hunter, type-design-analyzer (automated)
**Status:** ✅ Resolved

## Summary

42 findings: 7 Critical [FIX], 11 High [FIX/TASK/RULE], 14 Medium, 10 Low.
Three production subsystems are non-functional despite 48/48 tests passing:
logout does not navigate away (P10-001), the entire Tracking domain uses an empty
householdId (P10-004), and sync uploads are a no-op stub (P10-005). Two additional
Critical fixes required before any testing is meaningful: SyncWorker always reports
success regardless of outcome (P10-003), and QuestDetailScreen crashes on every first
navigation (P10-002). Eight missing test tasks, three ADRs, three convention gaps,
and nine captured suggestions round out the findings.

---

## Findings

### Fix-Now

#### [FIX] P10-001: Logout never navigates to login screen — session guard broken end-to-end
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/auth/AuthViewModel.kt:30`, `data/repository/AuthRepositoryImpl.kt:36-39`, `MainActivity.kt:30-49`
- **Severity:** Critical
- **Detail:** `AuthViewModel.isAuthenticated` is set to `true` on login/register but **never set to `false`**. `SettingsViewModel.logout()` clears tokens and stops sync, but the `StateFlow` value never changes, so `MainActivity`'s `LaunchedEffect(isAuthenticated)` never fires. Users are stranded in `MainScaffold` until the process is killed. Same failure path when `AuthAuthenticator` clears tokens on a failed token refresh. Violates FA-001 and FA-002.
- **Fix:** Back `TokenPreferences.accessToken` with a `MutableStateFlow<Boolean>` and have `AuthViewModel.isAuthenticated` observe it — or have `AuthRepositoryImpl.logout()` explicitly set `isAuthenticated.value = false`.
- **Relates to:** FA-001, FA-002
- **Status:** ✅ Fixed
- **Resolution:** Added `_isLoggedIn: MutableStateFlow<Boolean>` to `TokenPreferences`, exposed as `isLoggedInFlow: StateFlow<Boolean>`. `AuthViewModel.isAuthenticated` now derives from this flow. `AuthRepositoryImpl.logout()` calls `clearTokens()` in a `finally` block so tokens are always cleared.

#### [FIX] P10-002: `QuestDetailScreen` crashes on first navigation — `lateinit` race
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/QuestDetailViewModel.kt:31`, `QuestDetailScreen.kt:44-46`
- **Severity:** Critical
- **Detail:** `QuestDetailScreen` body reads `viewModel.quest` (which is `lateinit`) before the `LaunchedEffect` that calls `viewModel.init(questId)` has a chance to execute. `LaunchedEffect` only schedules — it does not run synchronously during composition. Result: `UninitializedPropertyAccessException` on every first navigation to a quest detail. Unit tests hide this by calling `init()` in `setUp()`. Also directly violates `kotlin-android.md`: "No `lateinit var` for nullable types."
- **Fix:** Pass `questId` via `SavedStateHandle` (as `NoteDetailViewModel` does at line 28) and initialize `quest` as a `val` at declaration. Remove `init()` and the `LaunchedEffect`.
- **Relates to:** FA-007
- **Status:** ✅ Fixed
- **Resolution:** `QuestDetailViewModel` now uses `SavedStateHandle` to get `questId` at construction. `val quest: StateFlow<QuestEntity?>` initialized at declaration via `questDao.watchById(questId).stateIn(...)`. Removed `init()` and the `LaunchedEffect` in `QuestDetailScreen`.

#### [FIX] P10-003: `SyncWorker` fire-and-forget — always reports `Result.success()` regardless of sync outcome
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/data/sync/SyncWorker.kt:16-22`, `SyncCoordinator.kt:40-44`
- **Severity:** Critical
- **Detail:** `SyncCoordinator.triggerSync()` is not `suspend` — it launches a coroutine into its own `SupervisorJob` scope and returns immediately. `SyncWorker.doWork()` calls it and immediately returns `Result.success()`. The actual `powerSyncDatabase.connect()` call runs in an escaped coroutine with no connection to the Worker's lifecycle. WorkManager's retry/backoff is permanently dead. FA-017–FA-020 are not being exercised.
- **Fix:** Make `triggerSync()` a `suspend` function that directly awaits the connect call. Call it inside `doWork`'s coroutine context.
- **Relates to:** FA-017, FA-018, FA-019, FA-020
- **Status:** ✅ Fixed
- **Resolution:** `SyncCoordinator.triggerSync()` converted to `suspend fun` that calls `powerSyncDatabase.connect(connector)` directly. `SyncWorker.doWork()` now awaits the result within its coroutine.

#### [FIX] P10-004: `TrackingViewModel.householdId` defaults to `""` — entire Tracking domain non-functional
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/tracking/TrackingViewModel.kt:46-48`
- **Severity:** Critical
- **Detail:** Constructor takes `private val householdId: String = ""` with a `// TODO: wire from session` comment. Every DAO query (`watchAll("")`) returns empty. Every INSERT writes `household_id = ""`. PowerSync RLS will reject all writes server-side. Users see empty lists; any items created are permanently corrupted. The PR claims all tracking screens are functional — they are not.
- **Fix:** Resolve household from the authenticated user before exposing the ViewModel. Do not ship with a hardcoded empty-string default.
- **Relates to:** FA-007 through FA-012 (Tracking domain screens)
- **Status:** ✅ Fixed
- **Resolution:** Removed `householdId` constructor param. Added `currentHouseholdId: StateFlow<String?>` via `db.watch()` SQL joining `household_memberships` and `users`. All DAO flows use `flatMapLatest` on `currentHouseholdId`. All write operations guard on non-null household ID.

#### [FIX] P10-005: `uploadToServer` lambda is a no-op stub — offline writes are silently lost
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/di/SyncModule.kt:31`, `data/sync/AltairPowerSyncConnector.kt`
- **Severity:** Critical
- **Detail:** The Koin module wires `uploadToServer = { _, _, _, _ -> /* wired in S016 */ }`. `AltairPowerSyncConnector.uploadData()` calls this no-op then calls `batch.complete(null)`, marking writes as uploaded when nothing was sent to the server. Directly violates the core invariant: "Sync conflicts must never silently lose data." FA-017–FA-020 cannot be satisfied while this stub is in place.
- **Fix:** Implement the actual upload-to-`SyncApi` call inline, or disable the sync worker and remove the `batch.complete(null)` call so the PowerSync outbox is not incorrectly marked as drained.
- **Relates to:** FA-003, FA-017, FA-018, FA-019, FA-020
- **Status:** ✅ Fixed
- **Resolution:** `AltairPowerSyncConnector` refactored to accept `syncApi: SyncApi` instead of `uploadToServer` lambda. `uploadData()` calls `syncApi.upsert()` for PUT/PATCH and `syncApi.delete()` for DELETE. `SyncModule` wired with `syncApi = get()`.

#### [FIX] P10-006: `AuthAuthenticator` bare `catch (e: Exception)` swallows `CancellationException` with zero logging
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/data/auth/AuthAuthenticator.kt:32-40`
- **Severity:** Critical
- **Detail:** The token refresh catch block: (1) catches `CancellationException`, violating the project convention; (2) produces zero log output — token refresh failures leave no diagnostic breadcrumb; (3) clears tokens for any exception including serialization errors or disk-full events. Users are silently logged out with no error shown and no log to debug from. Violates `kotlin-android.md`: "Catch blocks must not swallow `CancellationException`."
- **Fix:** Split into `catch (e: CancellationException) { throw e }`, then typed catches for `IOException` and `HttpException`, each with `Log.w(tag, message, e)`.
- **Relates to:** FA-001, FA-019
- **Status:** ✅ Fixed
- **Resolution:** Split catch into `CancellationException { throw e }`, `HttpException { Log.w + clear }`, `IOException { Log.w + clear }`, `Exception { Log.w + clear }`. Added `runBlocking` ADR comment (ADR-025).

#### [FIX] P10-007: `SyncWorker` bare `catch (e: Exception)` converts all failures — including cancellation — to infinite retry
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/data/sync/SyncWorker.kt:16-23`
- **Severity:** Critical
- **Detail:** Catches `CancellationException` (WorkManager cancellation → incorrectly rescheduled) and maps all permanent failures (expired refresh token, schema mismatch, 401) to `Result.retry()`. Causes silent background battery drain with no diagnostic log and no user notification. WorkManager retries indefinitely for failures that require user action (e.g., re-login).
- **Fix:** `catch (e: CancellationException) { throw e }`, then distinguish transient (network) → `Result.retry()` from permanent errors → `Result.failure(...)` with a logged message.
- **Relates to:** FA-017, FA-020
- **Status:** ✅ Fixed
- **Resolution:** `doWork()` catch split: `CancellationException { throw e }`, `IOException { Log.w + Result.retry() }`, `Exception { Log.e + Result.failure() }`.

#### [FIX] P10-008: `AuthAuthenticator` infinite-loop guard is one level deep — not full chain
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/data/auth/AuthAuthenticator.kt:20`
- **Severity:** High
- **Detail:** `response.priorResponse?.code == 401` checks only the immediately prior response. If the freshly-refreshed token also returns 401 (revoked session, server-side invalidation), the authenticator will attempt another refresh rather than breaking the loop. Standard OkHttp pattern is to walk the full `priorResponse` chain or count attempts.
- **Fix:** Walk the `priorResponse` chain (or count via a counter) and return `null` after 1 failed refresh attempt.
- **Relates to:** FA-019
- **Status:** ✅ Fixed
- **Resolution:** Replaced single-level check with full chain walk (`var prior = response.priorResponse; while (prior != null) { if (prior.code == 401) return null; prior = prior.priorResponse }`).

#### [FIX] P10-009: `FocusSessionViewModel` resolves `userId` asynchronously — completed sessions silently discarded
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/guidance/FocusSessionViewModel.kt:41-82`
- **Severity:** High
- **Detail:** `start()` launches a coroutine to fetch `userId` from the DB. `CountDownTimer.onFinish()` fires 25 minutes later. If the DB query fails (network hiccup, empty result), `userId` remains `null` and `recordSession()` silently returns early via `?: return`. The user sees "Session complete" but no `focus_sessions` row is written. Data is lost with no notification.
- **Fix:** Fetch `userId` eagerly at ViewModel construction time (as other VMs do via `currentUser` Flow). Guard `start()` with a non-null check that surfaces an error if userId is unavailable.
- **Relates to:** FA-007
- **Status:** ✅ Fixed
- **Resolution:** Added `currentUserId: StateFlow<String?>` via `db.watch()` with `SharingStarted.Eagerly`. `start()` guards on non-null `currentUserId.value` and emits error if unavailable. `recordSession()` takes `uid: String` parameter.

#### [FIX] P10-010: `KnowledgeViewModel`/`NoteDetailViewModel` fall back to `userId = ""` on JWT decode failure
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/knowledge/KnowledgeViewModel.kt:21-22`, `NoteDetailViewModel.kt:27-28`
- **Severity:** High
- **Detail:** `userId` is `get() = tokenPreferences.accessToken?.let { decodeUserIdFromJwt(it) } ?: ""`. When the token is null or JWT decoding fails, all notes are created with `user_id = ""`. PowerSync sync will reject these rows via server-side RLS. Notes appear to save but never sync. No error shown to user.
- **Fix:** Make `userId` nullable. Guard INSERT operations with a non-null check and emit an error UiState if the user cannot be identified. Mirror `TodayViewModel`'s `currentUser` Flow pattern.
- **Relates to:** FA-007 through FA-010
- **Status:** ✅ Fixed
- **Resolution:** Both VMs now use `currentUserId: StateFlow<String?>` via `db.watch()`. Removed `TokenPreferences` dependency. All write operations guard on non-null userId and emit error state if missing.

#### [FIX] P10-011: `SettingsViewModel.logout()` has no error handling — tokens may persist after failed logout
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/settings/SettingsViewModel.kt:12-16`
- **Severity:** High
- **Detail:** `logout()` calls `authRepository.logout()` with no try/catch. If `stopSync()` inside `AuthRepositoryImpl.logout()` throws, `clearTokens()` is never called (it follows the throw in sequence). The user sees the login screen (if FA-001 is fixed) but their tokens remain in `EncryptedSharedPreferences`. On next launch, `isLoggedIn()` returns `true`.
- **Fix:** Wrap `stopSync()` in try/catch and call `clearTokens()` in a `finally` block. Add a `_logoutState` StateFlow to surface errors to the UI.
- **Relates to:** FA-001
- **Status:** ✅ Fixed
- **Resolution:** `SettingsViewModel` has `_logoutError: MutableStateFlow<String?>` + `logoutError: StateFlow<String?>` with try/catch and `CancellationException` rethrow. `AuthRepositoryImpl.logout()` calls `clearTokens()` in `finally`.

#### [FIX] P10-012: `TodayViewModel.submitCheckin` passes `mood.toString()` — likely schema mismatch with server
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/today/TodayViewModel.kt`
- **Severity:** High
- **Detail:** The ViewModel accepts `mood: Int` and writes `mood.toString()` into `DailyCheckinEntity.mood: String?`. If the server's `daily_checkins.mood` column stores categorical string labels (e.g., `"happy"`, `"neutral"`) rather than numeric strings, synced rows will be rejected or parsed incorrectly. Clarification needed on whether mood is a numeric score (like `energyLevel`) or a labeled enum.
- **Fix:** Align type with server schema. If numeric, change entity column to `mood: Int?`. If string label, change ViewModel parameter to `mood: String` and remove `.toString()`.
- **Relates to:** FA-004
- **Status:** ✅ Fixed
- **Resolution:** Schema confirmed: server `daily_checkins.mood` is `VARCHAR(30)`. `mood.toString()` preserved with inline comment explaining alignment with server schema.

#### [FIX] P10-013: Multiple ViewModels call `db.execute()` inside `viewModelScope.launch {}` with no error handling
- **File:** `TodayViewModel.kt:113-148`, `QuestDetailViewModel.kt:44-52`, `NoteDetailViewModel.kt:60-89`, `FocusSessionViewModel.kt:85-94`, `KnowledgeViewModel.kt:51-78`
- **Severity:** High
- **Detail:** Uncaught exceptions from PowerSync propagate to `viewModelScope`'s default handler — a crash-level log with no UI feedback. User's action silently fails (swipe completes, save button dismisses, nothing changes). `TrackingViewModel` already handles this correctly with try/catch emitting to `_uiState`. The fix pattern exists in the codebase; it's just not applied consistently.
- **Fix:** Wrap every `db.execute()` call in `try { ... } catch (e: CancellationException) { throw e } catch (e: Exception) { Log.e(...); _uiState.value = UiState.Error(...) }`. Add `_uiState`/error state to `TodayViewModel` and `QuestDetailViewModel` which currently have none.
- **Status:** ✅ Fixed
- **Resolution:** All five ViewModels now wrap `db.execute()` calls in try/catch with `CancellationException` rethrow, `Log.e`, and error state emission. `TodayViewModel` and `QuestDetailViewModel` have `_error: MutableStateFlow<String?>` added.

#### [FIX] P10-014: `AuthRepositoryImpl.logout()` is non-atomic — `clearTokens()` skipped if `stopSync()` throws
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/data/repository/AuthRepositoryImpl.kt:36-39`
- **Severity:** High
- **Detail:** `logout()` calls `syncCoordinator.stopSync()` then `tokenPreferences.clearTokens()` sequentially. If `stopSync()` throws, tokens are never cleared. Partial-logout state: the session guard sees a valid token on next launch but sync never starts cleanly.
- **Fix:** Wrap `stopSync()` in try/catch with logging; call `clearTokens()` in a `finally` block.
- **Status:** ✅ Fixed
- **Resolution:** `stopSync()` wrapped in try/catch with `Log.w`. `clearTokens()` moved to `finally` block.

#### [FIX] P10-015: `AltairApplication` `NetworkCallback.onAvailable()` calls `get<SyncCoordinator>()` without error handling
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/AltairApplication.kt:56-62`
- **Severity:** High
- **Detail:** `get<SyncCoordinator>()` is a Koin resolution call inside a system callback. If Koin failed to initialize (e.g., `PowerSyncDatabase` construction threw), this crashes the app from the system network thread. No try/catch around `startKoin {}` means a Koin startup failure also produces an opaque crash on first launch rather than a diagnostic log.
- **Fix:** Wrap `startKoin {}` in try/catch with explicit logging. Wrap the `onAvailable()` body in try/catch.
- **Status:** ✅ Fixed
- **Resolution:** `startKoin {}`, `WorkManager.enqueueUniquePeriodicWork()`, and `onAvailable()` body each wrapped in try/catch with `Log.e`.

#### [FIX] P10-016: `KnowledgeViewModel.notes` StateFlow captures `userId` at construction — does not react to user changes
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/knowledge/KnowledgeViewModel.kt:23-45`
- **Severity:** Medium
- **Detail:** `notes` StateFlow is initialized from `noteDao.watchAll(userId)` at construction. If the VM is created before login completes (or JWT decode fails), the watch is bound to `""` permanently — no `flatMapLatest` on a token/user Flow. Contrast with `TodayViewModel` which correctly uses `flatMapLatest` on `currentUser`.
- **Fix:** Build a `currentUserId: Flow<String?>` and `flatMapLatest` into the DAO query.
- **Status:** ✅ Fixed
- **Resolution:** `notes` StateFlow rebuilt with `flatMapLatest` on `currentUserId`. Now reacts correctly to user availability after sync.

#### [FIX] P10-017: `TodayViewModel.submitCheckin()` silently returns early if `userId` not yet populated
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/today/TodayViewModel.kt:113`
- **Severity:** Medium
- **Detail:** `val uid = userId ?: return` — if `currentUser.value` is `null` on first launch (before PowerSync has synced), the check-in tap does nothing with no error or log entry. User taps submit, nothing happens.
- **Fix:** Emit an error state or show a loading indicator if userId is not yet available. Do not silently return.
- **Status:** ✅ Fixed
- **Resolution:** `submitCheckin()` now emits `_error.value = "User not available — try again after sync"` when userId is null instead of silently returning.

#### [FIX] P10-018: `AuthInterceptor` sends requests without a token and logs nothing
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/data/auth/AuthInterceptor.kt:9-21`
- **Severity:** Medium
- **Detail:** When `tokenPreferences.accessToken` is `null`, the request proceeds without an `Authorization` header and no log entry is made. For authenticated endpoints, this results in a 401, triggering `AuthAuthenticator` — but with no prior log, the debugging trail starts at the authenticator with no context on why the token was missing.
- **Fix:** Add `Log.d("AuthInterceptor", "Sending request without access token to ${request.url}")` in the null-token branch.
- **Status:** ✅ Fixed
- **Resolution:** Added `Log.d(TAG, "Sending request without access token to ${chain.request().url}")` in null-token branch.

#### [FIX] P10-019: WorkManager periodic sync enqueue has no error handling
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/AltairApplication.kt:39-51`
- **Severity:** Medium
- **Detail:** `WorkManager.getInstance(this).enqueueUniquePeriodicWork(...)` can throw `IllegalStateException` if WorkManager is not yet initialized. Background sync would be silently not scheduled with no log or user notification.
- **Fix:** Wrap in try/catch with `Log.e(...)`.
- **Status:** ✅ Fixed
- **Resolution:** Wrapped in try/catch with `Log.e`. Resolved together with P10-015.

---

### Missing Tasks

#### [TASK] P10-020: `AuthViewModel` has zero unit tests — auth loading/error states completely unverified
- **File:** `apps/android/app/src/test/java/com/getaltair/altair/ui/auth/` (missing)
- **Severity:** Critical
- **Detail:** `AuthViewModel.login()` and `register()` each contain a try/catch that maps exceptions to `AuthUiState.Error`. These paths are entirely untested. `LoginScreenTest` only verifies the button-click callback fires — it does not drive the VM through loading, success, or error states. A regression where the error message is swallowed or `isAuthenticated` is incorrectly set would not be caught.
- **Tasks to add:** `login_success_setsIsAuthenticatedTrue`, `login_failure_emitsErrorState`, `register_failure_emitsErrorState`
- **Relates to:** FA-001, S017-T (implied)
- **Status:** ✅ Task created
- **Resolution:** Added as S026-T in Steps.md

#### [TASK] P10-021: `AltairPowerSyncConnector.uploadData` has no tests — upload path is also a no-op (P10-005)
- **File:** `apps/android/app/src/test/java/com/getaltair/altair/data/sync/` (missing)
- **Severity:** High
- **Detail:** The upload path has no unit test. The test would also immediately surface P10-005 (no-op stub): `batch.complete(null)` is called unconditionally. When upload is implemented, tests must verify `batch.complete` is called on success and not called on failure.
- **Relates to:** FA-003, S015-T (implied)
- **Status:** ✅ Task created
- **Resolution:** Added as S027-T in Steps.md

#### [TASK] P10-022: `FocusSessionViewModel` timer-completion claimed covered by instrumented test — no such test exists
- **File:** `apps/android/app/src/androidTest/java/com/getaltair/altair/ui/guidance/` (FocusSessionScreenTest missing)
- **Severity:** High
- **Detail:** `FocusSessionViewModelTest.kt` contains the comment "The timer-completion path (onFinish → _isFinished = true) is covered by the instrumented test suite." No `FocusSessionScreenTest.kt` exists in the instrumented tests directory. The `_isFinished = true` state transition is never tested anywhere.
- **Relates to:** FA-007, S019-T (implied)
- **Status:** ✅ Task created
- **Resolution:** Added as S028-T in Steps.md

#### [TASK] P10-023: `SyncCoordinator` and `SyncWorker` have no tests — the retry/failure path is the entire value of background sync
- **File:** `apps/android/app/src/test/java/com/getaltair/altair/data/sync/` (missing)
- **Severity:** High
- **Detail:** `SyncWorker.doWork()` maps exceptions to `Result.retry()` — the retry path is untested. `SyncCoordinator.startSync()` / `triggerSync()` are untested. Combined with P10-003 and P10-007, these are critical paths with bugs and no test coverage.
- **Relates to:** FA-017, FA-018, FA-020, S016-T (implied)
- **Status:** ✅ Task created
- **Resolution:** Added as S029-T in Steps.md

#### [TASK] P10-024: FA-002 (unauthenticated redirect) has no test
- **File:** `apps/android/app/src/androidTest/java/com/getaltair/altair/ui/auth/` (redirect test missing)
- **Severity:** Medium
- **Detail:** No test verifies that setting `isAuthenticated` to `false` (or clearing tokens) navigates to Login and clears the back stack. The session guard behavior in `MainActivity.LaunchedEffect(isAuthenticated)` is untested.
- **Relates to:** FA-002, S017-T
- **Status:** ✅ Task created
- **Resolution:** Added as S030-T in Steps.md

#### [TASK] P10-025: `TokenPreferences.clearTokens()` has no unit test
- **File:** `apps/android/app/src/test/java/com/getaltair/altair/data/auth/TokenPreferencesTest.kt` (missing)
- **Severity:** Medium
- **Detail:** `clearTokens()` is called in three branches of `AuthAuthenticator` and `AuthRepositoryImpl.logout()`. `AuthAuthenticatorTest` verifies it is called but does not verify it actually nullifies both `accessToken` and `refreshToken` in the underlying `SharedPreferences`. Given this is a security surface, a unit test using a fake `SharedPreferences` is warranted.
- **Relates to:** FA-001, S017-T
- **Status:** ✅ Task created
- **Resolution:** Added as S031-T in Steps.md

#### [TASK] P10-026: `TodayViewModel` SQL-guard tests verify raw string, not database behavior — FA-006 inadequately covered
- **File:** `apps/android/app/src/test/java/com/getaltair/altair/ui/today/TodayViewModelTest.kt`
- **Severity:** Medium
- **Detail:** Tests assert `capturedSql.contains("status = 'in_progress'")`. This passes even if the SQL has a syntax error or malformed WHERE clause (e.g., a trailing space that breaks the clause). FA-006 requires the database-level guard to function: a `not_started` quest must not be affected by `completeQuest`. A Robolectric + in-memory Room integration test would catch query-level bugs that string inspection misses.
- **Relates to:** FA-006, S018-T
- **Status:** ✅ Task created
- **Resolution:** Added as S032-T in Steps.md

#### [TASK] P10-027: `LoginScreenTest` covers only the happy-path button click — Loading/Error UI states unverified
- **File:** `apps/android/app/src/androidTest/java/com/getaltair/altair/ui/auth/LoginScreenTest.kt`
- **Severity:** Medium
- **Detail:** The single instrumented test verifies the login button invokes `viewModel.login`. It does not render `AuthUiState.Loading` (no spinner test) or `AuthUiState.Error` (no error message test). Combined with zero unit tests for `AuthViewModel` (P10-020), the entire user-visible auth error flow is unverified.
- **Relates to:** FA-001
- **Status:** ✅ Task created
- **Resolution:** Added as S033-T in Steps.md

---

### Architectural Concerns

#### [ADR] P10-028: `UserEntity` stores and syncs `passwordHash` to the client
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/data/local/entity/UserEntity.kt`, `data/sync/PowerSyncSchema.kt`
- **Severity:** High
- **Detail:** `UserEntity.passwordHash: String?` persists a password hash in the client-side SQLite database. The hash cannot authenticate the user locally (server handles auth), so it serves no purpose on the client and represents unnecessary exposure of sensitive credential data. If the PowerSync sync rules include this column, it is also being transmitted over the sync channel to every client.
- **Decision needed:** Remove `passwordHash` from `UserEntity` and from the PowerSync schema's `users` table definition. Update sync rules to exclude the column.
- **Relates to:** SEC-6 (if applicable), ADR-012
- **Status:** ✅ Fixed
- **Resolution:** ADR-024 created. `passwordHash` removed from `UserEntity` and `AltairPowerSyncSchema`. `TodayViewModel` cursor indices updated. `AltairDatabase` bumped to version 2 with `MIGRATION_1_2` (table recreation).

#### [ADR] P10-029: `runBlocking` inside `AuthAuthenticator` — deviation from `kotlin-android.md` rule
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/data/auth/AuthAuthenticator.kt:31`
- **Severity:** Medium
- **Detail:** `kotlin-android.md` states: "No `runBlocking` outside of main function or tests." OkHttp's `Authenticator` is synchronous by contract (runs on OkHttp's thread pool, never the main thread), so this is the standard bridge. The usage is pragmatic, but the deviation from the project rule is not documented.
- **Decision needed:** Either document the deviation with an inline comment citing the OkHttp contract, or convert to a synchronous Retrofit call (`Call<AuthResponse>.execute()`) to eliminate the need for a coroutine bridge.
- **Status:** ✅ ADR created
- **Resolution:** ADR-025 created. Deviation documented with inline comment in `AuthAuthenticator.kt`.

#### [ADR] P10-030: `AuthRepository` returns `Unit` — failure domain is invisible in the type system
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/domain/repository/AuthRepository.kt`, `ui/auth/AuthViewModel.kt:42`
- **Severity:** Medium
- **Detail:** `login()` and `register()` return `Unit`, expressing success/failure only through exceptions. Callers use `catch (e: Exception) { e.message ?: "..." }` — `e.message` is nullable and opaque from Retrofit. There is no type-level distinction between network failure, invalid credentials, and server error. The type-design-analyzer recommends a sealed `AuthResult` type. This is a cross-cutting design decision affecting the repository interface, implementation, and all ViewModels.
- **Decision needed:** Adopt sealed `AuthResult` (Success, InvalidCredentials, NetworkError, UnknownError) or accept the current exception-based approach with documented rationale.
- **Status:** ✅ ADR created
- **Resolution:** ADR-026 created. Exception-based approach accepted for current feature; sealed result type deferred to backlog.

---

### Convention Gaps

#### [RULE] P10-031: `viewModelScope.launch { db.execute() }` without try/catch — missing from `kotlin-android.md`
- **Files:** `TodayViewModel.kt:113-148`, `QuestDetailViewModel.kt:44-52`, `NoteDetailViewModel.kt:60-89`, `FocusSessionViewModel.kt:85-94`, `KnowledgeViewModel.kt:51-78` (5 files)
- **Severity:** High
- **Detail:** The rule file has "Never swallow exceptions silently in catch blocks" but does not address the case of no catch block at all inside coroutines that perform DB writes. `TrackingViewModel` demonstrates the correct pattern (try/catch emitting to `_uiState`) but this is not codified in the rules. The same mistake appeared independently in 5 ViewModels.
- **Suggested rule:** Add to `kotlin-android.md` under Error Handling: "Every `viewModelScope.launch` block that performs a database write or network call must include a `try/catch` block that (1) rethrows `CancellationException`, (2) logs the failure with `Log.e`, and (3) emits to a `_uiState` error state. Silent failure from unhandled coroutine exceptions is prohibited."
- **Status:** ✅ Rule updated
- **Resolution:** Added to `kotlin-android.md` Error Handling section.

#### [RULE] P10-032: Public `MutableStateFlow` on ViewModel — no rule prohibiting it
- **Files:** `AuthViewModel.kt:30` (`isAuthenticated`), `TodayViewModel.kt:109-110` (`checkinEnergy`, `checkinMood`)
- **Severity:** Medium
- **Detail:** Both cases expose `MutableStateFlow` as public properties, allowing composables to mutate ViewModel state directly without going through validation functions. `isAuthenticated` being publicly mutable is what makes the C1 bug pattern possible (callers could set it from anywhere). `checkinEnergy`/`checkinMood` are written by `TodayScreen` directly via `.value = ...`.
- **Suggested rule:** Add to `kotlin-android.md` under Composable Functions or Data Layer: "ViewModel state must be exposed as `StateFlow<T>` (read-only). Back all mutable state with `private val _field: MutableStateFlow` and expose `val field: StateFlow` via `_field.asStateFlow()`. Composables must never write directly to ViewModel state."
- **Status:** ✅ Fixed + Rule updated
- **Resolution:** `TodayViewModel.checkinEnergy`/`checkinMood` made private; `setCheckinEnergy()`/`setCheckinMood()` added. `TodayScreen` updated to use setter methods. Rule added to `kotlin-android.md`.

#### [RULE] P10-033: `catch (e: Exception)` without re-throwing `CancellationException` — rule exists but violated in 3+ files
- **Files:** `AuthViewModel.kt:42/59`, `AuthRepositoryImpl.kt`, `AuthAuthenticator.kt:40`
- **Severity:** High
- **Detail:** `kotlin-android.md` already states "Catch blocks must not swallow `CancellationException`." The rule is not catching violations at review time. The same mistake appears independently in auth-related code across 3 files. Consider whether to add a lint rule or note in the convention that a utility `rethrowIfCancellation()` extension function can reduce the boilerplate.
- **Suggested rule update:** Add to `kotlin-android.md`: "When catching `Exception` broadly, always add `if (e is CancellationException) throw e` as the first statement, or use a project-provided `rethrowCancellation()` extension." Consider adding a Detekt custom rule.
- **Status:** ✅ Rule updated
- **Resolution:** Rule clarified in `kotlin-android.md` Coroutines section with explicit pattern. All three files fixed with `CancellationException` rethrow.

---

#### [ADR] P10-034: Stringly-typed status/enum fields across all 20 entities
- **Files:** All 20 entity files in `data/local/entity/` — `QuestEntity.status`, `RoutineEntity.frequencyType`, `HouseholdMembershipEntity.role`, `ShoppingListItemEntity.status`, `TrackingItemEventEntity.eventType`, `EntityRelationEntity.relationType`, `EntityTagEntity.entityType`, etc.
- **Severity:** Medium
- **Detail:** Every enumerated domain concept is stored and compared as a raw `String`. The domain spec defines exact state machines (e.g., quest status: `not_started → in_progress → completed / cancelled / deferred`) but these constraints are invisible in the type. A misspelled string literal in any ViewModel SQL or mapper is undetectable at compile time. The existing `EntityType` and `RelationType` enums in `Dtos.kt` are the correct model to follow. Given PowerSync writes raw SQL and entities must round-trip through PowerSync cursors without Room `@TypeConverter` on that path, the recommended approach is enum companion objects in the domain layer (not on the entity) used as the single source of truth for valid status strings.
- **Decision needed:** Adopt enum companion objects for all status/categorical fields and enforce their use in mappers and SQL string literals, or document acceptance of stringly-typed fields with a rationale.
- **Status:** ✅ ADR created
- **Resolution:** ADR-027 created. Stringly-typed fields accepted on entities; enum companion objects deferred to backlog for domain layer.

#### [FIX] P10-035: `TrackingUiState` has no `Loading` state
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/tracking/TrackingViewModel.kt`
- **Severity:** Medium
- **Detail:** `TrackingUiState` defines `Idle` and `Error` but no `Loading`. All write operations (`createItem`, `logConsumption`, `createLocation`, etc.) are `suspend` calls inside `viewModelScope.launch` — they have no spinner signal while in-flight. `AuthUiState` in the same PR includes `Loading` correctly; the pattern is inconsistent.
- **Fix:** Add `object Loading : TrackingUiState()`. Emit `Loading` at the start of every write operation and replace with `Idle` or `Error` on completion.
- **Status:** ✅ Fixed
- **Resolution:** Added `object Loading : TrackingUiState()`. All write operations in `TrackingViewModel` emit `Loading` at start, `Idle` or `Error` on completion.

#### [FIX] P10-036: `Screen.QuestDetail.route(id)` and `Screen.ItemDetail.route(id)` perform no ID validation
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/navigation/Screen.kt`
- **Severity:** Low
- **Detail:** Passing an empty string or null-coerced-to-string value to `route(id)` produces a syntactically valid but semantically broken route (`"quest/"`) that navigates without error. The most common callsite mistakes (empty string from a failed lookup, `toString()` on a null) are not caught at the navigation boundary.
- **Fix:** Add `require(id.isNotBlank()) { "route() requires a non-blank id" }` to both `QuestDetail.route()` and `ItemDetail.route()`.
- **Status:** ✅ Fixed
- **Resolution:** Added `require(id.isNotBlank())` to `QuestDetail.route()` and `ItemDetail.route()`.

#### [FIX] P10-037: `decodeUserIdFromJwt` in `KnowledgeUtils.kt` is hand-rolled string indexing
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/knowledge/KnowledgeUtils.kt:3-15`
- **Severity:** Low
- **Detail:** JWT decoding is implemented as manual string indexing into the base64-decoded JSON payload. Brittle if the server reorders JWT claims, adds nested objects, or changes claim key names. A `StringIndexOutOfBoundsException` or `IllegalArgumentException` from this function propagates as a `null` userId (see P10-010), silently corrupting all created notes. Use `kotlinx.serialization` to deserialize the payload into a typed data class, or use a JWT library.
- **Fix:** Replace the hand-rolled parser with `kotlinx.serialization` JSON decoding of the base64-decoded payload into a `@Serializable data class JwtPayload(val sub: String, ...)`.
- **Status:** ✅ Fixed
- **Resolution:** Replaced manual string indexing with `kotlinx.serialization`: added `@Serializable private data class JwtPayload(val sub: String)` and `jwtJson.decodeFromString<JwtPayload>(json).sub`.

#### [RULE] P10-038: `LocalDateTime.now()` used in 4+ places — strips timezone, risking sync conflict edge cases
- **Files:** `TodayViewModel.kt:150`, `QuestDetailViewModel.kt:54`, `FocusSessionViewModel.kt:101`, `NoteDetailViewModel.kt` (via shared `nowIso`)
- **Severity:** Low
- **Detail:** `kotlin-android.md` allows both `kotlinx-datetime` and `java.time`, but `LocalDateTime.now()` strips timezone information. The server stores `timestamptz` in PostgreSQL. An offline write from a device in a non-UTC timezone that uses `LocalDateTime.now().toString()` could produce an incorrectly-ordered `updated_at` after sync, triggering spurious last-write-wins conflicts. `KnowledgeUtils.nowIso()` correctly uses `kotlinx.datetime.Clock.System.now()`.
- **Suggested rule:** Update `kotlin-android.md` to: "Always use `kotlinx-datetime` (`Clock.System.now().toString()`) for timestamp generation. `LocalDateTime.now()` is prohibited — it strips timezone information and produces incorrect ordering against server `timestamptz` columns."
- **Status:** ✅ Rule updated + Fixed
- **Resolution:** Rule updated in `kotlin-android.md`. All four ViewModels migrated to `Clock.System.now().toString()`.

#### [TASK] P10-039: Periodic + network-callback sync workers can stack on flaky networks — add debounce
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/AltairApplication.kt:54-67`
- **Severity:** Low
- **Detail:** `NetworkCallback.onAvailable()` enqueues an expedited sync worker on every connectivity event. On a device that rapidly toggles between connected and disconnected (flaky WiFi, moving between cells), this fires many expedited workers in rapid succession, each backed by the 15-minute periodic worker. WorkManager's `KEEP` policy on the expedited enqueue helps, but the periodic worker adds a second axis. Consider tracking the last sync timestamp and skipping the expedited enqueue if a sync completed within the last N minutes.
- **Task to add:** S022 — Debounce expedited sync on rapid network reconnect events.
- **Status:** ✅ Task created
- **Resolution:** Added as S034 in Steps.md

#### [ADR] P10-040: `AuthUiState.Success` carries no payload — post-login identity requires a separate state query
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/auth/AuthViewModel.kt`
- **Severity:** Low
- **Detail:** `AuthUiState.Success` is a bare `object` with no data. After a successful login, the screen navigates away based on `isAuthenticated`, but the authenticated user's identity (display name, household, etc.) must be fetched from `TodayViewModel` after navigation completes — creating a timing window where the user has navigated to `MainScaffold` but `currentUser.value` is still null. Consider whether `AuthUiState.Success` should carry the user record (or at minimum the `userId`) to allow the receiving screen to begin its own loading without a cold start.
- **Decision needed:** Accept the current two-step pattern (authenticate → navigate → fetch user) or pass user identity through the navigation transition.
- **Status:** ✅ ADR created
- **Resolution:** ADR-028 created. Two-step pattern accepted; user identity not passed through auth state.

#### [TASK] P10-041: `QuestDetailScreenTest` stubs `validTransitions` with a hardcoded map — state machine regressions not caught
- **File:** `apps/android/app/src/androidTest/java/com/getaltair/altair/ui/guidance/QuestDetailScreenTest.kt`
- **Severity:** Low
- **Detail:** `buildViewModel` in the test replaces the real `validTransitions` logic with an inline map defined in the test. A regression in the actual `validTransitions` method (e.g., allowing `completed → not_started`) would not be caught by this screen test. `QuestDetailViewModelTest` covers the state machine logic, but the screen test is disconnected from it. Add a test that uses the real ViewModel (not a stub) and verifies the screen renders only the permitted transitions.
- **Task to add:** S019-T addendum — QuestDetailScreenTest with real ViewModel state machine.
- **Status:** ✅ Task created
- **Resolution:** Added as S035-T in Steps.md

#### [FIX] P10-042: `energyLevel: Int?` on `DailyCheckinEntity` — domain requires 1–5, no range check before write
- **File:** `apps/android/app/src/main/java/com/getaltair/altair/ui/today/TodayViewModel.kt` (submitCheckin)
- **Severity:** Low
- **Detail:** The domain spec requires `energyLevel` to be in the range 1–5. The type admits any `Int?`. `submitCheckin` passes the value directly to `db.execute()` without validation. A UI bug or future API change could write a value of `0` or `99` to the server, which may fail server-side validation and silently break sync for that row.
- **Fix:** Add `require(energy in 1..5) { "energyLevel must be 1–5, got $energy" }` in `submitCheckin` before the `db.execute()` call.
- **Status:** ✅ Fixed
- **Resolution:** Added `require(energy in 1..5)` guard in `submitCheckin()`.

---

## Resolution Checklist
- [x] All [FIX] findings resolved (P10-001 through P10-019, P10-035, P10-036, P10-037, P10-042)
- [x] All [TASK] findings added to Steps.md (P10-020 through P10-027, P10-039, P10-041)
- [x] All [ADR] findings have ADRs created or dismissed (P10-028, P10-029, P10-030, P10-034, P10-040)
- [x] All [RULE] findings applied or dismissed (P10-031, P10-032, P10-033, P10-038)
- [x] Review verified by review-verify agent

## Resolution Summary
**Resolved at:** 2026-04-16
**Session:** resolve-review P10 — Feature 009 Android client pre-merge fixes

| Category | Total | Resolved |
|---|---|---|
| [FIX] | 23 | 23 |
| [TASK] | 10 | 10 |
| [ADR] | 5 | 5 |
| [RULE] | 4 | 4 |
| **Total** | **42** | **42** |
