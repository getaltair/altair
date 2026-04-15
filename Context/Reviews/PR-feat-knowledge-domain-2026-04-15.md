# PR Review: feat/knowledge-domain → main

**Date:** 2026-04-15
**Feature:** Context/Features/006-KnowledgeDomain/
**Branch:** worktree-feat+knowledge-domain
**PR:** #5
**Reviewers:** code-reviewer, pr-test-analyzer, silent-failure-hunter, type-design-analyzer, comment-analyzer
**Status:** ✅ Resolved

## Summary

17 findings total from a 5-agent review of the Knowledge domain server module (Feature 006). 13 Fix-Now items (4 critical/high severity), 4 Missing Task items (2 critical). No architectural concerns — the module structure, ownership model, and immutability design are sound. One convention gap noted regarding inline `map_err` already documented in `rust-axum.md` but not enforced by `error.rs`. Key blockers before merge: NULL content crash in snapshot creation, wrong HTTP status codes for DB constraint violations, and missing cross-user mutation isolation tests.

---

## Findings

### Fix-Now

#### [FIX] P6-001: Add `impl From<sqlx::Error> for AppError` — inline `map_err` violates project rule
- **File:** `apps/server/server/src/error.rs` + `apps/server/server/src/knowledge/service.rs:31,51,62,80,103,118,152,173,206`
- **Severity:** Critical
- **Detail:** `rust-axum.md` explicitly forbids `.map_err(|e| AppError::Internal(anyhow::Error::from(e)))` inline and requires a single `impl From<sqlx::Error> for AppError` in `error.rs`. That impl is absent. The PR adds 8–10 instances of the forbidden form. Fix: add the `From` impl to `error.rs` and replace all inline `.map_err` calls with bare `?`. This also unblocks P6-005.
- **Status:** ✅ Fixed
- **Resolution:** Added `impl From<sqlx::Error> for AppError` to `error.rs` mapping `RowNotFound` → `NotFound`, `23505` → `Conflict`, `23503` → `BadRequest`, else `Internal`. All 9 inline `.map_err` calls replaced with bare `?` in `service.rs`.

#### [FIX] P6-002: Snapshot creation crashes when note has NULL content
- **File:** `apps/server/server/src/knowledge/service.rs:140-152`
- **Severity:** Critical
- **Detail:** `create_snapshot` INSERT copies `n.content` from `knowledge_notes` (nullable `TEXT`) into `knowledge_note_snapshots.content` (`TEXT NOT NULL` per migration 000018). `CreateNoteRequest.content` is `Option<String>`, so a note with no content is valid — but snapshotting it will fail with a PostgreSQL NOT NULL constraint violation at runtime. Fix: either use `COALESCE(n.content, '')` in the INSERT, or make the snapshot column nullable. Must be resolved in tandem with P6-007.
- **Relates to:** A-021 (snapshot content frozen at capture time)
- **Status:** ✅ Fixed
- **Resolution:** `create_snapshot` INSERT changed to `COALESCE(n.content, '')`. `SnapshotResponse.content` changed from `Option<String>` to `String` (matching DB `TEXT NOT NULL`). Unit test assertions updated accordingly.

#### [FIX] P6-003: `create_snapshot` returns 500 instead of 404 on TOCTOU race
- **File:** `apps/server/server/src/knowledge/service.rs:151`
- **Severity:** High
- **Detail:** `create_snapshot` uses `fetch_one` on the INSERT-SELECT. If the note is soft-deleted between the pre-check call to `get_note` (line 138) and the INSERT, `fetch_one` returns `sqlx::Error::RowNotFound` which maps to `AppError::Internal` → 500. All other service functions use `fetch_optional` + `.ok_or(AppError::NotFound)`. Fix: `let row = sqlx::query_as::<_, SnapshotResponse>(...).fetch_optional(db).await?.ok_or(AppError::NotFound)?;`
- **Status:** ✅ Fixed
- **Resolution:** Changed to `fetch_optional(db).await?.ok_or(AppError::NotFound)?`. TOCTOU race now returns 404 instead of 500.

