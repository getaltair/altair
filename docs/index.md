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
| Set up my dev environment | [Getting Started](./development/getting-started.md) |
| Know why we chose X | [Architecture Decisions](./adr/) |
| Look up a term | [Glossary](./reference/glossary.md) |

---

## Documentation Structure

```
docs/
├── requirements/          # What the system must do (PRDs)
│   ├── altair-prd-core.md
│   ├── altair-prd-guidance.md
│   ├── altair-prd-knowledge.md
│   └── altair-prd-tracking.md
├── architecture/          # How the system is designed
│   ├── overview.md
│   ├── system-architecture.md
│   ├── domain-model.md
│   ├── persistence.md
│   └── event-bus.md
├── adr/                   # Why we made key decisions
│   ├── 001-single-tauri-application.md
│   ├── 002-surrealdb-embedded.md
│   ├── 003-event-bus-for-modules.md
│   └── 004-ai-provider-adapters.md
├── development/           # Developer guides
│   ├── getting-started.md
│   ├── development-workflow.md
│   └── testing-strategy.md
└── reference/             # Glossary, changelog
    └── glossary.md
```

---

## Reading Order

### New to Altair?

1. **[Architecture Overview](./architecture/overview.md)** — What Altair is and how it's organized
2. **[PRD Core](./requirements/altair-prd-core.md)** — System-level requirements and ADHD design principles
3. **[Glossary](./reference/glossary.md)** — Terminology reference
4. **[PRD Guidance](./requirements/altair-prd-guidance.md)** — Task management module
5. **[PRD Knowledge](./requirements/altair-prd-knowledge.md)** — Personal knowledge management module
6. **[PRD Tracking](./requirements/altair-prd-tracking.md)** — Inventory management module

### Starting Development?

1. **[Getting Started](./development/getting-started.md)** — Dev environment setup
2. **[Development Workflow](./development/development-workflow.md)** — Git flow, commits, CI/CD
3. **[System Architecture](./architecture/system-architecture.md)** — Technical infrastructure
4. **[Testing Strategy](./development/testing-strategy.md)** — How we test

### Understanding the Architecture?

1. **[System Architecture](./architecture/system-architecture.md)** — Tech stack and project structure
2. **[Domain Model](./architecture/domain-model.md)** — Entities and relationships
3. **[Persistence](./architecture/persistence.md)** — SurrealDB schema and sync
4. **[Event Bus](./architecture/event-bus.md)** — Inter-module communication
5. **[ADR Index](./adr/)** — Why we made key decisions

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

- **Guidance** — Quest-Based Agile task management with WIP=1 enforcement, energy-based filtering, and Focus/Zen mode
- **Knowledge** — Obsidian-like PKM with bidirectional linking, mind mapping, and auto-relationship discovery
- **Tracking** — Inventory management with photo-first capture, BoM intelligence, and maintenance tracking

### ADHD Design Principles

- **Externalize executive function** — System enforces constraints so users don't rely on willpower
- **Variable capacity** — Energy system adapts to daily fluctuations; rest is valid
- **No shame** — Incomplete work is pausable and recoverable without judgment
- **Immediate feedback** — Visual confirmations and progress indicators
- **Progressive disclosure** — Hide complexity until needed

### Technology Stack

| Layer | Technology |
|-------|------------|
| Desktop Runtime | Tauri 2+ |
| Frontend | Svelte 5+ with shadcn-svelte |
| Backend | Rust 1.83+ (2024 edition) |
| Database | SurrealDB 2.0+ (embedded) |
| AI Inference | ort (ONNX Runtime) |
| AI Providers | Ollama, Anthropic, OpenAI, OpenRouter |

---

## Contributing

1. Read [Development Workflow](./development/development-workflow.md)
2. Follow conventional commits format
3. Document decisions using [ADR template](./adr/000-template.md)
4. Reference PRD requirement IDs (FR-X-NNN) in code and commits

---

## License

AGPL v3 or later

---

*Last updated: January 8, 2026*
