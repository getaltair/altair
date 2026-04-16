# Feature 009: Android Client — Technical Plan

| Field | Value |
|---|---|
| **Feature** | 009-AndroidClient |
| **Version** | 1.0 |
| **Status** | Draft |
| **Date** | 2026-04-16 |
| **ADRs** | ADR-022 (HTTP client), ADR-023 (minSdk deviation) |

---

## Architecture Overview

The Android client follows a strict MVVM + Repository layered architecture:

```
Screen (Compose)
  └─► ViewModel (StateFlow / UiState)
        └─► Repository (interface in domain/)
              ├─► DAO (Room Flow / suspend)
              └─► PowerSync (execute, watch)
```

**Data flow — reads:**
Room DAOs expose `Flow<List<T>>` queries. ViewModels collect these as `StateFlow` via `viewModelScope`. Compose screens observe `collectAsStateWithLifecycle()`.

**Data flow — writes:**
Screens call ViewModel methods → ViewModel calls Repository → Repository calls `db.execute()` on the PowerSync-backed Room database → PowerSync queues the change in its CRUD outbox → SyncWorker uploads to the server REST API.

**Data flow — incoming sync:**
PowerSync receives server changes → writes to the managed SQLite → Room DAO Flows emit → ViewModel StateFlow updates → screen recomposes.

---

## PowerSync + Room Integration

### Architecture

The PowerSync Kotlin SDK (`com.powersync:core`) provides a `RoomConnectionPool` that allows PowerSync to use an existing Room database as its SQLite backing store. The integration pattern is:

1. Build a Room `AltairDatabase` instance normally.
2. Pass it to `RoomConnectionPool` to obtain a `PowerSyncDatabase`.
3. Use the `PowerSyncDatabase` for all writes (via `execute()`) and for attaching sync subscriptions.
4. Room DAOs continue to work because they share the same underlying SQLite connection pool.

This means **Room owns schema management** (entities, migrations, DAOs) and PowerSync adds sync behaviour on top. There is no separate PowerSync-managed SQLite file.

### Critical Constraint — Schema Verification

PowerSync creates internal views (for its CRUD tracking and sync metadata) that are not defined in the Room schema. Room's default `openHelperFactory` will fail schema validation on first open because it detects these unexpected views.

Mitigation: configure the Room `databaseBuilder` to use the SQLite connection factory provided by the PowerSync `RoomConnectionPool`, which handles the open helper and bypasses Room's strict schema check for PowerSync-owned objects while still validating application-owned tables.

### Artifact Coordinates

```toml
# libs.versions.toml additions
[versions]
powersync = "1.11.2"

[libraries]
powersync-core = { module = "com.powersync:core", version.ref = "powersync" }
powersync-room = { module = "com.powersync:integration-room", version.ref = "powersync" }
sqlite-bundled = { module = "androidx.sqlite:sqlite-bundled", version = "2.5.0" }
```

`androidx.sqlite:sqlite-bundled` is **required** — PowerSync uses SQLite extensions (e.g., `json_each`, WAL mode) that are not available on all Android platform SQLite builds.

### Alpha Status

The `integration-room` module is in **alpha** as of late 2025. Risk: potential breaking API changes between alpha versions. Mitigation: pin to a specific version in `libs.versions.toml`; check CHANGELOG before updating.

### PowerSync Connector

The Kotlin equivalent of the web `AltairConnector`:

```kotlin
class AltairPowerSyncConnector(
    private val authService: AuthService,
) : PowerSyncBackendConnector {

    override suspend fun fetchCredentials(): PowerSyncCredentials {
        val token = authService.getPowerSyncToken()  // GET /api/auth/powersync-token
        return PowerSyncCredentials(
            endpoint = BuildConfig.POWERSYNC_URL,
            token = token.accessToken,
        )
    }

    override suspend fun uploadData(database: PowerSyncDatabase) {
        val batch = database.getCrudBatch(100) ?: return
        for (entry in batch.crud) {
            when (entry.op) {
                UpdateType.PUT -> restApi.upsert(entry.table, entry.id, entry.opData)
                UpdateType.PATCH -> restApi.patch(entry.table, entry.id, entry.opData)
                UpdateType.DELETE -> restApi.delete(entry.table, entry.id)
            }
            // Throws on failure — batch.complete() is NOT called; PowerSync retries
        }
        batch.complete()
    }
}
```

