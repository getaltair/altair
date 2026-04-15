# Feature 005: Guidance Domain (Server)

| Field | Value |
|---|---|
| **Feature** | 005-GuidanceDomain |
| **Phase** | Step 5 of PLAN-001-v1 |
| **Status** | Spec — Draft |
| **Last Updated** | 2026-04-15 |
| **Source Docs** | `docs/specs/01-PRD-002-guidance.md`, `docs/specs/03-invariants.md`, `docs/specs/05-erd.md`, `docs/specs/06-state-machines.md` |

---

## Overview

This feature implements the server-side REST API for the Guidance domain — the motivational backbone of Altair that connects long-term goals to daily actionable work. It adds five new entity modules to the Axum server: Epics, Quests, Routines, Focus Sessions, and Daily Check-ins. All database tables and sync rules are already in place from prior steps; this feature delivers the API layer.

---

## Problem Statement

Steps 1–4 established the server skeleton, auth, core domain (initiatives, tags, relations), and the sync engine. The Guidance domain tables were migrated and the PowerSync sync bucket was defined, but no API endpoints exist to create or manage Guidance entities. Until this feature ships, clients cannot build the planning and daily execution workflows that are the primary reason users adopt Altair.

---

## User Stories

- As a user, I want to create and manage Epics within my Initiatives so that I can break large goals into coherent milestone phases.
- As a user, I want to create Quests with status, priority, and due date so that I have discrete, actionable work items.
- As a user, I want Quest status transitions enforced server-side so that my progress data stays consistent across devices.
- As a user, I want to define Routines with a frequency so that recurring habits automatically generate Quests.
- As a user, I want to start and end Focus Sessions tied to a Quest so that my focused effort is tracked.
- As a user, I want to submit a Daily Check-in once per day so that I can reflect on energy and mood.

---

## Requirements

### Must Have

- **Epic CRUD**: Create, read (list + get), update, and soft-delete Epics scoped to an Initiative and the authenticated user.
- **Quest CRUD**: Create, read (list + get), update, and soft-delete Quests with status, priority, optional due date, and optional epic/initiative/routine assignment.
- **Quest status transitions**: Updates to quest status must be validated against the allowed state machine (`not_started` → `in_progress` → `completed` | `deferred` | `cancelled`); invalid transitions are rejected with a 422.
- **Quest initiative ownership**: If a Quest references an `initiative_id`, that initiative must be owned by the same user (invariant E-3); reject with 403 otherwise.
- **Routine CRUD**: Create, read, update, and soft-delete Routines with `frequency_type` and `frequency_config`; frequency must produce at least one occurrence per period (invariant E-4).
- **Routine → Quest spawning**: Active routines spawn Quests per their frequency configuration; spawned quests reference the routine via `routine_id`.
- **Focus Session CRUD**: Create (start), read, update (end with `ended_at`), and soft-delete Focus Sessions tied to a Quest.
- **Daily Check-in CRUD**: Create, read, and update Daily Check-ins; enforce unique-per-user-per-date (one check-in per calendar day per user).
- **Domain events**: Emit `QuestCompleted` when a quest transitions to `completed`; emit `RoutineDue` when a routine's next occurrence is reached.
- **User isolation**: All endpoints scope queries to the authenticated user's `user_id`; no cross-user data leakage (invariant SEC-1).
- **Auth middleware**: All guidance endpoints are protected by the existing JWT auth middleware (invariant SEC-2).

### Should Have

