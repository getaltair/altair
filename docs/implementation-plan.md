# Altair Implementation Plan

**Last updated:** 2026-01-26

Phased roadmap for implementing Altair. Follow phases sequentially — each phase builds on the previous.

---

## Package Structure

| Module | Source Path | Base Package |
|--------|-------------|--------------|
| shared | `src/commonMain/kotlin/` | `com.getaltair.altair.shared` |
| composeApp | `src/commonMain/kotlin/` | `com.getaltair.altair` |
| server | `src/main/kotlin/` | `com.getaltair.server` |

> **Note:** The `shared` module must use KMP structure (`commonMain/`, `androidMain/`, etc.), not JVM structure (`main/`). See migration guide if needed.

---

## Phase 0: Foundation ✅ COMPLETE

- [x] Compose Multiplatform client (Android, iOS, Desktop)
- [x] Ktor server with JWT, serialization, CORS
- [x] Shared KMP module
- [x] Monorepo with version catalog

---

## Phase 1: Core Libraries

**Goal:** Establish DI, navigation, and error handling foundations.  
**Ref:** [ADR-009](./adr/009-core-library-stack.md)

### Tasks

1. Add to version catalog: Koin 4.x, Decompose 3.x, Arrow 2.x, Mokkery, Turbine
2. Wire Koin into composeApp with platform-specific initialization
3. Create Decompose `RootComponent` with `Config` sealed class for navigation
4. Add Arrow to shared module with KSP for optics
5. Verify all targets build (Android, iOS, Desktop, JVM)

### Outputs

- `composeApp/.../di/AppModule.kt`
- `composeApp/.../navigation/RootComponent.kt`
- Updated `gradle/libs.versions.toml`

---

## Phase 2: Domain Models & Interfaces

**Goal:** Define all data structures, error types, and repository contracts.  
**Ref:** [Domain Model](./architecture/domain-model.md)

### 2.1 Domain Entities

Location: `shared/.../domain/`

**System:** User, Initiative, InboxItem, Routine  
**Guidance:** Epic, Quest, Checkpoint, EnergyBudget  
**Knowledge:** Note, NoteLink, Folder, Tag, Attachment, SourceDocument, SourceAnnotation  
**Tracking:** Item, Location, Container, ItemTemplate, CustomField

### 2.2 Supporting Types

**Enums:** UserRole, UserStatus, InitiativeStatus, QuestStatus, CaptureSource, ExtractionStatus  
**Value Objects:** Schedule (sealed class), ULID utility

### 2.3 Error Types

Location: `shared/.../domain/error/`

Define `sealed interface AltairError` with: NetworkError, AuthError, ValidationError, NotFoundError, ConflictError, StorageError

### 2.4 Repository Interfaces

Location: `shared/.../repository/`

Define interfaces returning `Either<AltairError, T>`:
- InitiativeRepository, InboxRepository, RoutineRepository
- QuestRepository, NoteRepository, ItemRepository

### 2.5 DTOs

Location: `shared/.../dto/`

Organize by domain: auth/, sync/, guidance/, knowledge/, tracking/, ai/

---

## Phase 3: Server Database

**Goal:** Server can persist and query data.  
**Ref:** [ADR-002](./adr/002-surrealdb-embedded.md), [Persistence](./architecture/persistence.md)

### Tasks

1. Add surrealdb-java dependency
2. Create DatabaseConfig and SurrealDbClient wrapper
3. Implement repository interfaces for each entity
4. Add user scoping (`WHERE userId = $userId`) to all queries
5. Set up Docker Compose with SurrealDB service

### Outputs

- `server/.../persistence/SurrealDbClient.kt`
- `server/.../persistence/repositories/*.kt`
- `docker-compose.yml` with surrealdb service

---

## Phase 4: REST API Layer

**Goal:** Client-server communication via HTTP/JSON.  
**Ref:** [ADR-016](./adr/016-ktor-rest-api.md)

### 4.1 Server Routes

