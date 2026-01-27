# Altair Documentation

**Life Management Ecosystem**

> Privacy-focused productivity suite consisting of Guidance (task execution), Knowledge (information
> capture), and Tracking (inventory management), unified by Initiatives, Universal Inbox, and
> Routines. Designed to externalize executive function with thoughtful defaults that work especially
> well for neurodivergent users. Self-hosted architecture keeps your data on your own infrastructure.

---

## Quick Links

| I want to...                | Go to...                                            |
| --------------------------- | --------------------------------------------------- |
| Understand what Altair does | [Architecture Overview](./architecture/overview.md) |
| Start building features     | [Implementation Plan](./implementation-plan.md)     |
| See all requirements        | [Requirements Index](./requirements/)               |
| Understand the data model   | [Domain Model](./architecture/domain-model.md)      |
| Set up my dev environment   | [README.md](../README.md)                           |
| Contribute to Altair        | [CONTRIBUTING.md](../CONTRIBUTING.md)               |
| Know why we chose X         | [Architecture Decisions](./adr/)                    |
| Look up a term              | [Glossary](./reference/glossary.md)                 |

---

## Documentation Structure

```
altair/
├── README.md              # Getting started, dev setup
├── CONTRIBUTING.md        # Contribution guidelines
├── AGENTS.md              # AI coding assistant instructions
└── docs/
    ├── index.md           # This file
    ├── implementation-plan.md  # 12-phase development roadmap
    ├── requirements/      # What the system must do (PRDs)
    │   ├── altair-prd-core.md
    │   ├── altair-prd-guidance.md
    │   ├── altair-prd-knowledge.md
    │   └── altair-prd-tracking.md
    ├── architecture/      # How the system is designed
    │   ├── overview.md
    │   ├── system-architecture.md
    │   ├── domain-model.md
    │   ├── persistence.md
    │   └── event-bus.md
    ├── adr/               # Why we made key decisions
    │   ├── 000-template.md
    │   ├── 001-*.md       # Kotlin Multiplatform Architecture
    │   ├── 002-*.md       # Hybrid Database Strategy
    │   ├── 003-*.md       # Event Bus for Modules
    │   ├── 004-*.md       # AI Provider Adapters (superseded)
    │   ├── 005-*.md       # kotlinx-rpc Communication
    │   ├── 006-*.md       # Server-Centralized AI
    │   ├── 007-*.md       # Docker Compose Deployment
    │   ├── 008-*.md       # Compose Unstyled + Altair Theme
    │   ├── 009-*.md       # Core Library Stack (Koin, Decompose, Arrow, Mokkery)
    │   ├── 010-*.md       # Universal Inbox Architecture
    │   ├── 011-*.md       # Initiative System Design
    │   ├── 012-*.md       # Multi-User Data Isolation
    │   ├── 013-*.md       # Routine Scheduling Strategy
    │   ├── 014-*.md       # Source Document Architecture
    │   ├── 015-*.md       # Rich Text Editing Library
    │   └── 016-*.md       # Ktor REST API (supersedes ADR-005)
    └── reference/
        ├── glossary.md
        └── desktop-service-architectures.md
```

---

## Reading Order

### New to Altair?

1. **[Architecture Overview](./architecture/overview.md)** — What Altair is and how it's organized
2. **[PRD Core](./requirements/altair-prd-core.md)** — System-level requirements and design
   principles
3. **[Glossary](./reference/glossary.md)** — Terminology reference

### Understanding the Design?

1. **[System Architecture](./architecture/system-architecture.md)** — Tech stack and component
   relationships
2. **[Domain Model](./architecture/domain-model.md)** — Entities, relationships, and business rules
3. **[Persistence](./architecture/persistence.md)** — Database schema and data lifecycle
4. **[Event Bus](./architecture/event-bus.md)** — Inter-module communication
5. **[ADR Index](./adr/)** — Key architectural decisions and rationale

### Starting Development?

1. **[README.md](../README.md)** — Prerequisites and setup
2. **[AGENTS.md](../AGENTS.md)** — AI assistant instructions and conventions
3. **[Implementation Plan](./implementation-plan.md)** — 12-phase development roadmap
4. **[System Architecture](./architecture/system-architecture.md)** — Technical overview

---

## Requirements Documents

| Document                                                          | Scope                                                     |
| ----------------------------------------------------------------- | --------------------------------------------------------- |
| [altair-prd-core.md](./requirements/altair-prd-core.md)           | System overview, Initiatives, Inbox, Routines, multi-user |
| [altair-prd-guidance.md](./requirements/altair-prd-guidance.md)   | Quest-Based Agile task execution (FR-G-\*)                |
| [altair-prd-knowledge.md](./requirements/altair-prd-knowledge.md) | Personal knowledge management (FR-K-\*)                   |
| [altair-prd-tracking.md](./requirements/altair-prd-tracking.md)   | Inventory and asset management (FR-T-\*)                  |