This mirrors the web connector's `uploadData` contract: complete only when all entries succeed; rethrow on failure so PowerSync retries the batch.

---

## Room Entity Design

### Complete Table List

All 20 tables from the web PowerSync schema get Room entities. Column names must exactly match PostgreSQL snake_case (invariant D-4):

| Entity Class | Table Name | Domain |
|---|---|---|
| `UserEntity` | `users` | Core |
| `HouseholdEntity` | `households` | Core |
| `HouseholdMembershipEntity` | `household_memberships` | Core |
| `InitiativeEntity` | `initiatives` | Guidance |
| `EpicEntity` | `epics` | Guidance |
| `QuestEntity` | `quests` | Guidance |
| `RoutineEntity` | `routines` | Guidance |
| `FocusSessionEntity` | `focus_sessions` | Guidance |
| `DailyCheckinEntity` | `daily_checkins` | Guidance |
| `NoteEntity` | `notes` | Knowledge |
| `NoteSnapshotEntity` | `note_snapshots` | Knowledge |
| `EntityRelationEntity` | `entity_relations` | Knowledge |
| `TagEntity` | `tags` | Core |
| `EntityTagEntity` | `entity_tags` | Core |
| `TrackingItemEntity` | `tracking_items` | Tracking |
| `TrackingItemEventEntity` | `tracking_item_events` | Tracking |
| `TrackingLocationEntity` | `tracking_locations` | Tracking |
| `TrackingCategoryEntity` | `tracking_categories` | Tracking |
| `ShoppingListEntity` | `shopping_lists` | Tracking |
| `ShoppingListItemEntity` | `shopping_list_items` | Tracking |

**Not included (deferred):** `attachments` — deferred to Step 10 per Spec Won't Have.

### Entity Convention

```kotlin
@Entity(tableName = "quests")
data class QuestEntity(
    @PrimaryKey
    @ColumnInfo(name = "id") val id: String,
    @ColumnInfo(name = "title") val title: String,
    @ColumnInfo(name = "description") val description: String?,
    @ColumnInfo(name = "status") val status: String,
    @ColumnInfo(name = "priority") val priority: String?,
    @ColumnInfo(name = "due_date") val dueDate: String?,
    @ColumnInfo(name = "epic_id") val epicId: String?,
    @ColumnInfo(name = "initiative_id") val initiativeId: String?,
    @ColumnInfo(name = "routine_id") val routineId: String?,
    @ColumnInfo(name = "user_id") val userId: String,
    @ColumnInfo(name = "created_at") val createdAt: String,
    @ColumnInfo(name = "updated_at") val updatedAt: String,
    @ColumnInfo(name = "deleted_at") val deletedAt: String?,
)
```

All UUID columns map to `String`. All timestamp columns map to `String` (ISO-8601 text). Enums stored as `String` (no `@TypeConverter` overhead for small enum sets). Nullable columns use `String?`.

### DAO Convention

```kotlin
@Dao
interface QuestDao {
    @Query("SELECT * FROM quests WHERE user_id = :userId AND deleted_at IS NULL ORDER BY updated_at DESC")
    fun watchAll(userId: String): Flow<List<QuestEntity>>

    @Query("SELECT * FROM quests WHERE id = :id")
    fun watchById(id: String): Flow<QuestEntity?>

    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: QuestEntity)

    @Delete
    suspend fun delete(entity: QuestEntity)
}
```

---

## Koin Dependency Injection

### Module Structure

```
DatabaseModule
  ├── AltairDatabase (Room)
  ├── RoomConnectionPool (PowerSync)
  └── All DAOs (scoped to AltairDatabase)

RepositoryModule
  ├── QuestRepository → QuestRepositoryImpl(questDao, db)
  ├── NoteRepository → NoteRepositoryImpl(noteDao, db)
  └── ... (one impl per domain)

ViewModelModule
  ├── TodayViewModel
  ├── QuestDetailViewModel
  └── ... (one ViewModel per screen)

SyncModule
  ├── PowerSyncDatabase (from RoomConnectionPool)
  └── AltairPowerSyncConnector

PreferencesModule
  └── EncryptedSharedPreferences (token storage)

NetworkModule
  ├── OkHttpClient (with AuthInterceptor + AuthAuthenticator)
  └── Retrofit (with kotlinx.serialization converter)
```

