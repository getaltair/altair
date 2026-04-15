# Feature 004: Sync Engine — Technical Research

| Field | Value |
|---|---|
| **Feature** | 004-SyncEngine |
| **Version** | 1.0 |
| **Status** | Draft |
| **Date** | 2026-04-13 |
| **Source Docs** | `Context/Features/004-SyncEngine/Spec.md`, `Context/Decisions/ADR-003-sync-protocol-conflict-resolution.md` |

---

## Architecture Overview

Feature 004 adds three layers to the existing server:

1. **`src/sync/` module** — new Axum module following the existing
   `mod.rs / models.rs / service.rs / handlers.rs` pattern. Implements `POST /api/sync/push`.
2. **Sync tables** — two new migrations (`sync_mutations` for dedup, `sync_conflicts` for
   conflict logging). No per-domain `row_version` column; see Decision 1.
3. **PowerSync sync rules** — `infra/compose/sync_rules.yaml` (separate file referenced from
   `powersync.yml`) defines bucket partitioning by `user_id` and `household_id`.

The auth module, AppError, and AppState from Feature 003 are reused directly. No new Cargo
dependencies are expected beyond `serde_json` (already present) for the JSONB payload handling.

---

## Open Questions Resolved

### Decision 1 — `row_version` column vs. `updated_at` as version proxy

**Spec M-4 is overridden.** No separate `row_version` column is added to domain tables.

`updated_at` serves as the conflict version marker. This works because:

- `updated_at` is already present on all domain tables with an auto-update trigger from
  Feature 003's migrations.
- The server sets `updated_at = now()` on every successfully applied mutation; clients never
  write `updated_at`.
- Clients include their last-known `updated_at` for an entity as `base_version` in the
  mutation envelope. This is read from the client's local SQLite before executing the local
  write (connector responsibility — Features 008/009).
- The server checks: if the row's current `updated_at` in Postgres > `base_version`, a conflict
  is detected. The row was modified by another device after the client's last sync point.

**Critical constraint (for client connector implementors):** Clients must never write
`updated_at` in their local SQLite mutations. If a client writes `updated_at` to local SQLite,
the PowerSync CrudEntry's `opData` will contain the client-set timestamp, making it useless as
`base_version`. The contract is: `updated_at` is a server-managed, read-only field for clients.
Client SQLite schemas should mark it as not writable. This is enforced in Features 008/009.

**Spec change:** Removes M-4 (row_version migration) and updates M-1 (sync_mutations) to not
include a `row_version` reference. `base_version` in mutation envelopes is now typed
`TIMESTAMPTZ | null` and compared directly against the row's `updated_at`.

---

### Decision 2 — `device_checkpoints` table dropped

**Spec M-3 is overridden.** No `device_checkpoints` table is created.

PowerSync's service manages LSN-based checkpointing internally between the service and each
connected client. The `sync_mutations` table (dedup store, Spec M-1) handles the only
server-side tracking needed: idempotent mutation replay by `mutation_id`.

Invariant S-4 (monotonic checkpoint advancement) is enforced by PowerSync's internal mechanism,
not by application code.

---

### Decision 3 — Soft-deleted rows in sync rules: include, not filter

