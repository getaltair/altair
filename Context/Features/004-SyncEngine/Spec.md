# Feature 004: Sync Engine

| Field | Value |
|---|---|
| **Feature** | 004-SyncEngine |
| **Version** | 1.0 |
| **Status** | Draft |
| **Date** | 2026-04-13 |
| **ADRs** | ADR-003 |
| **Source Docs** | `docs/specs/03-invariants.md`, `docs/specs/10-PLAN-001-v1.md` (Step 4), `Context/Decisions/ADR-003-sync-protocol-conflict-resolution.md` |

---

## Overview

Feature 004 delivers the server-side sync infrastructure that enables offline-first clients
(Android, Web) to synchronise with the PostgreSQL source of truth through PowerSync. It
implements the server's half of the PowerSync operation-based mutation model: a CRUD upload
endpoint that receives batched mutations from client connectors, validates them, detects
conflicts, and applies accepted mutations to Postgres. PowerSync's service then propagates
canonical state back to all clients via Postgres logical replication — the server needs no
pull endpoint.

This feature also configures PowerSync's sync rules (bucket partitioning + access filtering)
so the service knows which rows belong in which client's sync stream.

---

## Problem Statement

Feature 003 delivered authentication and core domain CRUD. However, clients still have no sync
path: mutations written to the local SQLite are never uploaded to the server, and server-side
changes never propagate to clients. PowerSync's service is running (via Docker Compose) but has
no sync rules, so it replicates nothing. The core architectural constraint — sync conflicts must
never silently lose data — is unverified.

---

## Architecture Context

### PowerSync data flow (corrected framing)

The PLAN-001 Step 4 description mentions a `/sync/pull` endpoint. This is a misframe. The
actual PowerSync architecture is:

- **Push (client → server):** The PowerSync client SDK queues mutations in a local outbox and
  uploads them via `PowerSyncBackendConnector.uploadData()`. This calls the server's CRUD upload
  endpoint (`POST /api/sync/push`). The server validates, deduplicates, and applies the mutation
  to Postgres.
- **Pull (server → client):** PowerSync's service monitors Postgres via logical replication and
  streams changes to connected clients automatically. The **server does not implement a pull
  endpoint.** Pull is entirely handled by the PowerSync service using the sync rules YAML.

This feature implements the server's push endpoint and the sync rules configuration. Client
connector implementations (web `PowerSyncBackendConnector`, Android `PowerSyncBackendConnector`)
are out of scope and belong to Features 008 and 009.

---

## User Stories

- As a client device going online after offline edits, I want my queued mutations uploaded and
  accepted by the server so my changes are durably stored.
- As a developer, I want conflicting mutations from two devices to be logged — never silently
  lost — so I can trust that offline edits are safe.
- As a user who edited a note on two devices simultaneously, I want a conflict copy created so
  I never lose either version.
- As a developer, I want PowerSync sync rules configured so the service correctly partitions data
  by user and household before any client syncs.
- As a developer, I want mutation replay to be idempotent so retry logic doesn't corrupt data.

---

## Requirements

### Must Have

**Tables (migrations):**
- M-1: `sync_mutations` table — dedup store for applied mutation envelopes
  - columns: `mutation_id` (UUID PK), `device_id` (UUID), `user_id` (UUID FK), `entity_type`
    (TEXT), `entity_id` (UUID), `operation` (TEXT: create/update/delete), `applied_at`
    (TIMESTAMPTZ NOT NULL DEFAULT now())
- M-2: `sync_conflicts` table — preserves both versions when a conflict occurs
  - columns: `id` (UUID PK), `mutation_id` (UUID), `entity_type` (TEXT), `entity_id` (UUID),
    `base_version` (TIMESTAMPTZ), `current_version` (TIMESTAMPTZ), `incoming_payload` (JSONB),
    `current_payload` (JSONB), `resolution` (TEXT DEFAULT 'pending': pending/accepted/rejected),
    `resolved_at` (TIMESTAMPTZ), `user_id` (UUID FK), `created_at` (TIMESTAMPTZ NOT NULL DEFAULT now())
- M-3: `device_checkpoints` table — tracks per-device sync progress
  - columns: `device_id` (UUID), `user_id` (UUID FK), `stream` (TEXT), `checkpoint` (BIGINT NOT NULL DEFAULT 0),
    `updated_at` (TIMESTAMPTZ NOT NULL DEFAULT now()), PRIMARY KEY (device_id, stream)
  > **Overridden by Tech.md Decision 2.** No `device_checkpoints` table is created. PowerSync's
  > service manages LSN-based checkpointing internally. The `sync_mutations` dedup store covers
  > all server-side tracking needed.