All modules must pass `checkModules()` in tests (FA-014).

---

## HTTP Client and Auth Interceptor

Per ADR-022: OkHttp + Retrofit + kotlinx.serialization.

### Auth Interceptor

```kotlin
class AuthInterceptor(private val prefs: TokenPreferences) : Interceptor {
    override fun intercept(chain: Interceptor.Chain): Response {
        val token = prefs.accessToken ?: return chain.proceed(chain.request())
        val request = chain.request().newBuilder()
            .header("Authorization", "Bearer $token")
            .build()
        return chain.proceed(request)
    }
}
```

### Auth Authenticator (401 refresh)

```kotlin
class AuthAuthenticator(
    private val prefs: TokenPreferences,
    private val authApi: AuthApi,
) : Authenticator {
    override fun authenticate(route: Route?, response: Response): Request? {
        val refreshToken = prefs.refreshToken ?: return null
        val result = runBlocking { authApi.refresh(refreshToken) }
        // On success: store new tokens, retry with new access token (FA-019)
        // On failure: clear tokens, return null (triggers redirect to login)
        prefs.accessToken = result.accessToken
        prefs.refreshToken = result.refreshToken
        return response.request.newBuilder()
            .header("Authorization", "Bearer ${result.accessToken}")
            .build()
    }
}
```

Note: `runBlocking` inside `Authenticator` is acceptable per OkHttp's contract — the authenticator runs on OkHttp's I/O thread, not the main thread.

---

## Navigation Architecture

Per resolved OQ-001 (Today-as-hub):

```
NavHost (root)
  ├── AuthGraph
  │     ├── /auth/login
  │     └── /auth/register
  └── MainScaffold (BottomNavigation)
        ├── TodayGraph (tab 1)
        │     ├── /today (Today screen)
        │     ├── /today/guidance (All Guidance entry point)
        │     ├── /today/guidance/quests
        │     ├── /today/guidance/quests/{id}
        │     ├── /today/guidance/quests/{id}/focus
        │     ├── /today/guidance/initiatives
        │     ├── /today/guidance/initiatives/{id}
        │     ├── /today/guidance/epics/{id}
        │     ├── /today/guidance/routines
        │     └── /today/checkin
        ├── KnowledgeGraph (tab 2)
        │     ├── /knowledge/notes
        │     └── /knowledge/notes/{id}
        ├── TrackingGraph (tab 3)
        │     ├── /tracking/items
        │     ├── /tracking/items/{id}
        │     ├── /tracking/items/new
        │     ├── /tracking/shopping-lists
        │     ├── /tracking/shopping-lists/{id}
        │     ├── /tracking/locations
        │     └── /tracking/categories
        └── SettingsGraph (tab 4)
              └── /settings
```

Deep links (`altair://quest/{id}`, `altair://item/{id}`, `altair://checkin`) are handled by NavHost deep link declarations on the corresponding destinations.

---

## Background Sync (WorkManager)

```
SyncWorker (PeriodicWorkRequest)
  - interval: 15 minutes
  - constraints: NetworkType.CONNECTED
  - tag: "sync_periodic"

ExpeditesSyncWorker (OneTimeWorkRequest)
  - triggers: app foreground, connectivity change, after any local write
  - constraints: NetworkType.CONNECTED
  - tag: "sync_expedited"
  - expedited: true
```

`SyncCoordinator` (singleton in SyncModule) manages WorkManager enqueue calls and deduplication. `NetworkCallback` (registered in the Application class) triggers expedited sync on `onAvailable()`.

---

## Design System Migration

`Color.kt`, `Theme.kt`, and `Type.kt` currently use the default Material purple palette and `FontFamily.Default`. Migration plan:

1. **Color.kt** — Replace all default purple/pink tokens with Ethereal Canvas roles mapped to Material 3 color scheme (`primary`, `onPrimary`, `surface`, `onSurface`, etc.). Verified against `docs/specs/09-PLAT-001-android.md` §Design Tokens.
2. **Theme.kt** — Disable dynamic color (`dynamicColor = false`). Build `LightColorScheme` and `DarkColorScheme` from the Ethereal Canvas token set.
3. **Type.kt** — Load Manrope (for `displayLarge`, `headlineLarge`) and Plus Jakarta Sans (for `bodyLarge`, `labelMedium`) via `downloadableFonts` or bundled font resources.

