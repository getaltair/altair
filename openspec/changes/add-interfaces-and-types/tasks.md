# Implementation Tasks

## 1. Domain Errors

- [ ] 1.1 Create `shared/.../domain/error/` package structure
- [ ] 1.2 Implement `QuestError` sealed interface with cases:
  - `QuestNotFound`, `EnergyBudgetExceeded`, `InvalidStatusTransition`, `WipLimitExceeded`
- [ ] 1.3 Implement `NoteError` sealed interface with cases:
  - `NoteNotFound`, `TitleConflict`, `InvalidWikiLink`, `CircularLink`, `FolderNotFound`
- [ ] 1.4 Implement `ItemError` sealed interface with cases:
  - `ItemNotFound`, `InvalidQuantity`, `ContainerCycle`, `LocationNotFound`
- [ ] 1.5 Implement `SyncError` sealed interface with cases:
  - `ConflictDetected`, `VersionMismatch`, `ServerUnreachable`, `InvalidChangeSet`
- [ ] 1.6 Implement `AuthError` sealed interface with cases:
  - `InvalidCredentials`, `TokenExpired`, `TokenInvalid`, `AccountLocked`, `InviteRequired`
- [ ] 1.7 Write property-based tests for error serialization round-trips

## 2. Repository Interfaces

- [ ] 2.1 Create `shared/.../repository/` package structure
- [ ] 2.2 Define base `Repository<T, ID>` interface with common CRUD operations
- [ ] 2.3 Implement `InitiativeRepository` with hierarchy queries
- [ ] 2.4 Implement `InboxRepository` with capture and triage operations
- [ ] 2.5 Implement `RoutineRepository` with schedule-based queries
- [ ] 2.6 Implement `EpicRepository` with quest aggregation
- [ ] 2.7 Implement `QuestRepository` with status transitions and WIP enforcement
- [ ] 2.8 Implement `CheckpointRepository` with ordering operations
- [ ] 2.9 Implement `EnergyBudgetRepository` with date-based queries
- [ ] 2.10 Implement `NoteRepository` with link and backlink queries
- [ ] 2.11 Implement `NoteLinkRepository` for bidirectional link management
- [ ] 2.12 Implement `FolderRepository` with hierarchy operations
- [ ] 2.13 Implement `TagRepository` with note association queries
- [ ] 2.14 Implement `AttachmentRepository` with storage key management
- [ ] 2.15 Implement `SourceDocumentRepository` with extraction status queries
- [ ] 2.16 Implement `ItemRepository` with location/container relations
- [ ] 2.17 Implement `LocationRepository` with hierarchy operations
- [ ] 2.18 Implement `ContainerRepository` with nesting operations
- [ ] 2.19 Implement `ItemTemplateRepository` with field definition queries
- [ ] 2.20 Implement `UserRepository` for multi-user support

## 3. DTOs

- [ ] 3.1 Create `shared/.../dto/` package structure with subpackages
- [ ] 3.2 Implement auth DTOs: `AuthRequest`, `AuthResponse`, `TokenRefreshRequest`, `TokenRefreshResponse`
- [ ] 3.3 Implement sync DTOs: `SyncRequest`, `SyncResponse`, `ChangeSet`, `EntityChange`, `ConflictInfo`
- [ ] 3.4 Implement Guidance DTOs: `CreateQuestRequest`, `UpdateQuestRequest`, `QuestResponse`
- [ ] 3.5 Implement Knowledge DTOs: `CreateNoteRequest`, `UpdateNoteRequest`, `NoteResponse`
- [ ] 3.6 Implement Tracking DTOs: `CreateItemRequest`, `UpdateItemRequest`, `ItemResponse`
- [ ] 3.7 Implement Initiative DTOs: `CreateInitiativeRequest`, `UpdateInitiativeRequest`
- [ ] 3.8 Implement Inbox DTOs: `CaptureRequest`, `TriageRequest`, `InboxItemResponse`
- [ ] 3.9 Implement Routine DTOs: `CreateRoutineRequest`, `UpdateRoutineRequest`
- [ ] 3.10 Write serialization tests for all DTOs

## 4. Integration

- [ ] 4.1 Verify all new types work with KSP optics generation
- [ ] 4.2 Run full test suite: `./gradlew :shared:allTests`
- [ ] 4.3 Verify builds for all targets (JVM, Android, iOS)
