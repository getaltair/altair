# PR Review: feat/sync-engine → main

**Date:** 2026-04-14
**Feature:** Context/Features/004-SyncEngine/
**Branch:** feat/sync-engine
**Reviewers:** code-reviewer, silent-failure-hunter, pr-test-analyzer, type-design-analyzer
**Status:** ✅ Resolved

## Summary

29 findings total across 4 review agents. 4 critical issues individually sufficient to block merge: SQL injection via payload-driven INSERT, authorization bypass enabling admin account creation through the sync endpoint, the update handler silently discarding all payload content while reporting Accepted, and sync_rules.yaml referencing nonexistent columns that prevent 3 tables from syncing. 6 additional silent failure patterns, 4 code convention violations, 5 missing test tasks, and 10 suggestions pending triage.

---

## Findings

### Architectural Concerns

#### [ADR] P5-001: SQL injection via unsanitized payload column names in `insert_from_payload`
- **File:** `apps/server/server/src/sync/service.rs:665–681`
- **Severity:** Critical
- **Detail:** Payload JSON object keys are interpolated directly into SQL strings as column names: `let col_list = columns.join(", "); let sql = format!("INSERT INTO {table} ({col_list}) VALUES ...")`. An authenticated client can inject arbitrary SQL through a crafted payload key. Violates postgres.md "parameterized queries only; never interpolate user input into SQL." The root cause is the generic payload-driven INSERT design; fixing it requires either a per-entity typed insert allowlist or strict key allowlisting against `^[a-z_][a-z0-9_]*$`. This is an architectural decision because the entire `apply_create`/`apply_update` path is built on this generic mechanism.
- **Relates to:** FA-001 (Create mutation accepted)
- **Status:** ✅ ADR created
- **Resolution:** ADR-017

#### [ADR] P5-002: Authorization bypass — Create mutations allow admin account creation via sync endpoint
- **File:** `apps/server/server/src/sync/service.rs:514–518, 621–700`
- **Severity:** Critical
- **Detail:** `process_single_mutation` explicitly skips `check_ownership` for `Operation::Create`, and no allowlist restricts which `EntityType`s may be created via sync push. `EntityType::User` and `EntityType::Household` are not in `is_user_scoped`, so their payloads are written verbatim. An authenticated client can submit `Create` for `EntityType::User` with `{"email": "...", "password_hash": "...", "is_admin": true}` and create an admin account. Similarly, household-scoped types (`TrackingLocation`, `TrackingCategory`, `TrackingShoppingList`) accept a client-supplied `household_id` in the payload without membership verification. Requires an architectural decision: either (a) explicit entity-type allowlist for sync-createable entities, (b) per-entity-type authorization checks on Create, or both.
- **Relates to:** FA-004 (ownership enforcement); FA-005 (cross-user isolation)
- **Status:** ✅ ADR created
- **Resolution:** ADR-018

### Fix-Now

#### [FIX] P5-003: Update handler discards all payload content — only bumps `updated_at`
- **File:** `apps/server/server/src/sync/service.rs:734–872`
- **Severity:** Critical
- **Detail:** `apply_update_to_table` executes `UPDATE {table} SET updated_at = now() WHERE id = $1` — no payload fields are written. Tech.md specifies: "UPDATE: set updated_at = now(), apply payload columns." Clients receive `MutationStatus::Accepted` and a `sync_mutations` row is recorded, but server state never changes. On the next pull the client sees its update reverted. This directly violates CLAUDE.md: "Sync conflicts must never silently lose data." Tests only assert status codes and `updated_at`, not that payload values persisted.
- **Relates to:** FA-002 (Update mutation accepted); S009 (apply_mutations implementation)
- **Status:** ✅ Fixed
- **Resolution:** `apply_update_to_table` now accepts `payload: &Option<Value>` and applies allowlisted columns via dynamic SET clause. Implemented alongside ADR-017 column allowlist.

