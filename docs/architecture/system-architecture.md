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
| Client-Server        | kotlinx-rpc (gRPC)        | Type-safe, streaming, compile-time checked                 |
| Server AI            | ort + whisper.cpp         | Local embeddings and transcription                         |
| Deployment           | Docker Compose            | Single-command self-hosted deployment                      |
| Testing              | Mokkery + Turbine         | Multiplatform mocking, Flow testing                        |

---

## Component Architecture

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                     Docker Compose (self-hosted server)                     │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                        altair-server (Ktor)                           │  │
│  │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────────┐   │  │
│  │  │  kotlinx-rpc    │  │   AI Service    │  │    Sync Engine      │   │  │
│  │  │   endpoints     │  │ embed/transcribe│  │  conflict resolve   │   │  │
│  │  └─────────────────┘  └─────────────────┘  └─────────────────────┘   │  │
│  └───────────────────────────────────────────────────────────────────────┘  │
│  ┌───────────────────────┐  ┌─────────────────────────────────────────┐    │
│  │      SurrealDB        │  │        Ollama (optional)                │    │
│  │      container        │  │        local completion                 │    │
│  └───────────────────────┘  └─────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────────┘
                                      ▲
                                      │ kotlinx-rpc (gRPC-compatible)
                      ┌───────────────┴───────────────┐
                      │                               │
           ┌──────────┴──────────┐        ┌───────────┴───────────┐
           │       Desktop       │        │        Mobile         │
           │  Compose Multiplatf │        │   Compose Multiplatf  │
           │  SurrealDB embedded │        │   SQLite embedded     │
           │  Full features      │        │   Quick capture       │
           └─────────────────────┘        └───────────────────────┘
```

---

## Platform Targets

### Desktop (Windows, Linux, macOS)

Full-featured application with:

- All three modules: Guidance, Knowledge, Tracking
- SurrealDB embedded for offline operation
- Graph queries, vector search, semantic similarity
- Offline-capable with sync when server available
- Focus mode, keyboard shortcuts, multi-window

### Mobile (Android, iOS)

Lightweight quick-capture application with:

- Basic CRUD for Quests, Notes, Items
- SQLite embedded for simplicity and reliability
- Server-dependent for AI features
- Sync-first architecture (requires server for full functionality)
- Voice capture, camera for item photos

### Server (Self-Hosted)

Docker Compose stack providing:

- Sync hub for all client devices
- AI services (embeddings, transcription, completion proxy)
- User authentication and data isolation
- SurrealDB as primary data store

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

- **Entities**: Quest, Epic, Note, Item, etc. (data classes)
- **Repositories**: Interfaces defining data operations
- **Use Cases**: Business logic operations (optional, for complex flows)
- **Validation**: Input validation rules

### Data Layer (Platform-Specific)

Implements repository interfaces with platform-specific storage:

**Desktop:**

- SurrealDB via surrealdb.java JNI bindings
- Full query capabilities (graph traversal, vector search)

**Mobile:**

- SQLite via SQLDelight
- Type-safe generated queries
- Simpler schema without graph relations

**Server:**

- SurrealDB primary store
- Sync version tracking
- Conflict resolution logic

### Network Layer

Client-server communication via kotlinx-rpc:

- **SyncService**: Pull/push changes, stream real-time updates
- **AiService**: Embeddings, transcription, completion
- **AuthService**: Login, token refresh, logout

---

## Key Boundaries

### Client ↔ Server

The kotlinx-rpc layer is a hard boundary. Clients:

- Cannot access server database directly
- Send typed RPC requests for all server operations
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

---

## Data Flow Patterns

### Local Operation (Desktop)

1. User interacts with UI
2. ViewModel receives intent
3. Repository executes SurrealDB query
4. Result flows back through ViewModel
5. UI re-renders

### Sync Flow

1. Client pulls changes from server (delta since last sync)
2. Server returns changed entities with versions
3. Client merges into local database
4. Client pushes local changes
5. Server resolves conflicts, returns authoritative versions
6. Client applies server decisions

### AI Request Flow

1. Client sends text/audio to server via AiService RPC
2. Server processes with local models (embeddings/transcription)
3. For completion, server proxies to configured provider
4. Result streams back to client (token-by-token for completion)

---

## Security Model

### Authentication

- JWT tokens issued by server AuthService
- Tokens stored in platform-secure storage (Keychain, Keystore, etc.)
- Refresh tokens for long-lived sessions

### Data Isolation

- Server enforces per-user data access
- SurrealDB row-level permissions by user namespace
- Clients cannot access other users' data

### Transport Security

- All client-server communication over TLS
- Users configure HTTPS via reverse proxy (Traefik, Nginx)
- gRPC uses HTTP/2 with TLS

### Secret Storage

- API keys for completion providers stored server-side
- No sensitive credentials on client devices

---

## Performance Considerations

### Startup

Target: < 2 seconds to interactive UI

- Database connections pooled and reused
- AI models loaded lazily on first use (server-side)
- UI renders immediately; data loads asynchronously

### Bundle Size

- Desktop: ~50-80MB (JVM runtime bundled)
- Android: ~15-25MB APK
- iOS: ~30-40MB IPA

### Database

Desktop SurrealDB queries should:

- Use indexes for frequently-filtered fields
- Leverage graph traversal instead of JOINs
- Batch embedding updates in background

Mobile SQLite queries should:

- Use prepared statements via SQLDelight
- Limit result sets for list views
- Sync incrementally (not full table scans)

---

## References

- [ADR-001: Kotlin Multiplatform Architecture](../adr/001-single-tauri-application.md)
- [ADR-002: Hybrid Database Strategy](../adr/002-surrealdb-embedded.md)
- [ADR-005: kotlinx-rpc Communication](../adr/005-kotlinx-rpc-communication.md)
- [ADR-006: Server-Centralized AI](../adr/006-server-centralized-ai.md)
- [ADR-007: Docker Compose Deployment](../adr/007-docker-compose-deployment.md)
- [ADR-008: Compose Unstyled + Altair Theme](../adr/008-compose-unstyled-altair-theme.md)
