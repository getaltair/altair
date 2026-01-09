# Altair Architecture Overview

## Purpose

This document provides a high-level map of Altair's architecture documentation.
It serves as the entry point for understanding system design and navigating to detailed specifications.

## System Synopsis

Altair is an ADHD-focused productivity ecosystem consisting of three interconnected modules—Guidance (task management),
Knowledge (PKM), and Tracking (inventory)—built with Kotlin Multiplatform and Compose Multiplatform for desktop and
mobile. The system emphasizes privacy-focused self-hosted architecture, externalized executive function, and seamless
cross-module integration.

### Technology Stack

| Layer            | Technology              | Version | Purpose                                    |
| ---------------- | ----------------------- | ------- | ------------------------------------------ |
| UI Framework     | Compose Multiplatform   | 1.8+    | Cross-platform UI (desktop, Android, iOS)  |
| UI Components    | Compose Unstyled        | 1.49+   | Headless primitives with Altair theme      |
| Shared Logic     | Kotlin Multiplatform    | 2.2+    | Domain models, validation, repositories    |
| Desktop Database | SurrealDB               | 2.0+    | Embedded graph database with vector search |
| Mobile Database  | SQLite (SQLDelight)     | 2.0+    | Lightweight embedded database              |
| Server Framework | Ktor                    | 3.1+    | Self-hosted backend with kotlinx-rpc       |
| Server Database  | SurrealDB               | 2.0+    | Primary data store and sync hub            |
| AI Services      | ort + whisper.cpp       | latest  | Server-side embeddings and transcription   |
| Deployment       | Docker Compose          | 3.8+    | Self-hosted server stack                   |

### Core Design Principles

1. **Privacy-focused**: Self-hosted server keeps all data on your infrastructure
2. **Offline-capable**: Desktop works fully offline; mobile requires server for AI
3. **Single codebase**: Kotlin Multiplatform shares 90%+ code across platforms
4. **Shared data layer**: Hybrid database strategy (SurrealDB desktop/server, SQLite mobile)
5. **Event-driven integration**: Loose coupling via internal event bus (desktop)
6. **ADHD-optimized**: UI enforces constraints that externalize executive function

---

## Architecture Documents

### Core Documents

| Document                                           | Purpose                     | Key Contents                                         |
| -------------------------------------------------- | --------------------------- | ---------------------------------------------------- |
| [system-architecture.md](./system-architecture.md) | Technical infrastructure    | KMP/Compose stack, platform targets, layer boundaries |
| [domain-model.md](./domain-model.md)               | Business logic and entities | Quest, Note, Item models; cross-module relationships |
| [persistence.md](./persistence.md)                 | Data storage                | Hybrid database schema, sync protocol, migrations    |
| [event-bus.md](./event-bus.md)                     | Inter-module communication  | Event types, pub/sub patterns, contracts             |

### Architecture Decision Records

| ADR | Decision | Status |
|-----|----------|--------|
| [ADR-001](../adr/001-single-tauri-application.md) | Kotlin Multiplatform + Compose Multiplatform | Accepted |
| [ADR-002](../adr/002-surrealdb-embedded.md) | Hybrid Database (SurrealDB + SQLite) | Accepted |
| [ADR-003](../adr/003-event-bus-for-modules.md) | Event Bus for Modules | Accepted |
| [ADR-005](../adr/005-kotlinx-rpc-communication.md) | kotlinx-rpc for Client-Server Communication | Accepted |
| [ADR-006](../adr/006-server-centralized-ai.md) | Server-Centralized AI Services | Accepted |
| [ADR-007](../adr/007-docker-compose-deployment.md) | Docker Compose Deployment | Accepted |
| [ADR-008](../adr/008-compose-unstyled-altair-theme.md) | Compose Unstyled + Custom Altair Theme | Accepted |

---

## Module Overview

### Guidance (Task Management)

Quest-Based Agile methodology with energy-based planning:

- **Epics**: Long-term goals containing multiple Quests
- **Quests**: Focused tasks with energy cost (1-5) and WIP=1 enforcement
- **Checkpoints**: Optional sub-steps within a Quest
- **Energy Budget**: Daily capacity tracking with soft limits

### Knowledge (PKM)

Personal knowledge management with bidirectional linking:

- **Notes**: Markdown content with wiki-link syntax `[[Title]]`
- **Folders**: Hierarchical organization
- **Tags**: Cross-cutting categorization
- **Semantic Search**: Vector similarity for discovering related notes (desktop)

### Tracking (Inventory)

Physical item management with location awareness:

- **Items**: Named objects with quantity and optional photo
- **Locations**: Hierarchical places (Room → Shelf)
- **Containers**: Movable storage (Box, Toolbox)
- **Custom Fields**: Template-based additional attributes

---

## Cross-Module Integration

Modules communicate via events and share linked data:

```
Quest ←→ Note      (Quest references notes for context)
Quest ←→ Item      (Quest links to required tools/materials)
Note  ←→ Item      (Notes mention items via [[Item:Name]])
```

The event bus enables reactive features:
- Quest completion prompts reflection Note creation
- Item quantity zero suggests restock Quest
- Note content changes trigger re-embedding for similarity search

---

## Platform Targets

| Platform | Scope | Database | AI Services |
|----------|-------|----------|-------------|
| Desktop (Win/Linux/macOS) | Full features | SurrealDB embedded | Server + local fallback |
| Mobile (Android/iOS) | Quick capture + view | SQLite embedded | Server required |
| Server (Docker) | Sync + AI hub | SurrealDB | Local inference |

---

## References

- [Implementation Plan](../implementation-plan.md) — Feature specs in development order
- [Requirements](../requirements/) — PRDs with functional requirements
- [AGENTS.md](../../AGENTS.md) — AI assistant coding conventions

---

*Last updated: January 9, 2026*