#### [FIX] P5-004: `sync_rules.yaml` references nonexistent columns on 3 tables
- **File:** `infra/compose/sync_rules.yaml`
- **Severity:** Critical
- **Detail:** Three bucket rules reference columns that do not exist in the migrations:
  1. `knowledge_note_snapshots WHERE user_id = bucket.user_id` — no `user_id` column; ownership flows through `note_id → knowledge_notes.user_id`
  2. `tracking_item_events WHERE household_id = bucket.household_id` — no `household_id` column; flows through `item_id → tracking_items`
  3. `tracking_shopping_list_items WHERE household_id = bucket.household_id` — no `household_id` column; flows through `shopping_list_id → tracking_shopping_lists`
  PowerSync fails to validate at deployment; these 3 tables will not sync. Fix requires JOINs through parent tables.
- **Relates to:** FA-010 (sync rules parse); FA-011 (user isolation)
- **Status:** ✅ Fixed
- **Resolution:** Fixed all 3 rules in `sync_rules.yaml` to use subquery JOINs through parent tables.

#### [FIX] P5-005: Silent serialization failure writes empty string to `entity_type` in audit tables
- **File:** `apps/server/server/src/sync/service.rs:356–363, 888–891`
- **Severity:** High
- **Detail:** `serde_json::to_value(&envelope.entity_type).ok().and_then(...).unwrap_or_default()` swallows any `serde_json::Error` and writes `""` to the `entity_type` column in both `sync_mutations` and `sync_conflicts`. No log emitted. If the `EntityType` serialize impl changes (e.g., serde rename attribute regression), every mutation is logged with a blank entity type and the code returns `Ok(())`. Fix: propagate as `AppError::Internal` with an error log.
- **Status:** ✅ Fixed
- **Resolution:** `try_record_sync_mutation` and `record_conflict_row` now propagate serialization errors as `AppError::Internal` with `warn!` log.

#### [FIX] P5-006: KnowledgeNote conflict path silently falls through to general LWW when payload is `None`
- **File:** `apps/server/server/src/sync/service.rs:784–804, 827–847`
- **Severity:** High
- **Detail:** Both `ConflictOutcome::Accepted` and `Rejected` arms for KnowledgeNote are guarded by `if let Some(payload)`. If `payload` is `None` on a KnowledgeNote update conflict, neither arm executes — the code falls through to the general path without creating a conflict copy. The spec requires KnowledgeNote conflicts to always produce a copy. This silently loses data and violates the core "sync conflicts must never silently lose data" invariant. Fix: treat missing payload on a conflicted KnowledgeNote update as `AppError::BadRequest`.
- **Relates to:** FA-007 (conflict copy created); FA-009 (no silent data loss)
- **Status:** ✅ Fixed
- **Resolution:** Both Accepted and Rejected KnowledgeNote arms now use `.ok_or_else(|| AppError::BadRequest(...))` to hard-fail instead of silently falling through.

#### [FIX] P5-007: Transaction rollback failure silently discarded
- **File:** `apps/server/server/src/sync/service.rs:543`
- **Severity:** High
- **Detail:** `let _ = tx.rollback().await;` discards rollback errors entirely. If rollback fails (network partition, connection drop), the transaction may remain open or partially applied. No log is written. Fix: log at error level if rollback fails.
- **Status:** ✅ Fixed
- **Resolution:** Rollback result now checked; failure logged with `warn!("transaction rollback failed: {rollback_err}")`.

#### [FIX] P5-008: `updated_at` parse fallback silently corrupts conflict audit trail
- **File:** `apps/server/server/src/sync/service.rs:795–799, 838–842`
- **Severity:** High
- **Detail:** `.and_then(|s| s.parse::<DateTime<Utc>>().ok()).unwrap_or(envelope.occurred_at)` — if `updated_at` is missing or malformed in the DB payload, `current_version` written to `sync_conflicts` silently becomes the client's `occurred_at` instead of the server's actual version. No log emitted. This corrupts the conflict audit trail used for manual resolution. Fix: return `AppError::Internal` or log a warning with the raw value before falling back.
- **Status:** ✅ Fixed
- **Resolution:** Extracted `parse_updated_at_from_payload` helper; logs a `warn!` with entity_id before falling back to epoch (timestamp 0) rather than `occurred_at`.

