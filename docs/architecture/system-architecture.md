# System Architecture

## Purpose

This document describes Altair's high-level technical architecture: how components relate, where
boundaries exist, and key patterns used throughout the system.

---

## Technology Stack

| Layer                | Technology                | Rationale                                                  |
| -------------------- | ------------------------- | ---------------------------------------------------------- |
| UI Framework         | Compose Multiplatform     | Single codebase for desktop, Android, iOS                  |
| UI Components        | Compose Unstyled + Altair | Headless primitives with custom Linear-inspired theme      |
| Navigation           | Decompose 3.x             | UI-agnostic, exceptional maintenance, back gesture support |
| Shared Logic         | Kotlin Multiplatform      | Domain models, validation shared across all targets        |
| Dependency Injection | Koin 4.x                  | Fast builds, Compose integration, 14M monthly downloads    |
| Error Handling       | Arrow 2.x (core + optics) | Typed errors, validation accumulation, nested state        |
| Desktop Database     | SurrealDB embedded        | Graph queries, vector search, full-text search             |
| Mobile Database      | SQLite (SQLDelight)       | Quick capture, proven mobile reliability                   |
| Server Framework     | Ktor                      | Kotlin-native, lightweight, kotlinx-rpc integration        |
| Server Database      | SurrealDB                 | Primary store, sync hub                                    |
| Storage Backend      | S3-compatible             | Attachments (local, MinIO, S3, Backblaze B2)               |
| Client-Server        | kotlinx-rpc (gRPC)        | Type-safe, streaming, compile-time checked                 |
| Server AI            | ort + whisper.cpp         | Local embeddings and transcription                         |
| Authentication       | JWT + Argon2              | Multi-user with complete data isolation                    |
| Deployment           | Docker Compose            | Single-command self-hosted deployment                      |
| Testing              | Mokkery + Turbine         | Multiplatform mocking, Flow testing                        |

---

## Component Architecture

```
┌───────────────────────────────────────────────────────────────────────────────┐
│                     Docker Compose (self-hosted server)                       │
│  ┌─────────────────────────────────────────────────────────────────────────┐  │
│  │                        altair-server (Ktor)                             │  │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐  │  │
│  │  │  kotlinx-rpc    │  │   Auth Service  │  │      AI Service         │  │  │
│  │  │   endpoints     │  │   JWT + Argon2  │  │  embed/transcribe       │  │  │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────────────┘  │  │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────────┐  │  │
│  │  │  Sync Engine    │  │ Storage Service │  │    Admin Service        │  │  │
│  │  │ conflict resolve│  │  S3-compatible  │  │   user management       │  │  │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────────────┘  │  │
│  └─────────────────────────────────────────────────────────────────────────┘  │
│  ┌───────────────────────┐  ┌─────────────────────────────────────────────┐   │
│  │      SurrealDB        │  │        Ollama (optional)                    │   │
│  │      container        │  │        local completion                     │   │
│  └───────────────────────┘  └─────────────────────────────────────────────┘   │
└───────────────────────────────────────────────────────────────────────────────┘
                                      ▲
                                      │ kotlinx-rpc (gRPC-compatible)
                      ┌───────────────┴───────────────┐
                      │                               │
           ┌──────────┴──────────┐        ┌──────────┴───────────┐
           │       Desktop       │        │        Mobile         │
           │  Compose Multiplatf │        │   Compose Multiplatf  │
           │  SurrealDB embedded │        │   SQLite embedded     │
           │  Full features      │        │   Daily driver +      │
           │  + offline capable  │        │   quick capture       │
           └─────────────────────┘        └───────────────────────┘
```

---

## Platform Targets

### Desktop (Windows, Linux, macOS)

Full-featured application with:

- All three modules: Guidance, Knowledge, Tracking
- System-level features: Universal Inbox, Initiatives, Routines
- SurrealDB embedded for offline operation
- Graph queries, vector search, semantic similarity
- Offline-capable with sync when server available
- Focus mode, keyboard shortcuts, multi-window