- **Focus Session auto-transition**: Starting a Focus Session on a `not_started` Quest automatically transitions it to `in_progress`.
- **Epic status derivation**: Epic status is derived from child quest states (`not_started` until first quest starts; `in_progress` until all quests done; `completed` when all quests are `completed` or `cancelled`) — consistent with the state machine in `06-state-machines.md`.
- **Quest list filtering**: List quests by `status`, `priority`, `due_date` (today's quests), and `initiative_id` to support the Today view and initiative tree.
- **Pagination**: List endpoints return paginated responses (cursor or offset-based, consistent with the existing core module pattern).

### Won't Have (this iteration)

- Client-side UI (Today view, offline completion) — Steps 8 and 9.
- Push notification triggers for `RoutineDue` — Step 11.
- AI-suggested quest extraction — v1.1 candidate (PRD OQ-G-1, G-G-11).
- Streak tracking for routines — v1.1 candidate (PRD G-G-13).
- Routine spawning scheduler/cron worker — the spawning logic is implemented in the server API; scheduled execution is a Step 11 concern.

---

## Testable Assertions

| ID | Assertion | Verification |
|---|---|---|
| A-G-01 | `POST /guidance/epics` with a valid `initiative_id` owned by the authenticated user returns 201 with the created epic | Integration test |
| A-G-02 | `POST /guidance/epics` with an `initiative_id` belonging to a different user returns 403 | Integration test |
| A-G-03 | `POST /guidance/quests` returns 201; the quest appears in `GET /guidance/quests` for the same user and is absent from another user's list | Integration test |
| A-G-04 | `PATCH /guidance/quests/:id` with `status: "in_progress"` on a `not_started` quest succeeds (200) | Integration test |
| A-G-05 | `PATCH /guidance/quests/:id` with `status: "completed"` on a `not_started` quest (skipping `in_progress`) returns 422 | Integration test |
| A-G-06 | `PATCH /guidance/quests/:id` with `status: "not_started"` on a `completed` quest (backward transition) returns 422 | Integration test |
| A-G-07 | `PATCH /guidance/quests/:id` with `initiative_id` referencing an initiative owned by another user returns 403 | Integration test |
| A-G-08 | `POST /guidance/routines` with a `frequency_type: "weekly"` and `frequency_config` specifying zero days returns 422 (invalid frequency — invariant E-4) | Integration test |
| A-G-09 | `POST /guidance/routines` with a valid frequency config returns 201; calling the spawn endpoint produces at least one Quest with `routine_id` set | Integration test |
| A-G-10 | `POST /guidance/focus-sessions` on a `not_started` quest transitions the quest to `in_progress` (auto-transition) | Integration test |
| A-G-11 | `PATCH /guidance/focus-sessions/:id` with `ended_at` set computes and persists `duration_minutes` | Integration test |
| A-G-12 | `POST /guidance/daily-checkins` for a given date succeeds (201); a second `POST` for the same user and same date returns 409 | Integration test |
| A-G-13 | `DELETE /guidance/quests/:id` sets `deleted_at` and excludes the quest from subsequent list responses | Integration test |
| A-G-14 | Completing a quest (transitioning to `completed`) emits a `QuestCompleted` domain event observable in the event log | Integration test |
| A-G-15 | `GET /guidance/quests?due_date=today` returns only quests with `due_date = current_date` and status not in `completed`, `cancelled` for the authenticated user | Integration test |

---

## Dependencies

| Dependency | Status | Notes |
|---|---|---|
| Database migrations (guidance tables) | Done — Step 3 | Migrations 12–16 in `infra/migrations/` |
| PowerSync sync rules (guidance bucket) | Done — Step 4 | Defined in `infra/compose/sync_rules.yaml` |
| Axum server skeleton + AppState | Done — Step 3 | `apps/server/server/src/lib.rs` |
| JWT auth middleware | Done — Step 3 | `src/auth/extractor.rs` |
| Initiative ownership (core module) | Done — Step 3 | `src/core/initiatives/service.rs` — used for E-3 validation |
| Error handling (`AppError`) | Done — Step 3 | `src/error.rs` |
| Sync push endpoint | Done — Step 4 | `src/sync/` — guidance mutations flow through this; no changes needed here |

---

## Integration Points

| System | Interface | Notes |
|---|---|---|
| Core / Initiatives | `initiative_id` FK + ownership check | Quests and Epics reference initiatives; E-3 validation calls initiative service |
| Core / Relations | `entity_relations` table | Cross-domain links (quest ↔ note) via existing relations module; not implemented in this feature |
| Core / Tags | `quest_tags` junction table | Tag assignment for quests; not in scope for this feature — Step 10 or later |
| Sync Engine | `/sync/push` MutationEnvelope | Guidance mutations (from clients) arrive through the existing sync push path |
| Notifications | `QuestCompleted`, `RoutineDue` events | Events emitted but delivery not implemented until Step 11 |

---

## Open Questions

- [ ] **OQ-005-1**: Epic status — should it be stored explicitly (with server updates triggered on quest transitions) or derived at query time? The state machine doc notes this is INFERRED. Decision needed before implementation.
- [ ] **OQ-005-2**: Routine spawning trigger — should the `POST /guidance/routines/spawn` endpoint be a manual trigger (called by worker/scheduler) or should spawning happen inline on a relevant API call (e.g., when listing today's quests)? The scheduler worker is Step 11 scope.
- [ ] **OQ-005-3**: Focus session `duration_minutes` — computed on `ended_at` set, or stored by client? Server should compute it to prevent client manipulation; confirm.

---

## Revision History

| Date | Change | ADR |
|---|---|---|
| 2026-04-15 | Initial spec | — |
