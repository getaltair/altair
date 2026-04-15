# Implementation Steps: Guidance Domain (Server)

**Spec:** Context/Features/005-GuidanceDomain/Spec.md
**Tech:** Context/Features/005-GuidanceDomain/Tech.md

## Progress
- **Status:** Complete
- **Current task:** --
- **Last milestone:** Validation complete — ready for commit

---

## Team Orchestration

### Team Members

- **builder**
  - Role: Implement all Rust/Axum guidance module code
  - Agent Type: backend-engineer
  - Resume: true

- **validator**
  - Role: Read-only quality validation of completed feature
  - Agent Type: quality-engineer
  - Resume: false

---

## Tasks

### Phase 1: Foundation

Cross-cutting setup that all entity modules depend on.

- [ ] S001: Add `impl From<sqlx::Error> for AppError` to `src/error.rs`; add `pub mod guidance;` to `src/lib.rs`; create `src/guidance/mod.rs` with an empty `pub fn router() -> Router<AppState>` that merges no routes yet; wire `.merge(guidance::router())` into `src/main.rs`
  - **Assigned:** builder
  - **Depends:** none
  - **Parallel:** false

> No test task for S001 — structural wiring only, no new behavior. The existing `cargo build` and `cargo test` passing after this task is the verification.

🏁 **MILESTONE: Foundation complete**
Verify: `cargo build` succeeds; existing test suite passes unchanged.

**Contracts:**
- `apps/server/server/src/error.rs` — `From<sqlx::Error> for AppError` impl; all guidance modules use `?` on sqlx queries
- `apps/server/server/src/guidance/mod.rs` — top-level guidance router; add sub-routers here as modules complete

---

### Phase 2: Epics + Daily Check-ins (parallel)

Two independent modules with no inter-dependency. Epics requires the initiative FK ownership check; daily check-ins requires the unique-per-day constraint. Neither depends on quests or routines.

- [ ] S002: Implement `src/guidance/epics/` module — `models.rs` (Epic, CreateEpicRequest, UpdateEpicRequest), `service.rs` (list by initiative, get, create with initiative ownership check, update, soft-delete), `handlers.rs` (thin Axum handlers), `mod.rs` (router at `/api/guidance/epics`)
  - **Assigned:** builder
  - **Depends:** S001
  - **Parallel:** true

- [ ] S002-T: Test epics service — (create epic with valid initiative_id succeeds; create with initiative owned by other user returns Forbidden; get epic with wrong user_id returns NotFound; soft-delete excludes row from list; partial update leaves unspecified fields unchanged)
  - **Assigned:** builder
  - **Depends:** S002
  - **Parallel:** true

- [ ] S003: Implement `src/guidance/daily_checkins/` module — `models.rs` (DailyCheckin, CreateCheckinRequest, UpdateCheckinRequest), `service.rs` (list, get, create with unique-per-day enforcement returning Conflict on duplicate, update, soft-delete), `handlers.rs`, `mod.rs` (router at `/api/guidance/daily-checkins`)
  - **Assigned:** builder
  - **Depends:** S001
  - **Parallel:** true

- [ ] S003-T: Test daily checkins service — (create check-in for a given date succeeds; second create for same user + same date returns Conflict; create for same user different dates both succeed; get returns NotFound for wrong user; update modifies energy_level correctly)
  - **Assigned:** builder
  - **Depends:** S003
  - **Parallel:** true

🏁 **MILESTONE: Phase 2 complete — Epics + Daily check-ins**
Verify assertions: A-G-01, A-G-02 (epics), A-G-12 (daily check-in duplicate).
`cargo test` passes; `cargo clippy` zero warnings.

**Contracts:**
- `apps/server/server/src/guidance/epics/models.rs` — Epic struct, request types
- `apps/server/server/src/guidance/epics/service.rs` — `get_initiative_ownership_check` pattern reused by quests

---

### Phase 3: Quests

The most complex module. Depends on Epics (epic status update on transition) and the initiative ownership pattern established in Phase 2.

