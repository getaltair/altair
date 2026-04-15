# PR Review: worktree-feat+guidance-domain → main

**Date:** 2026-04-15
**Feature:** Context/Features/005-GuidanceDomain/
**Branch:** worktree-feat+guidance-domain
**Reviewers:** code-reviewer, silent-failure-hunter, pr-test-analyzer, type-design-analyzer
**Status:** ✅ Resolved

## Summary

19 findings across the Guidance domain implementation (Feature 005). 2 Critical data-integrity bugs in `focus_sessions/service.rs` (negative duration stored silently; non-atomic quest+session writes). 7 High findings covering missing error handling, a behavioral gap in epic status recalculation, missing tests for QuestStatus transitions, and an incomplete A-G-14 assertion test. 10 lower-priority fixes and tasks covering defense-in-depth user_id scoping, type improvements, and API response cleanup.

---

## Findings

### Fix-Now

#### [FIX] P6-001: Negative `duration_minutes` written to DB when `ended_at < started_at`
- **File:** `apps/server/server/src/guidance/focus_sessions/service.rs` (duration computation block)
- **Severity:** Critical
- **Detail:** `(ended_at - current.started_at).num_seconds()` returns a negative `i64` when `ended_at` is before `started_at` (possible with clock drift or malformed client input). The result is cast to `i32` and written to `duration_minutes` with no guard. Client receives 200 with a negative duration. No test exists for this path. Fix: reject with `AppError::UnprocessableEntity("ended_at must be after started_at")` before the duration computation.
- **Relates to:** A-G-11
- **Status:** ✅ Fixed
- **Resolution:** Added `if ended_at <= current.started_at` guard in `update_session` returning `UnprocessableEntity` before duration computation.

#### [FIX] P6-002: Quest status flip and session INSERT are not in a transaction
- **File:** `apps/server/server/src/guidance/focus_sessions/service.rs` — `create_session`
- **Severity:** Critical
- **Detail:** The auto-transition `UPDATE guidance_quests SET status = 'in_progress'` and the focus session `INSERT` run as independent operations on the bare pool. If the INSERT fails (constraint violation, pool exhaustion), the quest is permanently stuck in `in_progress` with no session attached. The `update_quest` function correctly uses a `sqlx::Transaction` for its paired writes. Same pattern required here: `pool.begin()` / `tx.commit()`.
- **Relates to:** A-G-10
- **Status:** ✅ Fixed
- **Resolution:** Wrapped auto-transition UPDATE and session INSERT in a single `pool.begin()` / `tx.commit()` transaction in `create_session`. Also resolved P6-010 in the same change (added `AND user_id = $2` to the UPDATE).

#### [FIX] P6-003: `update_checkin` does not catch Postgres `23505` — unique violation becomes 500
- **File:** `apps/server/server/src/guidance/daily_checkins/service.rs` — `update_checkin`
- **Severity:** High
- **Detail:** `create_checkin` correctly intercepts `db_err.code() == Some("23505")` and returns `AppError::Conflict`. `update_checkin` allows changing `checkin_date` but uses bare `?`, so the same unique constraint (`(user_id, checkin_date)`) fires as an unhandled DB error → `AppError::Internal` → 500. Apply the same match arm used in `create_checkin`.
- **Relates to:** A-G-12
- **Status:** ✅ Fixed
- **Resolution:** Changed `update_checkin` to match on the result, catching `23505` → `AppError::Conflict` with the same pattern used in `create_checkin`.

#### [FIX] P6-004: Epic status recalculation only triggered on quest completion, not on `in_progress` transition
- **File:** `apps/server/server/src/guidance/quests/service.rs` — `update_quest`
- **Severity:** High
- **Detail:** `recalculate_epic_status` is called only when a quest transitions to `completed`. Per `06-state-machines.md`, the epic should also transition `not_started → in_progress` when the first child quest moves to `in_progress`. The recalculation function handles this case correctly — it is just never invoked. Fix: call `recalculate_epic_status` for any status change on quests that have an `epic_id`, not only completion.
- **Status:** ✅ Fixed
- **Resolution:** Extended the transactional block condition to trigger on `transitioning_to_in_progress || transitioning_to_completed` when `current.epic_id` is Some. `recalculate_epic_status` now fires for both transitions.