Location: `server/.../routes/`

Create route files: AuthRoutes, SyncRoutes, InboxRoutes, GuidanceRoutes, KnowledgeRoutes, TrackingRoutes, InitiativeRoutes, RoutineRoutes, AiRoutes

Wire all routes in Application.kt with JWT authentication.

### 4.2 Client API Classes

Location: `composeApp/.../api/`

Create API classes: AuthApi, SyncApi, InboxApi, GuidanceApi, KnowledgeApi, TrackingApi, InitiativeApi, RoutineApi, AiApi

### 4.3 HTTP Client

Create HttpClientFactory with: ContentNegotiation (JSON), Auth (bearer tokens), HttpTimeout, Logging

### 4.4 Koin Wiring

Create ApiModule providing HttpClient and all API class instances.

---

## Phase 5: Authentication

**Goal:** Multi-user support with proper data isolation.  
**Ref:** [ADR-012](./adr/012-multi-user-data-isolation.md)

### 5.1 Server Auth

- JWT generation/validation (access + refresh tokens)
- Argon2 password hashing
- Invite-code registration flow
- Auth routes: login, register, refresh, logout

### 5.2 Client Auth

- Platform-specific secure token storage (Keychain/Keystore)
- AuthRepository for login/logout/refresh
- Auth state management via Koin

### 5.3 Auth UI

- LoginScreen, RegisterScreen, SplashScreen
- Token validation on app launch

---

## Phase 6: UI Foundation

**Goal:** Design system and navigation structure.  
**Ref:** [ADR-008](./adr/008-compose-unstyled-altair-theme.md)

### 6.1 Design System

Location: `composeApp/.../ui/theme/`

- Add Compose Unstyled dependency
- Create AltairTheme with colors, typography, spacing tokens
- Build base components: Button, TextField, Card, Chip, Dialog, BottomSheet

### 6.2 Navigation

- Expand RootComponent with full Config (Home, Guidance, Knowledge, Tracking, Settings)
- Implement bottom navigation (mobile) / sidebar (desktop)
- Create child components for each section

### 6.3 Placeholder Screens

Create basic screens: HomeScreen, GuidanceScreen, KnowledgeScreen, TrackingScreen, SettingsScreen

---

## Phase 7: Inbox + Guidance MVP

**Goal:** First usable feature end-to-end.  
**Ref:** [ADR-010](./adr/010-universal-inbox-architecture.md), [PRD Guidance](./requirements/altair-prd-guidance.md)

### 7.1 Universal Inbox

- InboxScreen with item list
- CaptureSheet for quick input
- TriageDialog to convert items to Quest/Note/Item

### 7.2 Guidance Module

- Quest list with status filtering
- Quest detail/edit screen
- Status transitions (Backlog → Ready → In Progress → Completed)
- WIP=1 enforcement visualization
- Energy budget display
- Today view

### 7.3 Mobile Home

Two-tab layout: Inbox tab + Today tab

---

## Phase 8: Mobile Database (SQLDelight)

**Goal:** Offline support for mobile.  
**Ref:** [ADR-002](./adr/002-surrealdb-embedded.md)

### Tasks

1. Add SQLDelight plugin to shared module
2. Create .sq schema files for all entities
3. Generate type-safe queries
4. Implement SQLite repositories for Android/iOS
5. Configure platform-specific drivers

---

## Phase 9: Desktop Database (SurrealDB Embedded)

**Goal:** Offline support for desktop.  
**Ref:** [ADR-002](./adr/002-surrealdb-embedded.md)

### Tasks

1. Add surrealdb-java for desktop target (embedded mode)
2. Implement desktop repository variants
3. Configure file-based storage location
4. Platform-specific Koin bindings

---

## Phase 10: Knowledge & Tracking

**Goal:** Complete the three core modules.  
**Ref:** [PRD Knowledge](./requirements/altair-prd-knowledge.md), [PRD Tracking](./requirements/altair-prd-tracking.md)

