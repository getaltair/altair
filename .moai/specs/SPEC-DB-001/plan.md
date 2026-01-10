# SPEC-DB-001: Implementation Plan

## Traceability

- **SPEC Reference**: [SPEC-DB-001](./spec.md)
- **GitHub Issue**: #43

---

## Implementation Strategy

### Overview

This implementation establishes the foundational database layer for Altair desktop. The approach follows clean architecture principles with a focus on testability and platform abstraction. Implementation proceeds in phases to minimize risk and enable early verification.

### Architecture Decisions

| Decision                      | Rationale                                                  |
| ----------------------------- | ---------------------------------------------------------- |
| Repository in `shared` module | Enables future code sharing with server module             |
| expect/actual for paths       | Clean platform abstraction without conditional compilation |
| Suspend functions for DB ops  | Non-blocking UI; integrates with Compose coroutines        |
| Singleton connection pattern  | Matches SurrealDB's single-writer model                    |
| Test entity for verification  | Proves integration without coupling to domain entities     |

---

## Phase 1: Dependency Setup

**Goal**: Add surrealdb.java to the project with proper version catalog configuration.

### Tasks

#### 1.1 Research surrealdb.java Latest Version

- Check Maven Central for latest stable surrealdb.java release
- Verify compatibility with Kotlin 2.3.0 and JVM 21
- Document any platform-specific requirements

#### 1.2 Update Version Catalog

File: `gradle/libs.versions.toml`

Add entries:

```toml
[versions]
surrealdb-java = "X.Y.Z"  # Latest stable version
ulid = "5.2.3"

[libraries]
surrealdb-java = { module = "com.surrealdb:surrealdb-java", version.ref = "surrealdb-java" }
ulid-creator = { module = "com.github.f4b6a3:ulid-creator", version.ref = "ulid" }
```

#### 1.3 Configure Build Script

File: `composeApp/build.gradle.kts`

Add to `jvmMain.dependencies`:

```kotlin
implementation(libs.surrealdb.java)
implementation(libs.ulid.creator)
```

#### 1.4 Verify Build

- Run `./gradlew :composeApp:jvmJar` to verify dependency resolution
- Check for any transitive dependency conflicts

---

## Phase 2: Platform Data Directory

**Goal**: Implement platform-aware data directory resolution using expect/actual pattern.

### Tasks

#### 2.1 Define Expect Declaration

File: `shared/src/commonMain/kotlin/com/getaltair/altair/data/AppDataDirectory.kt`

```kotlin
expect fun getAppDataDirectory(): java.nio.file.Path
```

#### 2.2 Implement JVM Actual

File: `shared/src/jvmMain/kotlin/com/getaltair/altair/data/AppDataDirectory.jvm.kt`

Implementation:

- Detect OS using `System.getProperty("os.name")`
- Return appropriate path per OS
- Create directory if not exists

Path mapping:

- Linux: `$HOME/.local/share/altair/db/`
- macOS: `$HOME/Library/Application Support/altair/db/`
- Windows: `%APPDATA%/altair/db/`

#### 2.3 Add Unit Tests

File: `shared/src/jvmTest/kotlin/com/getaltair/altair/data/AppDataDirectoryTest.kt`

Test cases:

- Returns non-null path
- Path contains "altair"
- Directory is created if not exists

---

## Phase 3: SurrealDB Connection Management

**Goal**: Implement connection lifecycle management for embedded SurrealDB.

### Tasks

#### 3.1 Create Configuration Class

File: `shared/src/jvmMain/kotlin/com/getaltair/altair/data/db/SurrealDbConfig.kt`

Define:

- Connection URL builder for embedded mode
- Namespace and database constants ("altair", "main")
- Configuration validation

#### 3.2 Implement Connection Manager

File: `shared/src/jvmMain/kotlin/com/getaltair/altair/data/db/SurrealDbConnection.kt`

Features:

- Singleton pattern with lazy initialization
- `initialize()` suspend function for startup
- `close()` function for cleanup
- Connection state tracking
- Error handling with logging

#### 3.3 Add Connection Tests

File: `shared/src/jvmTest/kotlin/com/getaltair/altair/data/db/SurrealDbConnectionTest.kt`

Test cases:

- Connection initializes successfully
- Can use namespace and database
- Connection closes gracefully
- Reinitialization after close works

---

## Phase 4: Entity and Repository Layer

**Goal**: Define test entity and implement repository pattern.

### Tasks

#### 4.1 Create TestEntity

File: `shared/src/commonMain/kotlin/com/getaltair/altair/data/entity/TestEntity.kt`

