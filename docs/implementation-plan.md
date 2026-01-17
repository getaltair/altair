# Altair Implementation Plan

**Last updated:** 2026-01-15

This document provides a phased roadmap for implementing Altair, tracking progress and guiding
development across sessions.

---

## Phase 0: Foundation ✅ COMPLETE

What we accomplished in the initial session:

- [x] Compose Multiplatform client via KMP Wizard (Android, iOS, Desktop)
- [x] Ktor server via IntelliJ (JWT auth, serialization, CORS, logging)
- [x] Shared KMP module for domain models/DTOs
- [x] Monorepo structure with dependency wiring
- [x] Version catalog setup (`libs.versions.toml`)

---

## Phase 1: Core Libraries & Architecture -- Completed

### 1.1 Add ADR-009 Dependencies to Version Catalog

**File:** `gradle/libs.versions.toml`

Add versions and libraries for:

- **Koin 4.x** — Dependency injection
- **Decompose 3.x** — Navigation
- **Arrow 2.x** — Typed errors, validation, optics
- **Mokkery 3.x** — Testing/mocking
- **Turbine** — Flow testing

### 1.2 Wire Koin into Client

**Files:** `composeApp/build.gradle.kts`, new DI module files

- Add `koin-core`, `koin-compose`, `koin-compose-viewmodel`
- Create `di/AppModule.kt` with initial empty module
- Initialize Koin in platform entry points (Android `Application`, Desktop `main()`, iOS)

### 1.3 Wire Decompose into Client

**Files:** `composeApp/build.gradle.kts`, new navigation files

- Add Decompose dependencies
- Create `RootComponent` as navigation root
- Create basic `Config` sealed class for destinations
- Wire into Compose via `childStack`

### 1.4 Add Arrow to Shared Module

**Files:** `shared/build.gradle.kts`

- Add `arrow-core` and `arrow-optics` with KSP plugin
- Verify builds on all targets

---

## Phase 2: Domain Models in Shared -- Completed

### 2.1 System-Level Entities

**Location:** `shared/src/commonMain/kotlin/com/getaltair/altair/shared/domain/`

Create `@Serializable` data classes for:

- `User` (id, username, role, status, storageUsed, storageQuota)
- `Initiative` (id, userId, name, description, parentId, ongoing, targetDate, status, focused)
- `InboxItem` (id, userId, content, source, createdAt)
- `Routine` (id, userId, name, schedule, timeOfDay, energyCost, initiativeId, active, nextDue)

### 2.2 Guidance Module Entities

- `Epic` (id, userId, title, description, status, initiativeId)
- `Quest` (id, userId, title, description, energyCost, status, epicId, routineId)
- `Checkpoint` (id, questId, title, completed, order)
- `EnergyBudget` (userId, date, budget, spent)

### 2.3 Knowledge Module Entities

- `Note` (id, userId, title, content, folderId, initiativeId)
- `NoteLink` (id, sourceId, targetId, context)
- `Folder` (id, userId, name, parentId, order)
- `Tag` (id, userId, name, color)
- `Attachment` (id, userId, noteId, filename, mimeType, storageKey)
- `SourceDocument` (id, userId, title, sourceType, sourcePath, mimeType, contentHash, extractedText, status)
- `SourceAnnotation` (id, userId, sourceDocumentId, anchorType, anchorValue, content)

### 2.4 Tracking Module Entities

- `Item` (id, userId, name, description, quantity, templateId, locationId, containerId, initiativeId)
- `Location` (id, userId, name, description, parentId)
- `Container` (id, userId, name, description, locationId, parentId)
- `ItemTemplate` (id, userId, name, description, icon)
- `CustomField` (id, itemId, name, fieldType, value)

### 2.5 Supporting Types

- Enums: `UserRole`, `UserStatus`, `InitiativeStatus`, `QuestStatus`, `CaptureSource`, `ExtractionStatus`, etc.
- Value objects: `Schedule` sealed class for routine patterns
- ULID generation utility

---

## Phase 3: Repository Interfaces & Error Types -- In Progress

### 3.1 Define Domain Errors with Arrow

**Location:** `shared/src/commonMain/kotlin/com/getaltair/altair/shared/domain/error/`

```kotlin
sealed interface AltairError {
    sealed interface QuestError : AltairError { ... }
    sealed interface NoteError : AltairError { ... }
    // etc.
}
```

### 3.2 Repository Interfaces

