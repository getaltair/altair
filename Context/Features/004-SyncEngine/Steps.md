# Implementation Steps: Feature 004 — Sync Engine

**Spec:** Context/Features/004-SyncEngine/Spec.md
**Tech:** Context/Features/004-SyncEngine/Tech.md

## Progress
- **Status:** Complete
- **Current task:** —
- **Last milestone:** M8 (Post-review hardening complete — 2026-04-15)

---

## Team Orchestration

### Team Members

- **builder-rust**
  - Role: Rust/Axum sync module — models, service logic, handlers, wiring
  - Agent Type: backend-engineer
  - Resume: true

- **builder-infra**
  - Role: Migrations, PowerSync sync rules YAML, powersync.yml, contracts JSON
  - Agent Type: backend-engineer
  - Resume: true

- **validator**
  - Role: Quality validation — read-only inspection of completed work
  - Agent Type: quality-engineer
  - Resume: false

---

## Tasks

### Phase 1: Database Migrations

> builder-infra and builder-rust can both start Phase 1 tasks in parallel.
> builder-infra starts migration files; builder-rust starts the module scaffold.

- [x] S001: Create `infra/migrations/20260414000027_create_sync_mutations.up.sql` and `.down.sql`
  — schema per Tech.md: `mutation_id` (UUID PK), `device_id`, `user_id` (FK → users CASCADE),
  `entity_type`, `entity_id`, `operation`, `applied_at`; two indexes on entity_id and user_id
  - **Assigned:** builder-infra
  - **Depends:** none
  - **Parallel:** true

- [x] S001-T: Verify sync_mutations migration applies and reverts cleanly
  (apply against fresh test DB, confirm table + indexes created, revert removes table, FA-014 partial)
  - **Assigned:** builder-infra
  - **Depends:** S001

- [x] S002: Create `infra/migrations/20260414000028_create_sync_conflicts.up.sql` and `.down.sql`
  — schema per Tech.md: `id` (UUID PK gen_random_uuid), `mutation_id`, `entity_type`, `entity_id`,
  `base_version` (TIMESTAMPTZ), `current_version` (TIMESTAMPTZ), `incoming_payload` (JSONB),
  `current_payload` (JSONB), `resolution` (TEXT DEFAULT 'pending'), `resolved_at`, `user_id` (FK → users CASCADE),
  `created_at`; two indexes on entity_id and user_id
  - **Assigned:** builder-infra
  - **Depends:** none
  - **Parallel:** true

- [x] S002-T: Verify sync_conflicts migration applies and reverts cleanly
  (apply, confirm FK constraint to users, confirm both indexes, revert, FA-014 partial)
  - **Assigned:** builder-infra
  - **Depends:** S002

- [x] S002-D: Update `Context/Features/004-SyncEngine/Spec.md` — mark M-3 (device_checkpoints) and M-4 (row_version column)
  as overridden per Tech.md Decision 2 and Decision 1 respectively; add inline note at each requirement
  - **Assigned:** builder-infra
  - **Depends:** none
  - **Parallel:** true

🏁 **MILESTONE 1: Migrations complete** — verify FA-014
  **Contracts:**
  - `infra/migrations/20260414000027_create_sync_mutations.up.sql` — sync_mutations column names and types; service.rs INSERT must match exactly
  - `infra/migrations/20260414000028_create_sync_conflicts.up.sql` — sync_conflicts column names and types; service.rs INSERT must match exactly

---

### Phase 2: Module Scaffold

> builder-rust starts S003 in parallel with Phase 1. S003 and S004 target different files.

- [x] S003: Create `apps/server/server/src/sync/models.rs`
  — `MutationEnvelope` struct (all 8 fields per Tech.md); `SyncUploadRequest` (mutations: Vec); `SyncUploadResponse` (results: Vec);
  `MutationResult` (mutation_id, status); `MutationStatus` enum (Accepted, Deduplicated, Conflicted { conflict_id: Uuid });
  all types derive Serialize + Deserialize; use `chrono::DateTime<Utc>` for TIMESTAMPTZ fields
  - **Assigned:** builder-rust
  - **Depends:** none
  - **Parallel:** true

- [x] S004: Create `apps/server/server/src/sync/mod.rs`
  — declare submodules (`pub mod models`, `pub mod service`, `pub mod handlers`); add re-exports;
  `pub fn router() -> Router` stub (empty — filled after handlers.rs is complete)
  - **Assigned:** builder-rust
  - **Depends:** S003
  - **Parallel:** false

