# Altair Documentation

**ADHD-Focused Productivity Ecosystem**

> Privacy-focused productivity suite consisting of Guidance (task management), Knowledge (PKM), and
> Tracking (inventory), designed to externalize executive function for individuals with ADHD.
> Self-hosted architecture keeps your data on your own infrastructure.

---

## Quick Links

| I want to...                | Go to...                                            |
| --------------------------- | --------------------------------------------------- |
| Understand what Altair does | [Architecture Overview](./architecture/overview.md) |
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
    ├── implementation-plan.md  # Feature specs for development
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
    │   └── 009-*.md       # Core Library Stack (Koin, Decompose, Arrow, Mokkery)
    └── reference/
        ├── glossary.md
        └── desktop-service-architectures.md
```

---

## Reading Order

### New to Altair?

1. **[Architecture Overview](./architecture/overview.md)** — What Altair is and how it's organized
2. **[PRD Core](./requirements/altair-prd-core.md)** — System-level requirements and ADHD design
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
3. **[Implementation Plan](./implementation-plan.md)** — Feature specs in development order
4. **[System Architecture](./architecture/system-architecture.md)** — Technical overview

---

## Requirements Documents

| Document                                                          | Scope                                                      |
| ----------------------------------------------------------------- | ---------------------------------------------------------- |
| [altair-prd-core.md](./requirements/altair-prd-core.md)           | System overview, ADHD principles, cross-module integration |
| [altair-prd-guidance.md](./requirements/altair-prd-guidance.md)   | Quest-Based Agile task management (FR-G-\*)                |
| [altair-prd-knowledge.md](./requirements/altair-prd-knowledge.md) | Personal knowledge management (FR-K-\*)                    |
| [altair-prd-tracking.md](./requirements/altair-prd-tracking.md)   | Inventory and asset management (FR-T-\*)                   |

---

## Architecture Decisions

| ADR                                                   | Decision                                             | Status                |
| ----------------------------------------------------- | ---------------------------------------------------- | --------------------- |
| [ADR-001](./adr/001-single-tauri-application.md)      | Kotlin Multiplatform + Compose Multiplatform         | Accepted              |
| [ADR-002](./adr/002-surrealdb-embedded.md)            | Hybrid Database (SurrealDB + SQLite)                 | Accepted              |
| [ADR-003](./adr/003-event-bus-for-modules.md)         | Event Bus for Modules                                | Accepted              |
| [ADR-004](./adr/004-ai-provider-adapters.md)          | AI Provider Adapters                                 | Superseded by ADR-006 |
| [ADR-005](./adr/005-kotlinx-rpc-communication.md)     | kotlinx-rpc for Client-Server Communication          | Accepted              |
| [ADR-006](./adr/006-server-centralized-ai.md)         | Server-Centralized AI Services                       | Accepted              |
| [ADR-007](./adr/007-docker-compose-deployment.md)     | Docker Compose Deployment                            | Accepted              |
| [ADR-008](./adr/008-compose-unstyled-altair-theme.md) | Compose Unstyled + Custom Altair Theme               | Accepted              |
| [ADR-009](./adr/009-core-library-stack.md)            | Core Library Stack (Koin, Decompose, Arrow, Mokkery) | Accepted              |

---

## Key Concepts

### Application Modules

- **Guidance** — Quest-Based Agile task management with WIP=1 enforcement and energy-based planning
- **Knowledge** — Obsidian-style PKM with bidirectional linking and semantic search
- **Tracking** — Inventory management with photo-first capture and location tracking

### Platform Targets

- **Desktop** (Windows, Linux, macOS) — Full-featured, SurrealDB embedded, offline-capable
- **Mobile** (Android, iOS) — Quick capture + view, SQLite embedded, server-dependent for AI
- **Server** (Self-hosted Docker) — Sync hub, AI services, multi-device coordination

### ADHD Design Principles

- **Externalize executive function** — System enforces constraints so users don't rely on willpower
- **Variable capacity** — Energy system adapts to daily fluctuations
- **No shame** — Incomplete work is pausable without judgment
- **Immediate feedback** — Visual confirmations and progress indicators
- **Progressive disclosure** — Hide complexity until needed

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
| Server AI            | ort (embeddings), whisper.cpp (transcription) |
| Testing              | Mokkery + Turbine                             |
| Deployment           | Docker Compose                                |

---

_Last updated: January 9, 2026_