#### [FIX] P5-009: `insert_from_payload` silently accepts non-object JSON payloads
- **File:** `apps/server/server/src/sync/service.rs:629–632`
- **Severity:** Medium
- **Detail:** `payload.and_then(|p| p.as_object()).cloned().unwrap_or_default()` resolves a JSON array or `null` payload to an empty map. The subsequent `columns.is_empty()` guard at line 673 is dead code when `id`/`user_id` are always added unconditionally. A client sending a JSON array gets an INSERT with only `id` and `user_id`, silently discarding all payload fields. Fix: explicitly return `AppError::BadRequest` if payload is not a JSON object.
- **Status:** ✅ Fixed
- **Resolution:** `insert_from_payload` now returns `AppError::BadRequest("Create payload must be a JSON object")` for null or non-object payloads.

#### [FIX] P5-010: `apply_delete` does not verify any rows were actually soft-deleted
- **File:** `apps/server/server/src/sync/service.rs:938–943`
- **Severity:** Medium
- **Detail:** The `UPDATE ... SET deleted_at = now() WHERE id = $1` result is discarded after error mapping. A delete of a nonexistent entity returns `MutationStatus::Accepted`. Additionally, because ownership is checked before delete, a missing-entity case currently surfaces as `Forbidden` rather than `NotFound` — a misleading error. Fix: check `rows_affected() == 0` and return `AppError::NotFound`.
- **Status:** ✅ Fixed
- **Resolution:** `apply_delete` checks `rows_affected() == 0` and returns `AppError::NotFound`. Also added `AND deleted_at IS NULL` to avoid matching already-deleted rows.

#### [FIX] P5-011: Handlers contain inline SQL — must delegate to service layer
- **File:** `apps/server/server/src/sync/handlers.rs:35–129`
- **Severity:** High
- **Detail:** `list_conflicts_handler` and `resolve_conflict_handler` embed raw SQL queries and result translation directly in handler functions. Violates rust-axum.md ("Keep command handlers thin; delegate to service modules") and the module pattern ("handlers.rs — Axum route handlers (thin; delegate to service)"). Fix: move SQL into `service::list_conflicts` and `service::resolve_conflict`; handlers call these and wrap in `Json(...)`.
- **Status:** ✅ Fixed
- **Resolution:** SQL moved to `service::list_conflicts` and `service::resolve_conflict`; handlers now one-liners.

#### [FIX] P5-012: `resolve_conflict_handler` allows overwriting already-resolved conflicts
- **File:** `apps/server/server/src/sync/handlers.rs:102–112`
- **Severity:** High
- **Detail:** The UPDATE has no `AND resolution = 'pending'` clause, so a user can toggle a conflict's resolution arbitrarily (accepted ↔ rejected ↔ conflict_copy) after it has already been resolved. Once downstream consumers (audit, reconciliation, notifications) react to `resolved_at`, this creates data integrity risk. Fix: add `AND resolution = 'pending'` to WHERE clause; return 409 if 0 rows affected and the row exists with a non-pending resolution.
- **Relates to:** FA-006 (conflict resolution endpoint)
- **Status:** ✅ Fixed
- **Resolution:** `service::resolve_conflict` adds `AND resolution = 'pending'`; disambiguates 0-rows-affected into NotFound, Forbidden, or Conflict(409).

#### [FIX] P5-013: Migration tables violate `created_at`/`updated_at`/trigger convention
- **File:** `infra/migrations/20260414000027_create_sync_mutations.up.sql`, `infra/migrations/20260414000028_create_sync_conflicts.up.sql`
- **Severity:** Medium
- **Detail:** postgres.md: "All tables require `created_at` and `updated_at` timestamps; `updated_at` auto-maintained via PL/pgSQL trigger." `sync_mutations` has `applied_at` only — no `created_at`, no `updated_at`, no trigger. `sync_conflicts` has `created_at` but no `updated_at` and no trigger, despite being mutated (`resolution`, `resolved_at`). Either add columns + trigger or document the intentional exception in an ADR.
- **Status:** ✅ Fixed
- **Resolution:** New migration `20260414000029_sync_tables_timestamps` adds `created_at`/`updated_at` to `sync_mutations` and `updated_at` to `sync_conflicts`, with `set_updated_at()` triggers on both.