🏁 **MILESTONE 2: Module scaffold complete**
  **Contracts:**
  - `apps/server/server/src/sync/models.rs` — MutationEnvelope, SyncUploadRequest, SyncUploadResponse, MutationResult, MutationStatus types

---

### Phase 3: Service Logic

> All service tasks target `apps/server/server/src/sync/service.rs`.
> They are sequential within builder-rust. builder-infra starts Phase 5 after Milestone 1.

- [x] S005: Create `apps/server/server/src/sync/service.rs` — implement foundational helpers:
  (a) `entity_type_to_table(entity_type: &EntityType) -> &'static str` — exhaustive match over all 18 EntityType variants to Postgres table name;
  (b) `check_ownership(db, entity_type, entity_id, auth_user) -> Result<(), AppError>` — for user-scoped entities: `SELECT user_id FROM <table> WHERE id = $1`, compare with `auth_user.id`; for household-scoped entities: verify via `household_memberships`; return `AppError::Forbidden` if not owner
  - **Assigned:** builder-rust
  - **Depends:** S004, S001-T, S002-T
  - **Parallel:** false

- [x] S005-T: Unit tests for service helpers
  (entity_type_to_table returns correct table name for user-scoped and household-scoped variants, ownership check returns Ok for valid owner, Forbidden for non-owner, Forbidden for unknown entity_id)
  - **Assigned:** builder-rust
  - **Depends:** S005

- [x] S006: Implement `apply_mutations(db, mutations, auth_user) -> Vec<MutationResult>` and `record_sync_mutation(tx, envelope, user_id)`
  — iterate mutations serially; for each: (1) dedup check via SELECT from sync_mutations by mutation_id → return Deduplicated if found;
  (2) validate entity_type via EntityType::from_str → return Err(AppError::UnprocessableEntity) if invalid;
  (3) call check_ownership → propagate Forbidden;
  (4) dispatch to per-operation handlers (create/update/delete paths): create → INSERT with client-provided id and payload columns;
  update → apply payload fields with `updated_at = now()`; delete → `SET deleted_at = now()`;
  (5) call record_sync_mutation within the same sqlx transaction;
  (6) return MutationResult { status: Accepted }
  — each mutation wrapped in its own `db.begin()` / `tx.commit()` transaction
  - **Assigned:** builder-rust
  - **Depends:** S005
  - **Parallel:** false

- [x] S006-T: Unit / integration tests for apply_mutations core paths
  (valid create mutation → accepted + sync_mutations row inserted; replaying same mutation_id → deduplicated + no duplicate DB row; invalid entity_type string → UnprocessableEntity; mutation for entity owned by other user → Forbidden; delete mutation sets deleted_at and row remains in table, FA-009)
  - **Assigned:** builder-rust
  - **Depends:** S006

- [x] S007: Implement `detect_conflict(tx, entity_type, entity_id, base_version, occurred_at) -> Result<ConflictOutcome>`
  — `SELECT updated_at, <all columns> FROM <table> WHERE id = $1 FOR UPDATE`;
  if `current_row.updated_at > base_version`: conflict detected → return `ConflictOutcome::Conflict { current_row }`;
  if `base_version IS NULL` and operation != Create: apply LWW using `occurred_at` as tiebreaker;
  if no conflict: return `ConflictOutcome::Clean`;
  LWW logic: `if occurred_at >= current_row.updated_at → Accept (log conflict)` else `Reject (log conflict)`;
  INSERT into sync_conflicts in either LWW case (with pending resolution)
  - **Assigned:** builder-rust
  - **Depends:** S006
  - **Parallel:** false

- [x] S007-T: Tests for detect_conflict
  (no conflict when base_version == current updated_at; conflict when current_row.updated_at > base_version; LWW accepts when occurred_at is newer; LWW rejects when occurred_at is older; null base_version on non-create falls through to LWW; sync_conflicts row written on conflict)
  - **Assigned:** builder-rust
  - **Depends:** S007

- [x] S008: Implement `create_conflict_copy(tx, original_entity_id, payload, user_id) -> Result<Uuid>`
  — INSERT a new `knowledge_notes` row using payload columns (new `gen_random_uuid()` id); INSERT into `entity_relations` with `source_id = new_id`, `target_id = original_entity_id`, `relation_type = "duplicates"`, `source_type = "knowledge_note"`, `target_type = "knowledge_note"`; INSERT into `sync_conflicts` with `resolution = 'conflict_copy'`; return new note id as `conflict_id`
  — wire into apply_mutations: when entity_type == KnowledgeNote and operation == Update and conflict detected → call create_conflict_copy instead of LWW
  - **Assigned:** builder-rust
  - **Depends:** S007
  - **Parallel:** false