- [ ] S004: Implement `src/guidance/quests/models.rs` — `QuestStatus` enum with `can_transition_to()` method per `06-state-machines.md`; `Quest`, `CreateQuestRequest`, `UpdateQuestRequest`, `QuestListParams` (filter fields: status, priority, due_date, initiative_id)
  - **Assigned:** builder
  - **Depends:** S002-T
  - **Parallel:** false

- [ ] S004-T: Unit-test `QuestStatus::can_transition_to()` — (not_started→in_progress allowed; in_progress→completed allowed; in_progress→deferred allowed; deferred→not_started allowed; any→cancelled allowed; not_started→completed rejected; completed→not_started rejected; deferred→completed rejected)
  - **Assigned:** builder
  - **Depends:** S004
  - **Parallel:** false

- [ ] S012-T: Add missing forbidden-transition and terminal-state tests to `src/guidance/quests/models.rs` — (not_started→deferred rejected; in_progress→not_started rejected; deferred→in_progress rejected; completed→cancelled rejected); relates to A-G-05, A-G-06 (P6-006)
  - **Assigned:** builder
  - **Depends:** S004-T
  - **Parallel:** false

- [ ] S013: Introduce `QuestPriority` enum (`Low`, `Medium`, `High`) in `src/guidance/quests/models.rs` with `#[derive(sqlx::Type, Serialize, Deserialize)]`; replace `priority: String` fields on `Quest`, `CreateQuestRequest`, `UpdateQuestRequest`, and `QuestListParams`; add `#[sqlx(type_name = "varchar", rename_all = "snake_case")]` and `#[serde(rename_all = "snake_case")]` (P6-014)
  - **Assigned:** builder
  - **Depends:** S004
  - **Parallel:** false

- [ ] S005: Implement `src/guidance/quests/service.rs` — list (with filter params), get, create (initiative ownership check per E-3), update (transition guard → UnprocessableEntity on invalid transition; epic status recalculation inside sqlx transaction when quest has epic_id; QuestCompleted tracing event on completion), soft-delete
  - **Assigned:** builder
  - **Depends:** S004-T
  - **Parallel:** false

- [ ] S005-T: Integration-test quest service — (create quest returns row with correct user_id; create quest with initiative_id from other user returns Forbidden; list returns only calling user's quests — A-G-03; PATCH status not_started→in_progress succeeds — A-G-04; PATCH status not_started→completed returns UnprocessableEntity — A-G-05; PATCH status completed→not_started returns UnprocessableEntity — A-G-06; PATCH initiative_id to other user's initiative returns Forbidden — A-G-07; quest completion with epic_id updates epic status in same transaction; soft-delete excludes row from subsequent list — A-G-13; list?due_date=today returns only due today and non-terminal status — A-G-15)
  - **Assigned:** builder
  - **Depends:** S005
  - **Parallel:** false

- [ ] S006: Implement `src/guidance/quests/handlers.rs` and `mod.rs` (router at `/api/guidance/quests`); register quests sub-router in `src/guidance/mod.rs`
  - **Assigned:** builder
  - **Depends:** S005-T
  - **Parallel:** false

🏁 **MILESTONE: Phase 3 complete — Quests**
Verify assertions: A-G-03, A-G-04, A-G-05, A-G-06, A-G-07, A-G-13, A-G-14 (QuestCompleted in tracing output), A-G-15.
`cargo test` passes; `cargo clippy` zero warnings.

**Contracts:**
- `apps/server/server/src/guidance/quests/models.rs` — Quest, QuestStatus, request types; used by routines (spawn) and focus sessions (auto-transition)
- `apps/server/server/src/guidance/quests/service.rs` — `create_quest` signature used by routine spawn

---

### Phase 4: Routines + Focus Sessions (parallel)

Both depend on the quest module established in Phase 3. Routines spawn quests; focus sessions auto-transition quests.