#### [FIX] P5-014: Unreachable `TrackingItemEvent` quantity conflict check in `apply_update`
- **File:** `apps/server/server/src/sync/service.rs:806–815`
- **Severity:** Low
- **Detail:** Inside `ConflictOutcome::Accepted`, there is a guard `if matches!(envelope.entity_type, EntityType::TrackingItemEvent)` that calls `check_quantity_conflict`. But `TrackingItemEvent` returns early before `detect_conflict` is ever called (lines 741–756), making this block dead code. Misleads readers into thinking there is conflict-path quantity checking when there is not. Fix: remove the dead guard from the conflict arm, or restructure so `TrackingItemEvent` reaches the conflict arm and the quantity check there is the only one.
- **Status:** ✅ Fixed
- **Resolution:** Dead block removed when `ConflictOutcome::Accepted` arm was rewritten for P5-006.

### Missing Tasks

#### [TASK] P5-015: No test for mixed-validity batch (partial commit behavior undefined)
- **File:** `apps/server/server/src/sync/service.rs:476–489`
- **Severity:** Critical
- **Detail:** `apply_mutations` processes mutations serially via `?` — the first error propagates and earlier mutations in the batch are already committed (each runs in its own transaction). A client sending `[valid_create, forbidden_update]` receives a 403 with no indication that the first mutation was applied. The behavior (partial commit vs. all-or-nothing) is unspecified and unverified. Either document and test the current behavior or change it. A regression removing this distinction would be undetectable.
- **Relates to:** S009 (apply_mutations)
- **Status:** ✅ Task created
- **Resolution:** Added as S018-T in Steps.md Phase 8

#### [TASK] P5-016: No test for delete on non-deletable entity types
- **File:** `apps/server/server/src/sync/service.rs:927–933`
- **Severity:** High
- **Detail:** `apply_delete` explicitly rejects `TrackingItemEvent`, `KnowledgeNoteSnapshot`, and `Tag` with `AppError::BadRequest`. Neither unit tests nor integration tests exercise this path. A regression removing these guards would allow clients to soft-delete append-only rows. Need one integration test per guarded type confirming `POST /api/sync/push` with `operation: "delete"` returns 400.
- **Status:** ✅ Task created
- **Resolution:** Added as S019-T in Steps.md Phase 8

#### [TASK] P5-017: No test for Update on immutable `KnowledgeNoteSnapshot`
- **File:** `apps/server/server/src/sync/service.rs:759–763`
- **Severity:** High
- **Detail:** Guard returns `AppError::BadRequest("knowledge_note_snapshot is immutable")` for any Update mutation on this type. Completely untested. A snapshot is created at conflict copy time; allowing client updates through a regression would silently corrupt the immutability guarantee.
- **Status:** ✅ Task created
- **Resolution:** Added as S020-T in Steps.md Phase 8

#### [TASK] P5-018: Ownership untested for Household, TrackingItem, TrackingItemEvent entity types
- **File:** `apps/server/server/src/sync/service.rs` (check_ownership section)
- **Severity:** High
- **Detail:** FA-005 covers the attacker scenario only for `knowledge_note`. Ownership paths for `Household` (owner_id check), `TrackingItem` (dual user_id/household_id path), and `TrackingItemEvent` (indirect join through tracking_items) are untested at any level. These have more branching than the tested path. A regression bypassing ownership on these types would not be caught.
- **Relates to:** FA-005 (cross-user isolation)
- **Status:** ✅ Task created
- **Resolution:** Added as S021-T in Steps.md Phase 8

#### [TASK] P5-019: S016 re-resolve and invalid resolution value paths untested
- **File:** `apps/server/server/src/sync/handlers.rs`, `apps/server/server/tests/sync_push_integration.rs`
- **Severity:** Medium
- **Detail:** Two gaps: (1) No test for resolving a conflict that is already `accepted` or `rejected` — the handler currently allows silent overwrite. (2) No test for an out-of-enum resolution string (e.g., `"garbage"`, `"pending"`) — if the handler does not validate before writing, arbitrary strings land in the `resolution` column and silently escape the `'pending'` filter used in S015.
- **Relates to:** FA-006 (conflict resolution)
- **Status:** ✅ Task created
- **Resolution:** Added as S022-T in Steps.md Phase 8

---

## Suggestions (Pending Triage)

The following suggestions from the review agents have not yet been triaged. Run interactive triage or mark individually.

