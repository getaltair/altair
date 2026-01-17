# Implementation Tasks

## 1. Domain Errors

- [x] 1.1 Create `shared/.../domain/error/` package structure
  - Note: Errors placed in `com.getaltair.altair.domain` package (same as DomainError) due to sealed interface restrictions
- [x] 1.2 Implement `QuestError` sealed interface with cases:
  - `NotFound`, `EnergyBudgetExceeded`, `InvalidStatusTransition`, `WipLimitExceeded`
- [x] 1.3 Implement `NoteError` sealed interface with cases:
  - `NotFound`, `TitleConflict`, `InvalidWikiLink`, `CircularLink`, `FolderNotFound`
- [x] 1.4 Implement `ItemError` sealed interface with cases:
  - `NotFound`, `InvalidQuantity`, `ContainerCycle`, `LocationNotFound`, `ContainerNotFound`, `TemplateNotFound`
- [x] 1.5 Implement `SyncError` sealed interface with cases:
  - `ConflictDetected`, `VersionMismatch`, `ServerUnreachable`, `InvalidChangeSet`, `Timeout`
- [x] 1.6 Implement `AuthError` sealed interface with cases:
  - `InvalidCredentials`, `TokenExpired`, `TokenInvalid`, `AccountLocked`, `InviteRequired`, `InvalidInvite`, `EmailAlreadyExists`
- [x] 1.7 Write property-based tests for error serialization round-trips
  - See `ModuleErrorsTest.kt`

## 2. Repository Interfaces

- [x] 2.1 Create `shared/.../repository/` package structure
- [x] 2.2 Define base `Repository<T, ID>` interface with common CRUD operations
- [x] 2.3 Implement `InitiativeRepository` with hierarchy queries
- [x] 2.4 Implement `InboxRepository` with capture and triage operations
- [x] 2.5 Implement `RoutineRepository` with schedule-based queries
- [x] 2.6 Implement `EpicRepository` with quest aggregation
- [x] 2.7 Implement `QuestRepository` with status transitions and WIP enforcement
- [x] 2.8 Implement `CheckpointRepository` with ordering operations
- [x] 2.9 Implement `EnergyBudgetRepository` with date-based queries
- [x] 2.10 Implement `NoteRepository` with link and backlink queries
- [x] 2.11 Implement `NoteLinkRepository` for bidirectional link management
- [x] 2.12 Implement `FolderRepository` with hierarchy operations
- [x] 2.13 Implement `TagRepository` with note association queries
- [x] 2.14 Implement `AttachmentRepository` with storage key management
- [x] 2.15 Implement `SourceDocumentRepository` with extraction status queries
- [x] 2.16 Implement `ItemRepository` with location/container relations
- [x] 2.17 Implement `LocationRepository` with hierarchy operations
- [x] 2.18 Implement `ContainerRepository` with nesting operations
- [x] 2.19 Implement `ItemTemplateRepository` with field definition queries
- [x] 2.20 Implement `UserRepository` for multi-user support

## 3. DTOs

- [x] 3.1 Create `shared/.../dto/` package structure with subpackages
  - `dto/auth/`, `dto/sync/`, `dto/guidance/`, `dto/knowledge/`, `dto/tracking/`, `dto/system/`
- [x] 3.2 Implement auth DTOs: `AuthRequest`, `AuthResponse`, `TokenRefreshRequest`, `TokenRefreshResponse`
  - Also: `RegisterRequest`, `ChangePasswordRequest`, `ForgotPasswordRequest`, `ResetPasswordRequest`
- [x] 3.3 Implement sync DTOs: `SyncRequest`, `SyncResponse`, `ChangeSet`, `EntityChange`, `ConflictInfo`
  - Also: `ChangeOperation`, `ConflictResolution`, `ConflictResolutionRequest`
- [x] 3.4 Implement Guidance DTOs: `CreateQuestRequest`, `UpdateQuestRequest`, `QuestResponse`
  - Also: Epic DTOs, Checkpoint DTOs
- [x] 3.5 Implement Knowledge DTOs: `CreateNoteRequest`, `UpdateNoteRequest`, `NoteResponse`
  - Also: Folder DTOs, Tag DTOs, LinkNotesRequest
- [x] 3.6 Implement Tracking DTOs: `CreateItemRequest`, `UpdateItemRequest`, `ItemResponse`
  - Also: Location DTOs, Container DTOs, ItemTemplate DTOs
- [x] 3.7 Implement Initiative DTOs: `CreateInitiativeRequest`, `UpdateInitiativeRequest`
- [x] 3.8 Implement Inbox DTOs: `CaptureRequest`, `TriageRequest`, `InboxItemResponse`
- [x] 3.9 Implement Routine DTOs: `CreateRoutineRequest`, `UpdateRoutineRequest`
  - Also: `ScheduleRequest`, `ScheduleResponse`
- [x] 3.10 Write serialization tests for all DTOs
  - See `DtoSerializationTest.kt`

## 4. Integration

- [x] 4.1 Verify all new types work with KSP optics generation
  - Note: No optics were added as per design decision to defer
- [x] 4.2 Run full test suite: `./gradlew :shared:allTests`
  - All tests pass (JVM, iOS x64, iOS Simulator arm64)
- [x] 4.3 Verify builds for all targets (JVM, Android, iOS)
  - Build successful for all targets