### 10.1 Knowledge MVP

- Note list with folders
- Markdown editor (compose-rich-editor per [ADR-015](./adr/015-rich-text-editing-library.md))
- WikiLink `[[...]]` parsing and navigation
- Backlinks panel

### 10.2 Tracking MVP

- Item list with location filter
- Item detail/edit screen
- Photo capture (platform-specific)
- Quantity management
- Search

---

## Phase 11: Initiatives & Routines

**Goal:** Cross-cutting organization features.  
**Ref:** [ADR-011](./adr/011-initiative-system-design.md), [ADR-013](./adr/013-routine-scheduling-strategy.md)

### 11.1 Initiatives

- Initiative list (tree view)
- Initiative detail with linked entities
- Entity linking across modules
- Filter by Initiative

### 11.2 Routines

- Routine list and form with schedule picker
- Server-side instance generation (cron job)
- Routine instances in Today view
- Instance completion updates nextDue

---

## Phase 12: Sync & Offline

**Goal:** Multi-device experience.  
**Ref:** [PRD Core §2.4](./requirements/altair-prd-core.md)

### Tasks

1. Add version field to all entities
2. Implement pull/push sync protocol
3. Conflict detection (version mismatch)
4. Conflict resolution UI
5. Offline queue for failed requests
6. Sync status indicator

---

## Phase 13: AI Services

**Goal:** Voice capture, semantic search, AI assistance.  
**Ref:** [ADR-006](./adr/006-server-centralized-ai.md)

### 13.1 Server Infrastructure

- ONNX Runtime for embeddings
- whisper.cpp for transcription
- Completion provider proxy (Ollama/OpenAI)

### 13.2 Client Integration

- Voice capture → server transcription
- Semantic search via embeddings
- AI-assisted features (Quest breakdown, Note summarization)

---

## Phase 14: Platform Polish

**Goal:** Native platform integrations.

### Mobile

- Push notifications
- Widgets (Today, Quick Capture)
- Biometric auth
- Share extension

### Desktop

- System tray integration
- Global keyboard shortcuts
- Multi-window support

### Deployment

- Dockerfile for server
- docker-compose.yml with SurrealDB + optional Ollama
- Health check endpoint

---

## Phase 15: Testing & Documentation

### Unit Tests

- Domain model validation
- Repository tests with fakes
- ViewModel tests with Turbine

### Integration Tests

- API round-trip tests
- Database integration tests
- Sync protocol tests

### UI Tests

- Compose UI tests for critical flows
- Screenshot tests for design system

### Documentation

- AGENTS.md coding conventions
- API documentation
- User guide

---

## Development Notes

### Iterative Feature Development

For phases 7-11, repeat this cycle:
1. Implement server repositories
2. Add API routes
3. Create ViewModels
4. Build UI screens
5. Test end-to-end

### Testing Strategy

Write unit tests alongside implementation — don't wait for Phase 15.

---

## References

- [ADR-001: KMP Architecture](./adr/001-single-tauri-application.md)
- [ADR-002: Hybrid Database](./adr/002-surrealdb-embedded.md)
- [ADR-006: Server AI](./adr/006-server-centralized-ai.md)
- [ADR-007: Docker Deployment](./adr/007-docker-compose-deployment.md)
- [ADR-008: Compose Unstyled](./adr/008-compose-unstyled-altair-theme.md)
- [ADR-009: Core Library Stack](./adr/009-core-library-stack.md)
- [ADR-010: Universal Inbox](./adr/010-universal-inbox-architecture.md)
- [ADR-011: Initiative System](./adr/011-initiative-system-design.md)
- [ADR-012: Multi-User Isolation](./adr/012-multi-user-data-isolation.md)
- [ADR-013: Routine Scheduling](./adr/013-routine-scheduling-strategy.md)
- [ADR-015: Rich Text Editing](./adr/015-rich-text-editing-library.md)
- [ADR-016: Ktor REST API](./adr/016-ktor-rest-api.md)
