# SPEC-DB-001: Desktop SurrealDB Integration

## Metadata

| Field            | Value                         |
| ---------------- | ----------------------------- |
| **SPEC ID**      | SPEC-DB-001                   |
| **Title**        | Desktop SurrealDB Integration |
| **GitHub Issue** | #43                           |
| **Priority**     | High                          |
| **Status**       | Completed                     |
| **Created**      | 2026-01-09                    |
| **Labels**       | enhancement, database         |

## Traceability

- **Source**: [GitHub Issue #43](https://github.com/getaltair/altair/issues/43)
- **ADR Reference**: [ADR-002: Hybrid Database Strategy](../../../docs/adr/002-surrealdb-embedded.md)
- **Architecture Reference**: [Persistence Architecture](../../../docs/architecture/persistence.md)
- **Implementation Plan**: Section 1.3 - Desktop SurrealDB Integration

---

## Environment

### Platform Context

- **Framework**: Kotlin Multiplatform with Compose Multiplatform
- **Target**: Desktop (JVM) only for this SPEC
- **Build System**: Gradle with Kotlin DSL, version catalog at `gradle/libs.versions.toml`
- **Entry Point**: `composeApp/src/jvmMain/kotlin/com/getaltair/altair/Main.kt`

### Database Context

- **Database**: SurrealDB embedded with SurrealKV storage engine
- **Java SDK**: surrealdb.java (JNI bindings to native Rust)
- **Namespace**: "altair"
- **Database Name**: "main"

### Data Directory Requirements

| Platform | Data Directory Path                            |
| -------- | ---------------------------------------------- |
| Linux    | `$HOME/.local/share/altair/db/`                |
| macOS    | `$HOME/Library/Application Support/altair/db/` |
| Windows  | `%APPDATA%/altair/db/`                         |

---

## Assumptions

| ID  | Assumption                                                      | Confidence | Risk if Wrong                                           |
| --- | --------------------------------------------------------------- | ---------- | ------------------------------------------------------- |
| A1  | surrealdb.java SDK supports embedded mode with SurrealKV on JVM | High       | Need alternative SDK or server mode                     |
| A2  | JNI bindings are stable for macOS, Linux, and Windows platforms | Medium     | Platform-specific issues may require workarounds        |
| A3  | Single-writer, multiple-readers concurrency model is sufficient | High       | Need connection pooling or mutex implementation         |
| A4  | SurrealDB embedded initializes synchronously on startup         | Medium     | Async initialization with loading state may be required |
| A5  | Platform data directories are writable by the application       | High       | Need permission handling or fallback locations          |

---

## Requirements

### R1: Dependency Management (Ubiquitous)

The system shall include surrealdb.java as a dependency for the desktop (JVM) target.

#### R1.1: Version Catalog Entry

The system shall define the surrealdb.java version in `gradle/libs.versions.toml`:

- Version key: `surrealdb-java`
- Library alias: `surrealdb-java`
- Minimum version: 1.0.0+ (latest stable release)

#### R1.2: Gradle Configuration

The system shall add surrealdb.java to `composeApp/build.gradle.kts` under `jvmMain.dependencies`.

---

### R2: Platform Data Directory (Event-Driven)

**WHEN** the application starts **THEN** the system shall determine the platform-appropriate data directory.

#### R2.1: Directory Resolution

**WHEN** determining the data directory **THEN** the system shall use:

- Linux: `$HOME/.local/share/altair/db/`
- macOS: `$HOME/Library/Application Support/altair/db/`
- Windows: `%APPDATA%/altair/db/`

#### R2.2: Directory Creation

**WHEN** the data directory does not exist **THEN** the system shall create it with appropriate permissions.

#### R2.3: Expect/Actual Pattern

The system shall use Kotlin's expect/actual pattern:

- `expect fun getAppDataDirectory(): Path` in commonMain
- Platform-specific `actual fun` implementations in jvmMain

---

### R3: SurrealDB Initialization (Event-Driven)

**WHEN** the application initializes **THEN** the system shall establish a SurrealDB embedded connection.

#### R3.1: Connection Configuration

**WHEN** initializing SurrealDB **THEN** the system shall:

- Use embedded mode with SurrealKV storage engine
- Connect to the data directory resolved in R2
- Use namespace: "altair"
- Use database: "main"

#### R3.2: Connection Lifecycle

**WHEN** the application starts **THEN** the system shall:

- Initialize SurrealDB connection before UI is displayed
- Store the connection reference for repository access
- Log connection status for debugging

**WHEN** the application closes **THEN** the system shall:

- Gracefully close the SurrealDB connection
- Ensure all pending writes are flushed

#### R3.3: Initialization Error Handling

**IF** SurrealDB initialization fails **THEN** the system shall:

- Log the error with full stack trace
- Display a user-friendly error message
- Prevent application from proceeding in undefined state

---

### R4: Repository Interface (Ubiquitous)

The system shall define a repository interface for database access following clean architecture principles.

#### R4.1: Base Repository Pattern

The system shall define a generic repository interface supporting:

- Create: Insert new records with ULID string IDs
- Read: Query records by ID or criteria
- Update: Modify existing records
- Delete: Soft-delete by setting `deleted_at` field

#### R4.2: Entity Conventions

All entities shall follow these conventions per ADR-002:

- ID field: ULID string format
- Timestamps: `created_at`, `updated_at` as ISO 8601 strings
- Soft delete: `deleted_at` field (null when active)
- Sync tracking: `sync_version` integer field

---

### R5: Test Entity and Repository (Event-Driven)

**WHEN** verifying database connectivity **THEN** the system shall provide a test entity and repository.

#### R5.1: TestEntity Definition

The system shall define a TestEntity with:

- `id`: ULID string
- `name`: String (required)
- `value`: Int (required)
- `created_at`: ISO 8601 timestamp string
- `updated_at`: ISO 8601 timestamp string
- `deleted_at`: ISO 8601 timestamp string (nullable)
- `sync_version`: Int (default 0)

#### R5.2: TestRepository Implementation

**WHEN** the test repository is invoked **THEN** it shall:

- Create: Insert a TestEntity and return the created record
- Read: Retrieve a TestEntity by ID
- Delete: Soft-delete by setting `deleted_at`
- Verify: Confirm data persists across connection cycles

---

### R6: Connection Verification (Event-Driven)

**WHEN** the application starts **THEN** the system shall verify database connectivity.

#### R6.1: Startup Verification

**WHEN** SurrealDB initializes successfully **THEN** the system shall:

- Create a test record using TestRepository
- Read the test record back
- Verify the data matches
- Delete the test record (cleanup)
- Log success status

#### R6.2: Persistence Verification

**WHEN** verifying persistence across restarts **THEN** the system shall:

- Create a persistent marker record on first launch
- On subsequent launches, verify the marker exists
- This confirms SurrealKV storage is functioning correctly

---

### R7: Negative Requirements (Unwanted)

#### R7.1: No Server Mode

The system shall NOT use SurrealDB in server mode for desktop; embedded mode only.

#### R7.2: No Hardcoded Paths

The system shall NOT hardcode platform-specific paths; all paths shall be determined at runtime.

#### R7.3: No Synchronous Blocking

The system shall NOT block the UI thread during database operations; all operations shall be asynchronous.

#### R7.4: No Data Loss on Crash

The system shall NOT lose committed data on application crash; SurrealKV provides durability guarantees.

---

### R8: Optional Enhancements (Optional)

#### R8.1: Connection Pool (Optional)

**WHERE** performance profiling indicates need, the system may implement connection pooling.

#### R8.2: Schema Migrations (Optional)

**WHERE** schema evolution is required, the system may implement a migration framework.

---

## Specifications

### File Structure

```
shared/src/
├── commonMain/kotlin/com/getaltair/altair/
│   └── data/
│       ├── AppDataDirectory.kt      # expect declaration
│       ├── entity/
│       │   └── TestEntity.kt        # Test entity data class
│       └── repository/
│           └── Repository.kt        # Base repository interface
│
├── jvmMain/kotlin/com/getaltair/altair/
│   └── data/
│       ├── AppDataDirectory.jvm.kt  # actual implementation
│       └── db/
│           ├── SurrealDbConfig.kt   # Database configuration
│           ├── SurrealDbConnection.kt # Connection management
│           └── repository/
│               └── TestRepository.kt # Test repository impl

composeApp/src/jvmMain/kotlin/com/getaltair/altair/
└── Main.kt                          # Updated with DB initialization
```

### Technical Constraints

| Constraint        | Value                               |
| ----------------- | ----------------------------------- |
| SurrealDB Mode    | Embedded (in-process)               |
| Storage Engine    | SurrealKV                           |
| Namespace         | "altair"                            |
| Database          | "main"                              |
| ID Format         | ULID strings                        |
| Timestamp Format  | ISO 8601 strings                    |
| Deletion Strategy | Soft delete with `deleted_at` field |
| Concurrency Model | Single writer, multiple readers     |

### SurrealDB Connection String

```kotlin
// Embedded mode connection
surreal.connect("surreal+rocksdb://${dataDir.absolutePath}")
surreal.use("altair", "main")
```

---

## Dependencies

### External Libraries

| Library        | Version | Purpose                   | Source                           |
| -------------- | ------- | ------------------------- | -------------------------------- |
| surrealdb.java | 1.0.0+  | SurrealDB Java/Kotlin SDK | Maven Central or GitHub releases |
| ulid-creator   | 5.2.3+  | ULID generation           | Maven Central                    |

### Internal Dependencies

| Module     | Dependency Type                   |
| ---------- | --------------------------------- |
| shared     | Entity definitions, expect/actual |
| composeApp | JVM-specific implementation       |

---

## Risks and Mitigations

| Risk                                | Probability | Impact | Mitigation                                               |
| ----------------------------------- | ----------- | ------ | -------------------------------------------------------- |
| surrealdb.java JNI stability issues | Medium      | High   | Test on all platforms early; have fallback plan          |
| Data directory permission denied    | Low         | High   | Implement permission checking with user-friendly message |
| Database corruption on crash        | Low         | High   | SurrealKV has built-in durability; add recovery logging  |
| Memory usage with embedded DB       | Medium      | Medium | Profile memory; configure SurrealDB limits if needed     |
| Platform-specific path edge cases   | Medium      | Low    | Comprehensive testing on all three platforms             |

---

## Related SPECs

- **Predecessor**: SPEC-UI-001 (Design System Foundation - completed)
- **Successor**: SPEC-DB-002 (Entity Schema Implementation - future)
- **Related ADR**: ADR-002 (Hybrid Database Strategy)
- **Related Docs**: [Persistence Architecture](../../../docs/architecture/persistence.md)