- [ ] S007: Implement `src/guidance/routines/` module — `models.rs` (FrequencyType enum, FrequencyConfig typed struct with `daily`/`weekly`/`interval` variants, Routine, CreateRoutineRequest, UpdateRoutineRequest); `service.rs` (list, get, create with frequency validation per E-4 returning UnprocessableEntity on zero-occurrence config + spawn first occurrence, update, soft-delete, `spawn` function checking existing quests by routine_id+due_date for idempotency); `handlers.rs`; `mod.rs` with routes at `/api/guidance/routines` and `POST /api/guidance/routines/:id/spawn`
  - **Assigned:** builder
  - **Depends:** S006
  - **Parallel:** true

- [ ] S007-T: Integration-test routines service — (create routine with weekly frequency and empty days returns UnprocessableEntity — A-G-08; create valid daily routine succeeds and spawns first quest with routine_id set — A-G-09; calling spawn twice for same occurrence does not duplicate the quest; routine soft-delete excludes from list; frequency_config JSONB round-trips correctly through serde)
  - **Assigned:** builder
  - **Depends:** S007
  - **Parallel:** true

- [ ] S014: Introduce `RoutineStatus` enum (`Active`, `Paused`) in `src/guidance/routines/models.rs` with `#[derive(sqlx::Type, Serialize, Deserialize)]`; replace `status: String` on `Routine` to match the `QuestStatus`/`EpicStatus` pattern (P6-015)
  - **Assigned:** builder
  - **Depends:** S007
  - **Parallel:** true

- [ ] S015-T: Add cross-user isolation test for routines service — `get_routine(pool, routine.id, other_user_id)` must return `NotFound`; mirrors pattern in epics, quests, focus sessions, and daily checkins modules (P6-007)
  - **Assigned:** builder
  - **Depends:** S007-T
  - **Parallel:** true

- [ ] S008: Implement `src/guidance/focus_sessions/` module — `models.rs` (FocusSession, CreateFocusSessionRequest, UpdateFocusSessionRequest with optional ended_at); `service.rs` (list by quest_id, get, create with quest auto-transition to in_progress if not_started, update with server-computed duration_minutes when ended_at provided, soft-delete); `handlers.rs`; `mod.rs` (router at `/api/guidance/focus-sessions`)
  - **Assigned:** builder
  - **Depends:** S006
  - **Parallel:** true

- [ ] S008-T: Integration-test focus sessions service — (create session on not_started quest auto-transitions quest to in_progress — A-G-10; PATCH with ended_at computes and persists duration_minutes correctly — A-G-11; create session on already in_progress quest leaves quest status unchanged; duration_minutes = floor((ended_at - started_at) / 60 seconds); soft-delete excludes from list)
  - **Assigned:** builder
  - **Depends:** S008
  - **Parallel:** true

- [ ] S016-T: Strengthen A-G-14 QuestCompleted event test — current test verifies 200 response only; add `tracing_test::traced_test` subscriber to assert `tracing::info!` with `quest_id` and message `QuestCompleted` fires when quest transitions to completed; if `tracing_test` crate is unavailable, promote `QuestCompleted` to a structured `DomainEvent` type assertable in unit tests (P6-008)
  - **Assigned:** builder
  - **Depends:** S010
  - **Parallel:** false

- [ ] S017-T: Document and test focus session creation on completed/cancelled quests — decide and enforce one of: (a) return 422 UnprocessableEntity (quests in terminal status cannot be focused), (b) allow and document explicitly; add a test that captures the chosen behavior so a future change is visible (P6-019)
  - **Assigned:** builder
  - **Depends:** S008-T
  - **Parallel:** true

- [ ] S009: Register routines and focus_sessions sub-routers in `src/guidance/mod.rs`; register epics and daily_checkins sub-routers (complete the guidance router)
  - **Assigned:** builder
  - **Depends:** S007-T, S008-T
  - **Parallel:** false

🏁 **MILESTONE: Phase 4 complete — all 5 modules wired**
Verify assertions: A-G-08, A-G-09 (routines), A-G-10, A-G-11 (focus sessions).
`cargo build` succeeds; `cargo test` passes; `cargo clippy` zero warnings.