### Mobile (Android, iOS)

Primary daily driver with quick capture:

- Basic CRUD for Quests, Notes, Items
- Full Universal Inbox access (primary capture point)
- Routine completion and tracking
- Initiative context and filtering
- SQLite embedded for simplicity and reliability
- Server-dependent for AI features
- Sync-first architecture (requires server for full functionality)
- Voice capture, camera for item photos

### Server (Self-Hosted)

Docker Compose stack providing:

- **Sync hub**: Multi-device synchronization for all client devices
- **AI services**: Embeddings, transcription, completion proxy
- **User authentication**: Multi-user with complete data isolation
- **Storage backend**: S3-compatible attachment storage
- **Admin interface**: Web dashboard for server management
- **SurrealDB**: Primary data store

### Web (Minimal)

View-only dashboard:

- Admin user management
- Server health monitoring
- No editing capabilities
- Responsive for mobile admin access

---

## Layer Responsibilities

### UI Layer (Compose Multiplatform)

Handles user interaction and presentation using the Altair design system.

- **Screens**: Route-level composables, navigation hosts
- **Components**: Reusable UI elements built on Compose Unstyled
- **ViewModels**: State holders using Kotlin Multiplatform ViewModel
- **Theme**: Altair design tokens (colors, typography, spacing)

The UI layer communicates with repositories; it never accesses databases directly.

### Domain Layer (Shared Kotlin)

Contains business logic shared across all platforms:

- **Entities**: User, Initiative, InboxItem, Routine, Quest, Epic, Note, Item, etc.
- **Repositories**: Interfaces defining data operations
- **Use Cases**: Business logic operations (optional, for complex flows)
- **Validation**: Input validation rules

### Data Layer (Platform-Specific)

Implements repository interfaces with platform-specific storage:

**Desktop:**

- SurrealDB via surrealdb.java JNI bindings
- Full query capabilities (graph traversal, vector search)
- All queries scoped by authenticated user

**Mobile:**

- SQLite via SQLDelight
- Type-safe generated queries
- Simpler schema without graph relations
- User ID filtering on all queries

**Server:**

- SurrealDB primary store
- Sync version tracking
- Conflict resolution logic
- User data isolation enforcement

### Network Layer

Client-server communication via kotlinx-rpc:

- **SyncService**: Pull/push changes, stream real-time updates
- **AiService**: Embeddings, transcription, completion
- **AuthService**: Login, token refresh, logout, user management
- **StorageService**: Attachment upload/download, quota management

---

## Key Boundaries

### Client ↔ Server

The kotlinx-rpc layer is a hard boundary. Clients:

- Cannot access server database directly
- Send typed RPC requests for all server operations
- Include JWT token with user scope in all requests
- Handle offline scenarios gracefully (desktop more capable than mobile)

### UI ↔ Domain

ViewModels expose state; UI observes and renders. UI:

- Cannot import repository implementations
- Cannot execute database queries
- Dispatches intents to ViewModels

### Desktop ↔ Mobile

While sharing UI components and domain logic:

- Desktop has SurrealDB-specific repository implementations
- Mobile has SQLite-specific repository implementations
- Some features are desktop-only (semantic search, graph traversal)
- Mobile is primary for daily operations (capture, completion)
- Desktop is primary for deep work (Harvest, writing)

### User ↔ User

Complete data isolation:

- All entities include `user_id` field
- Server enforces user scope on all queries
- No cross-user data access in v1
- Future sharing will use explicit opt-in

---

## Data Flow Patterns

### Local Operation (Desktop)

1. User interacts with UI
2. ViewModel receives intent
3. Repository executes SurrealDB query (with user scope)
4. Result flows back through ViewModel
5. UI re-renders

### Sync Flow

