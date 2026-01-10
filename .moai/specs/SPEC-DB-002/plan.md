# SPEC-DB-002 Implementation Plan

## Traceability

- **SPEC Reference**: [SPEC-DB-002](spec.md) - Mobile SQLite Integration with SQLDelight
- **GitHub Issue**: #44 - 1.4 Mobile SQLite Integration
- **Related SPEC**: [SPEC-DB-001](../SPEC-DB-001/spec.md) - Desktop SurrealDB Integration

---

## Overview

This plan outlines the implementation of SQLDelight-based SQLite integration for Android and iOS mobile platforms. The implementation maintains schema parity with SPEC-DB-001's desktop SurrealDB entities while leveraging SQLDelight for type-safe, cross-platform database access.

---

## Phase 1: Gradle Configuration

### Primary Goal

Configure SQLDelight plugin and dependencies in the Gradle build system.

### Tasks

#### 1.1 Update Version Catalog

Add SQLDelight entries to `gradle/libs.versions.toml`:

```toml
[versions]
sqldelight = "2.0.2"

[libraries]
sqldelight-runtime = { module = "app.cash.sqldelight:runtime", version.ref = "sqldelight" }
sqldelight-android-driver = { module = "app.cash.sqldelight:android-driver", version.ref = "sqldelight" }
sqldelight-native-driver = { module = "app.cash.sqldelight:native-driver", version.ref = "sqldelight" }
sqldelight-coroutines = { module = "app.cash.sqldelight:coroutines-extensions", version.ref = "sqldelight" }

[plugins]
sqldelight = { id = "app.cash.sqldelight", version.ref = "sqldelight" }
```

#### 1.2 Apply SQLDelight Plugin

Update `shared/build.gradle.kts`:

```kotlin
plugins {
    alias(libs.plugins.sqldelight)
}

sqldelight {
    databases {
        create("AltairDatabase") {
            packageName.set("com.getaltair.altair.database")
            schemaOutputDirectory.set(file("build/sqldelight/schema"))
            verifyMigrations.set(true)
        }
    }
}
```

#### 1.3 Add Platform Dependencies

Configure source set dependencies in `shared/build.gradle.kts`:

```kotlin
sourceSets {
    commonMain.dependencies {
        implementation(libs.sqldelight.runtime)
        implementation(libs.sqldelight.coroutines)
    }
    androidMain.dependencies {
        implementation(libs.sqldelight.android.driver)
    }
    iosMain.dependencies {
        implementation(libs.sqldelight.native.driver)
    }
}
```

### Deliverables

- Updated `gradle/libs.versions.toml` with SQLDelight entries
- Updated `shared/build.gradle.kts` with plugin and configuration
- Successful Gradle sync without errors

### Validation

- Run `./gradlew shared:dependencies` to verify dependency resolution
- Run `./gradlew shared:compileKotlinAndroid` to verify Android compilation
- Run `./gradlew shared:compileKotlinIosArm64` to verify iOS compilation

---

## Phase 2: Schema Definition

### Primary Goal

Create SQLDelight schema files matching SPEC-DB-001 entity conventions.

### Tasks

#### 2.1 Create SQLDelight Directory Structure

```bash
mkdir -p shared/src/commonMain/sqldelight/com/getaltair/altair/database
```

#### 2.2 Define Quest Schema

Create `shared/src/commonMain/sqldelight/com/getaltair/altair/database/Quest.sq`:

```sql
CREATE TABLE Quest (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'pending',
    epic_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_quest_status ON Quest(status) WHERE deleted_at IS NULL;
CREATE INDEX idx_quest_epic ON Quest(epic_id) WHERE deleted_at IS NULL;

-- Queries
selectAll:
SELECT * FROM Quest WHERE deleted_at IS NULL ORDER BY created_at DESC;

selectById:
SELECT * FROM Quest WHERE id = ? AND deleted_at IS NULL;

insert:
INSERT INTO Quest (id, title, description, status, epic_id, created_at, updated_at, deleted_at, sync_version)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);

updateById:
UPDATE Quest SET title = ?, description = ?, status = ?, epic_id = ?, updated_at = ?, sync_version = ?
WHERE id = ?;

softDeleteById:
UPDATE Quest SET deleted_at = ?, updated_at = ? WHERE id = ?;
```

#### 2.3 Define Note Schema

Create `shared/src/commonMain/sqldelight/com/getaltair/altair/database/Note.sq`:

```sql
CREATE TABLE Note (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    content TEXT,
    folder_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_note_folder ON Note(folder_id) WHERE deleted_at IS NULL;

-- Queries
selectAll:
SELECT * FROM Note WHERE deleted_at IS NULL ORDER BY created_at DESC;

selectById:
SELECT * FROM Note WHERE id = ? AND deleted_at IS NULL;

selectByFolderId:
SELECT * FROM Note WHERE folder_id = ? AND deleted_at IS NULL ORDER BY created_at DESC;

insert:
INSERT INTO Note (id, title, content, folder_id, created_at, updated_at, deleted_at, sync_version)
VALUES (?, ?, ?, ?, ?, ?, ?, ?);

updateById:
UPDATE Note SET title = ?, content = ?, folder_id = ?, updated_at = ?, sync_version = ?
WHERE id = ?;

softDeleteById:
UPDATE Note SET deleted_at = ?, updated_at = ? WHERE id = ?;
```

#### 2.4 Define Item Schema

Create `shared/src/commonMain/sqldelight/com/getaltair/altair/database/Item.sq`:

```sql
CREATE TABLE Item (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    location_id TEXT,
    container_id TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,
    sync_version INTEGER NOT NULL DEFAULT 0
);

CREATE INDEX idx_item_location ON Item(location_id) WHERE deleted_at IS NULL;
CREATE INDEX idx_item_container ON Item(container_id) WHERE deleted_at IS NULL;

-- Queries
selectAll:
SELECT * FROM Item WHERE deleted_at IS NULL ORDER BY created_at DESC;

selectById:
SELECT * FROM Item WHERE id = ? AND deleted_at IS NULL;

selectByLocationId:
SELECT * FROM Item WHERE location_id = ? AND deleted_at IS NULL ORDER BY created_at DESC;

selectByContainerId:
SELECT * FROM Item WHERE container_id = ? AND deleted_at IS NULL ORDER BY created_at DESC;

insert:
INSERT INTO Item (id, name, description, location_id, container_id, created_at, updated_at, deleted_at, sync_version)
VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?);

updateById:
UPDATE Item SET name = ?, description = ?, location_id = ?, container_id = ?, updated_at = ?, sync_version = ?
WHERE id = ?;

softDeleteById:
UPDATE Item SET deleted_at = ?, updated_at = ? WHERE id = ?;
```

### Deliverables

- `Quest.sq` with table definition, indexes, and CRUD queries
- `Note.sq` with table definition, indexes, and CRUD queries
- `Item.sq` with table definition, indexes, and CRUD queries

### Validation

- Run `./gradlew shared:generateCommonMainAltairDatabaseInterface` to verify code generation
- Verify generated code in `shared/build/generated/sqldelight/`

---

## Phase 3: Database Connection

### Primary Goal

Implement platform-specific SQLite drivers using expect/actual pattern.

### Tasks

#### 3.1 Create Common Driver Factory

Create `shared/src/commonMain/kotlin/com/getaltair/altair/data/db/DriverFactory.kt`:

```kotlin
package com.getaltair.altair.data.db

import app.cash.sqldelight.db.SqlDriver

expect class DriverFactory {
    fun createDriver(): SqlDriver
}
```

#### 3.2 Implement Android Driver

Create `shared/src/androidMain/kotlin/com/getaltair/altair/data/db/DriverFactory.android.kt`:

```kotlin
package com.getaltair.altair.data.db

import android.content.Context
import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.android.AndroidSqliteDriver
import com.getaltair.altair.database.AltairDatabase

actual class DriverFactory(private val context: Context) {
    actual fun createDriver(): SqlDriver {
        return AndroidSqliteDriver(
            schema = AltairDatabase.Schema,
            context = context,
            name = "altair.db"
        )
    }
}
```

#### 3.3 Implement iOS Driver

Create `shared/src/iosMain/kotlin/com/getaltair/altair/data/db/DriverFactory.ios.kt`:

```kotlin
package com.getaltair.altair.data.db

import app.cash.sqldelight.db.SqlDriver
import app.cash.sqldelight.driver.native.NativeSqliteDriver
import com.getaltair.altair.database.AltairDatabase

actual class DriverFactory {
    actual fun createDriver(): SqlDriver {
        return NativeSqliteDriver(
            schema = AltairDatabase.Schema,
            name = "altair.db"
        )
    }
}
```

#### 3.4 Create Database Singleton

Create `shared/src/commonMain/kotlin/com/getaltair/altair/data/db/DatabaseProvider.kt`:

```kotlin
package com.getaltair.altair.data.db

import com.getaltair.altair.database.AltairDatabase

object DatabaseProvider {
    private var database: AltairDatabase? = null

    fun initialize(driverFactory: DriverFactory) {
        if (database == null) {
            database = AltairDatabase(driverFactory.createDriver())
        }
    }

    fun getDatabase(): AltairDatabase {
        return database ?: throw IllegalStateException("Database not initialized")
    }

    fun close() {
        database = null
    }
}
```

