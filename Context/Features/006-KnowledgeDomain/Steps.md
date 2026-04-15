# Implementation Steps: Feature 006 — Knowledge Domain

**Spec:** Context/Features/006-KnowledgeDomain/Spec.md
**Tech:** Context/Features/006-KnowledgeDomain/Tech.md

## Progress
- **Status:** In progress
- **Status:** Complete
- **Current task:** —
- **Last milestone:** M5 — Feature 006 complete (2026-04-15)

---

## Team Orchestration

### Team Members

- **builder-rust**
  - Role: Rust/Axum knowledge module — models, service, handlers, wiring
  - Agent Type: backend-engineer
  - Resume: true

- **validator**
  - Role: Quality validation — read-only inspection of completed work
  - Agent Type: quality-engineer
  - Resume: false

---

## Tasks

### Phase 1: Module Scaffold

> No migrations — 000017 and 000018 are already correct. Start directly with module files.

- [ ] S001: Create `apps/server/server/src/knowledge/models.rs`
  — `CreateNoteRequest` (id: Option<Uuid>, title: String, content: Option<String>, initiative_id: Option<Uuid>);
  `UpdateNoteRequest` (title: Option<String>, content: Option<String>);
  `NoteResponse` (id, title, content, initiative_id, user_id, created_at, updated_at, deleted_at — match knowledge_notes columns exactly);
  `NoteListQuery` (initiative_id: Option<Uuid> — query param struct with `#[serde(default)]`);
  `CreateSnapshotRequest` (captured_at: DateTime<Utc>);
  `SnapshotResponse` (id, note_id, content, captured_at, created_at — no updated_at per invariant E-6);
  all types derive Serialize + Deserialize; use `chrono::DateTime<Utc>` for timestamp fields; use `uuid::Uuid` for UUID fields
  - **Assigned:** builder-rust
  - **Depends:** none
  - **Parallel:** false

- [ ] S002: Create `apps/server/server/src/knowledge/mod.rs`
  — declare `pub mod models`, `pub mod service`, `pub mod handlers`;
  `pub fn router() -> Router<AppState>` stub (returns empty Router — filled after handlers.rs is complete);
  add `pub mod knowledge;` to `apps/server/server/src/lib.rs` or `main.rs` as appropriate for the crate layout
  - **Assigned:** builder-rust
  - **Depends:** S001
  - **Parallel:** false

🏁 **MILESTONE 1: Module scaffold complete** — `cargo check` exits 0
  **Contracts:**
  - `apps/server/server/src/knowledge/models.rs` — NoteResponse, SnapshotResponse, CreateNoteRequest, UpdateNoteRequest, NoteListQuery, CreateSnapshotRequest types; handlers.rs must use these exactly

---

### Phase 2: Service — Note CRUD

- [ ] S003: Create `apps/server/server/src/knowledge/service.rs` — implement note CRUD:
  (a) `create_note(db, req: CreateNoteRequest, user_id: Uuid) -> Result<NoteResponse, AppError>` —
  INSERT INTO knowledge_notes (id, title, content, initiative_id, user_id) VALUES ($1, $2, $3, $4, $5) RETURNING *;
  use `req.id.unwrap_or_else(Uuid::new_v4)` to support client-generated UUIDs (E-2);
  (b) `list_notes(db, user_id: Uuid, initiative_id: Option<Uuid>) -> Result<Vec<NoteResponse>, AppError>` —
  SELECT * FROM knowledge_notes WHERE user_id = $1 AND deleted_at IS NULL [AND initiative_id = $2 if Some];
  (c) `get_note(db, note_id: Uuid, user_id: Uuid) -> Result<NoteResponse, AppError>` —
  SELECT * FROM knowledge_notes WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL; return AppError::NotFound if no row;
  (d) `update_note(db, note_id: Uuid, req: UpdateNoteRequest, user_id: Uuid) -> Result<NoteResponse, AppError>` —
  UPDATE knowledge_notes SET title = COALESCE($1, title), content = COALESCE($2, content), updated_at = NOW() WHERE id = $3 AND user_id = $4 AND deleted_at IS NULL RETURNING *; return NotFound if no row;
  (e) `delete_note(db, note_id: Uuid, user_id: Uuid) -> Result<(), AppError>` —
  UPDATE knowledge_notes SET deleted_at = NOW(), updated_at = NOW() WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL; return NotFound if no row affected;
  use `anyhow::Error::from(e)` (not `e.to_string()`) for all sqlx error mapping (Tech.md Decision 4)
  - **Assigned:** builder-rust
  - **Depends:** S002
  - **Parallel:** false

- [ ] S003-T: Unit/integration tests for note CRUD
  (`#[sqlx::test(migrations = "../../../infra/migrations")]` with transaction rollback);
  (create with explicit UUID → same UUID stored and returned; A-018 partial);
  (create without UUID → UUID auto-generated);
  (list with initiative_id filter → only matching notes returned; TC-K-5);
  (update with only title → content unchanged, COALESCE correct);
  (update with only content → title unchanged);
  (get non-existent note → NotFound);
  (delete → subsequent get returns NotFound; TC-K-1);
  (list → soft-deleted notes excluded)
  - **Assigned:** builder-rust
  - **Depends:** S003

