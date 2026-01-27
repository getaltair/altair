# Altair Architecture Overview

## Purpose

This document provides a high-level map of Altair's architecture documentation. It serves as the
entry point for understanding system design and navigating to detailed specifications.

## System Synopsis

Altair is a life management ecosystem designed to externalize executive function, consisting of three
interconnected modules—Guidance (task execution), Knowledge (information capture), and Tracking
(inventory management)—unified by system-level features: Universal Inbox, Initiatives, and Routines.
Built with Kotlin Multiplatform and Compose Multiplatform for desktop and mobile, the system emphasizes
privacy-focused self-hosted architecture, multi-user support with complete data isolation, and
seamless cross-module integration.

### Technology Stack

| Layer                | Technology            | Version | Purpose                                       |
| -------------------- | --------------------- | ------- | --------------------------------------------- |
| UI Framework         | Compose Multiplatform | 1.8+    | Cross-platform UI (desktop, Android, iOS)     |
| UI Components        | Compose Unstyled      | 1.49+   | Headless primitives with Altair theme         |
| Navigation           | Decompose             | 3.x     | UI-agnostic navigation with back handling     |
| Shared Logic         | Kotlin Multiplatform  | 2.2+    | Domain models, validation, repositories       |
| Dependency Injection | Koin                  | 4.x     | Runtime DI with Compose integration           |
| Error Handling       | Arrow                 | 2.x     | Typed errors, validation, optics              |
| Desktop Database     | SurrealDB             | 2.0+    | Embedded graph database with vector search    |
| Mobile Database      | SQLite (SQLDelight)   | 2.0+    | Lightweight embedded database                 |
| Server Framework     | Ktor                  | 3.1+    | Self-hosted backend with kotlinx-rpc          |
| Server Database      | SurrealDB             | 2.0+    | Primary data store and sync hub               |
| Storage Backend      | S3-compatible         | -       | Attachment storage (local/MinIO/S3/B2)        |
| AI Services          | ort + whisper.cpp     | latest  | Server-side embeddings and transcription      |
| Auth                 | JWT + Argon2          | -       | Multi-user authentication with data isolation |
| Testing              | Mokkery + Turbine     | 3.x/1.x | Multiplatform mocking and Flow testing        |
| Deployment           | Docker Compose        | 3.8+    | Self-hosted server stack                      |

### Core Design Principles

1. **Privacy-focused**: Self-hosted server keeps all data on your infrastructure
2. **Multi-user support**: Complete data isolation between users on shared instances
3. **Offline-capable**: Desktop and mobile work fully offline; sync when connected
4. **Single codebase**: Kotlin Multiplatform shares 90%+ code across platforms
5. **Shared data layer**: Hybrid database strategy (SurrealDB desktop/server, SQLite mobile)
6. **Event-driven integration**: Loose coupling via internal event bus (desktop)
7. **ADHD-optimized**: UI enforces constraints that externalize executive function

---

## Architecture Documents

### Core Documents

| Document                                           | Purpose                     | Key Contents                                          |
| -------------------------------------------------- | --------------------------- | ----------------------------------------------------- |
| [system-architecture.md](./system-architecture.md) | Technical infrastructure    | KMP/Compose stack, platform targets, layer boundaries |
| [domain-model.md](./domain-model.md)               | Business logic and entities | User, Initiative, Quest, Note, Item; relationships    |
| [persistence.md](./persistence.md)                 | Data storage                | Multi-user schema, sync protocol, migrations          |
| [event-bus.md](./event-bus.md)                     | Inter-module communication  | Event types, pub/sub patterns, contracts              |

### Architecture Decision Records

| ADR                                                    | Decision                                     | Status   |
| ------------------------------------------------------ | -------------------------------------------- | -------- |
| [ADR-001](../adr/001-single-tauri-application.md)      | Kotlin Multiplatform + Compose Multiplatform | Accepted |
| [ADR-002](../adr/002-surrealdb-embedded.md)            | Hybrid Database (SurrealDB + SQLite)         | Accepted |
| [ADR-003](../adr/003-event-bus-for-modules.md)         | Event Bus for Modules                        | Accepted |
| [ADR-005](../adr/005-kotlinx-rpc-communication.md)     | kotlinx-rpc for Client-Server Communication  | Accepted |
| [ADR-006](../adr/006-server-centralized-ai.md)         | Server-Centralized AI Services               | Accepted |
| [ADR-007](../adr/007-docker-compose-deployment.md)     | Docker Compose Deployment                    | Accepted |
| [ADR-008](../adr/008-compose-unstyled-altair-theme.md) | Compose Unstyled + Custom Altair Theme       | Accepted |
| [ADR-009](../adr/009-core-library-stack.md)            | Core Library Stack (Koin, Decompose, Arrow)  | Accepted |
| [ADR-010](../adr/010-universal-inbox-architecture.md)  | Universal Inbox Architecture                 | Accepted |
| [ADR-011](../adr/011-initiative-system-design.md)      | Initiative System Design                     | Accepted |
| [ADR-012](../adr/012-multi-user-data-isolation.md)     | Multi-User Data Isolation                    | Accepted |
| [ADR-013](../adr/013-routine-scheduling-strategy.md)   | Routine Scheduling Strategy                  | Accepted |
| [ADR-014](../adr/014-source-document-architecture.md)  | Source Document Architecture                 | Accepted |