### Deliverables

- `DriverFactory.kt` expect declaration in commonMain
- `DriverFactory.android.kt` actual implementation for Android
- `DriverFactory.ios.kt` actual implementation for iOS
- `DatabaseProvider.kt` singleton for database access

### Validation

- Android: Build and run on Android emulator, verify database file created
- iOS: Build and run on iOS simulator, verify database file created

---

## Phase 4: Repository Implementation

### Primary Goal

Implement type-safe repository classes using generated SQLDelight queries.

### Tasks

#### 4.1 Create Timestamp Utility

Create `shared/src/commonMain/kotlin/com/getaltair/altair/data/util/Timestamps.kt`:

```kotlin
package com.getaltair.altair.data.util

import kotlinx.datetime.Clock
import kotlinx.datetime.Instant

object Timestamps {
    fun now(): String = Clock.System.now().toString()

    fun parse(timestamp: String): Instant = Instant.parse(timestamp)
}
```

#### 4.2 Implement QuestRepository

Create `shared/src/commonMain/kotlin/com/getaltair/altair/data/repository/QuestRepositoryImpl.kt`:

```kotlin
package com.getaltair.altair.data.repository

import com.getaltair.altair.data.db.DatabaseProvider
import com.getaltair.altair.data.entity.Quest
import com.getaltair.altair.data.util.Timestamps
import com.github.f4b6a3.ulid.UlidCreator

class QuestRepositoryImpl : Repository<Quest, String> {
    private val queries get() = DatabaseProvider.getDatabase().questQueries

    override suspend fun create(entity: Quest): Quest {
        val id = UlidCreator.getUlid().toString()
        val now = Timestamps.now()
        queries.insert(
            id = id,
            title = entity.title,
            description = entity.description,
            status = entity.status,
            epic_id = entity.epicId,
            created_at = now,
            updated_at = now,
            deleted_at = null,
            sync_version = 0
        )
        return entity.copy(id = id, createdAt = now, updatedAt = now)
    }

    override suspend fun findById(id: String): Quest? {
        return queries.selectById(id).executeAsOneOrNull()?.toQuest()
    }

    override suspend fun findAll(): List<Quest> {
        return queries.selectAll().executeAsList().map { it.toQuest() }
    }

    override suspend fun update(entity: Quest): Quest {
        val now = Timestamps.now()
        queries.updateById(
            title = entity.title,
            description = entity.description,
            status = entity.status,
            epic_id = entity.epicId,
            updated_at = now,
            sync_version = entity.syncVersion + 1,
            id = entity.id
        )
        return entity.copy(updatedAt = now, syncVersion = entity.syncVersion + 1)
    }

    override suspend fun delete(id: String): Boolean {
        val now = Timestamps.now()
        queries.softDeleteById(deleted_at = now, updated_at = now, id = id)
        return true
    }
}
```

#### 4.3 Implement NoteRepository

Create `shared/src/commonMain/kotlin/com/getaltair/altair/data/repository/NoteRepositoryImpl.kt` following the same pattern as QuestRepository.

#### 4.4 Implement ItemRepository

Create `shared/src/commonMain/kotlin/com/getaltair/altair/data/repository/ItemRepositoryImpl.kt` following the same pattern as QuestRepository.

### Deliverables

- `Timestamps.kt` utility for ISO 8601 timestamp handling
- `QuestRepositoryImpl.kt` with full CRUD operations
- `NoteRepositoryImpl.kt` with full CRUD operations
- `ItemRepositoryImpl.kt` with full CRUD operations

### Validation

- Unit tests for each repository method
- Verify ULID generation produces valid 26-character strings
- Verify timestamps are valid ISO 8601 format

---

## Phase 5: Testing and Verification

### Primary Goal

Create integration tests verifying database persistence on Android and iOS.

### Tasks

#### 5.1 Create Android Integration Tests

Create `shared/src/androidTest/kotlin/com/getaltair/altair/data/repository/QuestRepositoryTest.kt`:

