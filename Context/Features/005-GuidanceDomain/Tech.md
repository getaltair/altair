# Tech Plan: Guidance Domain (Server)

**Spec:** `Context/Features/005-GuidanceDomain/Spec.md`
**Stacks involved:** Rust / Axum (server only)

---

## Architecture Overview

The Guidance domain follows the same structure as `src/core/` — one subdirectory per entity, each with `mod.rs` (router), `models.rs` (request/response types), `service.rs` (DB queries + business logic), and `handlers.rs` (thin Axum handlers). A new top-level `src/guidance/` module is added and registered in `main.rs` alongside the existing core and sync routers.

All five tables (`guidance_epics`, `guidance_quests`, `guidance_routines`, `guidance_focus_sessions`, `guidance_daily_checkins`) and the PowerSync `guidance` sync bucket are already provisioned. No schema or sync rule changes are needed.

### Module layout

```
apps/server/server/src/
└── guidance/
    ├── mod.rs                  ← top-level router (merges submodule routers)
    ├── epics/
    │   ├── mod.rs
    │   ├── models.rs
    │   ├── service.rs
    │   └── handlers.rs
    ├── quests/
    │   ├── mod.rs
    │   ├── models.rs
    │   ├── service.rs          ← includes transition validation + epic status update
    │   └── handlers.rs
    ├── routines/
    │   ├── mod.rs
    │   ├── models.rs
    │   ├── service.rs          ← includes frequency validation + spawn logic
    │   └── handlers.rs
    ├── focus_sessions/
    │   ├── mod.rs
    │   ├── models.rs
    │   ├── service.rs          ← includes duration_minutes computation
    │   └── handlers.rs
    └── daily_checkins/
        ├── mod.rs
        ├── models.rs
        ├── service.rs          ← includes unique-per-day enforcement
        └── handlers.rs
```

Registration in `src/main.rs`:
```rust
.merge(guidance::router())
```

---

## Key Decisions

### Decision 1: Epic status — stored explicitly, updated on quest transitions

**Options considered:**
- **Option A: Derived at query time** — JOIN guidance_quests on epic_id, compute status from child states. No stored column updates needed. Simple service layer; more expensive query.
- **Option B: Stored explicitly, updated by quest service** — quest service updates the parent epic's status column whenever a quest transitions. Consistent with the DB schema (which already has `status` on `guidance_epics`). O(1) status reads.

**Chosen:** Option B — stored explicitly.

**Rationale:** The ERD schema already defines a `status VARCHAR(20)` column on `guidance_epics`. Leaving it unused in favor of derived logic would diverge from the established DB design. The query pattern (list epics within an initiative) reads status far more often than quests transition. Stored status makes list queries simple. The coupling (quest service touches epic table) is acceptable given they are co-located in `src/guidance/`.

The update rule, per `06-state-machines.md`:
- First quest under an epic moves to `in_progress` → epic transitions `not_started` → `in_progress`
- All quests under an epic are `completed` or `cancelled` → epic transitions to `completed`
- No backward transitions for epics.

**Related ADRs:** None existing. This resolves OQ-005-1.

---

### Decision 2: Routine spawning — explicit endpoint, not inline

**Options considered:**
- **Option A: Inline, on quest-list request** — spawn missing quests when `GET /guidance/quests?due_date=today` is called. No separate endpoint needed.
- **Option B: Explicit `POST /guidance/routines/:id/spawn` endpoint** — caller (future worker, test, admin) triggers spawning explicitly. Returns the list of spawned quests.
- **Option C: Spawn-on-create only** — spawn first occurrence when the routine is created; scheduler handles subsequent occurrences.

**Chosen:** Option B — explicit spawn endpoint, plus spawn-on-create for the first occurrence.

**Rationale:** Inline spawning (Option A) mutates state inside a GET handler, which violates HTTP semantics and makes the quest-list endpoint non-idempotent. Option C alone leaves subsequent spawning with no mechanism until Step 11. Option B is the cleanest interface for a Step 11 worker to call and is testable in isolation. Spawn-on-create covers the common case of "start a routine, get today's quest immediately."

The spawn endpoint computes which occurrence(s) are due (based on `frequency_type` / `frequency_config` and existing `guidance_quests.routine_id` rows) and inserts only the missing ones. Idempotent — calling it twice for the same occurrence does nothing.

**Related ADRs:** None existing. This resolves OQ-005-2.

---

### Decision 3: Focus session `duration_minutes` — server-computed

**Options considered:**
- **Option A: Client-supplied** — client sends `duration_minutes` in the PATCH body alongside `ended_at`.
- **Option B: Server-computed** — server computes `duration_minutes = floor((ended_at - started_at) / 60 seconds)` when `ended_at` is set. Client provides only `ended_at`.

**Chosen:** Option B — server-computed.

