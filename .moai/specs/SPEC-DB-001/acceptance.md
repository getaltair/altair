# SPEC-DB-001: Acceptance Criteria

## Traceability

- **SPEC Reference**: [SPEC-DB-001](./spec.md)
- **Implementation Plan**: [plan.md](./plan.md)
- **GitHub Issue**: #43

---

## Test Scenarios

### TS-001: Dependency Configuration

**Given** a fresh project checkout
**When** the developer runs `./gradlew :composeApp:dependencies`
**Then** surrealdb.java appears in the jvmMain configuration
**And** no dependency conflicts are reported

---

### TS-002: Data Directory Resolution - Linux

**Given** the application runs on Linux
**When** the data directory is resolved
**Then** the path equals `$HOME/.local/share/altair/db/`
**And** the directory exists or is created

---

### TS-003: Data Directory Resolution - macOS

**Given** the application runs on macOS
**When** the data directory is resolved
**Then** the path equals `$HOME/Library/Application Support/altair/db/`
**And** the directory exists or is created

---

### TS-004: Data Directory Resolution - Windows

**Given** the application runs on Windows
**When** the data directory is resolved
**Then** the path equals `%APPDATA%\altair\db\`
**And** the directory exists or is created

---

### TS-005: SurrealDB Connection Initialization

**Given** the data directory is available
**When** the application starts
**Then** SurrealDB embedded connection is established
**And** the namespace is set to "altair"
**And** the database is set to "main"
**And** connection success is logged

---

### TS-006: SurrealDB Connection Failure Handling

**Given** the data directory is not writable
**When** SurrealDB initialization is attempted
**Then** an error is logged with details
**And** a user-friendly error message is displayed
**And** the application does not crash unexpectedly

---

### TS-007: Test Entity Creation

**Given** SurrealDB connection is active
**When** a TestEntity is created with name="Test" and value=42
**Then** the entity is persisted with a ULID id
**And** created_at and updated_at are set to current timestamp
**And** deleted_at is null
**And** sync_version is 0

---

### TS-008: Test Entity Read

**Given** a TestEntity exists with id="01HXYZ..."
**When** findById is called with that id
**Then** the entity is returned with all fields intact
**And** the name equals "Test"
**And** the value equals 42

---

### TS-009: Test Entity Update

**Given** a TestEntity exists with value=42
**When** the entity is updated with value=100
**Then** the entity's value is 100
**And** updated_at is newer than before
**And** sync_version is incremented by 1

---

### TS-010: Test Entity Soft Delete

**Given** a TestEntity exists with id="01HXYZ..."
**When** delete is called with that id
**Then** deleted_at is set to current timestamp
**And** the entity still exists in the database
**And** findById returns null (respecting soft delete filter)

---

### TS-011: Persistence Across Restart (Primary Verification)

**Given** the application starts fresh
**When** a TestEntity is created with name="PersistenceTest"
**And** the application is closed gracefully
**And** the application is started again
**Then** the TestEntity with name="PersistenceTest" exists
**And** all fields match the original values

---

### TS-012: Connection Graceful Shutdown

**Given** SurrealDB connection is active with pending operations
**When** the application is closed
**Then** all pending writes are flushed
**And** the connection is closed without error
**And** no data corruption occurs

---

### TS-013: ULID Generation

**Given** a new TestEntity is being created
**When** the repository generates an ID
**Then** the ID is a valid ULID (26 characters, Crockford base32)
**And** the ID is unique from previously generated IDs

---

### TS-014: Timestamp Format

**Given** a TestEntity is created
**When** the timestamps are examined
**Then** created_at is ISO 8601 format (e.g., "2026-01-09T12:00:00Z")
**And** updated_at is ISO 8601 format
**And** deleted_at when set is ISO 8601 format

---

### TS-015: Non-Blocking UI Thread

**Given** the application is running with UI displayed
**When** a database operation is performed
**Then** the UI remains responsive
**And** the operation completes on IO dispatcher
**And** no ANR or freeze occurs

---

### TS-016: Startup Verification Logging

**Given** the application starts successfully
**When** database initialization completes
**Then** a log entry confirms "SurrealDB connected successfully"
**And** namespace and database are logged
**And** data directory path is logged

---

## Quality Gates

### Coverage Requirements

| Metric                   | Threshold | Measurement                   |
| ------------------------ | --------- | ----------------------------- |
| Line coverage            | >= 85%    | New code in data module       |
| Branch coverage          | >= 80%    | New code in data module       |
| Repository test coverage | 100%      | All CRUD operations tested    |
| Connection test coverage | 100%      | Init/close/error paths tested |

### Performance Requirements

| Metric               | Threshold | Measurement                   |
| -------------------- | --------- | ----------------------------- |
| Startup time impact  | < 500ms   | Additional time for DB init   |
| Simple query latency | < 50ms    | Single entity CRUD operations |
| UI thread blocking   | 0ms       | All DB ops are suspend funcs  |

### Platform Requirements

| Platform       | Status Required | Test Method                  |
| -------------- | --------------- | ---------------------------- |
| Linux x86_64   | Pass            | CI pipeline or manual test   |
| macOS arm64    | Pass            | Manual test on Apple Silicon |
| macOS x86_64   | Pass            | Manual test or CI            |
| Windows x86_64 | Pass            | Manual test or CI            |

---

## Verification Checklist

### Pre-Merge Verification

- [ ] All unit tests pass (`./gradlew :shared:jvmTest`)
- [ ] Integration tests pass (`./gradlew :composeApp:jvmTest`)
- [ ] Application starts without errors on local machine
- [ ] Entity persists across restart (TS-011 manual verification)
- [ ] Code coverage meets thresholds
- [ ] Linting passes (`./gradlew spotlessCheck detekt`)
- [ ] No compiler warnings

### Post-Merge Verification

- [ ] CI pipeline passes on all platforms
- [ ] Manual verification on at least two platforms
- [ ] Documentation updated in persistence.md if needed
- [ ] GitHub Issue #43 updated with implementation notes

---

## Error Handling Verification

### TS-E01: Database Directory Permission Denied

**Given** the data directory parent is read-only
**When** directory creation is attempted
**Then** a clear error message explains the permission issue
**And** a suggested fix is provided (e.g., "Check permissions for ~/.local/share/")
**And** the application exits gracefully

### TS-E02: Database Corruption Recovery

**Given** the database files are corrupted
**When** SurrealDB initialization is attempted
**Then** an error is logged with the corruption details
**And** the user is informed of potential data loss
**And** a backup/recovery option is suggested

### TS-E03: JNI Native Library Missing

**Given** the surrealdb native library is not found
**When** the connection is initialized
**Then** a clear error message identifies the missing library
**And** the platform is logged for debugging
**And** the error is reported to crash analytics (if available)

---

## Definition of Done

This SPEC is considered complete when:

1. All acceptance test scenarios (TS-001 through TS-016) pass
2. All quality gate thresholds are met
3. Platform verification is complete for all three desktop platforms
4. Error handling scenarios are verified
5. Code is merged to main branch
6. GitHub Issue #43 is closed with reference to merged PR
