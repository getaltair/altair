# Spec: Feature 006 — Knowledge Domain (Server)

| Field | Value |
|---|---|
| **Feature** | 006-KnowledgeDomain |
| **Step** | Step 6 of 10-PLAN-001-v1.md |
| **Status** | Draft |
| **Created** | 2026-04-15 |
| **Source** | `docs/specs/01-PRD-003-knowledge.md`, `docs/specs/03-invariants.md`, `docs/specs/05-erd.md`, `docs/specs/10-PLAN-001-v1.md` §Step 6 |

---

## Overview

Implement the Knowledge domain server module for the Altair Rust/Axum backend. This adds Note CRUD, snapshot creation, and backlink query endpoints. The module follows the existing pattern used by `core/initiatives`, `core/tags`, and `core/relations`.

Database tables (`knowledge_notes`, `knowledge_note_snapshots`) were created during Step 3 (ServerCore) in migrations 000017 and 000018.

---

## Scope

### In Scope

- Note CRUD: create, list (with optional `initiative_id` filter), get by ID, update (title + content), soft delete
- Note snapshot creation: user-triggered POST that inserts an immutable row
- Note snapshot list: GET `/notes/:id/snapshots` ordered by `captured_at` DESC
- Backlink query: GET `/notes/:id/backlinks` — derives from `entity_relations` WHERE the target is the given note
- Domain event stub: log `NoteLinked` when a note-to-note relation is created (future hook point, no event bus)
- Router wiring into `apps/server/server/src/main.rs`
- Unit tests (service-layer) and integration tests for all endpoints

### Out of Scope

- Full-text search and semantic search (Step 10)
- Attachment upload/download (Step 10)
- Client implementations — Android and Web clients (Steps 8-9)
- AI pipeline: OCR, transcription, suggested links (P2 / v1+ candidates)
- Collaborative editing or CRDT merge (v2 candidate per ADR-003)
- Note-to-note or note-to-entity relation creation — already supported by `core/relations` CRUD built in Step 3; this step adds the backlink query view only
- Cross-domain search surfacing notes from Guidance domain (Step 10)

---

## Open Questions — Resolved

| OQ | Resolution |
|---|---|
| OQ-K-1: Plain text vs rich text | Plain text for v1. Markdown / rich text deferred to v2 candidates list. |
| OQ-K-3: Auto vs manual snapshots | Manual-triggered (user POSTs explicitly). Auto-on-every-edit would create unbounded snapshot volume with no client UI to manage it in v1. |
| OQ-K-4: Conflict resolution for concurrent edits | Conflict copies per ADR-003 (sync engine already enforces this for all note mutations via `base_version`). No domain-specific override needed. |

---

## Requirements

### P0 — Must Have

| ID | Requirement | Assertion |
|---|---|---|
| FR-K-1 | Note CRUD — create, list, get, update, soft delete | A-018 (server-side) |
| FR-K-2 | Notes scoped to `user_id` — no cross-user access | SEC-1 |
| FR-K-3 | Notes accept client-generated UUIDs for offline-first creation | E-2, A-018 |
| FR-K-4 | Notes can reference an `initiative_id` (nullable) | ERD §knowledge_notes |
| FR-K-5 | Snapshot creation — POST inserts immutable row, no UPDATE path | A-021, E-6 |
| FR-K-6 | Snapshot list — GET returns snapshots for a note ordered by `captured_at` DESC | A-021 |
| FR-K-7 | Backlink query — GET returns notes that reference the given note via `entity_relations` | A-019, E-5 |
| FR-K-8 | All routes protected by JWT auth middleware (SEC-2) | A-001 |

### P1 — Should Have (deferred to later steps)

| ID | Requirement | Deferred To |
|---|---|---|
| FR-K-9 | Full-text search across notes | Step 10 |
| FR-K-10 | Attachment metadata on notes | Step 10 |

---

## Testable Assertions (server-side coverage)