**Rationale:** Client-supplied values can be manipulated or drift due to clock skew. PRD assertion A-015 states that "Focus session duration is accurately recorded even if the app is backgrounded" — backgrounded apps cannot reliably track their own elapsed time, but a server can compute it from the stored `started_at` and the submitted `ended_at`. Server computation is also simpler for clients (no calculation needed). `ended_at` is provided by the client (the client knows when the user stopped); the server derives `duration_minutes` from it. This resolves OQ-005-3.

**Related ADRs:** None existing.

---

### Decision 4: Quest status transition enforcement — service layer enum method

**Pattern (already documented in `06-state-machines.md`):**

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
pub enum QuestStatus {
    NotStarted,
    InProgress,
    Completed,
    Deferred,
    Cancelled,
}

impl QuestStatus {
    pub fn can_transition_to(&self, target: &QuestStatus) -> bool {
        matches!(
            (self, target),
            (Self::NotStarted, Self::InProgress)
                | (Self::InProgress, Self::Completed)
                | (Self::InProgress, Self::Deferred)
                | (Self::Deferred, Self::NotStarted)
                | (_, Self::Cancelled)
        )
    }
}
```

Invalid transitions return `AppError::UnprocessableEntity` (422). The spec's state machine `06-state-machines.md` is the authoritative source; the `can_transition_to` method encodes it exactly. This logic lives in `src/guidance/quests/service.rs`, not the handler.

The same pattern applies to `EpicStatus`, `RoutineStatus`, and `FocusSessionStatus` — each gets an enum with a transition guard.

---

### Decision 5: `From<sqlx::Error> for AppError` — implement once, use `?` everywhere

The existing `src/core/initiatives/service.rs` uses the inline pattern `.map_err(|e| AppError::Internal(anyhow::anyhow!(e.to_string())))`. This violates the `rust-axum.md` convention which explicitly bans it (`.to_string()` flattens the error chain; inline form is a maintenance hazard).

The guidance module is new code. It will implement the convention correctly:

```rust
// In src/error.rs — add once:
impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Internal(anyhow::Error::from(e))
    }
}
```

All guidance service functions then use `?` directly on sqlx queries, with no inline `.map_err`. The `From` impl is added to `error.rs` once and benefits all future modules.

> **Note:** The existing `core/initiatives/service.rs` uses the old inline pattern. The guidance module does not touch that code (per CLAUDE.md surgical-changes rule), but this `From` impl in `error.rs` is a one-time addition that doesn't break anything existing.

---

## Stack-Specific Details

### Rust / Axum (`apps/server/server/`)

**Files to create:**
- `src/guidance/mod.rs` — top-level guidance module; merges 5 sub-routers
- `src/guidance/epics/{mod,models,service,handlers}.rs`
- `src/guidance/quests/{mod,models,service,handlers}.rs`
- `src/guidance/routines/{mod,models,service,handlers}.rs`
- `src/guidance/focus_sessions/{mod,models,service,handlers}.rs`
- `src/guidance/daily_checkins/{mod,models,service,handlers}.rs`
- `tests/guidance_integration.rs` — integration tests covering all 15 spec assertions

**Files to modify:**
- `src/lib.rs` — add `pub mod guidance;`
- `src/main.rs` — add `.merge(guidance::router())`
- `src/error.rs` — add `impl From<sqlx::Error> for AppError`

**Patterns to follow:**
- `.claude/rules/rust-axum.md` — module structure, error handling, async safety
- `src/core/initiatives/` — exact structural template for new modules
- `#[sqlx::test(migrations = "../../../infra/migrations")]` for integration tests (see `tests/auth_integration.rs`)

**Dependencies:** No new crates needed. `chrono`, `uuid`, `sqlx`, `serde`, `axum`, `anyhow` are already in `Cargo.toml`.

### Route table

| Method | Path | Handler | Notes |
|---|---|---|---|
| GET | `/api/guidance/epics` | `epics::handlers::list` | Scoped by `initiative_id` query param |
| POST | `/api/guidance/epics` | `epics::handlers::create` | initiative ownership check |
| GET | `/api/guidance/epics/:id` | `epics::handlers::get_one` | |
| PATCH | `/api/guidance/epics/:id` | `epics::handlers::update` | |
| DELETE | `/api/guidance/epics/:id` | `epics::handlers::delete` | Soft delete |
| GET | `/api/guidance/quests` | `quests::handlers::list` | Filter: `status`, `priority`, `due_date`, `initiative_id` |
| POST | `/api/guidance/quests` | `quests::handlers::create` | initiative ownership check (E-3) |
| GET | `/api/guidance/quests/:id` | `quests::handlers::get_one` | |
| PATCH | `/api/guidance/quests/:id` | `quests::handlers::update` | transition guard + epic status update |
| DELETE | `/api/guidance/quests/:id` | `quests::handlers::delete` | Soft delete |
| GET | `/api/guidance/routines` | `routines::handlers::list` | |
| POST | `/api/guidance/routines` | `routines::handlers::create` | frequency validation (E-4) + spawn first occurrence |
| GET | `/api/guidance/routines/:id` | `routines::handlers::get_one` | |
| PATCH | `/api/guidance/routines/:id` | `routines::handlers::update` | |
| DELETE | `/api/guidance/routines/:id` | `routines::handlers::delete` | Soft delete |
| POST | `/api/guidance/routines/:id/spawn` | `routines::handlers::spawn` | Idempotent; returns spawned quests |
| GET | `/api/guidance/focus-sessions` | `focus_sessions::handlers::list` | Filter by `quest_id` |
| POST | `/api/guidance/focus-sessions` | `focus_sessions::handlers::create` | Quest auto-transition to `in_progress` |
| GET | `/api/guidance/focus-sessions/:id` | `focus_sessions::handlers::get_one` | |
| PATCH | `/api/guidance/focus-sessions/:id` | `focus_sessions::handlers::update` | Compute `duration_minutes` on `ended_at` set |
| DELETE | `/api/guidance/focus-sessions/:id` | `focus_sessions::handlers::delete` | Soft delete |
| GET | `/api/guidance/daily-checkins` | `daily_checkins::handlers::list` | |
| POST | `/api/guidance/daily-checkins` | `daily_checkins::handlers::create` | Conflict on duplicate date (409) |
| GET | `/api/guidance/daily-checkins/:id` | `daily_checkins::handlers::get_one` | |
| PATCH | `/api/guidance/daily-checkins/:id` | `daily_checkins::handlers::update` | |

