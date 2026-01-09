# Altair Documentation

**ADHD-Focused Productivity Ecosystem**

> Local-first productivity suite consisting of Guidance (task management), Knowledge (PKM), and Tracking (inventory), designed to externalize executive function for individuals with ADHD.

---

## Quick Links

| I want to... | Go to... |
|--------------|----------|
| Understand what Altair does | [Architecture Overview](./architecture/overview.md) |
| See all requirements | [Requirements Index](./requirements/) |
| Understand the data model | [Domain Model](./architecture/domain-model.md) |
| Set up my dev environment | [README.md](../README.md) |
| Contribute to Altair | [CONTRIBUTING.md](../CONTRIBUTING.md) |
| Know why we chose X | [Architecture Decisions](./adr/) |
| Look up a term | [Glossary](./reference/glossary.md) |

---

## Documentation Structure

```
altair/
├── README.md              # Getting started, dev setup
├── CONTRIBUTING.md        # Contribution guidelines
└── docs/
    ├── index.md           # This file
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
    │   ├── 001-single-tauri-application.md
    │   ├── 002-surrealdb-embedded.md
    │   ├── 003-event-bus-for-modules.md
    │   └── 004-ai-provider-adapters.md
    └── reference/
        ├── glossary.md
        └── desktop-service-architectures.md
```

---

## Reading Order

### New to Altair?

1. **[Architecture Overview](./architecture/overview.md)** — What Altair is and how it's organized
2. **[PRD Core](./requirements/altair-prd-core.md)** — System-level requirements and ADHD design principles
3. **[Glossary](./reference/glossary.md)** — Terminology reference

### Understanding the Design?

1. **[System Architecture](./architecture/system-architecture.md)** — Tech stack and component relationships
2. **[Domain Model](./architecture/domain-model.md)** — Entities, relationships, and business rules
3. **[Persistence](./architecture/persistence.md)** — SurrealDB schema and data lifecycle
4. **[Event Bus](./architecture/event-bus.md)** — Inter-module communication
5. **[ADR Index](./adr/)** — Key architectural decisions and rationale

### Starting Development?

1. **[README.md](../README.md)** — Prerequisites and setup
2. **[CONTRIBUTING.md](../CONTRIBUTING.md)** — Workflow, commits, code style
3. **[System Architecture](./architecture/system-architecture.md)** — Technical overview

---

## Requirements Documents

| Document | Scope |
|----------|-------|
| [altair-prd-core.md](./requirements/altair-prd-core.md) | System overview, ADHD principles, cross-module integration |
| [altair-prd-guidance.md](./requirements/altair-prd-guidance.md) | Quest-Based Agile task management (FR-G-*) |
| [altair-prd-knowledge.md](./requirements/altair-prd-knowledge.md) | Personal knowledge management (FR-K-*) |
| [altair-prd-tracking.md](./requirements/altair-prd-tracking.md) | Inventory and asset management (FR-T-*) |

---

## Architecture Decisions

| ADR | Decision | Status |
|-----|----------|--------|
| [ADR-001](./adr/001-single-tauri-application.md) | Single Tauri Application | Accepted |
| [ADR-002](./adr/002-surrealdb-embedded.md) | SurrealDB for Persistence | Accepted |
| [ADR-003](./adr/003-event-bus-for-modules.md) | Event Bus for Modules | Accepted |
| [ADR-004](./adr/004-ai-provider-adapters.md) | AI Provider Adapters | Accepted |

---

## Key Concepts

### Application Modules

- **Guidance** — Quest-Based Agile task management with WIP=1 enforcement and energy-based planning
- **Knowledge** — Obsidian-style PKM with bidirectional linking and semantic search
- **Tracking** — Inventory management with photo-first capture and location tracking

### ADHD Design Principles

- **Externalize executive function** — System enforces constraints so users don't rely on willpower
- **Variable capacity** — Energy system adapts to daily fluctuations
- **No shame** — Incomplete work is pausable without judgment
- **Immediate feedback** — Visual confirmations and progress indicators
- **Progressive disclosure** — Hide complexity until needed

### Technology Stack

| Layer | Technology |
|-------|------------|
| Desktop Runtime | Tauri 2 |
| Frontend | Svelte 5, shadcn-svelte |
| Backend | Rust (2024 edition) |
| Database | SurrealDB embedded |
| Local AI | ort (embeddings), whisper-rs (transcription) |
| Cloud AI | Anthropic, OpenAI, OpenRouter, Ollama |

---

*Last updated: January 8, 2026*