- M-4: Add `row_version` (TIMESTAMPTZ NOT NULL DEFAULT now()) column to all domain tables that
  participate in sync (see M-4 table in Open Questions) — used as `base_version` for conflict
  detection. `row_version` is updated by the server on every successful mutation; clients include
  the `row_version` they last saw as `base_version` in their mutation envelope.
  > **Overridden by Tech.md Decision 1.** No separate `row_version` column is added to domain
  > tables. The existing `updated_at` column (already present on all domain tables with an
  > auto-update trigger) serves as the conflict version marker. Clients send their last-known
  > `updated_at` as `base_version` in the mutation envelope.

**Server — CRUD upload endpoint:**
- M-5: `POST /api/sync/push` — accepts a `SyncUploadRequest` (array of `MutationEnvelope`);
  requires valid JWT (auth middleware); applies each mutation in order; returns per-mutation
  results (accepted / conflicted / deduplicated)
- M-6: `MutationEnvelope` structure:
  - `mutation_id` (UUID) — client-generated unique ID per mutation
  - `device_id` (UUID) — client device identifier
  - `entity_type` (EntityType) — validated against contracts registry
  - `entity_id` (UUID) — client-generated entity UUID (invariant E-2)
  - `operation` (enum: create / update / delete)
  - `base_version` (TIMESTAMPTZ | null) — last known `row_version` for the entity; null on
    create
  - `payload` (JSONB) — full entity state for create/update; empty for delete
  - `occurred_at` (TIMESTAMPTZ) — client wall-clock time of the mutation
- M-7: Mutation dedup — if `mutation_id` already exists in `sync_mutations`, return
  `status: "deduplicated"` and skip processing (invariant S-2)
- M-8: `entity_type` validation — reject with HTTP 422 if not in the EntityType registry
  (invariant C-1)
- M-9: Ownership check — verify the authenticated user owns the entity being mutated (or is a
  household member for household-scoped entities); reject with HTTP 403 if not
- M-10: Conflict detection — compare `base_version` against current row's `row_version`:
  - If equal (or entity is new on create): no conflict; apply mutation
  - If different: conflict detected → apply LWW (last write wins by `occurred_at`) and write a
    `sync_conflicts` row preserving both versions (invariant S-1)
- M-11: Notes conflict copy — when a `knowledge_note` update conflicts, instead of LWW, create
  a new note row (conflict copy) linked to the original via `entity_relations` with relation type
  `duplicates`; mark the `sync_conflicts` row with resolution `conflict_copy` (ADR-003)
- M-12: Quantity conflict for tracking items — when a `tracking_item_event` update conflicts
  on a quantity field, reject with HTTP 409 and return a conflict response (no LWW); the client
  must re-read and resubmit (invariant S-3)
- M-13: Soft delete handling — a delete mutation sets `deleted_at = now()` on the target row;
  the row is not physically removed; it remains queryable for sync reconciliation (invariant S-6)
- M-14: On successful mutation application, record the mutation in `sync_mutations` (dedup store)
  and update the row's `row_version` to `now()`

**Infra — PowerSync sync rules:**
- M-15: PowerSync sync rules YAML — defines bucket partitioning by user_id for personal data and
  by household_id for household data; must enforce access boundaries (invariant S-7)
- M-16: Sync rule buckets covering all auto-subscribed tables per ADR-003:
  - `user_data` bucket: users, initiatives, tags, entity_relations, attachments (user_id scoped)
  - `household` bucket: households, household_memberships, tracking_locations,
    tracking_categories (household_id scoped)
  - `guidance` bucket: epics, quests, routines, focus_sessions, daily_checkins (user_id scoped)
  - `knowledge` bucket: notes, note_snapshots (user_id scoped)
  - `tracking` bucket: items, item_events, shopping_lists, shopping_list_items (household_id
    scoped)
- M-17: Sync rules must filter `deleted_at IS NULL` rows OR include soft-deleted rows for
  reconciliation — make an explicit decision in Tech.md
- M-18: Update `infra/compose/powersync.yml` to reference the sync rules file
- M-19: Update `packages/contracts/sync-streams.json` to mark the `provisional` flag false and
  align stream names with the finalised bucket names from M-16

### Should Have

- S-1: `GET /api/sync/conflicts` — paginated list of unresolved conflicts for the authenticated
  user; returns entity_type, entity_id, both versions
- S-2: `POST /api/sync/conflicts/:id/resolve` — mark a conflict as accepted or rejected;
  for conflict copies, selecting "accepted" deletes the copy; selecting "rejected" deletes the
  accepted version
- S-3: Device checkpoint endpoint — `GET /api/sync/checkpoint` returns the current checkpoint
  per stream for the authenticated device (aids debugging; PowerSync SDK manages checkpoints
  internally)

### Won't Have (this feature)

- Web client `PowerSyncBackendConnector` implementation — Feature 009
- Android client sync connector — Feature 008
- Conflict resolution UI — Feature 012
- Attachment binary sync (invariant S-5) — attachment metadata syncs; binaries use separate
  upload/download API (Feature 010)