---

## Integration Points

| System | Interface | Notes |
|---|---|---|
| Core / Initiatives | `src/core/initiatives/service::get_initiative` | Quest and Epic creation checks initiative ownership via existing service function |
| AppError | `src/error.rs` | `From<sqlx::Error>` impl added here; all guidance modules use `?` |
| Auth middleware | `src/auth/extractor.rs` `AuthUser` | Extracted via existing extractor; no changes needed |
| Sync engine | `/sync/push` (existing) | Guidance mutations from clients flow through existing sync push path; no guidance-specific changes |
| Domain events | Internal in-process log (tracing) | `QuestCompleted` and `RoutineDue` emitted as structured tracing events for Step 11 to wire to notification delivery |

---

## Domain Event Strategy

`QuestCompleted` and `RoutineDue` are emitted in Step 5 as structured `tracing::info!` events with structured fields. Step 11 will replace these with real pub/sub or in-process channel delivery. This keeps Step 5 self-contained without premature infrastructure:

```rust
tracing::info!(
    event = "QuestCompleted",
    quest_id = %quest.id,
    user_id = %quest.user_id,
);
```

This satisfies spec assertion A-G-14 ("observable in the event log") and is trivially replaceable in Step 11.

---

## Risks & Unknowns

- **Risk: Epic status update adds a second write per quest transition**
  - If the quest belongs to an epic, the quest UPDATE must be followed by an epic status recalculation. These two writes should run in a transaction to prevent partial state.
  - **Mitigation:** Use `sqlx` transactions (`pool.begin()`) in the quest update service function when an `epic_id` is set.

- **Risk: Routine frequency validation edge cases**
  - `frequency_type: "custom"` with a JSONB config is open-ended. An invalid config (e.g. empty days array) must be caught before persist.
  - **Mitigation:** Define a `FrequencyConfig` enum/struct with typed variants for `daily`, `weekly` (with days vec), and `interval` (with period). Deserialize from request; reject if the config produces zero occurrences.

- **Risk: Spawn idempotency**
  - The spawn endpoint must not double-create quests if called twice for the same occurrence date.
  - **Mitigation:** Before inserting a spawned quest, check if a quest with the same `routine_id` and `due_date` already exists (non-deleted). If it does, skip it.

- **Unknown: `daily_checkins` timezone handling**
  - `checkin_date DATE` is server-local date. If a user submits from a different timezone, their "today" may differ from the server's.
  - **Resolution plan:** Accept `checkin_date` from the client (client knows their local date). The unique constraint is on `(user_id, checkin_date)`, so the client's date value is authoritative. Document this in the handler.

---

## Testing Strategy

All 15 spec assertions (A-G-01 through A-G-15) are covered by `sqlx::test` integration tests in `tests/guidance_integration.rs`. Each test uses `#[sqlx::test(migrations = "../../../infra/migrations")]` to run against a real PostgreSQL instance with fresh migrations, matching the pattern established in `tests/auth_integration.rs` and `tests/sync_push_integration.rs`.

Key test categories:
- **CRUD round-trips**: create → get → list → update → soft-delete for each entity
- **Transition enforcement**: valid and invalid quest status transitions (A-G-04, A-G-05, A-G-06)
- **Cross-user isolation**: each entity's list/get returns 403/404 for wrong user (A-G-03, A-G-07)
- **Business rules**: initiative ownership (A-G-02), daily checkin duplicate (A-G-12), invalid frequency (A-G-08)
- **Derived behavior**: focus session auto-transition (A-G-10), duration computation (A-G-11), routine spawn (A-G-09)

---

## Revision History

| Date | Change | ADR |
|---|---|---|
| 2026-04-15 | Initial tech plan | — |