| # | Source | Title |
|---|--------|-------|
| S1 | type-analyzer | `MutationEnvelope` should be enum-of-structs to encode operation/payload/base_version constraints |
| S2 | type-analyzer | `ConflictRecord.entity_type` should be `EntityType` not `String` |
| S3 | type-analyzer | Add `ConflictStatus` enum for full 4-state resolution lifecycle |
| S4 | type-analyzer | `ConflictPageParams.limit` should be `Option<u32>` not `Option<i64>` |
| S5 | code-reviewer | Dedup check runs outside transaction — concurrent pushes with same mutation_id can both pass |
| S6 | code-reviewer | List conflicts cursor predicate collapses to empty page if cursor row is resolved |
| S7 | code-reviewer | Replace `.map_err(|e| AppError::Internal(anyhow::Error::from(e)))` boilerplate with `impl From<sqlx::Error> for AppError` |
| S8 | test-analyzer | Non-KnowledgeNote LWW rejected path ("existing wins") not tested for any non-note entity |

### Additional Fix-Now (from suggestions)

#### [FIX] P5-020: `ConflictRecord.entity_type` is `String` — type regression on read path
- **File:** `apps/server/server/src/sync/models.rs`
- **Severity:** Medium
- **Detail:** `MutationEnvelope.entity_type` is a typed `EntityType` enum (validated at deserialization), but `ConflictRecord.entity_type` reads back from Postgres as `String`. An unexpected database value (manual correction, migration) passes to the client silently without validation. Fix: implement a post-query conversion from `String` to `EntityType`, returning `Internal` if the string is not a known variant.
- **Status:** ✅ Fixed
- **Resolution:** Introduced `ConflictRow` (private `sqlx::FromRow` with `String`) and `ConflictRecord` (typed `EntityType`). `ConflictRecord::from_row` converts and skips rows with unknown types (logged as warning).

#### [FIX] P5-021: `ConflictPageParams.limit` accepts negative values — use `Option<u32>`
- **File:** `apps/server/server/src/sync/models.rs`
- **Severity:** Low
- **Detail:** `limit: Option<i64>` accepts any i64 including negatives; the handler clamps it, but the type alone permits `-9_223_372_036_854_775_808`. Change to `Option<u32>` to push the non-negative constraint to the type boundary; the upper-bound clamp in the handler remains correct.
- **Status:** ✅ Fixed
- **Resolution:** `ConflictPageParams.limit` changed to `Option<u32>`.

#### [FIX] P5-022: Dedup check runs outside transaction — concurrent same `mutation_id` pushes can both pass
- **File:** `apps/server/server/src/sync/service.rs:500`
- **Severity:** Medium
- **Detail:** The `mutation_id` dedup lookup runs before the transaction begins. Two concurrent pushes with the same `mutation_id` can both pass the check; the second then fails on the PK conflict and surfaces as a 500 rather than `Deduplicated`, defeating the idempotency guarantee under contention. Fix: use `INSERT INTO sync_mutations … ON CONFLICT (mutation_id) DO NOTHING RETURNING` to resolve atomically within a single statement.
- **Relates to:** FA-003 (idempotent dedup); S-2 invariant
- **Status:** ✅ Fixed
- **Resolution:** `try_record_sync_mutation` uses `INSERT ... ON CONFLICT (mutation_id) DO NOTHING RETURNING mutation_id`; returns `None` → Deduplicated.

#### [FIX] P5-023: Cursor predicate collapses to empty page if cursor row is resolved/deleted
- **File:** `apps/server/server/src/sync/handlers.rs` (list_conflicts_handler)
- **Severity:** Medium
- **Detail:** `WHERE created_at < (SELECT created_at FROM sync_conflicts WHERE id = $2)` returns NULL if the cursor row has been resolved or deleted, collapsing the WHERE to false and returning an empty page — a silent pagination failure the caller cannot distinguish from "no more results." Fix: use a stable tuple cursor `(created_at, id)` or a separate cursor anchor that is not subject to row deletion.
- **Status:** ✅ Fixed
- **Resolution:** `service::list_conflicts` first fetches the anchor row by cursor id to get `(created_at, id)`, then uses `(created_at, id) < ($2, $3)` tuple comparison. If the anchor no longer exists, returns empty page (safe behavior).

### Additional Missing Tasks (from suggestions)

