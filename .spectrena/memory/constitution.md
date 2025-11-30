<!-- prettier-ignore-start -->

<!--
================================================================================
SYNC IMPACT REPORT
================================================================================
Version Change: 0.0.0 → 1.0.0 (Initial constitution)
Modified Principles: N/A (new document)
Added Sections:
  - I. Local-First Architecture
  - II. ADHD-Friendly Design
  - III. Ubiquitous Language
  - IV. Soft Delete & Data Recovery
  - V. Plugin Architecture for Extensibility
  - VI. Privacy by Default
  - VII. Spec-Driven Development
  - Development Standards section
  - Workflow Requirements section
  - Governance section
Removed Sections: N/A
Templates Requiring Updates:
  - .spectrena/templates/plan-template.md ✅ (Constitution Check section present)
  - .spectrena/templates/spec-template.md ✅ (No constitution-specific sections needed)
  - .spectrena/templates/tasks-template.md ✅ (No constitution-specific sections needed)
Follow-up TODOs: None
================================================================================
-->

<!-- prettier-ignore-end -->

# Altair Constitution

> **Guiding principles for the Altair ADHD-focused productivity ecosystem**

## Core Principles

### I. Local-First Architecture

All features MUST work fully offline. Cloud connectivity is an enhancement, never a requirement.

**Non-negotiable rules:**

- Embedded SurrealDB for all data storage
- Local ONNX embeddings for semantic search (~25MB model)
- S3-compatible object storage runs locally (Minio)
- Cloud sync is optional and user-initiated
- No feature may require network connectivity to function

**Rationale:** ADHD users need reliability without distractions. Network failures, cloud outages,
or connectivity issues MUST NOT interrupt a user's focus session or data access.

### II. ADHD-Friendly Design

All user-facing features MUST reduce cognitive load and support variable executive function.

**Non-negotiable rules:**

- WIP = 1 strictly enforced (only one quest "In Progress" at a time)
- Progressive disclosure: show only what's needed, hide complexity
- Zero-friction capture: one tap, no decisions required at capture time
- Energy system respects daily capacity variations
- Visual timers combat time blindness (not just numbers)
- Forgiveness mechanisms: grace periods for streaks, soft delete for recovery

**Rationale:** The target users have ADHD. The system externalizes executive function rather
than demanding willpower. Every feature must answer: "Does this reduce cognitive load?"

### III. Ubiquitous Language

All documentation, code, and UI MUST use consistent terminology from docs/glossary.md.

**Non-negotiable rules:**

- Quest (not Task, Todo, or Ticket)
- Campaign (not Project or Epic)
- Note (not Document or Page)
- Item (not Product or Asset)
- Capture (not Inbox item or Draft)
- Archive (not Delete) for soft deletion
- Energy levels: Tiny, Small, Medium, Large, Huge (not Low/Medium/High)

**Rationale:** Consistent language reduces confusion for users and developers. The Quest-Based
Agile metaphor creates an adventure framing that supports ADHD engagement.

### IV. Soft Delete & Data Recovery

All deletions MUST be soft deletes. Hard deletion requires explicit user action.

**Non-negotiable rules:**

- All entities use `status: archived` instead of hard delete
- Archived items remain in database with sync support (tombstones)
- "Empty Archive" action required for permanent deletion
- Change feeds track all state changes including archives
- Cascade behavior is user-configurable

**Rationale:** ADHD users often act impulsively. Permanent data loss from accidental deletion
causes anxiety and breaks trust. Recovery must always be possible.

### V. Plugin Architecture for Extensibility

Auth providers and AI providers MUST be implemented as plugins, not hardcoded.

**Non-negotiable rules:**

- All providers implement trait-based interfaces (AuthProvider, AiProvider)
- Built-in providers: local auth, OAuth (Google, GitHub), OIDC for auth
- Built-in providers: Ollama, OpenAI, Anthropic, whisper-local for AI
- Users choose which providers to enable via configuration
- New providers can be added without core code changes