#### [FIX] P6-004: NoteLinked TODO comment absent — PR description claim does not match code
- **File:** `apps/server/server/src/knowledge/service.rs` (no line — absence)
- **Severity:** Medium
- **Detail:** PR description states "NoteLinked domain event stubbed as `tracing::info!` with a TODO comment." No such code, comment, `TODO`, `FIXME`, `NoteLinked`, or `tracing` import exists anywhere in the knowledge module. The planned stub was never written. Fix: add `// TODO: emit NoteLinked domain event` (and optionally a `tracing::info!`) at the appropriate point in `create_note`, or remove the claim from the PR body.
- **Status:** ✅ Fixed
- **Resolution:** Added `// TODO: emit NoteLinked domain event when this note is linked to another note` in `create_note`.

#### [FIX] P6-005: Duplicate UUID (23505) and FK violation (23503) return 500 instead of 409/400
- **File:** `apps/server/server/src/error.rs` + `apps/server/server/src/knowledge/service.rs`
- **Severity:** High
- **Detail:** All sqlx errors collapse to `AppError::Internal` → 500. Two semantically meaningful cases are lost: (1) A client retrying a `create_note` with the same UUID (expected offline-sync behavior per A-018) receives 500 instead of 409 Conflict. (2) An invalid `initiative_id` FK violation receives 500 instead of 400 Bad Request. Fix: inspect `sqlx::Error::Database` in the `From<sqlx::Error>` impl (from P6-001) for codes `"23505"` → `AppError::Conflict` and `"23503"` → `AppError::BadRequest`. Depends on P6-001.
- **Relates to:** A-018 (client-generated UUID)
- **Status:** ✅ Fixed
- **Resolution:** Resolved via P6-001 `From<sqlx::Error>` impl — code `23505` → `Conflict`, `23503` → `BadRequest`.

#### [FIX] P6-006: `NoteResponse` exposes `deleted_at` — always `None` at API boundary
- **File:** `apps/server/server/src/knowledge/models.rs:28`
- **Severity:** Medium
- **Detail:** `deleted_at: Option<DateTime<Utc>>` is included in `NoteResponse`. Every service query that produces a `NoteResponse` filters `WHERE deleted_at IS NULL`, so this field serializes as `null` in every API response. It leaks the soft-delete implementation to clients and creates a misleading API contract. Fix: introduce a private `NoteRow` struct with `sqlx::FromRow` (includes `deleted_at` for DB mapping), add `From<NoteRow> for NoteResponse`, and remove `deleted_at` from the public response type.
- **Status:** ✅ Fixed
- **Resolution:** Added private `NoteRow` struct (with `sqlx::FromRow` + `deleted_at`) in `service.rs`. Added `From<NoteRow> for NoteResponse`. Removed `deleted_at` and `sqlx::FromRow` from public `NoteResponse`. All service queries now use `NoteRow` internally.

#### [FIX] P6-007: `SnapshotResponse.content` is `Option<String>` but DB column is `TEXT NOT NULL`
- **File:** `apps/server/server/src/knowledge/models.rs:46`
- **Severity:** High
- **Detail:** The `content` field is `Option<String>` but `knowledge_note_snapshots.content` is `TEXT NOT NULL`. The type claims content can be null but it never will be. This misleads client consumers who will defensively handle a null that will never appear. Must be resolved in tandem with P6-002 — whichever nullability direction is chosen for the schema/model, both sides must match.
- **Relates to:** A-021
- **Status:** ✅ Fixed
- **Resolution:** Changed `content: Option<String>` → `content: String` in `SnapshotResponse`. Resolved in tandem with P6-002 (COALESCE ensures NOT NULL). Doc comment added explaining the COALESCE rationale.

#### [FIX] P6-008: `captured_at` future-timestamp validation missing
- **File:** `apps/server/server/src/knowledge/service.rs` (create_snapshot)
- **Severity:** Medium
- **Detail:** `CreateSnapshotRequest.captured_at` admits any `DateTime<Utc>` including far-future values. A client sending `captured_at: "3000-01-01T00:00:00Z"` would permanently float that snapshot to the top of `list_snapshots` (ordered `captured_at DESC`), burying all real snapshots. Fix: add a guard at the top of `service::create_snapshot`: `if req.captured_at > Utc::now() + Duration::seconds(30) { return Err(AppError::BadRequest(...)) }`. The leeway accounts for clock skew.
- **Status:** ✅ Fixed
- **Resolution:** Added guard `if req.captured_at > Utc::now() + Duration::seconds(30)` → `BadRequest` at top of `create_snapshot`.

