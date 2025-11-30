<!--
================================================================================
SYNC IMPACT REPORT
================================================================================
Version change: 0.0.0 → 1.0.0 (MAJOR - initial ratification)

Modified principles: N/A (initial version)

Added sections:
  - I. Tauri-First IPC (from ADR-008)
  - II. SurrealDB Native (from ADR-001)
  - III. Privacy-by-Default (from ADR-005, ADR-007, ADR-011)
  - IV. Soft Delete Everywhere (from ADR-010)
  - V. ADHD-Friendly Design (from project purpose)
  - VI. Terminology Consistency (from glossary.md)
  - Development Constraints (from technical-architecture.md)
  - Governance (new)

Removed sections: N/A (initial version)

Templates requiring updates:
  - .spectrena/templates/plan-template.md ✅ (no updates required - already generic)
  - .spectrena/templates/spec-template.md ✅ (no updates required - already generic)
  - .spectrena/templates/tasks-template.md ✅ (no updates required - already generic)

Follow-up TODOs: None
================================================================================
-->

# Altair Constitution

## Core Principles

### I. Tauri-First IPC

Desktop applications MUST use Tauri IPC commands for frontend-backend communication, not
REST endpoints. REST is reserved exclusively for mobile/cloud communication paths.

**Rationale:**
- Type-safe boundary via `tauri-specta` (Rust → TypeScript)
- No HTTP serialization overhead for local operations
- No port management or localhost networking complexity
- Same business logic powers both IPC commands and REST handlers

**Implementation rules:**
- All desktop operations use `invoke()` with generated TypeScript types
- REST endpoints exist only in `backend/src/api/` for mobile
- Tauri command handlers live in `backend/src/commands/`
- Never duplicate business logic; commands and API handlers call shared service layer

### II. SurrealDB Native

All data access MUST use SurrealDB's native features: graph edges for relationships,
HNSW indexes for vector search, and change feeds for sync. No abstraction layers that
hide these capabilities.

**Rationale:**
- Graph edges eliminate JOIN tables and simplify relationship queries
- Built-in vector search enables semantic search without external services
- Change feeds provide sync infrastructure without CRDT complexity
- Single database works embedded (desktop/mobile) and server (cloud)

**Implementation rules:**
- Use `DEFINE TABLE ... TYPE RELATION` for entity relationships
- Use `CHANGEFEED 7d` on all tables that require sync
- Use `DEFINE INDEX ... HNSW` for embedding-based search
- Query with graph traversal (`->edge->`) not manual JOINs
- All tables MUST be `SCHEMAFULL` with explicit field definitions

### III. Privacy-by-Default

User data MUST remain local unless the user explicitly enables sync. Local embeddings
MUST be always-on. Location auto-tagging MUST be opt-in with configurable precision.

**Rationale:**
- Single-user focus means no reason to transmit data by default
- Local ONNX embeddings (~25MB) eliminate cloud API dependencies
- Location data is sensitive; user controls disclosure level
- Semantic search works offline without configuration

**Implementation rules:**
- Cloud sync is disabled until user configures it
- Embeddings generate locally on note save (desktop only; mobile receives via sync)
- Location auto-tag preference stored per user with precision options
- API keys stored in OS-native secure storage (keychain/Credential Manager)
- Never log or transmit PII without explicit user action

### IV. Soft Delete Everywhere

All entity deletions MUST be soft deletes (status change to `archived`). Hard deletes
require explicit user action via "Empty Archive" and MUST respect cascade settings.

**Rationale:**
- Prevents accidental data loss
- Enables recovery without backup restoration
- Sync handles tombstones cleanly
- Maintains audit trail of what existed

**Implementation rules:**
- All delete operations set `status: archived`, never remove records
- UI shows archived items in dedicated Archive view
- "Empty Archive" triggers permanent deletion with confirmation
- Cascade behavior is user-configurable per entity type
- Deleted references show as "(archived)" in linking entities

### V. ADHD-Friendly Design

All user-facing features MUST minimize decision fatigue. Quick Capture MUST require
zero destination decisions. Energy-based filtering MUST be the primary quest
organization method.

**Rationale:**
- Target users have executive function challenges
- Deferred classification reduces capture friction
- Energy cost is more actionable than arbitrary priority
- AI assists decisions but never removes user agency

**Implementation rules:**
- Quick Capture: one tap/click, no required fields except content
- Captures route to inbox with AI-suggested (not AI-decided) destinations
- Quest views default to energy-level filtering, not due-date sorting
- Batch review for pending captures, not per-item interruptions
- Notifications are badges, not intrusive alerts

### VI. Terminology Consistency

All code, documentation, and UI MUST use domain terminology from the glossary.
Quest (not Task), Campaign (not Project), Note (not Document), Item (not Product),
Capture (not Inbox item), Archive (not Delete).

**Rationale:**
- Ubiquitous language reduces cognitive overhead
- Quest-Based Agile (QBA) methodology requires consistent framing
- Users learn one vocabulary across all apps
- Code readability improves with domain alignment

**Implementation rules:**
- Variable names use glossary terms: `quest`, `campaign`, `note`, `item`, `capture`
- UI text matches glossary exactly
- Database tables use glossary terms (lowercase snake_case)
- Comments and documentation reference glossary for domain concepts
- Never use "task", "project", "document", "product", or "inbox" for core entities

## Development Constraints

### Technology Stack

All implementations MUST use:
- **Backend**: Rust with Axum framework
- **Desktop**: Tauri 2.0 with Svelte frontend
- **Mobile**: Tauri 2.0 Android (same codebase as desktop)
- **Database**: SurrealDB 2.x (embedded for local, server for cloud)
- **Object Storage**: S3-compatible API (Minio local, any provider cloud)
- **Embeddings**: ONNX runtime with all-MiniLM-L6-v2 model
- **Type Generation**: `tauri-specta` for Rust → TypeScript

### Sync Protocol

- Last-Write-Wins (LWW) conflict resolution via `updated_at` timestamp
- Change feed retention: 7 days (`CHANGEFEED 7d`)
- Extended offline (>7 days) triggers full resync for affected tables
- Offline queue persists pending operations in local SurrealDB

### Testing Requirements

- All database tables MUST have migration files in `backend/migrations/`
- All Tauri commands MUST have corresponding TypeScript types in `packages/bindings/`
- Integration tests MUST use embedded SurrealDB instance
- Sync tests MUST verify LWW behavior with conflicting timestamps

## Governance

This constitution supersedes all other development practices for the Altair project.
All pull requests and code reviews MUST verify compliance with these principles.

### Amendment Process

1. Propose change via issue with `constitution` label
2. Document rationale and impact analysis
3. Update constitution with new version number
4. Update dependent templates if principles change
5. Merge requires explicit approval

### Versioning Policy

- **MAJOR**: Principle removal, redefinition, or backward-incompatible governance change
- **MINOR**: New principle added or existing principle materially expanded
- **PATCH**: Wording clarification, typo fix, non-semantic refinement

### Compliance Review

- All specs MUST include Constitution Check section referencing relevant principles
- Plan reviews verify alignment with technology stack constraints
- Code reviews verify terminology consistency and soft-delete implementation

**Version**: 1.0.0 | **Ratified**: 2025-11-29 | **Last Amended**: 2025-11-29
