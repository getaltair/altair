# Feature 006: Knowledge Domain — Technical Research

| Field | Value |
|---|---|
| **Feature** | 006-KnowledgeDomain |
| **Version** | 1.0 |
| **Status** | Draft |
| **Date** | 2026-04-15 |
| **Source Docs** | `Context/Features/006-KnowledgeDomain/Spec.md`, `docs/specs/03-invariants.md`, `docs/specs/05-erd.md` |

---

## Architecture Overview

Feature 006 adds one new top-level domain module to the Axum server:

1. **`src/knowledge/` module** — follows the established `mod.rs / models.rs / service.rs / handlers.rs`
   pattern used by `core/initiatives`, `core/tags`, and `core/relations`. Router is exported as
   `knowledge::router()` and merged into `main.rs` under the `/knowledge` prefix with the existing
   JWT auth layer.
2. **No new migrations** — `knowledge_notes` (migration 000017) and `knowledge_note_snapshots`
   (migration 000018) were created during Step 3 (ServerCore). Both are confirmed correct against
   the ERD. The snapshot table intentionally has no `updated_at` column or trigger, satisfying
   invariant E-6.
3. **No new Cargo dependencies** — `sqlx`, `axum`, `serde`, `uuid`, `anyhow`, `thiserror`,
   `tracing` are all already present.
4. **No new PowerSync sync rules** — `SyncStream::Knowledge` already exists in `contracts.rs`.
   The knowledge bucket is already defined in `infra/compose/sync_rules.yaml` from Step 4.
5. **Backlink query** — derived at query time from the existing `entity_relations` table (E-5).
   No new table or index is required; `idx_entity_relations_to` (created in Step 3) covers the
   query pattern.

---

## Open Questions Resolved

### Decision 1 — Module placement: `src/knowledge/` vs. `src/core/knowledge/`

**Resolution:** New top-level `src/knowledge/` directory, not under `src/core/`.

`src/core/` currently holds cross-cutting infrastructure shared by multiple domains: `relations`,
`tags`, `initiatives`, `households`. Knowledge is a first-class domain with its own bounded
context — it does not belong under `core/`. The directory structure signals ownership and will
make future parallel domains (Guidance, Tracking) easier to navigate.

The router is wired identically to other modules: `knowledge::router()` called in `main.rs`.

---

### Decision 2 — Backlink query implementation

**Resolution:** Query `entity_relations` directly in the knowledge service. Do not add a
backlink query to `core/relations`.

The backlink endpoint (`GET /knowledge/notes/:id/backlinks`) must return `KnowledgeNote` structs,
not `EntityRelation` structs. The knowledge service owns the response type and must JOIN or
perform a secondary lookup against `knowledge_notes`. Putting this logic in `core/relations`
would create an awkward dependency on `knowledge::models`.

**Query:**
```sql
SELECT n.*
FROM knowledge_notes n
JOIN entity_relations er
    ON er.from_entity_id = n.id
    AND er.from_entity_type = 'knowledge_note'
WHERE er.to_entity_id = $1          -- target note id
  AND er.to_entity_type = 'knowledge_note'
  AND er.deleted_at IS NULL
  AND n.deleted_at IS NULL
  AND n.user_id = $2                -- caller's user_id (SEC-1)
```

The index `idx_entity_relations_to` on `(to_entity_type, to_entity_id)` covers the filter.
The `n.user_id = $2` guard enforces that the caller can only discover their own notes in
another user's backlink graph — cross-user backlinks are not possible because the source
note `n` must belong to the requesting user.

---

### Decision 3 — Snapshot immutability enforcement layer

**Resolution:** Enforce at the service layer only. No database trigger or constraint beyond
the absence of `updated_at`/`deleted_at` columns.

Two enforcement layers are sufficient:
- **API**: no PUT/PATCH/DELETE handler exists for the snapshots sub-resource (E-6).
- **Service**: no `update_snapshot` or `delete_snapshot` function is written.

A database-level `BEFORE UPDATE` trigger on `knowledge_note_snapshots` would be redundant
given that sqlx queries are parameterized and the service never issues an UPDATE on that table.
The existing migration 000018 already omits `updated_at` and `deleted_at`, making accidental
UPDATE paths produce obvious compile-time failures (no column to set).

Assertion E-6 is verified by code review: the snapshot module has no update handler or
service function. The integration test (TC-K-3) confirms no PATCH/PUT endpoint exists.

---

### Decision 4 — sqlx error mapping in new code

**Resolution:** Use `anyhow::Error::from(e)` (not `anyhow::anyhow!(e.to_string())`) in the
knowledge module.

The existing codebase uses the anti-pattern `.map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))`,
which flattens the sqlx error chain and leaks schema detail into logs. New knowledge module
code avoids this:

```rust
// Preferred (new knowledge module):
.map_err(|e| AppError::Internal(anyhow::Error::from(e)))?
```

Adding `impl From<sqlx::Error> for AppError` to `error.rs` is out of scope for this step
(it would touch shared infrastructure and require updating all existing call sites or accepting
inconsistency). The knowledge module uses the `anyhow::Error::from(e)` form consistently.
A follow-up backlog item is noted to clean up the anti-pattern across all existing modules.

