# Design: Repository Interfaces, Domain Errors, and DTOs

## Context

Phase 2 established the domain model layer with entities like Quest, Note, and Item. Phase 3 builds
the infrastructure for:

1. **Data access abstraction** - Repository pattern separates domain logic from persistence
2. **Error handling** - Module-specific errors enable precise failure handling
3. **Client-server contracts** - DTOs define the API surface for kotlinx-rpc

This design follows ADR-009 (Core Library Stack) using Arrow for typed errors and
kotlinx.serialization for DTOs.

## Goals / Non-Goals

### Goals

- Define repository interfaces that work across all platforms (JVM, Android, iOS)
- Create granular error types that map to specific failure modes
- Establish DTO conventions for consistent client-server communication
- Support offline-first with sync conflict detection

### Non-Goals

- Implement concrete repository classes (Phase 5: Database Layer)
- Define RPC service interfaces (Phase 4: kotlinx-rpc Service Layer)
- Implement authentication logic (Phase 6: Authentication & Multi-User)

## Decisions

### 1. Error Type Hierarchy

**Decision**: Module-specific sealed interfaces extending `DomainError`

```kotlin
sealed interface QuestError : DomainError {
    data class NotFound(val id: Ulid) : QuestError
    data class EnergyBudgetExceeded(val required: Int, val available: Int) : QuestError
    // ...
}
```

**Rationale**: Enables exhaustive when-matching per module while maintaining a common base for
generic error handling. Each error carries contextual data for logging and user feedback.

**Alternatives considered**:
- Single flat error hierarchy: Rejected because it doesn't scale with module count
- String error codes: Rejected because it loses compile-time safety

### 2. Repository Interface Pattern

**Decision**: Suspending functions returning `Either<Error, T>` with Flow for queries

```kotlin
interface QuestRepository {
    suspend fun findById(id: Ulid): Either<QuestError, Quest>
    fun findByStatus(status: QuestStatus): Flow<List<Quest>>
    suspend fun save(quest: Quest): Either<QuestError, Quest>
    suspend fun delete(id: Ulid): Either<QuestError, Unit>
}
```

**Rationale**:
- `suspend` for single-shot operations that may involve I/O
- `Flow<List<T>>` for reactive queries that update when data changes
- `Either<Error, T>` for explicit error handling without exceptions

**Alternatives considered**:
- Throwing exceptions: Rejected per CLAUDE.md architectural guidelines
- Result<T>: Rejected because Arrow's Either provides richer combinators and typed errors

### 3. DTO Separation from Domain Models

**Decision**: Distinct DTO types in `shared/.../dto/` separate from domain entities

```kotlin
// Domain model (internal state)
data class Quest(val id: Ulid, val userId: Ulid, val title: String, ...)

// DTO (API contract)
@Serializable
data class CreateQuestRequest(val title: String, val energyCost: Int, ...)
```

**Rationale**:
- Domain models have invariants (validated constructors, internal fields like `userId`)
- DTOs are flat, serializable contracts without business logic
- Enables API evolution without changing domain models

**Alternatives considered**:
- Reusing domain models as DTOs: Rejected because domain models have validation that
  may not apply during deserialization, and exposing internal fields like `userId` in
  requests is incorrect (it comes from auth context)

### 4. User Scoping Convention

**Decision**: Repository interfaces do NOT include `userId` in method signatures

```kotlin
// Correct: userId comes from repository configuration/context
interface QuestRepository {
    suspend fun findById(id: Ulid): Either<QuestError, Quest>
}

// Wrong: userId in every method
interface QuestRepository {
    suspend fun findById(userId: Ulid, id: Ulid): Either<QuestError, Quest>
}
```

**Rationale**:
- Implementations receive authenticated user context during construction
- Eliminates redundant userId parameters and potential misuse
- Server implementations inject user from JWT; client implementations use local user

### 5. Sync DTOs with Conflict Detection

**Decision**: Sync protocol uses version vectors for conflict detection

```kotlin
@Serializable
data class SyncRequest(
    val clientId: String,
    val lastSyncVersion: Long,
    val changes: List<EntityChange>
)

@Serializable
data class EntityChange(
    val entityType: String,
    val entityId: Ulid,
    val operation: ChangeOperation,
    val version: Long,
    val data: JsonElement?
)
```

**Rationale**:
- Version-based sync enables efficient delta synchronization
- JsonElement allows heterogeneous entity serialization
- Client tracks last sync point for resumable sync

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Repository interface explosion | Group related entities (e.g., Item + Container + Location) |
| DTO duplication with domain models | Use mapping extensions, consider code generation later |
| Error type proliferation | Keep errors module-scoped, share common patterns |
| Flow backpressure on large datasets | Pagination built into repository interfaces |

## Migration Plan

This is additive work with no migration required:

1. Create new packages and interfaces
2. Existing domain models unchanged
3. No database schema changes
4. Phase 5 will implement concrete repositories

## Open Questions

- [ ] Should repositories support batch operations (e.g., `saveAll`)? (Deferred to implementation)
- [ ] Should DTOs use `@optics` for update builders? (Test compile times first)