🏁 **MILESTONE 2: Note CRUD service complete** — `cargo test` exits 0 for note CRUD tests

---

### Phase 3: Service — Snapshots and Backlinks

- [ ] S004: Add snapshot functions to `apps/server/server/src/knowledge/service.rs`:
  (a) `create_snapshot(db, note_id: Uuid, req: CreateSnapshotRequest, user_id: Uuid) -> Result<SnapshotResponse, AppError>` —
  first verify note ownership: SELECT id FROM knowledge_notes WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL; return NotFound if absent;
  then SELECT content FROM knowledge_notes WHERE id = $1; capture current content;
  INSERT INTO knowledge_note_snapshots (id, note_id, content, captured_at) VALUES (gen_random_uuid(), $1, $2, $3) RETURNING *;
  (b) `list_snapshots(db, note_id: Uuid, user_id: Uuid) -> Result<Vec<SnapshotResponse>, AppError>` —
  verify note ownership (same check as above); SELECT * FROM knowledge_note_snapshots WHERE note_id = $1 ORDER BY captured_at DESC;
  — NO update_snapshot or delete_snapshot function (E-6 enforcement by absence)
  - **Assigned:** builder-rust
  - **Depends:** S003
  - **Parallel:** false

- [ ] S004-T: Integration tests for snapshots (A-021)
  (POST snapshot → content matches note content at time of capture; TC-K-3 partial);
  (GET snapshots → ordered by captured_at DESC);
  (snapshot for note owned by other user → NotFound);
  (no update_snapshot function exists — verified by absence in service.rs; E-6);
  (snapshot content is preserved even after note content is updated)
  - **Assigned:** builder-rust
  - **Depends:** S004

- [ ] S005: Add backlink query to `apps/server/server/src/knowledge/service.rs`:
  `list_backlinks(db, note_id: Uuid, user_id: Uuid) -> Result<Vec<NoteResponse>, AppError>` —
  verify target note ownership: SELECT id FROM knowledge_notes WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL; return NotFound if absent;
  then query:
  ```sql
  SELECT n.*
  FROM knowledge_notes n
  JOIN entity_relations er
      ON er.from_entity_id = n.id
      AND er.from_entity_type = 'knowledge_note'
  WHERE er.to_entity_id = $1
    AND er.to_entity_type = 'knowledge_note'
    AND er.deleted_at IS NULL
    AND n.deleted_at IS NULL
    AND n.user_id = $2
  ```
  covered by idx_entity_relations_to; user_id guard enforces SEC-1 (Tech.md Decision 2)
  - **Assigned:** builder-rust
  - **Depends:** S004
  - **Parallel:** false

- [ ] S005-T: Integration tests for backlinks (A-019)
  (create two notes A and B; POST /relations with from=A, to=B, from_type=knowledge_note, to_type=knowledge_note; GET backlinks for B → A appears; TC-K-4);
  (note A backlinks → empty when no relations exist);
  (backlinks for note owned by other user → NotFound; SEC-1 via user_id guard);
  (deleted entity_relation not returned in backlinks)
  - **Assigned:** builder-rust
  - **Depends:** S005

🏁 **MILESTONE 3: Full service complete** — `cargo test` exits 0; all service functions present; no update_snapshot function in service.rs
  **Contracts:**
  - `apps/server/server/src/knowledge/service.rs` — create_note, list_notes, get_note, update_note, delete_note, create_snapshot, list_snapshots, list_backlinks signatures (no update_snapshot or delete_snapshot)

---

### Phase 4: Handlers + Wiring

- [ ] S006: Create `apps/server/server/src/knowledge/handlers.rs` — 8 thin Axum handlers:
  `create_note_handler` → `service::create_note` → 201 Created with NoteResponse;
  `list_notes_handler` → `service::list_notes` (extract NoteListQuery from Query<>) → 200 with Vec<NoteResponse>;
  `get_note_handler` → `service::get_note` → 200 with NoteResponse or 404;
  `update_note_handler` → `service::update_note` → 200 with NoteResponse or 404;
  `delete_note_handler` → `service::delete_note` → 204 No Content or 404;
  `create_snapshot_handler` → `service::create_snapshot` → 201 Created with SnapshotResponse;
  `list_snapshots_handler` → `service::list_snapshots` → 200 with Vec<SnapshotResponse>;
  `list_backlinks_handler` → `service::list_backlinks` → 200 with Vec<NoteResponse>;
  all handlers: `State(state): State<AppState>`, `Extension(auth): Extension<AuthUser>`, extract note_id from `Path<Uuid>` where needed;
  update `knowledge::router()` in mod.rs to mount all 8 routes under `/knowledge` with auth middleware layer
  - **Assigned:** builder-rust
  - **Depends:** S005
  - **Parallel:** false