- [x] S008-T: Integration tests for conflict copy (FA-007)
  (conflicting knowledge_note update → two knowledge_notes rows exist; entity_relations row inserted with relation_type=duplicates; sync_conflicts row has resolution=conflict_copy; original note unchanged; returned status is Conflicted with conflict_id)
  - **Assigned:** builder-rust
  - **Depends:** S008

- [x] S009: Implement `check_quantity_conflict(payload) -> bool`
  — returns true if the mutation payload contains a quantity field (`quantity`, `consumed_quantity`, or similar per tracking_item_events schema);
  wire into apply_mutations: when entity_type == TrackingItemEvent and operation == Update and conflict detected and check_quantity_conflict(payload) → ROLLBACK and return `Err(AppError::Conflict("quantity conflict — re-read and resubmit"))`
  - **Assigned:** builder-rust
  - **Depends:** S007
  - **Parallel:** false

- [x] S009-T: Tests for quantity conflict path (FA-008)
  (update with quantity field + conflict → 409-equivalent Conflict error; update without quantity field + conflict → LWW path taken; non-tracking entity with conflict → not routed to quantity check)
  - **Assigned:** builder-rust
  - **Depends:** S009

🏁 **MILESTONE 3: Service complete**
  **Contracts:**
  - `apps/server/server/src/sync/service.rs` — apply_mutations, detect_conflict, create_conflict_copy, check_quantity_conflict, record_sync_mutation signatures

---

### Phase 4: Handler + Wiring

- [x] S010: Create `apps/server/server/src/sync/handlers.rs` — `push_handler(State(state): State<AppState>, auth: AuthUser, Json(req): Json<SyncUploadRequest>) -> Result<Json<SyncUploadResponse>, AppError>` — extract auth user, call `service::apply_mutations(&state.db, req.mutations, auth)`, return `Json(SyncUploadResponse { results })`; update `mod.rs` router() to mount `POST /api/sync/push` → push_handler
  - **Assigned:** builder-rust
  - **Depends:** S009
  - **Parallel:** false