#### [FIX] P6-009: `UpdateNoteRequest` all-`None` no-op is unguarded
- **File:** `apps/server/server/src/knowledge/service.rs` (update_note)
- **Severity:** Low
- **Detail:** `UpdateNoteRequest { title: None, content: None }` passes deserialization, hits the DB, and returns the unchanged note as if an update occurred. Fix: add an early-return guard at the top of `service::update_note`: `if req.title.is_none() && req.content.is_none() { return Err(AppError::BadRequest("at least one field required".to_string())) }`.
- **Status:** ✅ Fixed
- **Resolution:** Added all-None early-return guard in `update_note`.

#### [FIX] P6-010: `CreateNoteRequest.title` admits empty strings — no validation
- **File:** `apps/server/server/src/knowledge/models.rs` / `service.rs` (create_note)
- **Severity:** Medium
- **Detail:** `title: String` accepts `""` and whitespace-only values. No validation exists at the type boundary or in the service. An empty-title note is written to the DB unchecked. Fix: add a non-empty check at the top of `service::create_note`: `if req.title.trim().is_empty() { return Err(AppError::BadRequest("title must not be empty".to_string())) }`. Same check should be applied to `UpdateNoteRequest.title` when `Some`.
- **Status:** ✅ Fixed
- **Resolution:** Added `trim().is_empty()` guard in `create_note`. Also added guard in `update_note` when `title` is `Some`.

#### [FIX] P6-011: Spec tag references in service.rs lack document context
- **File:** `apps/server/server/src/knowledge/service.rs` (multiple)
- **Severity:** Low
- **Detail:** Comments throughout `service.rs` reference invariant tags (`E-6`, `SEC-1`, `A-019`) without naming the source document. A maintainer without `Spec.md` open cannot resolve them. Also, the SEC-1 guard on list_snapshots and list_backlinks should note that `NotFound` (not `Forbidden`) is returned intentionally to avoid leaking existence information. Fix: expand tags inline, e.g. `// SEC-1 (Spec.md): returns NotFound — not Forbidden — to avoid leaking whether the note exists at all.`
- **Status:** ✅ Fixed
- **Resolution:** All SEC-1 ownership guards now read `// SEC-1 (Spec.md): verify ... Returns NotFound — not Forbidden — to avoid leaking whether the note exists at all.`

#### [FIX] P6-012: `updated_at = NOW()` in UPDATE queries is redundant with DB trigger
- **File:** `apps/server/server/src/knowledge/service.rs:93,111`
- **Severity:** Low
- **Detail:** `knowledge_notes` has a `BEFORE UPDATE` trigger (`knowledge_notes_updated_at`) that calls `set_updated_at()`. The `update_note` and `delete_note` queries also manually `SET updated_at = NOW()`. The trigger overwrites the value anyway, but the manual SET implies to readers that removing it would break timestamp updates — which is false. Fix: remove the `updated_at = NOW()` from both UPDATE statements and let the trigger handle it exclusively.
- **Status:** ✅ Fixed
- **Resolution:** Removed `updated_at = NOW()` from both `update_note` and `delete_note` SQL. Added inline comment noting the trigger owns `updated_at`.

#### [FIX] P6-013: `captured_at` vs `created_at` distinction undocumented in `SnapshotResponse`
- **File:** `apps/server/server/src/knowledge/models.rs` (SnapshotResponse)
- **Severity:** Low
- **Detail:** `SnapshotResponse` has two timestamps with distinct semantics: `captured_at` is when the user triggered the capture (may be in the past for offline clients), `created_at` is when the row was physically written to the DB. This distinction is non-obvious to API consumers. Fix: add a doc comment to `SnapshotResponse` or the individual fields explaining the offline-first intent: "`captured_at`: client-reported capture time (may precede server ingestion); `created_at`: server insertion timestamp."
- **Status:** ✅ Fixed
- **Resolution:** Added doc comment to `SnapshotResponse` explaining `captured_at` (client-reported, offline-first) vs `created_at` (server insertion timestamp).

