# Change: Add Repository Interfaces, Domain Errors, and DTOs

## Why

Phase 2 established domain models but lacks the infrastructure for data access and client-server
communication. Repository interfaces enable clean separation between domain logic and persistence,
while DTOs provide type-safe contracts for RPC/REST endpoints. Module-specific error types enable
granular error handling across different parts of the system.

## What Changes

- **Domain Errors**: Extend `DomainError` hierarchy with module-specific errors:
  - `QuestError` for Guidance module failures (not found, energy exceeded, etc.)
  - `NoteError` for Knowledge module failures (title conflict, invalid link, etc.)
  - `ItemError` for Tracking module failures (invalid quantity, container cycles, etc.)
  - `SyncError` for synchronization failures (conflict, version mismatch, etc.)
  - `AuthError` for authentication failures (invalid credentials, token expired, etc.)

- **Repository Interfaces**: Define platform-agnostic interfaces using Arrow `Either<Error, T>`:
  - `InitiativeRepository` - CRUD for initiatives with hierarchy support
  - `InboxRepository` - Capture and triage operations
  - `RoutineRepository` - Routine management with schedule queries
  - `QuestRepository` - Quest lifecycle with status transitions
  - `EpicRepository` - Epic management with quest aggregation
  - `NoteRepository` - Note operations with link management
  - `FolderRepository` - Folder hierarchy operations
  - `ItemRepository` - Item management with location/container relations
  - `LocationRepository` - Location hierarchy operations
  - `ContainerRepository` - Container management with nesting

- **DTOs for Client-Server Communication**: Create serializable request/response types:
  - Auth DTOs: `AuthRequest`, `AuthResponse`, `TokenRefreshRequest`
  - Sync DTOs: `SyncRequest`, `SyncResponse`, `ChangeSet`, `ConflictResolution`
  - CRUD DTOs: `Create*Request`, `Update*Request` for each entity type

## Impact

- **Affected specs**: `error-handling` (modified), new `repositories`, new `dto`
- **Affected code**:
  - `shared/src/commonMain/kotlin/com/getaltair/altair/domain/error/` (new errors)
  - `shared/src/commonMain/kotlin/com/getaltair/altair/repository/` (new interfaces)
  - `shared/src/commonMain/kotlin/com/getaltair/altair/dto/` (new DTOs)
- **Dependencies**: Arrow Core (already configured), kotlinx.serialization (already configured)
- **No breaking changes**: This adds new interfaces and types without modifying existing code
