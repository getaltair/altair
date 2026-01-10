---
id: SPEC-DB-002
version: "1.0.0"
status: draft
created: "2026-01-10"
updated: "2026-01-10"
author: Robert
priority: high
---

# HISTORY

| Version | Date       | Author | Changes                                 |
| ------- | ---------- | ------ | --------------------------------------- |
| 1.0.0   | 2026-01-10 | Robert | Initial SPEC creation for Mobile SQLite |

---

# SPEC-DB-002: Mobile SQLite Integration with SQLDelight

## Metadata

| Field            | Value                                       |
| ---------------- | ------------------------------------------- |
| **SPEC ID**      | SPEC-DB-002                                 |
| **Title**        | Mobile SQLite Integration with SQLDelight   |
| **GitHub Issue** | #44                                         |
| **Priority**     | High                                        |
| **Status**       | Draft                                       |
| **Created**      | 2026-01-10                                  |
| **Labels**       | enhancement, database, mobile, android, ios |

## Traceability

- **Source**: [GitHub Issue #44](https://github.com/getaltair/altair/issues/44) - 1.4 Mobile SQLite Integration
- **Related SPEC**: [SPEC-DB-001](../SPEC-DB-001/spec.md) - Desktop SurrealDB Integration (schema parity required)
- **Implementation Plan**: Section 1.4 - Mobile SQLite Integration

---

## Environment

### Platform Context

- **Framework**: Kotlin Multiplatform with Compose Multiplatform
- **Targets**: Android and iOS mobile platforms
- **Build System**: Gradle with Kotlin DSL, version catalog at `gradle/libs.versions.toml`
- **Kotlin Version**: 2.3.0
- **SQLDelight Version**: 2.0.2 (compatible with Kotlin 2.3.0)

### Android Context

- **Compile SDK**: 36
- **Minimum SDK**: 24
- **Target SDK**: 36
- **Driver**: AndroidSqliteDriver from SQLDelight

### iOS Context

- **Architectures**: arm64 (device) + simulatorArm64 (simulator)
- **Driver**: NativeSqliteDriver from SQLDelight

### Database Context

- **Database Engine**: SQLite (platform-native)
- **Code Generation**: SQLDelight type-safe query generation
- **Package Name**: com.getaltair.altair.database

---

## Assumptions

| ID  | Assumption                                                       | Confidence | Risk if Wrong                                      |
| --- | ---------------------------------------------------------------- | ---------- | -------------------------------------------------- |
| A1  | SQLDelight 2.0.2 is compatible with Kotlin 2.3.0                 | High       | Need to update SQLDelight or adjust Kotlin version |
| A2  | AndroidSqliteDriver works with SDK 24-36 range                   | High       | May need min SDK adjustment                        |
| A3  | NativeSqliteDriver supports arm64 and simulatorArm64 iOS targets | High       | Platform-specific driver configuration required    |
| A4  | Schema from SPEC-DB-001 can be directly mapped to SQLite DDL     | High       | Schema adaptation may require type conversions     |
| A5  | ULID string IDs work efficiently as SQLite TEXT primary keys     | Medium     | May need index optimization for query performance  |

---

## Requirements

### R1: SQLDelight Dependency Integration (Ubiquitous)

The SQLDelight plugin shall be compatible with Kotlin 2.3.0.

#### R1.1: Version Catalog Entry

The system shall define SQLDelight version in `gradle/libs.versions.toml`:

- Version key: `sqldelight`
- Version value: `2.0.2`
- Plugin alias: `sqldelight`
- Library aliases: `sqldelight-android-driver`, `sqldelight-native-driver`, `sqldelight-runtime`

#### R1.2: Gradle Plugin Application

The system shall apply the SQLDelight plugin in `shared/build.gradle.kts`:

- Plugin ID: `app.cash.sqldelight`
- Database name: `AltairDatabase`
- Package name: `com.getaltair.altair.database`

#### R1.3: Platform Driver Dependencies

The system shall include platform-specific drivers:

- androidMain: `app.cash.sqldelight:android-driver`
- iosMain: `app.cash.sqldelight:native-driver`
- commonMain: `app.cash.sqldelight:runtime`

---

### R2: Schema Definition (Event-Driven)

**WHEN** building mobile targets, **THEN** SQLDelight shall generate type-safe query classes from .sq schema files.

#### R2.1: Quest Table Schema

The system shall define Quest.sq with the following columns:

| Column       | Type    | Constraints                  |
| ------------ | ------- | ---------------------------- |
| id           | TEXT    | PRIMARY KEY, NOT NULL        |
| title        | TEXT    | NOT NULL                     |
| description  | TEXT    | NULL                         |
| status       | TEXT    | NOT NULL, DEFAULT 'pending'  |
| epic_id      | TEXT    | NULL (foreign key reference) |
| created_at   | TEXT    | NOT NULL (ISO 8601)          |
| updated_at   | TEXT    | NOT NULL (ISO 8601)          |
| deleted_at   | TEXT    | NULL (ISO 8601, soft delete) |
| sync_version | INTEGER | NOT NULL, DEFAULT 0          |

#### R2.2: Note Table Schema

The system shall define Note.sq with the following columns:

| Column       | Type    | Constraints                  |
| ------------ | ------- | ---------------------------- |
| id           | TEXT    | PRIMARY KEY, NOT NULL        |
| title        | TEXT    | NOT NULL                     |
| content      | TEXT    | NULL                         |
| folder_id    | TEXT    | NULL (foreign key reference) |
| created_at   | TEXT    | NOT NULL (ISO 8601)          |
| updated_at   | TEXT    | NOT NULL (ISO 8601)          |
| deleted_at   | TEXT    | NULL (ISO 8601, soft delete) |
| sync_version | INTEGER | NOT NULL, DEFAULT 0          |

#### R2.3: Item Table Schema

The system shall define Item.sq with the following columns:

| Column       | Type    | Constraints                  |
| ------------ | ------- | ---------------------------- |
| id           | TEXT    | PRIMARY KEY, NOT NULL        |
| name         | TEXT    | NOT NULL                     |
| description  | TEXT    | NULL                         |
| location_id  | TEXT    | NULL (foreign key reference) |
| container_id | TEXT    | NULL (foreign key reference) |
| created_at   | TEXT    | NOT NULL (ISO 8601)          |
| updated_at   | TEXT    | NOT NULL (ISO 8601)          |
| deleted_at   | TEXT    | NULL (ISO 8601, soft delete) |
| sync_version | INTEGER | NOT NULL, DEFAULT 0          |

#### R2.4: Index Definitions

The system shall define indexes for common query patterns:

- `idx_quest_status` on Quest(status) WHERE deleted_at IS NULL
- `idx_quest_epic` on Quest(epic_id) WHERE deleted_at IS NULL
- `idx_note_folder` on Note(folder_id) WHERE deleted_at IS NULL
- `idx_item_location` on Item(location_id) WHERE deleted_at IS NULL
- `idx_item_container` on Item(container_id) WHERE deleted_at IS NULL

---

### R3: Type-Safe Query Generation (State-Driven)

**WHILE** database connection is open, **THEN** queries shall execute synchronously using generated type-safe classes.

#### R3.1: Generated Query Interface

**WHEN** SQLDelight generates code, **THEN** the following query methods shall be available for each entity:

- `insert(entity)`: Insert new record with all fields
- `selectById(id)`: Select single record by primary key
- `selectAll()`: Select all non-deleted records
- `updateById(entity)`: Update record by ID
- `softDeleteById(id, deleted_at)`: Set deleted_at timestamp

#### R3.2: Query Return Types

**WHEN** calling generated queries, **THEN** return types shall be:

- Single record queries: `Entity?` (nullable for not found)
- Collection queries: `List<Entity>`
- Insert/Update/Delete: `Unit` (void)

---

### R4: Database Connection (Event-Driven)

**WHEN** the mobile application initializes, **THEN** the system shall establish SQLite connection using platform-specific drivers.

#### R4.1: Connection Factory Pattern

The system shall implement expect/actual pattern for database driver:

- `expect fun createDriver(): SqlDriver` in commonMain
- `actual fun createDriver(): SqlDriver` in androidMain using AndroidSqliteDriver
- `actual fun createDriver(): SqlDriver` in iosMain using NativeSqliteDriver

#### R4.2: Android Driver Configuration

**WHEN** initializing on Android, **THEN** the system shall:

- Use Context to create AndroidSqliteDriver
- Database file name: `altair.db`
- Schema version: 1

#### R4.3: iOS Driver Configuration

**WHEN** initializing on iOS, **THEN** the system shall:

- Use NativeSqliteDriver with database name
- Database file name: `altair.db`
- Schema version: 1

#### R4.4: Connection Lifecycle

**WHEN** database connection is created, **THEN** the system shall:

- Store connection reference in singleton or DI container
- Provide connection to repositories via dependency injection
- Close connection on application termination

---

### R5: Repository Implementation (Ubiquitous)

The system shall implement repositories conforming to the existing `Repository<T, ID>` interface from commonMain.

#### R5.1: QuestRepository

The system shall implement QuestRepository with:

- `create(quest: Quest): Quest` - Generate ULID, set timestamps, insert
- `findById(id: String): Quest?` - Query by ID, exclude soft-deleted
- `findAll(): List<Quest>` - Query all non-deleted, order by created_at
- `update(quest: Quest): Quest` - Update fields, set updated_at
- `delete(id: String): Boolean` - Soft delete, set deleted_at

#### R5.2: NoteRepository

The system shall implement NoteRepository with:

- `create(note: Note): Note` - Generate ULID, set timestamps, insert
- `findById(id: String): Note?` - Query by ID, exclude soft-deleted
- `findAll(): List<Note>` - Query all non-deleted, order by created_at
- `findByFolderId(folderId: String): List<Note>` - Query by folder
- `update(note: Note): Note` - Update fields, set updated_at
- `delete(id: String): Boolean` - Soft delete, set deleted_at

#### R5.3: ItemRepository

The system shall implement ItemRepository with:

- `create(item: Item): Item` - Generate ULID, set timestamps, insert
- `findById(id: String): Item?` - Query by ID, exclude soft-deleted
- `findAll(): List<Item>` - Query all non-deleted, order by created_at
- `findByLocationId(locationId: String): List<Item>` - Query by location
- `findByContainerId(containerId: String): List<Item>` - Query by container
- `update(item: Item): Item` - Update fields, set updated_at
- `delete(id: String): Boolean` - Soft delete, set deleted_at

#### R5.4: ULID Generation

All repository create operations shall generate ULID using:

- Library: `com.github.f4b6a3:ulid-creator` (already in project)
- Format: 26-character string (Crockford Base32)

#### R5.5: Timestamp Handling

All repository operations shall use ISO 8601 format:

- Format: `yyyy-MM-dd'T'HH:mm:ss.SSS'Z'`
- Timezone: UTC
- Library: `kotlinx-datetime` for cross-platform support

---

### R6: Persistence Verification (Event-Driven)

**WHEN** testing database integration, **THEN** the system shall verify data persistence on both platforms.

#### R6.1: Android Verification

**WHEN** running integration tests on Android emulator, **THEN** the system shall:

- Create test record with known data
- Read test record back
- Verify all fields match
- Update test record
- Verify update persists
- Soft-delete test record
- Verify record excluded from queries
- Clean up test data

#### R6.2: iOS Verification

**WHEN** running integration tests on iOS simulator, **THEN** the system shall:

- Create test record with known data
- Read test record back
- Verify all fields match
- Update test record
- Verify update persists
- Soft-delete test record
- Verify record excluded from queries
- Clean up test data

---

### R7: Unwanted Behavior Requirements

#### R7.1: No Plaintext Sensitive Data

The system shall NOT store sensitive data (passwords, tokens) in plaintext.

#### R7.2: No Hard-Deleted Records

The system shall NOT permanently delete records; all deletions shall be soft deletes.

#### R7.3: No Blocking UI Thread

The system shall NOT execute database queries on the main/UI thread.

#### R7.4: Database Corruption Handling

**IF** database file is corrupted, **THEN** the app shall show error and offer reset option.

---

### R8: Optional Feature Requirements

#### R8.1: Encryption Support

**WHERE** encryption is enabled, **THEN** database shall use SQLCipher for at-rest encryption.

#### R8.2: Database Migrations

**WHERE** schema evolution is required, the system may implement SQLDelight migration scripts.

#### R8.3: Query Caching

**WHERE** performance profiling indicates need, the system may implement query result caching.

---

## Specifications

### File Structure

```
shared/src/
├── commonMain/
│   ├── kotlin/com/getaltair/altair/
│   │   └── data/
│   │       ├── repository/
│   │       │   └── Repository.kt        # Existing interface
│   │       └── entity/
│   │           ├── Quest.kt             # Quest data class
│   │           ├── Note.kt              # Note data class
│   │           └── Item.kt              # Item data class
│   └── sqldelight/
│       └── com/getaltair/altair/database/
│           ├── Quest.sq                 # Quest schema and queries
│           ├── Note.sq                  # Note schema and queries
│           └── Item.sq                  # Item schema and queries
│
├── androidMain/kotlin/com/getaltair/altair/
│   └── data/
│       └── db/
│           └── AndroidSqliteConnection.kt  # Android driver
│
└── iosMain/kotlin/com/getaltair/altair/
    └── data/
        └── db/
            └── IosSqliteConnection.kt      # iOS driver

shared/src/androidTest/
└── kotlin/com/getaltair/altair/
    └── data/
        └── repository/
            ├── QuestRepositoryTest.kt
            ├── NoteRepositoryTest.kt
            └── ItemRepositoryTest.kt

shared/src/iosTest/
└── kotlin/com/getaltair/altair/
    └── data/
        └── repository/
            ├── QuestRepositoryTest.kt
            ├── NoteRepositoryTest.kt
            └── ItemRepositoryTest.kt
```

### Technical Constraints

| Constraint        | Value                               |
| ----------------- | ----------------------------------- |
| SQLDelight Mode   | Gradle plugin code generation       |
| Storage Engine    | SQLite (platform-native)            |
| ID Format         | ULID strings (26 characters)        |
| Timestamp Format  | ISO 8601 strings                    |
| Deletion Strategy | Soft delete with `deleted_at` field |
| Sync Field        | `sync_version` INTEGER              |

### SQLDelight Configuration

```kotlin
// shared/build.gradle.kts
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

---

## Dependencies

### External Libraries

| Library                                | Version | Purpose                    | Target      |
| -------------------------------------- | ------- | -------------------------- | ----------- |
| app.cash.sqldelight                    | 2.0.2   | SQL code generation plugin | Plugin      |
| app.cash.sqldelight:runtime            | 2.0.2   | Runtime queries            | commonMain  |
| app.cash.sqldelight:android-driver     | 2.0.2   | Android SQLite driver      | androidMain |
| app.cash.sqldelight:native-driver      | 2.0.2   | iOS SQLite driver          | iosMain     |
| com.github.f4b6a3:ulid-creator         | 5.2.3+  | ULID generation            | commonMain  |
| org.jetbrains.kotlinx:kotlinx-datetime | 0.6.0+  | Cross-platform timestamps  | commonMain  |

### Internal Dependencies

| Module | Dependency Type                          |
| ------ | ---------------------------------------- |
| shared | Entity definitions, repository interface |

---

## Risks and Mitigations

| Risk                                       | Probability | Impact | Mitigation                                      |
| ------------------------------------------ | ----------- | ------ | ----------------------------------------------- |
| SQLDelight incompatibility with Kotlin 2.3 | Low         | High   | Version 2.0.2 confirmed compatible; test early  |
| iOS driver initialization issues           | Medium      | High   | Test on both device and simulator architectures |
| Schema mismatch with SPEC-DB-001 entities  | Low         | Medium | Direct mapping from existing entity conventions |
| Query performance with ULID text PKs       | Medium      | Medium | Add indexes; profile query execution times      |
| Database file corruption on crash          | Low         | High   | SQLite has built-in journaling; test recovery   |

---

## Non-Functional Requirements

### NFR-001: Query Performance

Single record operations (insert, select by ID, update, delete) shall complete in under 50ms.

### NFR-002: Database File Size

Database file size shall be efficient using SQLite native compression and appropriate data types.

### NFR-003: Thread Safety

Database access shall be thread-safe with proper connection management per platform.

### NFR-004: Cross-Platform Consistency

Query results shall be consistent across Android and iOS platforms for identical data.

---

## Related SPECs

- **Predecessor**: SPEC-DB-001 (Desktop SurrealDB Integration - completed, provides schema parity reference)
- **Successor**: SPEC-DB-003 (Sync Implementation - future)
- **Related Issue**: GitHub Issue #44 - 1.4 Mobile SQLite Integration