- [x] S010-T: Integration tests for push_handler auth gate and basic paths (FA-001, FA-002, FA-003, FA-004, FA-005)
  (no JWT → 401; valid JWT + valid create mutation → 200 + accepted; same mutation replayed → 200 + deduplicated; unknown entity_type → 422; mutation for other user's entity → 403)
  - **Assigned:** builder-rust
  - **Depends:** S010

- [x] S011: Wire sync module into server crate — add `pub mod sync;` to `apps/server/server/src/lib.rs`; add `.merge(sync::router())` to router in `apps/server/server/src/main.rs` following existing module pattern
  - **Assigned:** builder-rust
  - **Depends:** S010
  - **Parallel:** false

- [x] S011-T: Integration tests for conflict and delete paths (FA-006, FA-007, FA-008, FA-009)
  (two devices submit update with same base_version → sync_conflicts row written + Conflicted status; knowledge_note conflict → conflict copy created; tracking_item_event quantity conflict → 409; delete mutation → deleted_at set + row remains in table)
  - **Assigned:** builder-rust
  - **Depends:** S011

🏁 **MILESTONE 4: Push endpoint live** — verify FA-001 through FA-009; `cargo test` exits 0

---

### Phase 5: PowerSync Infra

> builder-infra starts this phase after Milestone 1 (migrations confirmed). Independent of builder-rust Phase 3–4 work.

- [x] S012: Create `infra/compose/sync_rules.yaml` — 5 bucket definitions per Tech.md:
  `user_data` (user_id scoped: users, initiatives, tags, attachments, entity_relations);
  `household` (household_id via membership subquery: households, household_memberships);
  `guidance` (user_id scoped: guidance_epics, guidance_quests, guidance_routines, guidance_focus_sessions, guidance_daily_checkins);
  `knowledge` (user_id scoped: knowledge_notes, knowledge_note_snapshots);
  `tracking` (household_id via membership subquery: tracking_locations, tracking_categories, tracking_items, tracking_item_events, tracking_shopping_lists, tracking_shopping_list_items);
  — no `WHERE deleted_at IS NULL` filter in any bucket (Tech.md Decision 3); use `request.user_id()` per ADR-012
  - **Assigned:** builder-infra
  - **Depends:** S001-T, S002-T
  - **Parallel:** true

- [x] S012-T: Verify sync_rules.yaml — confirm all 5 buckets present, all table names match migration filenames, `request.user_id()` used correctly, no deleted_at filter; note FA-010 and FA-011 require `docker compose up` (REQUIRES_LIVE_ENV)
  - **Assigned:** builder-infra
  - **Depends:** S012

- [x] S013: Update `infra/compose/powersync.yml` — add `sync_rules:` block with `path: ./sync_rules.yaml` under the top-level config; remove stale comment referencing `apps/web/src/lib/sync/schema.ts`
  - **Assigned:** builder-infra
  - **Depends:** S012
  - **Parallel:** false

- [x] S014: Update `packages/contracts/sync-streams.json` — set `provisional: false`; update `note` field to reflect finalised bucket names; verify 5 stream id values match bucket keys in sync_rules.yaml exactly (FA-012)
  - **Assigned:** builder-infra
  - **Depends:** S012
  - **Parallel:** true

🏁 **MILESTONE 5: Infra complete** — verify FA-012; FA-010 and FA-011 are **REQUIRES_LIVE_ENV** (`docker compose up`, PowerSync healthy, no sync_rules parse errors in logs; cross-user bucket isolation verified via PowerSync client or bucket inspection)

---

### Phase 6: Should-Have — Conflict Resolution Endpoints

- [x] S015: Implement `GET /api/sync/conflicts` — paginated query of `sync_conflicts` WHERE `user_id = auth.id` AND `resolution = 'pending'`; default page size 20; response includes `id`, `entity_type`, `entity_id`, `base_version`, `current_version`, `incoming_payload`, `current_payload`, `created_at`; add handler + route to sync router
  - **Assigned:** builder-rust
  - **Depends:** S011
  - **Parallel:** false

- [x] S015-T: Integration tests for GET /api/sync/conflicts
  (returns only auth user's pending conflicts; pagination cursor works; empty list when no conflicts; 401 without JWT)
  - **Assigned:** builder-rust
  - **Depends:** S015

- [x] S016: Implement `POST /api/sync/conflicts/:id/resolve` — validate `resolution` field (accepted | rejected); UPDATE sync_conflicts SET `resolution = $1`, `resolved_at = now()` WHERE `id = $2` AND `user_id = auth.id`; return 200 on success; 404 if not found; 403 if conflict belongs to other user; add handler + route to sync router
  - **Assigned:** builder-rust
  - **Depends:** S015
  - **Parallel:** false

- [x] S016-T: Integration tests for POST /api/sync/conflicts/:id/resolve
  (accepted → resolution=accepted + resolved_at set; rejected → resolution=rejected + resolved_at set; unknown id → 404; other user's conflict → 403; 401 without JWT)
  - **Assigned:** builder-rust
  - **Depends:** S016

- [x] S016-D: Update `CLAUDE.md` active work section — Feature 004 Sync Engine complete; next: Feature 005 Guidance Domain (or 006/007 per parallel track)
  - **Assigned:** builder-infra
  - **Depends:** S016

🏁 **MILESTONE 6: Should-have complete** — verify S-1, S-2, S-3 from Spec.md; `cargo test` exits 0

---

### Phase 7: Validation

- [x] S017: Full drift check — read Spec.md testable assertions FA-001 through FA-014 against implemented code; verify cargo test exits 0; confirm no TODO/FIXME stubs in sync module; confirm sync_rules.yaml table names exactly match migration table names; confirm sync-streams.json provisional=false (FA-012); flag any REQUIRES_LIVE_ENV assertions for manual verification
  - **Assigned:** validator
  - **Depends:** S016, S014
  - **Parallel:** false

🏁 **MILESTONE 7: Feature 004 complete** — all assertions verified; REQUIRES_LIVE_ENV assertions noted for post-deploy check

---

### Phase 8: Post-Review Fixes — Tests

Tasks added from P5 code review findings. All relate to missing test coverage identified during review.

- [x] S018-T: Integration test for mixed-validity batch — partial commit behavior (P5-015)
  Document and test the current behavior: `apply_mutations` processes serially; the first error aborts with already-committed mutations unrolled. Assert: `[valid_create, forbidden_update]` returns 403 and first mutation IS committed; document partial-commit semantics in code comment.
  - **Assigned:** builder-rust
  - **Depends:** S009
  - **Relates to:** FA-001, FA-004

- [x] S019-T: Integration tests for delete on non-deletable entity types (P5-016)
  One test per guarded type (`TrackingItemEvent`, `KnowledgeNoteSnapshot`, `Tag`): POST /api/sync/push with `operation: "delete"` returns 400 Bad Request.
  - **Assigned:** builder-rust
  - **Depends:** S010

- [x] S020-T: Integration test for Update on immutable `KnowledgeNoteSnapshot` (P5-017)
  POST /api/sync/push with `operation: "update"` for entity_type `knowledge_note_snapshot` returns 400 Bad Request with message "knowledge_note_snapshot is immutable".
  - **Assigned:** builder-rust
  - **Depends:** S009

- [x] S021-T: Integration tests for ownership on Household, TrackingItem, TrackingItemEvent (P5-018)
  One test per type: non-owner/non-member attempting Update returns 403 Forbidden. Covers FA-005 for all indirect ownership paths.
  - **Assigned:** builder-rust
  - **Depends:** S005
  - **Relates to:** FA-005

- [x] S022-T: Integration tests for resolve endpoint edge cases (P5-019)
  (1) Resolving an already-resolved conflict returns 409. (2) Submitting an out-of-enum resolution string (e.g. `"garbage"`) returns 400 Bad Request (serde will reject at deserialization boundary).
  - **Assigned:** builder-rust
  - **Depends:** S016
  - **Relates to:** FA-006

- [x] S023: Refactor `MutationEnvelope` to enum-of-structs (P5-024)
  Replace flat struct with `#[serde(tag = "operation")] enum MutationPayload { Create { payload: Value, … }, Update { payload: Value, base_version: DateTime<Utc>, … }, Delete { … } }`. Wire format preserved; invalid cross-field states become unrepresentable at the serde boundary. Update service.rs dispatch accordingly.
  - **Assigned:** builder-rust
  - **Depends:** S009

- [x] S024: Add `ConflictStatus` enum replacing `'pending'` string literals (P5-025)
  Add `pub enum ConflictStatus { Pending, Accepted, Rejected, ConflictCopy }` with `as_str()` impl. Use in `service::list_conflicts` and `service::resolve_conflict` WHERE clauses. Eliminates silent typo risk on the `'pending'` filter.
  - **Assigned:** builder-rust
  - **Depends:** S015

- [x] S025-T: Integration test for non-KnowledgeNote LWW Rejected path (P5-026)
  For e.g. `GuidanceQuest`: submit Update with `occurred_at` older than server's `updated_at`; assert response contains `status: "conflicted"`, row data is unchanged, and `sync_conflicts` row exists with `resolution = 'pending'`.
  - **Assigned:** builder-rust
  - **Depends:** S009
  - **Relates to:** FA-008

🏁 **MILESTONE 8: Post-review hardening complete** — all P5 test tasks pass; `cargo test` exits 0

---

## Acceptance Criteria

- [x] FA-001 through FA-014 all verified (FA-010, FA-011 marked REQUIRES_LIVE_ENV)
- [x] `cargo test` exits 0 in `apps/server/` ← REQUIRES_LIVE_ENV (29/29 integration tests + 110/110 lib unit tests with `--test-threads=1`; parallel run limited by sqlx test pool size)
- [x] `sqlx migrate run` and `sqlx migrate revert` exit 0 against fresh DB (FA-014) ← REQUIRES_LIVE_ENV (verified 2026-04-15)
- [x] `packages/contracts/sync-streams.json` has `provisional: false` (FA-012)
- [x] No TODO/FIXME stubs in `src/sync/`
- [x] sync_rules.yaml bucket table names match Postgres migration table names exactly
- [x] Spec.md M-3 and M-4 annotated as overridden

## Validation Commands

```bash
# Server tests
cd apps/server && cargo test

# Migration apply + revert
sqlx migrate run --source infra/migrations
sqlx migrate revert --source infra/migrations

# Contracts assertion
jq '.provisional' packages/contracts/sync-streams.json  # → false

# Sync rules file exists
ls infra/compose/sync_rules.yaml

# REQUIRES_LIVE_ENV (post-deploy):
# docker compose up → check PowerSync logs for sync_rules parse errors
# docker compose ps → confirm PowerSync healthy
```