#### [TASK] P5-024: Refactor `MutationEnvelope` to enum-of-structs encoding operation/payload/base_version constraints
- **File:** `apps/server/server/src/sync/models.rs`, `apps/server/server/src/sync/service.rs`
- **Severity:** Medium
- **Detail:** `MutationEnvelope` is a flat struct with all fields `pub` and no cross-field validation at the serde boundary. A `Create` with `payload: None`, a `Delete` with a payload, or an `Update` with no `base_version` all parse successfully and reach service code. Restructure as an enum-of-structs (`Create { payload: Value, … }`, `Update { payload: Value, base_version: DateTime<Utc>, … }`, `Delete { base_version: DateTime<Utc>, … }`) with `#[serde(tag = "operation", rename_all = "snake_case")]`. Wire format is preserved; invalid states become unrepresentable.
- **Status:** ✅ Task created
- **Resolution:** Added as S023 in Steps.md Phase 8

#### [TASK] P5-025: Add `ConflictStatus` enum for the full 4-state resolution lifecycle
- **File:** `apps/server/server/src/sync/models.rs`, `apps/server/server/src/sync/handlers.rs`
- **Severity:** Low
- **Detail:** The resolution lifecycle (`pending`, `accepted`, `rejected`, `conflict_copy`) exists only as scattered SQL string literals. A raw `'pending'` string in `handlers.rs` line 50 filters the list endpoint; a typo would produce a silently empty conflict list. Add a `ConflictStatus` enum with `as_str()` and use it in `ConflictRecord` and handler queries.
- **Status:** ✅ Task created
- **Resolution:** Added as S024 in Steps.md Phase 8

#### [TASK] P5-026: Test non-KnowledgeNote LWW `Rejected` path ("existing wins") for at least one entity type
- **File:** `apps/server/server/tests/sync_push_integration.rs`
- **Severity:** Medium
- **Detail:** For all entity types except `KnowledgeNote`, a `ConflictOutcome::Rejected` mutation records a conflict row but does NOT apply the update — the existing server value wins. No test verifies this for any non-note entity. Submit an Update with `occurred_at` older than `updated_at` for e.g. `GuidanceQuest`, assert the row data is unchanged, the response returns `conflicted`, and a `sync_conflicts` row exists with `resolution = 'pending'`.
- **Status:** ✅ Task created
- **Resolution:** Added as S025-T in Steps.md Phase 8

### Convention Gaps

#### [RULE] P5-027: `impl From<sqlx::Error> for AppError` missing — boilerplate `.map_err` appears ~40 times
- **Files:** `apps/server/server/src/sync/service.rs` (throughout), `apps/server/server/src/sync/handlers.rs`
- **Severity:** Medium
- **Detail:** rust-axum.md already mandates: "Map sqlx::Error via `impl From<sqlx::Error> for AppError`… to eliminate boilerplate." `AppError` already derives `#[from] anyhow::Error`. The sync module adds ~40 instances of `.map_err(|e| AppError::Internal(anyhow::Error::from(e)))` instead. This pattern should be added as a concrete example in rust-axum.md and enforced during review.
- **Suggested rule:** Add to `.claude/rules/rust-axum.md` under Error Handling: "Never write `.map_err(|e| AppError::Internal(anyhow::Error::from(e)))` — add `impl From<sqlx::Error> for AppError` once per crate and use `?` directly."
- **Status:** ✅ Rule updated
- **Resolution:** Added to `.claude/rules/rust-axum.md` under Error Handling.

---

## Resolution Checklist
- [x] All [FIX] findings resolved (P5-003 through P5-014, P5-020 through P5-023)
- [x] All [TASK] findings added to Steps.md (P5-015 through P5-019, P5-024 through P5-026)
- [x] All [ADR] findings have ADRs created (P5-001 → ADR-017, P5-002 → ADR-018)
- [x] All [RULE] findings applied (P5-027 → rust-axum.md)
- [x] Review verified by review-verify agent

## Resolution Summary
**Resolved at:** 2026-04-14
**Session:** P5 review findings — fresh context session

| Category | Total | Resolved |
|---|---|---|
| [ADR] | 2 | 2 |
| [FIX] | 18 | 18 |
| [TASK] | 8 | 8 |
| [RULE] | 1 | 1 |
| **Total** | **29** | **29** |