```kotlin
data class TestEntity(
    val id: String,           // ULID
    val name: String,
    val value: Int,
    val createdAt: String,    // ISO 8601
    val updatedAt: String,    // ISO 8601
    val deletedAt: String?,   // nullable for soft delete
    val syncVersion: Int = 0
)
```

#### 4.2 Define Base Repository Interface

File: `shared/src/commonMain/kotlin/com/getaltair/altair/data/repository/Repository.kt`

```kotlin
interface Repository<T, ID> {
    suspend fun create(entity: T): T
    suspend fun findById(id: ID): T?
    suspend fun update(entity: T): T
    suspend fun delete(id: ID): Boolean
}
```

#### 4.3 Implement TestRepository

File: `shared/src/jvmMain/kotlin/com/getaltair/altair/data/db/repository/TestRepository.kt`

Implementation:

- Uses SurrealDbConnection singleton
- Generates ULID for new entities
- Implements soft delete
- Maps SurrealDB responses to TestEntity

#### 4.4 Add Repository Tests

File: `shared/src/jvmTest/kotlin/com/getaltair/altair/data/db/repository/TestRepositoryTest.kt`

Test cases:

- Create returns entity with generated ID
- FindById returns created entity
- Update modifies entity
- Delete sets deletedAt field
- FindById returns null for deleted entities

---

## Phase 5: Application Integration

**Goal**: Integrate database initialization into application startup.

### Tasks

#### 5.1 Update Main.kt

File: `composeApp/src/jvmMain/kotlin/com/getaltair/altair/Main.kt`

Changes:

- Initialize SurrealDbConnection before Window
- Add connection close on exitApplication
- Handle initialization errors gracefully

#### 5.2 Implement Startup Verification

Add verification that:

- Creates test record
- Reads test record back
- Verifies data matches
- Deletes test record (cleanup)
- Logs success/failure

#### 5.3 Add Persistence Test

Create integration test that:

- Starts application, creates entity
- Stops application
- Restarts application
- Verifies entity persists

---

## Phase 6: Testing and Verification

**Goal**: Comprehensive testing on all desktop platforms.

### Tasks

#### 6.1 Unit Test Suite

- All repository CRUD operations
- Connection lifecycle management
- Error handling paths
- Edge cases (empty data, special characters)

#### 6.2 Integration Tests

- Full application startup with database
- Persistence across restart
- Concurrent read operations

#### 6.3 Platform Testing

Test on:

- Linux (Ubuntu/Fedora)
- macOS (Apple Silicon and Intel)
- Windows 10/11

Verify:

- Correct data directory paths
- File permissions
- JNI native library loading

---

## Technical Approach

### SurrealDB Query Patterns

```kotlin
// Create
db.query("""
    CREATE test_entity SET
        id = $id,
        name = $name,
        value = $value,
        created_at = $createdAt,
        updated_at = $updatedAt,
        sync_version = 0
""", params)

// Read
db.query("SELECT * FROM test_entity WHERE id = $id", params)

// Update
db.query("""
    UPDATE test_entity SET
        name = $name,
        value = $value,
        updated_at = $updatedAt,
        sync_version = sync_version + 1
    WHERE id = $id
""", params)

// Soft Delete
db.query("""
    UPDATE test_entity SET
        deleted_at = $deletedAt,
        updated_at = $updatedAt
    WHERE id = $id
""", params)
```

### Error Handling Strategy

```kotlin
sealed class DatabaseResult<T> {
    data class Success<T>(val data: T) : DatabaseResult<T>()
    data class Error<T>(val message: String, val cause: Throwable?) : DatabaseResult<T>()
}
```

### Coroutine Integration

```kotlin
// All DB operations are suspend functions
suspend fun <T> withDatabase(block: suspend (Surreal) -> T): T =
    withContext(Dispatchers.IO) {
        block(SurrealDbConnection.instance)
    }
```

---

## Milestone Summary

| Phase | Milestone                  | Dependencies |
| ----- | -------------------------- | ------------ |
| 1     | Dependency configuration   | None         |
| 2     | Platform directory support | Phase 1      |
| 3     | Connection management      | Phase 2      |
| 4     | Entity and repository      | Phase 3      |
| 5     | Application integration    | Phase 4      |
| 6     | Testing and verification   | Phase 5      |

---

## Definition of Done

- All unit tests pass with minimum 85% coverage on new code
- Integration test verifies persistence across restart
- Application runs successfully on Linux, macOS, and Windows
- No blocking operations on UI thread
- Connection closes gracefully on application exit
- Error messages are user-friendly
- Code passes linting (ktlint, detekt)
- Documentation updated in architecture docs