Font loading approach: **bundle the fonts** as assets in `res/font/` (avoids runtime network request dependency on Google Fonts availability). Both fonts are available under OFL license.

---

## Dependency Version Additions to libs.versions.toml

The following entries need to be added (scaffold currently has none of these):

```toml
[versions]
room = "2.7.1"
powersync = "1.11.2"
datastore = "1.1.4"
workmanager = "2.10.1"
security-crypto = "1.1.0-alpha06"
okhttp = "4.12.0"
retrofit = "2.11.0"
kotlinx-serialization = "1.8.0"
kotlinx-datetime = "0.6.0"

[libraries]
# Room
room-runtime = { module = "androidx.room:room-runtime", version.ref = "room" }
room-ktx = { module = "androidx.room:room-ktx", version.ref = "room" }
room-compiler = { module = "androidx.room:room-compiler", version.ref = "room" }  # ksp
# PowerSync
powersync-core = { module = "com.powersync:core", version.ref = "powersync" }
powersync-room = { module = "com.powersync:integration-room", version.ref = "powersync" }
sqlite-bundled = { module = "androidx.sqlite:sqlite-bundled", version = "2.5.0" }
# DataStore / Preferences
security-crypto = { module = "androidx.security:security-crypto", version.ref = "security-crypto" }
# WorkManager
workmanager = { module = "androidx.work:work-runtime-ktx", version.ref = "workmanager" }
# HTTP
okhttp = { module = "com.squareup.okhttp3:okhttp", version.ref = "okhttp" }
okhttp-logging = { module = "com.squareup.okhttp3:logging-interceptor", version.ref = "okhttp" }
retrofit = { module = "com.squareup.retrofit2:retrofit", version.ref = "retrofit" }
retrofit-kotlinx-serialization = { module = "com.squareup.retrofit2:converter-kotlinx-serialization", version.ref = "retrofit" }
kotlinx-serialization-json = { module = "org.jetbrains.kotlinx:kotlinx-serialization-json", version.ref = "kotlinx-serialization" }
# Utilities
kotlinx-datetime = { module = "org.jetbrains.kotlinx:kotlinx-datetime", version.ref = "kotlinx-datetime" }

[plugins]
room = { id = "androidx.room", version.ref = "room" }
kotlinx-serialization = { id = "org.jetbrains.kotlin.plugin.serialization", version.ref = "kotlin" }
ksp = { id = "com.google.devtools.ksp", version = "2.3.20-2.0.21" }
```

---

## Risks and Unknowns

| Risk | Likelihood | Mitigation |
|---|---|---|
| `integration-room` alpha API breaks | Medium | Pin version; review CHANGELOG before upgrades; have a fallback path using raw PowerSync `execute()` without Room DAOs |
| Font bundling increases APK size | Low | Manrope + Plus Jakarta Sans add ~1–2 MB; acceptable for a self-hosted personal app |
| PowerSync views incompatible with Room schema verification | Known | Use `RoomConnectionPool`-provided `openHelperFactory`; follow SDK integration guide exactly |
| 20 Room entities with matching column names — D-4 violations found late | Medium | Schema parity test (`checkModules()` + column-name assertion tests) mirrors web `schema.spec.ts` approach |
| `EncryptedSharedPreferences` API deprecation | Low | API remains functional through API 36; note in code; acceptable per OQ-002 resolution |
| WorkManager expedited work limits (Android 12+) | Low | Use `setExpedited(OutOfQuotaPolicy.RUN_AS_NON_EXPEDITED_WORK_REQUEST)` fallback |

---

## Integration Points

| System | Transport | Auth |
|---|---|---|
| Altair Server REST API | OkHttp + Retrofit | Bearer JWT (AuthInterceptor) with refresh (AuthAuthenticator) |
| PowerSync Service | PowerSync SDK internal WebSocket | `fetchCredentials()` returns server-issued sync token |
| Google Fonts (dev only) | Network | None (fonts bundled in production) |

---

## Open Questions

None. OQ-001 (navigation) and OQ-002 (token storage) resolved in Spec.md. ADR-022 and ADR-023 record the remaining architectural decisions.