**Rationale:** User choice is paramount. No vendor lock-in. Different users have different
requirements for auth (offline vs SSO) and AI (local vs cloud, cost vs capability).

### VI. Privacy by Default

User data MUST stay local unless explicitly configured otherwise.

**Non-negotiable rules:**

- Local ONNX embeddings (data never leaves device for semantic search)
- AI providers are optional and user-configured
- Secrets stored in OS-native keychains (never in config files)
- No telemetry without explicit opt-in
- Health integrations are read-only plugins, not built-in tracking

**Rationale:** ADHD users track sensitive information (energy levels, productivity patterns).
This data must be protected. Cloud features are enhancements, not requirements.

### VII. Spec-Driven Development

All significant features MUST follow the spectrena workflow: specify → clarify → plan → tasks → implement.

**Non-negotiable rules:**

- Features start with a specification in specs/ directory
- Ambiguities resolved via /spectrena.clarify before planning
- Implementation plans include Constitution Check gate
- Tasks organized by user story for independent testing
- Lineage tracking connects requirements → tasks → code changes

**Rationale:** ADHD developers benefit from externalized structure just like ADHD users.
The workflow prevents scope creep, ensures requirements traceability, and supports
incremental delivery.

## Development Standards

### Code Quality

- Rust backend: `cargo fmt` and `cargo clippy` before every commit
- TypeScript/Svelte: `pnpm lint` before every commit
- All Tauri commands use tauri-specta for type generation
- SurrealDB tables: `SCHEMAFULL` with `CHANGEFEED 7d` for sync support
- No secrets in code or config files (use OS keychain references)

### Architecture Patterns

- Desktop apps: Tauri IPC (not REST) for frontend-backend communication
- Mobile apps: REST + WebSocket to cloud backend
- Graph edges for relationships (not junction tables)
- Last-Write-Wins sync with change feeds
- Presigned URLs for direct S3 uploads (no backend proxy)

### Testing Requirements

- User stories must be independently testable
- TDD encouraged: write failing tests before implementation
- Integration tests for cross-domain features (Quest → Note → Item)
- Performance targets defined in docs/requirements.md

## Workflow Requirements

### Tauri Command Pattern

All desktop features expose functionality via Tauri commands:

```rust
#[tauri::command]
async fn operation_name(
    state: State<'_, AppState>,
    param: Type,
) -> Result<Response, ApiError> {
    // Implementation
}
```

TypeScript types auto-generate via tauri-specta to packages/bindings/.

### Database Schema Pattern

All synced tables include change feed:

```surql
DEFINE TABLE entity SCHEMAFULL CHANGEFEED 7d;
DEFINE FIELD owner ON entity TYPE record<user>;
DEFINE FIELD status ON entity TYPE string DEFAULT 'active';
DEFINE FIELD created_at ON entity TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON entity TYPE datetime VALUE time::now();
DEFINE FIELD device_id ON entity TYPE string;
```

### Embedding Generation

- Desktop: Local ONNX model generates embeddings on note save
- Mobile: Receives embeddings via sync (no on-device generation)
- Embedding dimension: 384 (all-MiniLM-L6-v2)

## Governance

This constitution supersedes all other development practices for the Altair project.

### Amendment Process

1. Propose change with rationale in GitHub discussion or PR
2. Document impact on existing code and templates
3. Approval required from project maintainer
4. Amendment includes migration plan for affected code
5. Update version according to semantic versioning

### Versioning Policy

- **MAJOR**: Backward incompatible changes (principle removal/redefinition)
- **MINOR**: New principles added or materially expanded guidance
- **PATCH**: Clarifications, wording fixes, non-semantic refinements

### Compliance Review

All PRs and code reviews MUST verify:

- Terminology matches glossary (Principle III)
- No hard deletes (Principle IV)
- Offline functionality preserved (Principle I)
- Cognitive load reduced, not increased (Principle II)

Use CLAUDE.md and AGENTS.md for runtime development guidance.

**Version**: 1.0.0 | **Ratified**: 2025-11-30 | **Last Amended**: 2025-11-30