**Location:** `shared/src/commonMain/kotlin/com/getaltair/altair/shared/repository/`

Define interfaces for each entity:

- `InitiativeRepository`
- `InboxRepository`
- `RoutineRepository`
- `QuestRepository`
- `NoteRepository`
- `ItemRepository`
- etc.

Use Arrow `Either<Error, T>` return types.

### 3.3 DTOs for Client-Server Communication

**Location:** `shared/src/commonMain/kotlin/com/getaltair/altair/shared/dto/`

Create request/response DTOs:

- `CreateQuestRequest`, `UpdateQuestRequest`
- `SyncRequest`, `SyncResponse`
- `AuthRequest`, `AuthResponse`
- etc.

---

## Phase 4: kotlinx-rpc Service Layer

### 4.1 Add kotlinx-rpc Dependencies

**Files:** `shared/build.gradle.kts`, `server/build.gradle.kts`, `composeApp/build.gradle.kts`

- Add `kotlinx-rpc-core`, `kotlinx-rpc-krpc-client`, `kotlinx-rpc-krpc-server`
- Add `kotlinx-rpc-krpc-ktor-client`, `kotlinx-rpc-krpc-ktor-server`

### 4.2 Define RPC Service Interfaces in Shared

**Location:** `shared/src/commonMain/kotlin/com/getaltair/altair/shared/rpc/`

```kotlin
@Rpc
interface SyncService : RemoteService {
    suspend fun pull(since: Long): SyncResponse
    suspend fun push(changes: List<ChangeSet>): PushResult
}

@Rpc
interface AuthService : RemoteService { ... }

@Rpc
interface AiService : RemoteService { ... }
```

### 4.3 Implement Services on Server

**Location:** `server/src/main/kotlin/com/getaltair/rpc/`

- `SyncServiceImpl`
- `AuthServiceImpl`
- `AiServiceImpl`

### 4.4 Wire RPC into Ktor

**File:** `server/.../Application.kt`

Configure Ktor to serve RPC endpoints.

### 4.5 Create RPC Client in composeApp

**Location:** `composeApp/src/commonMain/.../rpc/`

Create client that connects to server RPC endpoints.

---

## Phase 5: Database Layer

### 5.1 Add SurrealDB to Server

**Files:** `server/build.gradle.kts`, new repository implementations

- Add `surrealdb-java` dependency
- Create `DatabaseConfig` for connection
- Implement repository interfaces (e.g., `SurrealQuestRepository`)

### 5.2 Add SQLDelight to Mobile/Shared

**Files:** `shared/build.gradle.kts`, `.sq` files

- Add SQLDelight plugin and dependencies
- Create `.sq` schema files mirroring domain entities
- Generate type-safe queries

### 5.3 Desktop Database (SurrealDB Embedded)

**Files:** `composeApp/src/jvmMain/...`

- Add SurrealDB embedded for desktop offline support
- Implement repository interfaces for desktop

---

## Phase 6: Authentication & Multi-User

### 6.1 Server Auth Implementation

- JWT token generation/validation
- Argon2 password hashing
- User registration (invite-code flow)
- Login/logout/refresh endpoints

### 6.2 Client Auth Flow

- Secure token storage (platform-specific: Keychain, Keystore)
- Auth state management in Koin
- Login/registration UI screens

### 6.3 User Scoping

- Ensure all repository queries filter by `userId`
- Middleware to extract user from JWT on server

---

## Phase 7: UI Foundation

### 7.1 Altair Design System (ADR-008)

**Location:** `composeApp/src/commonMain/.../ui/theme/`

- Add Compose Unstyled dependency
- Create `AltairTheme` object with colors, typography, spacing
- Create base components: `AltairButton`, `AltairTextField`, `AltairCard`

### 7.2 Navigation Structure

- `RootComponent` with child stack
- `Config` entries for: Home, Guidance, Knowledge, Tracking, Settings
- Bottom navigation or sidebar (platform-dependent)

### 7.3 Placeholder Screens

Create minimal placeholder screens for:

- Home / Today view
- Guidance (Quest list)
- Knowledge (Note list)
- Tracking (Item list)
- Settings

---

## Phase 8: Feature Modules (Iterative)

### 8.1 Universal Inbox (ADR-010)

- `InboxItem` capture UI (text, voice placeholder)
- Triage flow (convert to Quest/Note/Item)
- Mobile home screen with Inbox + Today summary

### 8.2 Guidance MVP