```kotlin
package com.getaltair.altair.data.repository

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.getaltair.altair.data.db.DatabaseProvider
import com.getaltair.altair.data.db.DriverFactory
import com.getaltair.altair.data.entity.Quest
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull

@RunWith(AndroidJUnit4::class)
class QuestRepositoryTest {
    private lateinit var repository: QuestRepositoryImpl

    @Before
    fun setup() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        DatabaseProvider.initialize(DriverFactory(context))
        repository = QuestRepositoryImpl()
    }

    @After
    fun teardown() {
        DatabaseProvider.close()
    }

    @Test
    fun createAndReadQuest() = runTest {
        val quest = Quest(title = "Test Quest", status = "pending")
        val created = repository.create(quest)

        assertNotNull(created.id)
        assertEquals("Test Quest", created.title)

        val retrieved = repository.findById(created.id)
        assertNotNull(retrieved)
        assertEquals(created.id, retrieved.id)
    }

    @Test
    fun softDeleteExcludesFromQueries() = runTest {
        val quest = Quest(title = "Delete Test", status = "pending")
        val created = repository.create(quest)

        repository.delete(created.id)

        val retrieved = repository.findById(created.id)
        assertNull(retrieved)
    }
}
```

#### 5.2 Create iOS Integration Tests

Create `shared/src/iosTest/kotlin/com/getaltair/altair/data/repository/QuestRepositoryTest.kt` with equivalent tests for iOS platform.

#### 5.3 Create Note and Item Repository Tests

Replicate test patterns for NoteRepository and ItemRepository on both platforms.

### Deliverables

- Android integration tests for Quest, Note, and Item repositories
- iOS integration tests for Quest, Note, and Item repositories
- Test coverage report

### Validation

- Run Android tests: `./gradlew shared:connectedAndroidTest`
- Run iOS tests: `./gradlew shared:iosSimulatorArm64Test`
- Verify all tests pass on both platforms

---

## Technical Approach

### Architecture Decisions

1. **Expect/Actual Pattern**: Used for platform-specific driver instantiation while maintaining common repository interfaces
2. **Soft Delete**: All deletions set `deleted_at` timestamp rather than removing records
3. **ULID IDs**: 26-character sortable unique identifiers generated at create time
4. **ISO 8601 Timestamps**: String format for cross-platform compatibility
5. **Singleton Database**: Single database instance per application lifecycle

### Code Organization

```
shared/src/
├── commonMain/
│   ├── kotlin/.../data/
│   │   ├── db/
│   │   │   ├── DriverFactory.kt          # expect
│   │   │   └── DatabaseProvider.kt       # singleton
│   │   ├── entity/
│   │   │   ├── Quest.kt
│   │   │   ├── Note.kt
│   │   │   └── Item.kt
│   │   ├── repository/
│   │   │   ├── Repository.kt             # interface
│   │   │   ├── QuestRepositoryImpl.kt
│   │   │   ├── NoteRepositoryImpl.kt
│   │   │   └── ItemRepositoryImpl.kt
│   │   └── util/
│   │       └── Timestamps.kt
│   └── sqldelight/.../database/
│       ├── Quest.sq
│       ├── Note.sq
│       └── Item.sq
├── androidMain/
│   └── kotlin/.../data/db/
│       └── DriverFactory.android.kt      # actual
└── iosMain/
    └── kotlin/.../data/db/
        └── DriverFactory.ios.kt          # actual
```

### Risk Mitigation

| Risk                               | Mitigation Strategy                                  |
| ---------------------------------- | ---------------------------------------------------- |
| SQLDelight version incompatibility | Pin to 2.0.2, test compilation early in Phase 1      |
| iOS driver issues                  | Test on both arm64 device and simulatorArm64 targets |
| Schema mismatch                    | Direct mapping from SPEC-DB-001 entity definitions   |
| Query performance                  | Add indexes in schema, profile in Phase 5            |

---

## Milestones

### Milestone 1: Build Configuration Complete

- Phase 1 deliverables complete
- Gradle sync successful
- Compilation passes for Android and iOS targets

### Milestone 2: Schema and Code Generation Complete

- Phase 2 deliverables complete
- SQLDelight generates type-safe code
- No compilation errors in generated code

### Milestone 3: Database Connection Operational

- Phase 3 deliverables complete
- Database initializes on Android emulator
- Database initializes on iOS simulator

### Milestone 4: Repository Implementation Complete

- Phase 4 deliverables complete
- All repository methods implemented
- ULID and timestamp utilities working

### Milestone 5: Verification Complete

- Phase 5 deliverables complete
- All integration tests pass on Android
- All integration tests pass on iOS
- Ready for SPEC completion

---

## Dependencies

### Prerequisites

- Kotlin 2.3.0 configured in project
- Android SDK 36 installed
- Xcode with iOS simulator support
- ulid-creator library (already in project)
- kotlinx-datetime library (already in project)

### External Documentation

- [SQLDelight Documentation](https://cashapp.github.io/sqldelight/2.0.2/)
- [SQLDelight Multiplatform Guide](https://cashapp.github.io/sqldelight/2.0.2/multiplatform_sqlite/)
- [Kotlin Multiplatform Mobile](https://kotlinlang.org/docs/multiplatform-mobile-getting-started.html)