#### [FIX] P6-009: `energy_level: i32` has no bounds validation
- **File:** `apps/server/server/src/guidance/daily_checkins/service.rs` — `create_checkin` and `update_checkin`
- **Severity:** Medium
- **Detail:** `energy_level` accepts any `i32`. Add a range check against the spec-defined valid range (e.g., 1–10) before the insert in both functions. Return `AppError::UnprocessableEntity` for out-of-range values.
- **Status:** ✅ Fixed
- **Resolution:** Added `!(1..=10).contains(&energy_level)` guard in both `create_checkin` and `update_checkin` returning `UnprocessableEntity` for out-of-range values.

#### [FIX] P6-010: Focus session auto-transition UPDATE missing `user_id` in WHERE clause
- **File:** `apps/server/server/src/guidance/focus_sessions/service.rs` — auto-transition UPDATE in `create_session`
- **Severity:** Medium
- **Detail:** The UPDATE that transitions `not_started → in_progress` does not include `AND user_id = $2`. Current flow is safe (ownership verified by the SELECT just above), but violates defense-in-depth: a future refactor that reorders operations could allow updating another user's quest. Add `AND user_id = $2` and bind `user_id`.
- **Status:** ✅ Fixed
- **Resolution:** Added `AND user_id = $2` to the auto-transition UPDATE and bound `user_id` as `$2`. Resolved as part of the P6-002 transaction fix.

#### [FIX] P6-011: `recalculate_epic_status` UPDATE has no `user_id` guard
- **File:** `apps/server/server/src/guidance/quests/service.rs` — `recalculate_epic_status`
- **Severity:** Medium
- **Detail:** The UPDATE targets the epic by `id` only. Ownership is verified earlier in the call chain, so there is no active security gap. Adding `AND user_id = $3` maintains the defense-in-depth pattern used throughout the codebase.
- **Status:** ✅ Fixed
- **Resolution:** Added `user_id: Uuid` parameter to `recalculate_epic_status` and `AND user_id = $3` to the UPDATE. Updated all call sites.

#### [FIX] P6-012: `spawn_routine_quest` idempotency check missing `AND user_id`
- **File:** `apps/server/server/src/guidance/routines/service.rs` — `spawn_routine_quest`
- **Severity:** Low
- **Detail:** The idempotency guard checks for an existing quest by `routine_id` and `due_date` without filtering by `user_id`. UUID collisions are extremely unlikely in practice, but adding `AND user_id = $3` is consistent with the defense-in-depth pattern everywhere else.
- **Status:** ✅ Fixed
- **Resolution:** Added `AND user_id = $3` to the idempotency check query and bound `user_id` as `$3`.

#### [FIX] P6-013: `EpicStatus` raw string literals in `recalculate_epic_status`
- **File:** `apps/server/server/src/guidance/quests/service.rs` — `recalculate_epic_status`
- **Severity:** Medium
- **Detail:** The function binds `"completed"`, `"in_progress"`, `"not_started"` as raw string literals in SQL rather than using `EpicStatus` enum values. If the enum's `serde` rename/serialize attribute ever changes, the raw strings diverge silently. Bind typed enum values to keep the SQL in sync with the type definition.
- **Status:** ✅ Fixed
- **Resolution:** Imported `EpicStatus` from `guidance::epics::models` and replaced all three raw string bindings with typed enum variants (`EpicStatus::Completed`, `EpicStatus::InProgress`, `EpicStatus::NotStarted`).