---

### Decision 5 — `NoteLinked` domain event

**Resolution:** Emit a `tracing::info!` log stub. No event bus infrastructure.

The spec requires a domain event hook point for when a note-to-note relation is created. In
this step, no event bus exists and no consumer is wired up. The event is represented as:

```rust
tracing::info!(
    note_from = %from_entity_id,
    note_to   = %to_entity_id,
    event     = "NoteLinked",
    "knowledge domain event"
);
```

This log line is emitted from the knowledge service after a note-to-note relation is created
via the existing `/relations` endpoint — specifically, from the `create_note_backlink_stub`
path if called, or noted as a TODO comment indicating where a future event bus call goes.

**Scope clarification:** The spec states note-to-note relation *creation* is handled by
`core/relations`. The `NoteLinked` stub is therefore a post-hoc log call. It does not require
any handler changes to `core/relations`. A tracing span annotation in the knowledge service
documents the intent.

---

## Module Structure

```
apps/server/server/src/knowledge/
├── mod.rs          # pub mod declarations + pub fn router() -> Router<AppState>
├── models.rs       # CreateNoteRequest, UpdateNoteRequest, NoteResponse,
│                   # CreateSnapshotRequest, SnapshotResponse, NoteListQuery
├── service.rs      # create_note, list_notes, get_note, update_note, delete_note,
│                   # create_snapshot, list_snapshots, list_backlinks
└── handlers.rs     # Axum extractors → service calls → JSON responses
```

### Request / Response Types

| Type | Fields | Notes |
|---|---|---|
| `CreateNoteRequest` | `id: Option<Uuid>`, `title: String`, `content: Option<String>`, `initiative_id: Option<Uuid>` | `id` supports offline-first client UUIDs (E-2); defaults to `gen_random_uuid()` |
| `UpdateNoteRequest` | `title: Option<String>`, `content: Option<String>` | COALESCE partial update — unchanged fields unaffected |
| `NoteResponse` | `id`, `title`, `content`, `initiative_id`, `user_id`, `created_at`, `updated_at`, `deleted_at` | Mirrors `knowledge_notes` columns |
| `NoteListQuery` | `initiative_id: Option<Uuid>` | Query param for GET `/knowledge/notes` |
| `CreateSnapshotRequest` | `captured_at: DateTime<Utc>` | Client supplies capture timestamp |
| `SnapshotResponse` | `id`, `note_id`, `content`, `captured_at`, `created_at` | No `updated_at` — snapshot is immutable |

---

## Integration Points

| System | Interaction | Notes |
|---|---|---|
| `AppState` | Pool, config | Passed via Axum state extractor; no changes to AppState |
| Auth middleware | `Extension<AuthUser>` | Applied via `.layer()` in `knowledge::router()` — matches pattern in `core/initiatives` |
| `entity_relations` table | SELECT (backlink query only) | No INSERT/UPDATE from knowledge module — use `/relations` endpoint for creation |
| `knowledge_notes` table | SELECT, INSERT, UPDATE (soft delete) | All queries bind `user_id` from JWT claims |
| `knowledge_note_snapshots` table | SELECT, INSERT only | No UPDATE or DELETE path (E-6) |
| `PowerSync` | No change | SyncStream::Knowledge already wired; note mutations flow through existing sync infrastructure |

---

## Test Strategy

### Unit tests (`#[cfg(test)]` in `service.rs`)
- `create_note` — client UUID is stored as-is; server-generated UUID is used when absent
- `update_note` — COALESCE behavior: only supplied fields are updated
- No `update_snapshot` function exists (E-6 verified by absence)

### Integration tests (`tests/knowledge_integration.rs`)
Uses `#[sqlx::test(migrations = "../../../infra/migrations")]` with transaction rollback.

| Test | Assertions |
|---|---|
| TC-K-1: note lifecycle | POST → GET → PUT → GET → DELETE → GET 404 |
| TC-K-2: user isolation | User A creates note; User B GET returns 404 (SEC-1) |
| TC-K-3: snapshot immutability | POST snapshot → GET confirms content; no PATCH/PUT endpoint (E-6) |
| TC-K-4: backlinks | Create relation via `/relations`; GET `/knowledge/notes/:B/backlinks` → note A present (A-019) |
| TC-K-5: initiative filter | GET `?initiative_id=X` filters correctly |
| SEC-2: unauthenticated | No auth header → 401 on all `/knowledge/*` routes |
| A-018: offline UUID | POST with client UUID → GET returns same UUID |

---

## Risks and Unknowns

| Risk | Likelihood | Mitigation |
|---|---|---|
| Backlink query performance with large `entity_relations` | Low (v1 scale) | `idx_entity_relations_to` covers the filter; monitor with EXPLAIN ANALYZE if needed |
| Snapshot content drift if `knowledge_notes.content` is updated before snapshot POST | Not a risk | Snapshot captures content at POST time by reading current note content in the same transaction |
| `NoteLinked` stub diverging from future event bus interface | Low | Documented as TODO; interface not defined yet |