---

## Architecture Decisions

| ADR                                                   | Decision                                             | Status                |
| ----------------------------------------------------- | ---------------------------------------------------- | --------------------- |
| [ADR-001](./adr/001-single-tauri-application.md)      | Kotlin Multiplatform + Compose Multiplatform         | Accepted              |
| [ADR-002](./adr/002-surrealdb-embedded.md)            | Hybrid Database (SurrealDB + SQLite)                 | Accepted              |
| [ADR-003](./adr/003-event-bus-for-modules.md)         | Event Bus for Modules                                | Accepted              |
| [ADR-004](./adr/004-ai-provider-adapters.md)          | AI Provider Adapters                                 | Superseded by ADR-006 |
| [ADR-005](./adr/005-kotlinx-rpc-communication.md)     | kotlinx-rpc for Client-Server Communication          | Superseded by ADR-016 |
| [ADR-006](./adr/006-server-centralized-ai.md)         | Server-Centralized AI Services                       | Accepted              |
| [ADR-007](./adr/007-docker-compose-deployment.md)     | Docker Compose Deployment                            | Accepted              |
| [ADR-008](./adr/008-compose-unstyled-altair-theme.md) | Compose Unstyled + Custom Altair Theme               | Accepted              |
| [ADR-009](./adr/009-core-library-stack.md)            | Core Library Stack (Koin, Decompose, Arrow, Mokkery) | Accepted              |
| [ADR-010](./adr/010-universal-inbox-architecture.md)  | Universal Inbox Architecture                         | Accepted              |
| [ADR-011](./adr/011-initiative-system-design.md)      | Initiative System Design                             | Accepted              |
| [ADR-012](./adr/012-multi-user-data-isolation.md)     | Multi-User Data Isolation                            | Accepted              |
| [ADR-013](./adr/013-routine-scheduling-strategy.md)   | Routine Scheduling Strategy                          | Accepted              |
| [ADR-014](./adr/014-source-document-architecture.md)  | Source Document Architecture                         | Accepted              |
| [ADR-015](./adr/ADR-015-rich-text-editing-library.md) | Rich Text Editing Library (compose-rich-editor)      | Proposed              |
| [ADR-016](./adr/016-ktor-rest-api.md)                 | Ktor REST API (replaces kotlinx-rpc)                 | Accepted              |

---

## Key Concepts

### System-Level Features

- **Universal Inbox** — System-wide capture point; items are untyped until triaged into Quest, Note, or Item
- **Initiatives** — Cross-cutting organizational units linking content across Guidance, Knowledge, and Tracking
- **Routines** — Recurring templates that spawn Quest instances on schedule (daily, weekly, monthly, custom)

### Application Modules

- **Guidance** — Quest-Based Agile task execution with WIP=1 default and energy-based planning
- **Knowledge** — Markdown PKM with bidirectional linking and semantic search
- **Tracking** — Inventory management with photo-first capture and location tracking

### Platform Strategy

- **Mobile (Android, iOS)** — Daily driver for capture, routine completion, Quest execution
- **Desktop (Windows, Linux, macOS)** — Deep work for reflection, Harvest, Knowledge writing
- **Server (Self-hosted Docker)** — Sync hub, AI services, multi-user authentication

### Design Principles

- **Externalize executive function** — System enforces constraints so users don't rely on willpower
- **Variable capacity** — Energy system adapts to daily fluctuations
- **No shame** — Incomplete work is pausable without judgment
- **Immediate feedback** — Visual confirmations and progress indicators
- **Progressive disclosure** — Hide complexity until needed

### Multi-User Architecture

- **Data isolation** — Complete separation between users on shared instances
- **Invite-only** — Admin generates invite codes for new user registration
- **Storage quotas** — Optional per-user limits for attachments
- **Admin separation** — Admins manage accounts but cannot view user content

### Technology Stack

| Layer                | Technology                                    |
| -------------------- | --------------------------------------------- |
| UI Framework         | Compose Multiplatform                         |
| UI Components        | Compose Unstyled + Altair                     |
| Navigation           | Decompose                                     |
| Shared Logic         | Kotlin Multiplatform                          |
| Dependency Injection | Koin                                          |
| Error Handling       | Arrow (core + optics)                         |
| Desktop Database     | SurrealDB embedded                            |
| Mobile Database      | SQLite (SQLDelight)                           |
| Server Framework     | Ktor + kotlinx-rpc                            |
| Server Database      | SurrealDB                                     |
| Storage Backend      | S3-compatible (local, MinIO, S3, Backblaze)   |
| Server AI            | ONNX Runtime (embeddings), whisper.cpp (transcription) |
| Authentication       | JWT + Argon2                                  |
| Testing              | Mokkery + Turbine                             |
| Deployment           | Docker Compose                                |

---

_Last updated: January 26, 2026_