#### [FIX] P6-017: `deleted_at` serialized as `null` in every API response
- **File:** All five guidance domain row structs (`Epic`, `Quest`, `Routine`, `FocusSession`, `DailyCheckin`)
- **Severity:** Low
- **Detail:** `deleted_at` is always `None` for records returned by the API (soft-delete filter applied in queries) but is serialized as `"deleted_at": null` in every response, leaking the internal soft-delete implementation detail. Apply `#[serde(skip_serializing_if = "Option::is_none")]` or `#[serde(skip)]` to these fields on all row structs.
- **Status:** ✅ Fixed
- **Resolution:** Added `#[serde(skip_serializing_if = "Option::is_none")]` to `deleted_at` on all five row structs: `Epic`, `Quest`, `Routine`, `FocusSession`, `DailyCheckin`.

#### [FIX] P6-018: `From<sqlx::Error>` missing comment about `fetch_one` safety contract
- **File:** `apps/server/server/src/error.rs`
- **Severity:** Low
- **Detail:** The impl maps `RowNotFound → NotFound` globally. This is safe because all row lookups use `fetch_optional` + `.ok_or(...)`. Future contributors using `fetch_one` on zero-row-possible queries would silently produce 404s instead of 500s. Add a `// NOTE:` comment making this contract explicit.
- **Status:** ✅ Fixed
- **Resolution:** Added `// NOTE:` comment in `From<sqlx::Error>` explaining the `fetch_one` safety contract and why all row lookups must use `fetch_optional`.

---

### Missing Tasks

#### [TASK] P6-006: Three forbidden `QuestStatus` transitions have no tests
- **File:** `apps/server/server/src/guidance/quests/models.rs` — `#[cfg(test)]` block
- **Severity:** High
- **Detail:** Three forbidden transitions are untested: `NotStarted → Deferred`, `InProgress → NotStarted`, and `Deferred → InProgress`. Also `Completed → Cancelled` is untested (returns `false` via terminal-state early return but no assertion verifies it). A refactor of the `matches!` arm could introduce regressions with no test catching them.
- **Relates to:** A-G-05, A-G-06
- **Status:** ✅ Task created
- **Resolution:** Added as S012-T in Steps.md Phase 3.

#### [TASK] P6-007: No cross-user isolation test for the routines module
- **File:** `apps/server/server/src/guidance/routines/service.rs`
- **Severity:** Medium
- **Detail:** Every other module (quests, epics, focus sessions, daily checkins) has at least one test calling a service function with a different user's ID and asserting `NotFound` or `Forbidden`. The routines module has no such test. Add a test for `get_routine(pool, routine.id, other_user_id)` asserting `NotFound`.
- **Status:** ✅ Task created
- **Resolution:** Added as S015-T in Steps.md Phase 4.

#### [TASK] P6-008: A-G-14 event test does not verify `QuestCompleted` event is emitted
- **File:** `apps/server/server/tests/guidance_integration.rs`
- **Severity:** High
- **Detail:** The A-G-14 integration test verifies the PATCH returns 200 but does not capture or assert the `tracing::info!(quest_id = %id, "QuestCompleted")` event. If the `tracing::info!` line is deleted, the test still passes. Use `tracing_test::traced_test` or a test subscriber to verify the event fires. Alternatively, move to a structured `DomainEvent` type that can be asserted in unit tests.
- **Relates to:** A-G-14
- **Status:** ✅ Task created
- **Resolution:** Added as S016-T in Steps.md Phase 5.

#### [TASK] P6-014: `priority: String` — introduce `QuestPriority` enum
- **File:** `apps/server/server/src/guidance/quests/models.rs`
- **Severity:** Low
- **Detail:** `priority` is `String` on `Quest`, `CreateQuestRequest`, `UpdateQuestRequest`, and `QuestListParams`. Any arbitrary string (including empty string) is accepted without validation. The `QuestStatus` enum pattern is already established in this PR. Introduce `QuestPriority { Low, Medium, High }` with `#[derive(sqlx::Type, serde::Deserialize, serde::Serialize)]`.
- **Status:** ✅ Task created
- **Resolution:** Added as S013 in Steps.md Phase 3.