---

### Missing Tasks

#### [TASK] P6-014: Cross-user UPDATE isolation untested (SEC-1)
- **File:** `apps/server/server/src/knowledge/service.rs` + `tests/knowledge_integration.rs`
- **Severity:** Critical
- **Detail:** `service::update_note` includes `AND user_id = $2` in the WHERE clause, but no unit or integration test verifies that User B's PUT against User A's note returns 404. If the `user_id` guard were accidentally dropped during a future refactor, the entire test suite would continue to pass. Required tests: (1) unit — User B calls `update_note(pool, note_a.id, ..., user_b_id)` → `Err(AppError::NotFound)`; (2) integration — User B sends `PUT /api/knowledge/notes/{note_a_id}` → 404.
- **Relates to:** SEC-1 (Spec.md)
- **Status:** ✅ Task created
- **Resolution:** Added as S009 in Steps.md. Unit test `update_other_users_note_returns_not_found` added to `service.rs`. Integration test `test_user_isolation_update_other_users_note_returns_404` added to `knowledge_integration.rs`.

#### [TASK] P6-015: Cross-user DELETE isolation untested (SEC-1)
- **File:** `apps/server/server/src/knowledge/service.rs` + `tests/knowledge_integration.rs`
- **Severity:** Critical
- **Detail:** Same gap as P6-014 but for the DELETE path. `service::delete_note` includes `AND user_id = $2` but no test covers User B attempting to delete User A's note. Required tests: (1) unit — User B calls `delete_note(pool, note_a.id, user_b_id)` → `Err(AppError::NotFound)`; (2) integration — User B sends `DELETE /api/knowledge/notes/{note_a_id}` → 404.
- **Relates to:** SEC-1 (Spec.md)
- **Status:** ✅ Task created
- **Resolution:** Added as S009 in Steps.md. Unit test `delete_other_users_note_returns_not_found` added to `service.rs`. Integration test `test_user_isolation_delete_other_users_note_returns_404` added to `knowledge_integration.rs`.

#### [TASK] P6-016: E-6 assertion not verified for DELETE verb on snapshot endpoint
- **File:** `apps/server/server/tests/knowledge_integration.rs`
- **Severity:** Medium
- **Detail:** The integration test at line 447–460 verifies that `PUT /api/knowledge/notes/{note_id}/snapshots/{snap_id}` → 404/405 (E-6 for PUT). There is no test for the DELETE verb. The router registers only POST and GET on the snapshots route, so DELETE would 404/405 by construction — but the invariant needs explicit proof at the routing level, not inference from absence. Required test: send `DELETE /api/knowledge/notes/{note_id}/snapshots/{snap_id}` with valid auth → assert 404 or 405.
- **Relates to:** E-6 (Spec.md)
- **Status:** ✅ Task created
- **Resolution:** Added as S010 in Steps.md. DELETE assertion added inline to `test_create_snapshot_returns_201_with_content` in `knowledge_integration.rs`.

#### [TASK] P6-017: `create_snapshot` against soft-deleted note untested
- **File:** `apps/server/server/src/knowledge/service.rs`
- **Severity:** Medium
- **Detail:** `service::create_snapshot` calls `get_note(db, note_id, user_id)` before inserting to enforce ownership (and implicitly, that the note is not deleted). There is no test that creates a note, soft-deletes it, then attempts `create_snapshot` on its ID — verifying `Err(AppError::NotFound)`. A regression that removed the `get_note` pre-check would go undetected.
- **Relates to:** A-021 (Spec.md)
- **Status:** ✅ Task created
- **Resolution:** Added as S010 in Steps.md. Unit test `create_snapshot_on_deleted_note_returns_not_found` added to `service.rs`.

---

## Resolution Checklist
- [x] All [FIX] findings resolved (P6-001 through P6-013)
- [x] All [TASK] findings added to Steps.md (P6-014 through P6-017)
- [ ] Review verified by review-verify agent

## Resolution Summary
**Resolved at:** 2026-04-15
**Session:** review-resolve — all 17 findings executed in one pass

| Category | Total | Resolved |
|---|---|---|
| [FIX] | 13 | 13 |
| [TASK] | 4 | 4 |
| **Total** | **17** | **17** |