---

## System-Level Features

### Universal Inbox

System-wide capture point that defers type decisions until triage:

- **Capture methods**: Keyboard, voice, camera, share, widget, watch
- **Triage actions**: Convert to Quest, Note, or Item
- **Mobile home**: Default screen shows Inbox + Today summary
- **Desktop access**: Global keyboard shortcut, sidebar navigation

### Initiatives

Cross-cutting organizational units that group related content:

- **Scope**: Links Epics (Guidance), Notes (Knowledge), and Items (Tracking)
- **Types**: Projects (have end date) vs. Areas (maintained indefinitely)
- **Nesting**: Up to 3 levels (Area → Project → Sub-project)
- **Initiative Card**: Persistent cross-app context display

### Routines

Recurring templates that spawn Quest instances:

- **Schedules**: Daily, weekly, monthly, custom intervals
- **Notifications**: Optional time-of-day reminders
- **Flexibility**: Skip, complete early, pause without breaking routine
- **Tracking**: Instances appear in Harvest like regular Quests

---

## Module Overview

### Guidance (Task Execution)

Quest-Based Agile methodology with energy-based planning:

- **Epics**: Long-term goals containing multiple Quests
- **Quests**: Focused tasks with energy cost (1-5) and WIP=1 default
- **Checkpoints**: Optional sub-steps within a Quest
- **Energy Budget**: Daily capacity tracking with soft limits
- **Today View**: Morning landing showing routines + Quests for the day

### Knowledge (Information Capture)

Personal knowledge management with bidirectional linking:

- **Notes**: Markdown content with wiki-link syntax `[[Title]]`
- **SourceDocuments**: Imported files (PDF, Markdown) searchable with annotations
- **Folders**: Hierarchical organization
- **Tags**: Cross-cutting categorization (shared across modules)
- **Semantic Search**: Vector similarity for discovering related notes (desktop)

### Tracking (Inventory Management)

Physical item management with location awareness:

- **Items**: Named objects with quantity and optional photo
- **Locations**: Hierarchical places (Room → Shelf)
- **Containers**: Movable storage (Box, Toolbox)
- **Custom Fields**: Template-based additional attributes

---

## Cross-Module Integration

Modules communicate via events and share linked data through Initiatives:

```
Initiative (cross-cutting context)
├── Epic (Guidance) ←→ Note (Knowledge)
│   └── Quest → Item requirements (Tracking)
├── Note ←→ Item references
├── SourceDocument (Knowledge) ←→ Annotation Notes
└── Tag (shared taxonomy)

Universal Inbox
└── Triage → Quest | Note | Item | SourceDocument

Routine (Guidance)
└── Spawns → Quest instances

WatchedFolder
└── Discovers → SourceDocument (auto-import)
```

The event bus enables reactive features:

- Quest completion prompts reflection Note creation
- Item quantity zero suggests restock Quest
- Note content changes trigger re-embedding for similarity search
- Initiative focus updates all module displays
- Inbox triage notifies destination modules
- Routine due time triggers notifications
- SourceDocument extraction completes and becomes searchable
- WatchedFolder scan discovers new or modified files

---

## Platform Targets

| Platform                  | Scope                   | Database           | AI Services             |
| ------------------------- | ----------------------- | ------------------ | ----------------------- |
| Desktop (Win/Linux/macOS) | Full features           | SurrealDB embedded | Server + local fallback |
| Mobile (Android/iOS)      | Daily driver + capture  | SQLite embedded    | Server required         |
| Server (Docker)           | Sync + AI + Auth hub    | SurrealDB          | Local inference         |
| Web (minimal)             | View-only admin access  | N/A                | N/A                     |

### Platform Roles

- **Mobile**: Primary for daily operations (energy check-in, routine completion, capture, Quest execution)
- **Desktop**: Primary for reflection and deep work (Weekly Harvest, Knowledge writing, complex views)
- **Server**: Central hub for sync, AI services, user authentication, storage

---

## Multi-User Architecture

### Data Isolation

- Every entity includes `user_id` field
- All queries filter by authenticated user
- No cross-user data access
- Admin users manage accounts but cannot view content

### Authentication

- JWT tokens with user scope
- Argon2 password hashing
- Invite-only registration (default)
- Optional 2FA (v1.1)

### Storage

- S3-compatible backend for attachments
- Per-user storage quotas (optional)
- Quota enforcement on upload

---

## References

- [Implementation Plan](../implementation-plan.md) — Feature specs in development order
- [Requirements](../requirements/) — PRDs with functional requirements
- [AGENTS.md](../../AGENTS.md) — AI assistant coding conventions

---

_Last updated: January 14, 2026_