**Contracts:**
- `apps/server/server/src/guidance/mod.rs` — complete router merging all 5 sub-routers; ready for integration test

---

### Phase 5: HTTP Integration Tests + Documentation

End-to-end HTTP-layer tests through the full Axum router. These test the handler → service → DB path and cover the assertions not already covered by service-layer unit tests.

- [ ] S010: Write `apps/server/server/tests/guidance_integration.rs` — HTTP-level integration tests using the full `build_app` pattern from `tests/auth_integration.rs`; cover remaining assertion surface: A-G-01 (201 on epic create), A-G-02 (403 on wrong-user initiative), A-G-03 (quest list isolation), A-G-12 (409 on duplicate check-in), A-G-14 (QuestCompleted tracing event captured)
  - **Assigned:** builder
  - **Depends:** S009
  - **Parallel:** false

- [ ] S010-T: Run full test suite and confirm all 15 spec assertions pass — (`CARGO_TARGET_DIR=/tmp/cargo-target cargo test` green; no warnings in clippy; verify each A-G-## maps to a passing test case)
  - **Assigned:** builder
  - **Depends:** S010
  - **Parallel:** false

- [ ] S010-D: Update `CLAUDE.md` active work section — mark Feature 004 Sync Engine complete, mark Feature 005 Guidance Domain in progress; update route table in any API reference docs if they exist
  - **Assigned:** builder
  - **Depends:** S010
  - **Parallel:** true

🏁 **MILESTONE: Feature complete — all assertions verified, full drift check**
Verify all 15 assertions A-G-01 through A-G-15.
`CARGO_TARGET_DIR=/tmp/cargo-target cargo test` fully green.
`cargo clippy -- -D warnings` zero diagnostics.
No TODO/FIXME stubs remaining.

---

### Phase 6: Validation

- [ ] S011: Read-only validation — inspect all 5 guidance modules against Spec.md requirements and Tech.md decisions; verify `From<sqlx::Error>` impl is present and inline `.map_err` not introduced; confirm quest transition guard matches `06-state-machines.md`; confirm epic status update is inside a transaction; confirm `duration_minutes` is server-computed; confirm spawn endpoint is idempotent; confirm daily checkin returns 409 on duplicate
  - **Assigned:** validator
  - **Depends:** S010-T
  - **Parallel:** false

🏁 **MILESTONE: Validation complete — ready for commit**

---

## Acceptance Criteria

- [ ] All 15 testable assertions from Spec.md verified (A-G-01 through A-G-15)
- [ ] `CARGO_TARGET_DIR=/tmp/cargo-target cargo test` fully green
- [ ] `cargo clippy -- -D warnings` zero diagnostics
- [ ] `impl From<sqlx::Error> for AppError` present in `error.rs`; no inline `.map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))` in guidance modules
- [ ] Quest status transitions enforced by `QuestStatus::can_transition_to()`; invalid transitions return 422
- [ ] Epic status update wrapped in sqlx transaction when quest has `epic_id`
- [ ] `duration_minutes` computed server-side from `started_at` / `ended_at`; not accepted from client
- [ ] Routine spawn endpoint is idempotent (no duplicate quests for same routine + due_date)
- [ ] Daily check-in returns 409 on duplicate `(user_id, checkin_date)`
- [ ] No TODO/FIXME stubs remaining in guidance modules
- [ ] `CLAUDE.md` active work section updated

## Validation Commands

```bash
# Build
CARGO_TARGET_DIR=/tmp/cargo-target cargo build --manifest-path apps/server/Cargo.toml

# Full test suite
CARGO_TARGET_DIR=/tmp/cargo-target cargo test --manifest-path apps/server/Cargo.toml

# Lint (zero warnings required)
CARGO_TARGET_DIR=/tmp/cargo-target cargo clippy --manifest-path apps/server/Cargo.toml -- -D warnings

# Guidance-specific tests only
CARGO_TARGET_DIR=/tmp/cargo-target cargo test --manifest-path apps/server/Cargo.toml guidance
```