1. Client pulls changes from server (delta since last sync, filtered by user)
2. Server returns changed entities with versions
3. Client merges into local database
4. Client pushes local changes
5. Server resolves conflicts, returns authoritative versions
6. Client applies server decisions

### AI Request Flow

1. Client sends text/audio to server via AiService RPC
2. Server validates user authentication
3. Server processes with local models (embeddings/transcription)
4. For completion, server proxies to configured provider
5. Result streams back to client (token-by-token for completion)

### Inbox Triage Flow

1. User captures content to Universal Inbox
2. InboxItem stored locally with `user_id`
3. Syncs to server when connected
4. User triages on any device
5. Triage creates Quest/Note/Item, deletes InboxItem
6. TriageEvent published for reactive updates
7. Changes sync to all user's devices

### Routine Instance Flow

1. Routine scheduler runs periodically (server-side or desktop)
2. Checks Routines where `next_due <= now`
3. Creates Quest instance with `routine_id` reference
4. Updates Routine's `next_due` based on schedule
5. Publishes `RoutineInstanceSpawned` event
6. Quest appears in user's Today view
7. Optional notification at `time_of_day`

---

## Security Model

### Authentication

- JWT tokens issued by server AuthService
- Argon2 password hashing for stored credentials
- Invite-only registration (default, configurable)
- Tokens stored in platform-secure storage (Keychain, Keystore, etc.)
- Refresh tokens for long-lived sessions
- Optional 2FA via TOTP (v1.1)
- Optional OAuth/OIDC identity provider (v1.1)

### Multi-User Data Isolation

- Every entity includes `user_id` field
- All server queries filter by authenticated user
- Repository layer enforces user scope
- Admin users manage accounts but cannot view content
- No cross-user data access in v1
- Future collaboration via explicit sharing

### Transport Security

- All client-server communication over TLS
- Users configure HTTPS via reverse proxy (Traefik, Nginx)
- gRPC uses HTTP/2 with TLS

### Secret Storage

- API keys for completion providers stored server-side
- User passwords hashed with Argon2
- No sensitive credentials on client devices
- JWT tokens are the only client-stored secrets

### Storage Security

- Attachments stored with content-hash keys
- Per-user storage quotas (optional)
- Quota enforcement before upload
- S3-compatible API (local or cloud)

---

## Performance Considerations

### Startup

Target: < 2 seconds to interactive UI

- Database connections pooled and reused
- AI models loaded lazily on first use (server-side)
- UI renders immediately; data loads asynchronously
- User authentication cached until expiry

### Bundle Size

- Desktop: ~50-80MB (JVM runtime bundled)
- Android: ~15-25MB APK
- iOS: ~30-40MB IPA

### Database

Desktop SurrealDB queries should:

- Use indexes for frequently-filtered fields
- Leverage graph traversal instead of JOINs
- Batch embedding updates in background
- Always include `user_id` in WHERE clause

Mobile SQLite queries should:

- Use prepared statements via SQLDelight
- Limit result sets for list views
- Sync incrementally (not full table scans)
- Index `user_id` and common filter columns

### Sync

- Target: < 5 seconds for cross-device visibility
- Priority sync: Quests and Routines first (daily use)
- Batch attachment sync separately
- Exponential backoff for failed syncs

---

## References

- [ADR-001: Kotlin Multiplatform Architecture](../adr/001-single-tauri-application.md)
- [ADR-002: Hybrid Database Strategy](../adr/002-surrealdb-embedded.md)
- [ADR-005: kotlinx-rpc Communication](../adr/005-kotlinx-rpc-communication.md)
- [ADR-006: Server-Centralized AI](../adr/006-server-centralized-ai.md)
- [ADR-007: Docker Compose Deployment](../adr/007-docker-compose-deployment.md)
- [ADR-008: Compose Unstyled + Altair Theme](../adr/008-compose-unstyled-altair-theme.md)
- [ADR-009: Core Library Stack](../adr/009-core-library-stack.md)

---

_Last updated: January 14, 2026_