- Domain CRUD for Guidance, Knowledge, or Tracking entities — Features 005, 006, 007
- `PATCH`/`PUT` endpoints beyond sync push — direct CRUD remains the domain feature pattern
- On-demand or selective sync stream subscription — all buckets auto-subscribed in v1; demand
  subscriptions are a v1.1 optimization

---

## Testable Assertions

| ID | Assertion | Verification |
|---|---|---|
| FA-001 | `POST /api/sync/push` without a valid JWT returns HTTP 401 | Integration test: no token |
| FA-002 | `POST /api/sync/push` with a valid JWT and a valid create mutation returns HTTP 200 with `status: "accepted"` for that mutation | Integration test |
| FA-003 | Replaying the same `mutation_id` returns `status: "deduplicated"` and does not double-apply the mutation (invariant S-2) | Integration test: submit same envelope twice; verify DB row unchanged |
| FA-004 | A mutation with `entity_type: "unknown_type"` returns HTTP 422 (invariant C-1) | Integration test |
| FA-005 | A mutation for an entity owned by User B submitted by User A returns HTTP 403 (invariant SEC-1) | Integration test: two-user fixture |
| FA-006 | Two update mutations for the same entity with the same `base_version` produce a `sync_conflicts` row; the entity is not silently overwritten (invariant S-1) | Integration test: simulate two-device conflict scenario |
| FA-007 | A conflicting `knowledge_note` update mutation creates a conflict copy note linked to the original via entity_relations with type `duplicates` (ADR-003) | Integration test: verify two notes exist after conflict |
| FA-008 | A conflicting `tracking_item_event` mutation for a quantity field returns HTTP 409 (invariant S-3) | Integration test |
| FA-009 | A delete mutation sets `deleted_at` on the target row; the row remains in the table | Integration test: verify `deleted_at IS NOT NULL` after delete mutation |
| FA-010 | PowerSync sync rules compile without errors and the service starts with sync_rules loaded | `docker compose up` shows PowerSync healthy; no sync_rules parse errors in logs |
| FA-011 | PowerSync service does not replicate User A's rows to User B's sync stream (invariant S-7) | Integration test via PowerSync client or by inspecting bucket partitioning |
| FA-012 | `packages/contracts/sync-streams.json` has `provisional: false` after this feature | File assertion: `jq .provisional packages/contracts/sync-streams.json` returns `false` |
| FA-013 | `cargo test` passes for the server crate including new sync module tests | `cargo test` exits 0 in `apps/server/` |
| FA-014 | All new migrations apply cleanly against a fresh database and are reversible | `sqlx migrate run` and `sqlx migrate revert` exit 0 |

---

## Open Questions

- [ ] **row_version column scope**: Which tables need `row_version`? The 16 auto-subscribed tables
  per ADR-003 (users, households, household_memberships, initiatives, tags, entity_relations,
  attachments, locations, categories, items, item_events, shopping_lists, shopping_list_items,
  notes, note_snapshots, epics + others in guidance). All domain tables already have `updated_at`
  — should `row_version` be a separate column or an alias for `updated_at`? Decision required in
  Tech.md.
- [ ] **Soft-deleted rows in sync rules**: Should PowerSync sync rules include soft-deleted rows
  (for reconciliation) or exclude them (cleaner client state)? Including them means clients see
  tombstones; excluding them means clients may not learn about deletions correctly. Decision
  required in Tech.md.
- [ ] **Batch vs. serial mutation application**: Should mutations in a single `POST /api/sync/push`
  request be applied in a single transaction (all-or-nothing) or individually (partial success)?
  Individual application allows partial success responses but complicates rollback. Decision in
  Tech.md.
- [ ] **sync-streams.json bucket naming**: The contracts define 5 stream names (`user_data`,
  `household`, `guidance`, `knowledge`, `tracking`). PowerSync bucket names in sync rules must
  match what clients subscribe to. Verify naming alignment during Tech phase.
- [ ] **`row_version` vs. PowerSync-native checkpointing**: PowerSync's own LSN-based checkpoint
  may make an explicit `device_checkpoints` table redundant. Evaluate in Tech.md.

---

## Dependencies

| Dependency | Status |
|---|---|
| Feature 001 — Foundation (Docker Compose, CI) | Complete |
| Feature 002 — Shared Contracts (EntityType, SyncStream registries) | Complete |
| Feature 003 — Server Core (auth, migrations, AppError, AppState) | Complete |
| PowerSync service running in Docker Compose | In place |
| All domain table migrations (007–026) applied | Complete |

---

## Revision History

| Date | Change | ADR |
|---|---|---|
| 2026-04-13 | Initial spec | ADR-003 |