| ID | Assertion | Coverage |
|---|---|---|
| A-018 (partial) | A note created with a client-generated UUID is stored and retrievable | Integration test: POST with offline UUID → GET returns same note |
| A-019 | A backlink from note A to note B appears in note B's backlinks list | Integration test: create entity_relation (note→note), GET /notes/:b/backlinks → note A appears |
| A-021 | Note snapshots preserve the content at the time of capture; no UPDATE path exists | Unit test: no service function for snapshot update; integration test: POST snapshot → GET confirms content |
| SEC-1 | User A cannot list or get User B's notes | Integration test: two-user setup, User B's GET returns empty/404 |
| SEC-2 | Unauthenticated requests to `/knowledge/*` return 401 | Integration test: no auth header → 401 |
| E-6 | No UPDATE endpoint exists for `knowledge_note_snapshots` | Code review: no update handler or service function in snapshot paths |

---

## Functional Test Cases

### TC-K-1: Note lifecycle
1. POST `/knowledge/notes` with `{ id: <uuid>, title: "...", content: "..." }` → 201
2. GET `/knowledge/notes/:id` → 200 with same content
3. PUT `/knowledge/notes/:id` with `{ title: "updated" }` → 200
4. GET `/knowledge/notes/:id` → title is updated, content unchanged
5. DELETE `/knowledge/notes/:id` → 204
6. GET `/knowledge/notes/:id` → 404

### TC-K-2: User isolation
1. User A creates note N
2. User B calls GET `/knowledge/notes/:N` → 404

### TC-K-3: Snapshot immutability
1. POST `/knowledge/notes/:id/snapshots` with `{ captured_at: "..." }` → 201
2. No PATCH/PUT endpoint exists at `/knowledge/notes/:id/snapshots/:sid` → 405 or 404

### TC-K-4: Backlinks
1. Note A and Note B exist (User X owns both)
2. POST `/relations` `{ from_entity_type: "knowledge_note", from_entity_id: A, to_entity_type: "knowledge_note", to_entity_id: B, relation_type: "references", source_type: "user" }` → 201
3. GET `/knowledge/notes/:B/backlinks` → returns a list containing Note A

### TC-K-5: Initiative filter
1. Create note with `initiative_id: <I>`
2. GET `/knowledge/notes?initiative_id=<I>` → returns the note
3. GET `/knowledge/notes?initiative_id=<other>` → empty list

---

## API Surface

| Method | Path | Description |
|---|---|---|
| POST | `/knowledge/notes` | Create note (accepts client-generated UUID) |
| GET | `/knowledge/notes` | List notes for current user (optional `?initiative_id=`) |
| GET | `/knowledge/notes/:id` | Get note by ID |
| PUT | `/knowledge/notes/:id` | Update note title and/or content |
| DELETE | `/knowledge/notes/:id` | Soft-delete note |
| POST | `/knowledge/notes/:id/snapshots` | Create snapshot of current note content |
| GET | `/knowledge/notes/:id/snapshots` | List snapshots for note |
| GET | `/knowledge/notes/:id/backlinks` | List notes that reference this note |

---

## Invariants

| ID | Invariant | Enforcement in this step |
|---|---|---|
| E-2 | Client-generated UUIDs accepted at create time | `id` field accepted in POST body; fallback to `gen_random_uuid()` if absent |
| E-5 | Backlinks are symmetric — derived from entity_relations at query time | Backlink endpoint queries entity_relations; no separate backlink table |
| E-6 | Note snapshots immutable once created | No UPDATE handler or service function for snapshots |
| SEC-1 | Per-user data isolation | All queries bind `user_id` from JWT claims |
| SEC-2 | All routes authenticated | Axum auth middleware on `/knowledge/*` |

---

## Dependencies

| Dependency | Status | Notes |
|---|---|---|
| Feature 001 — Foundation | Complete | Rust workspace, Cargo.toml |
| Feature 002 — SharedContracts | Complete | `EntityType::KnowledgeNote` and `KnowledgeNoteSnapshot` in `contracts.rs` |
| Feature 003 — ServerCore | Complete | Auth middleware, `AppError`, `AppState`, migrations 000017 and 000018 exist |
| Feature 004 — SyncEngine | Complete | Sync push/pull wired; note mutations will flow through existing sync infrastructure |
| `core/relations` CRUD | Complete | Note-to-note and note-to-entity relations created via existing `/relations` endpoints |

---

## Non-Goals (explicit)

- No new entity types or relation types are introduced in this step
- No changes to existing migrations (000017, 000018 already correct)
- No PowerSync sync rules changes (knowledge stream already defined in contracts.rs; PowerSync YAML wired in Step 4)
- No client code