- [ ] S006-T: Integration tests for all endpoints (SEC-2, A-018, A-019, A-021, SEC-1)
  (no auth header on any /knowledge/* route → 401; SEC-2);
  (note lifecycle end-to-end: POST → GET → PUT → GET → DELETE → GET 404; TC-K-1);
  (POST with client UUID → GET returns same UUID; A-018);
  (User A creates note; User B GET returns 404; TC-K-2, SEC-1);
  (POST snapshot → 201; GET snapshots → 200 with content; no PATCH/PUT endpoint exists → 404/405; TC-K-3, A-021);
  (backlink end-to-end via /relations then GET /backlinks; TC-K-4, A-019);
  (GET /notes?initiative_id filter; TC-K-5)
  - **Assigned:** builder-rust
  - **Depends:** S006

- [ ] S007: Wire knowledge module into server — add `.merge(knowledge::router())` to the router in
  `apps/server/server/src/main.rs` following the existing pattern (after sync, before any future domain modules);
  ensure `pub mod knowledge;` is declared in `lib.rs` or the appropriate module tree entry point
  - **Assigned:** builder-rust
  - **Depends:** S006
  - **Parallel:** false

- [ ] S007-T: Smoke test — `cargo build` exits 0; `cargo test` exits 0; confirm /knowledge routes appear in router (log output or route listing test)
  - **Assigned:** builder-rust
  - **Depends:** S007

🏁 **MILESTONE 4: All endpoints live** — verify A-018, A-019, A-021, SEC-1, SEC-2, E-6; `cargo test` exits 0

---

### Phase 5: Validation

- [ ] S008: Full drift check — read Spec.md testable assertions A-018, A-019, A-021, SEC-1, SEC-2, E-6 against implemented code;
  confirm no `update_snapshot` or `delete_snapshot` function exists anywhere in `src/knowledge/` (E-6);
  confirm all 8 routes are mounted and documented in API surface;
  confirm user_id binding present on all queries (SEC-1);
  confirm auth middleware applied on `/knowledge/*` router (SEC-2);
  confirm `anyhow::Error::from(e)` used (not `e.to_string()`) throughout knowledge module (Tech.md Decision 4);
  confirm no TODO/FIXME stubs in `src/knowledge/`;
  flag any REQUIRES_LIVE_ENV items
  - **Assigned:** validator
  - **Depends:** S007-T
  - **Parallel:** false

🏁 **MILESTONE 5: Feature 006 complete** — all assertions verified; `cargo test` exits 0; no stubs in knowledge module

---

### Phase 6: Post-Review Tests (review-driven additions, 2026-04-15)

- [ ] S009: Cross-user mutation isolation tests (SEC-1)
  — unit: `update_note(note_a.id, ..., user_b_id)` → NotFound; `delete_note(note_a.id, user_b_id)` → NotFound
  — integration: `PUT /knowledge/notes/{note_a_id}` as User B → 404; `DELETE /knowledge/notes/{note_a_id}` as User B → 404
  Ensures that if the `AND user_id = $N` guard is ever accidentally dropped, the test suite fails rather than silently passing.
  - **Relates to:** SEC-1 (Spec.md), P6-014, P6-015
  - **Status:** Complete (added in service.rs unit tests + knowledge_integration.rs)

- [ ] S010: Snapshot invariant edge-case tests
  — unit: `create_snapshot` on a soft-deleted note → NotFound (A-021, P6-017)
  — integration: `DELETE /knowledge/notes/{id}/snapshots/{snap_id}` with valid auth → 404 or 405 (E-6, P6-016)
  - **Relates to:** A-021, E-6 (Spec.md), P6-016, P6-017
  - **Status:** Complete (unit test in service.rs; DELETE assertion extended in test_create_snapshot_returns_201_with_content)

---

## Acceptance Criteria

- [ ] A-018 (partial): POST with client-generated UUID → GET returns same note
- [ ] A-019: Backlink from note A to note B appears in note B's backlinks list
- [ ] A-021: Snapshot content preserved at capture time; no UPDATE endpoint or service function
- [ ] SEC-1: User A cannot list or get User B's notes
- [ ] SEC-2: Unauthenticated requests to `/knowledge/*` return 401
- [ ] E-6: No `update_snapshot` or `delete_snapshot` function in `src/knowledge/`
- [ ] `cargo test` exits 0 in `apps/server/`
- [ ] `cargo build` exits 0 with knowledge module wired into main.rs
- [ ] No TODO/FIXME stubs in `src/knowledge/`

## Validation Commands

```bash
# Build and test
cd apps/server && cargo test
cd apps/server && cargo build
cd apps/server && cargo clippy -- -D warnings

# Confirm no snapshot update/delete paths (E-6)
grep -r "update_snapshot\|delete_snapshot" apps/server/server/src/knowledge/
# ^ should return empty

# Confirm anyhow::Error::from used (not e.to_string())
grep -r "to_string()" apps/server/server/src/knowledge/
# ^ should return empty

# REQUIRES_LIVE_ENV (post-deploy):
# curl -X POST /knowledge/notes (no JWT) → 401
# Full TC-K-1 through TC-K-5 against running server
```