**Rationale:** If sync rules filter `WHERE deleted_at IS NULL`, when a row is soft-deleted on
the server, PowerSync removes it from the bucket. PowerSync sends a DELETE operation to the
client, removing the row from local SQLite. This appears correct but breaks for offline clients:
if the client was offline during the delete and comes back after the fact, PowerSync will still
send the DELETE correctly (the row has `deleted_at IS NOT NULL`, so it's not in the bucket, and
PowerSync's diff logic will issue a delete).

However, including soft-deleted rows is safer for reconciliation: clients receive rows with
`deleted_at IS NOT NULL` and can filter them locally. This matches invariant S-6 (soft-deleted
records remain queryable for sync reconciliation).

**Implementation:** Sync rule bucket queries do NOT include `WHERE deleted_at IS NULL`. All rows
(including tombstones) are replicated. Client applications filter `deleted_at IS NULL` in their
display queries.

---

### Decision 4 — Serial mutation application with partial success

Each mutation in a `POST /api/sync/push` batch is processed independently within its own
database transaction. The endpoint returns a per-mutation result array:

```json
{
  "results": [
    { "mutation_id": "...", "status": "accepted" },
    { "mutation_id": "...", "status": "deduplicated" },
    { "mutation_id": "...", "status": "conflicted", "conflict_id": "..." }
  ]
}
```

Rationale: all-or-nothing batch transactions make retry logic more complex and can cause
correctly-applied mutations to fail because of a single conflicting one. Serial per-mutation
transactions allow partial success and idempotent retry of failed mutations only.

---

### Decision 5 — `conflict_copy` relation type

Conflict copies for `knowledge_note` entities are linked to the original via an `entity_relations`
row with `relation_type: "duplicates"`. The existing `RelationType::Duplicates` variant in
`contracts.rs` covers this; no new contract values are needed.

The `sync_conflicts` row for this case uses `resolution = 'conflict_copy'` (a new value beyond
the pending/accepted/rejected set defined in Spec M-2). This is additive and does not change the
schema; it's a runtime value in a TEXT column.

---

## Mutation Envelope Format

The PowerSync client SDK's `uploadData()` method receives `CrudTransaction` objects from its
internal outbox. The client connector implementations (Features 008/009) are responsible for
transforming these into `MutationEnvelope` objects and calling `POST /api/sync/push`.

```
CrudEntry (PowerSync SDK internal):
  op:     "PUT" | "PATCH" | "DELETE"
  table:  string (Postgres table name)
  id:     string (UUID)
  opData: Record<string, any> (changed fields)

↓ Transformed by client connector ↓

MutationEnvelope (Altair server format):
  mutation_id:  UUID       -- client-generated; stable across retries
  device_id:    UUID       -- client device identifier
  entity_type:  EntityType -- validated against contracts registry
  entity_id:    UUID       -- same as CrudEntry.id
  operation:    "create" | "update" | "delete"
  base_version: TIMESTAMPTZ | null  -- pre-write updated_at from local SQLite; null on create
  payload:      JSONB      -- full row for create/update; empty for delete
  occurred_at:  TIMESTAMPTZ
```

**Table-to-EntityType mapping** is defined in the client connector and does not need to be
defined server-side. The server validates `entity_type` against the `EntityType` registry
(`contracts.rs`) and maps it to the correct Postgres table for the mutation.

**`base_version` protocol:**
- `create`: `base_version = null`
- `update`: `base_version` = the `updated_at` value from local SQLite for this row,
  read before executing the local write
- `delete`: `base_version` = same as update

---

## Server Module: `src/sync/`

Following the existing module pattern:

```
apps/server/server/src/sync/
  mod.rs       — module declaration + re-exports + router function
  models.rs    — MutationEnvelope, SyncUploadRequest, SyncUploadResponse, MutationResult,
                 MutationStatus enum (Accepted, Deduplicated, Conflicted { conflict_id })
  service.rs   — apply_mutations(), detect_conflict(), create_conflict_copy(),
                 check_quantity_conflict(), record_sync_mutation()
  handlers.rs  — push_handler() (POST /api/sync/push)
```

`push_handler` extracts `AuthUser` via the existing extractor, validates the request, and
delegates to `service.apply_mutations()`.

`service.apply_mutations()` processes each `MutationEnvelope` in order:
1. Check dedup (SELECT from sync_mutations by mutation_id)
2. Validate entity_type
3. Check ownership
4. Detect conflict (SELECT row's updated_at; compare with base_version)
5. Apply mutation or create conflict copy
6. Record to sync_mutations

All database operations in step 4+5+6 execute within a single sqlx transaction per mutation.

---

## Tables

Two new migrations. Both include rollback `.down.sql` files.

### `sync_mutations` (dedup store)

```sql
CREATE TABLE sync_mutations (
  mutation_id  UUID        PRIMARY KEY,
  device_id    UUID        NOT NULL,
  user_id      UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  entity_type  TEXT        NOT NULL,
  entity_id    UUID        NOT NULL,
  operation    TEXT        NOT NULL, -- create | update | delete
  applied_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_sync_mutations_entity ON sync_mutations(entity_id, applied_at);
CREATE INDEX idx_sync_mutations_user   ON sync_mutations(user_id, applied_at);
```

### `sync_conflicts` (conflict log)

```sql
CREATE TABLE sync_conflicts (
  id               UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
  mutation_id      UUID        NOT NULL,
  entity_type      TEXT        NOT NULL,
  entity_id        UUID        NOT NULL,
  base_version     TIMESTAMPTZ,            -- client's base_version (incoming)
  current_version  TIMESTAMPTZ,            -- server's updated_at at conflict time
  incoming_payload JSONB       NOT NULL,
  current_payload  JSONB       NOT NULL,
  resolution       TEXT        NOT NULL DEFAULT 'pending',
                               -- pending | accepted | rejected | conflict_copy
  resolved_at      TIMESTAMPTZ,
  user_id          UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_sync_conflicts_entity ON sync_conflicts(entity_id, created_at);
CREATE INDEX idx_sync_conflicts_user   ON sync_conflicts(user_id, resolution);
```

---

## PowerSync Sync Rules

### File location

`infra/compose/sync_rules.yaml` — referenced from `powersync.yml` via:
```yaml
sync_rules:
  path: ./sync_rules.yaml
```

### Bucket design

Five buckets matching the `SyncStream` contract values. All buckets include soft-deleted rows
(no `deleted_at IS NULL` filter) per Decision 3.

```yaml
bucket_definitions:

  # User-owned personal data
  user_data:
    parameters: SELECT request.user_id() AS user_id
    data:
      - SELECT * FROM users       WHERE id = bucket.user_id
      - SELECT * FROM initiatives WHERE user_id = bucket.user_id
      - SELECT * FROM tags        WHERE user_id = bucket.user_id
      - SELECT * FROM attachments WHERE user_id = bucket.user_id
      - SELECT * FROM entity_relations WHERE user_id = bucket.user_id

  # Household-scoped shared data (user must be a member)
  household:
    parameters: >
      SELECT hm.household_id
        FROM household_memberships hm
       WHERE hm.user_id = request.user_id()
    data:
      - SELECT * FROM households            WHERE id = bucket.household_id
      - SELECT * FROM household_memberships WHERE household_id = bucket.household_id

  # Guidance domain — user-owned
  guidance:
    parameters: SELECT request.user_id() AS user_id
    data:
      - SELECT * FROM guidance_epics         WHERE user_id = bucket.user_id
      - SELECT * FROM guidance_quests        WHERE user_id = bucket.user_id
      - SELECT * FROM guidance_routines      WHERE user_id = bucket.user_id
      - SELECT * FROM guidance_focus_sessions WHERE user_id = bucket.user_id
      - SELECT * FROM guidance_daily_checkins WHERE user_id = bucket.user_id

  # Knowledge domain — user-owned
  knowledge:
    parameters: SELECT request.user_id() AS user_id
    data:
      - SELECT * FROM knowledge_notes          WHERE user_id = bucket.user_id
      - SELECT * FROM knowledge_note_snapshots WHERE user_id = bucket.user_id

  # Tracking domain — household-scoped
  tracking:
    parameters: >
      SELECT hm.household_id
        FROM household_memberships hm
       WHERE hm.user_id = request.user_id()
    data:
      - SELECT * FROM tracking_locations          WHERE household_id = bucket.household_id
      - SELECT * FROM tracking_categories         WHERE household_id = bucket.household_id
      - SELECT * FROM tracking_items              WHERE household_id = bucket.household_id
      - SELECT * FROM tracking_item_events        WHERE household_id = bucket.household_id
      - SELECT * FROM tracking_shopping_lists     WHERE household_id = bucket.household_id
      - SELECT * FROM tracking_shopping_list_items WHERE household_id = bucket.household_id
```

**Note on `request.user_id()`:** PowerSync extracts this from the JWT `sub` claim. This is the
same UUID issued by the Altair auth system (ADR-012). No additional configuration needed.

**Access boundary enforcement (S-7):** The `household` and `tracking` buckets use a
parameterized subquery against `household_memberships` — a user only receives household rows
they're actually a member of. User-scoped buckets use `request.user_id()` directly.

---

## Conflict Detection Logic (summary)

```
receive MutationEnvelope for entity E:

1. SELECT mutation_id FROM sync_mutations WHERE mutation_id = $1
   → if found: return status=Deduplicated

2. Validate entity_type against EntityType enum
   → if invalid: return 422

3. Ownership check (entity's user_id == auth.user_id, or household member)
   → if not owned: return 403

BEGIN transaction:

4. SELECT updated_at, <all columns> FROM <table> WHERE id = $entity_id FOR UPDATE
   current_row = result

5a. If operation == Create and current_row is None:
    → INSERT new row; record sync_mutation; return Accepted

5b. If operation == Create and current_row exists:
    → mutation is a duplicate create; return Accepted (idempotent)

6. If base_version IS NULL and operation != Create:
    → treat as "client has no version info"; apply with LWW using occurred_at

7. Conflict check: if current_row.updated_at > base_version:
    conflict detected →
      if entity_type == KnowledgeNote and operation == Update:
        create conflict copy (INSERT new note linked via entity_relations)
        write sync_conflicts row (resolution='conflict_copy')
        return Conflicted { conflict_id }
      elif entity_type == TrackingItemEvent and is_quantity_field(payload):
        ROLLBACK
        return 409 (conflict response — client must re-read and resubmit)
      else:
        apply LWW: if occurred_at >= current_row.updated_at → accept (log conflict)
                   if occurred_at < current_row.updated_at → reject (log conflict)
        write sync_conflicts row
        return Conflicted { conflict_id }

8. No conflict: apply mutation (UPDATE or DELETE)
   UPDATE: set updated_at = now(), apply payload columns
   DELETE: set deleted_at = now()

9. INSERT INTO sync_mutations (mutation_id, device_id, ...)
COMMIT
return Accepted
```

---

## Integration Points

| Component | Change | Notes |
|---|---|---|
| `src/lib.rs` | Add `pub mod sync;` | No AppState changes needed |
| `src/main.rs` | `.merge(sync::router())` | Same pattern as core modules |
| `apps/server/Cargo.toml` | No new deps expected | `serde_json` already present for JSONB |
| `infra/compose/powersync.yml` | Add `sync_rules: path: ./sync_rules.yaml` | |
| `infra/compose/sync_rules.yaml` | New file | Bucket definitions per Decision 3 |
| `packages/contracts/sync-streams.json` | Set `provisional: false` | Stream names finalised |
| `infra/migrations/` | Two new migrations (sync_mutations, sync_conflicts) | Next available numbers after 026 |

---

## Risks and Mitigations

| Risk | Mitigation |
|---|---|
| Clock skew between devices affects LWW ordering | Server-assigned `updated_at = now()` is the canonical timestamp; `occurred_at` is for LWW tiebreaking only, not the primary conflict signal |
| `updated_at` written by client connector breaks conflict detection | Document the constraint explicitly; client connector implementations (Features 008/009) must treat `updated_at` as read-only |
| PowerSync sync rules table name mismatch | Table names in `sync_rules.yaml` must exactly match Postgres table names from migrations; CI should validate via `docker compose up` smoke test |
| Sync rules accidentally expose cross-user data | Bucket queries are parameterized by `request.user_id()` or household membership; FA-011 integration test verifies isolation |
| Conflict copy notes accumulate indefinitely | `sync_conflicts` cleanup and conflict resolution UI are Feature 012; no cleanup implemented here |

---

## ADRs

No new ADRs required. All decisions are governed by:
- **ADR-003** — Sync protocol: LWW + conflict logging (accepted; this feature implements it)
- **ADR-012** — Built-in auth: JWT `sub` = user UUID, used in `request.user_id()` in sync rules

---

## Revision History

| Date | Change |
|---|---|
| 2026-04-13 | Initial tech research; drops Spec M-3 (device_checkpoints) and M-4 (row_version column); finalises conflict model |