#### [TASK] P6-015: `Routine.status: String` — introduce `RoutineStatus` enum
- **File:** `apps/server/server/src/guidance/routines/models.rs`
- **Severity:** Low
- **Detail:** `status: String` accepts any value despite having only two valid states (`active`, `paused`). Introduce `RoutineStatus { Active, Paused }` to match the pattern used by `QuestStatus` and `EpicStatus` elsewhere in this PR.
- **Status:** ✅ Task created
- **Resolution:** Added as S014 in Steps.md Phase 4.

#### [TASK] P6-019: Focus session creation on `completed`/`cancelled` quest silently allowed
- **File:** `apps/server/server/src/guidance/focus_sessions/service.rs` — `create_session`
- **Severity:** Low
- **Detail:** `create_session` branches only on `quest_status == "not_started"`. A quest in `completed` or `cancelled` status will have a focus session inserted with no error and no quest status change. This behavior is neither documented nor tested. The product should decide: return 422 (can't focus on a finished quest) or allow it (user is reviewing finished work). Add a test to document whichever behavior is chosen.
- **Status:** ✅ Task created
- **Resolution:** Added as S017-T in Steps.md Phase 4.

---

### Architectural Concerns

#### [ADR] P6-005: COALESCE partial update pattern cannot clear nullable fields
- **File:** `apps/server/server/src/guidance/quests/service.rs`, `daily_checkins/service.rs`, `routines/service.rs`, `epics/service.rs`, `focus_sessions/service.rs`
- **Severity:** High
- **Detail:** All `update_*` functions use `COALESCE($N, column)` for partial updates. When a client sends `null` for a nullable field (JSON `null` → serde `None`), `COALESCE(NULL, column)` silently preserves the existing value. A user cannot clear `due_date`, `description`, `initiative_id`, `epic_id`, `mood`, or `notes` through a PATCH. The standard Rust fix is `Option<Option<T>>` with `serde_with::double_option` for nullable-optional update fields, combined with a SQL pattern that handles the three-way distinction (absent / explicit-null / value).
- **Relates to:** OQ-005 (update semantics)
- **Status:** ✅ ADR created
- **Resolution:** ADR-019 — PATCH semantics are set-only for v1; clearing nullable fields deferred until a clear user need emerges.

#### [ADR] P6-016: `days_of_week: Vec<String>` accepts invalid weekday names
- **File:** `apps/server/server/src/guidance/routines/models.rs`
- **Severity:** Low
- **Detail:** `FrequencyConfig::Weekly { days_of_week }` accepts any string. The non-empty check in `validate()` catches `vec![]` but not `vec!["banana"]`. A `DayOfWeek` enum (`Monday`..`Sunday`) would make invalid values unrepresentable. This is an ADR concern because it changes the serialization contract stored in JSONB and requires a migration consideration for any existing data.
- **Status:** ✅ ADR created
- **Resolution:** ADR-020 — defer `DayOfWeek` enum until S014 (`RoutineStatus` enum task); no migration needed since `#[serde(rename_all = "snake_case")]` produces identical JSON.

---

## Resolution Checklist
- [x] All [FIX] findings resolved
- [x] All [TASK] findings added to Steps.md
- [x] All [ADR] findings have ADRs created or dismissed
- [x] All [RULE] findings applied or dismissed
- [x] Review verified by review-verify agent

## Resolution Summary
**Resolved at:** 2026-04-15
**Session:** review-resolve for PR feat/guidance-domain

| Category | Total | Resolved |
|---|---|---|
| [FIX] | 11 | 11 |
| [TASK] | 6 | 6 |
| [ADR] | 2 | 2 |
| [RULE] | 0 | 0 |
| **Total** | **19** | **19** |