- Quest list with status filtering
- Quest detail/edit screen
- WIP=1 enforcement
- Energy budget display
- Basic Today view

### 8.3 Knowledge MVP

- Note list with folders
- Markdown editor (compose-rich-editor per ADR-015)
- WikiLink `[[...]]` parsing and navigation
- Backlinks panel

### 8.4 Tracking MVP

- Item list with locations
- Photo capture for items
- Quantity management (+/- buttons)
- Basic search

### 8.5 Initiatives (ADR-011)

- Initiative CRUD
- Linking entities to Initiatives
- Initiative Card component
- Filtering by Initiative

### 8.6 Routines (ADR-013)

- Routine CRUD with schedule picker
- Instance generation logic (server-side)
- Routine instances in Today view

---

## Phase 9: Sync & Offline

### 9.1 Sync Protocol Implementation

- Version tracking on all entities
- Pull/push endpoints
- Conflict detection
- Conflict resolution UI for complex cases

### 9.2 Offline Queue

- Queue local changes when offline
- Sync on reconnect
- Sync status indicator

---

## Phase 10: AI Services (ADR-006)

### 10.1 Server AI Setup

- ONNX Runtime integration for embeddings
- whisper.cpp integration for transcription
- Completion provider proxy (Ollama, OpenAI, etc.)

### 10.2 Client AI Integration

- Voice capture → server transcription
- Semantic search (send query, get embedding matches)
- AI-assisted features (Quest breakdown, Note summarization)

---

## Phase 11: Platform Polish

### 11.1 Mobile (Android/iOS)

- Push notifications
- Widgets (Today, Quick Capture)
- Biometric auth
- Share extension (capture from other apps)

### 11.2 Desktop

- System tray integration
- Global keyboard shortcuts
- Multi-window support
- Native file dialogs

### 11.3 Server Deployment (ADR-007)

- Dockerfile for `altair-server`
- `docker-compose.yml` with SurrealDB + optional Ollama
- Environment variable configuration
- Health check endpoint

---

## Phase 12: Testing & Documentation

### 12.1 Unit Tests

- Domain model validation
- Repository interface tests with fakes
- Use case / business logic tests

### 12.2 Integration Tests

- RPC round-trip tests
- Database integration tests
- Sync protocol tests

### 12.3 UI Tests

- Compose UI tests for critical flows
- Screenshot tests for design system

### 12.4 Documentation

- Update AGENTS.md with coding conventions
- API documentation
- User guide basics

---

## Recommended Order of Attack

| Priority | Phase                                | Rationale                              |
| -------- | ------------------------------------ | -------------------------------------- |
| 1        | Phase 1 (Core Libraries)             | Foundation for everything else         |
| 2        | Phase 2 (Domain Models)              | Shared module becomes useful           |
| 3        | Phase 7.1-7.2 (UI Foundation)        | Can see something on screen            |
| 4        | Phase 6 (Auth)                       | Required for multi-user data isolation |
| 5        | Phase 5.1 (Server DB)                | Server can persist data                |
| 6        | Phase 4 (kotlinx-rpc)                | Client-server communication            |
| 7        | Phase 8.1-8.2 (Inbox + Guidance MVP) | First usable feature                   |
| 8        | Remaining phases iteratively         | Build out based on priorities          |

---

## References

- [ADR-001: Kotlin Multiplatform Architecture](./adr/001-single-tauri-application.md)
- [ADR-002: Hybrid Database Strategy](./adr/002-surrealdb-embedded.md)
- [ADR-005: kotlinx-rpc Communication](./adr/005-kotlinx-rpc-communication.md)
- [ADR-006: Server-Centralized AI](./adr/006-server-centralized-ai.md)
- [ADR-007: Docker Compose Deployment](./adr/007-docker-compose-deployment.md)
- [ADR-008: Compose Unstyled + Altair Theme](./adr/008-compose-unstyled-altair-theme.md)
- [ADR-009: Core Library Stack](./adr/009-core-library-stack.md)
- [ADR-010: Universal Inbox Architecture](./adr/010-universal-inbox-architecture.md)
- [ADR-011: Initiative System Design](./adr/011-initiative-system-design.md)
- [ADR-013: Routine Scheduling Strategy](./adr/013-routine-scheduling-strategy.md)
- [ADR-015: Rich Text Editing Library](./adr/ADR-015-rich-text-editing-library.md)

---

_This plan will be updated as implementation progresses._
