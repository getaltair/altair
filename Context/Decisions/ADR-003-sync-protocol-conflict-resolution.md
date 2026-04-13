# ADR-003: Sync Protocol and Conflict Resolution

## Status

Accepted

## Date

2026-04-12

## Context

Altair is offline-first. Clients write to local SQLite, sync changes when connected. The core architectural constraint is: **sync conflicts must never silently lose data.**

PowerSync Open Edition is the chosen sync layer (see ADR-002 for deployment context). PowerSync uses operation-based mutations — clients record changes as CRUD operations in a local outbox and upload them to the server via a connector endpoint. The server applies mutations to PostgreSQL, and PowerSync propagates changes back to all clients via Postgres logical replication.

The open question is conflict resolution strategy when two clients modify the same record while disconnected.

### Sync Architecture

**Client → Server flow:**
1. Client writes to local SQLite (immediate)
2. Mutation recorded in outbox with: `mutation_id`, `device_id`, `user_id`, `entity_type`, `entity_id`, `operation` (create/update/delete), `base_version`, `payload`, `occurred_at`
3. When connected, outbox uploads mutations to server CRUD endpoint
4. Server validates, applies, or flags conflicts
5. PowerSync replicates canonical state back to all clients

**Sync streams** (PowerSync Sync Streams model):
- **Auto-subscribed** (always replicate): user profile, household memberships, personal data, household shared data, relations, attachment metadata — 16 tables
- **On-demand** (subscribed when navigating): initiative detail, note detail, item history, quest detail — 9 tables
- **Selective** (replicate cautiously): note snapshots, item events, focus sessions, daily check-ins — 4 tables growing quickly

**Scope boundaries:** Sync by user, household, initiative, or detail view. No arbitrary graph traversal.

## Decision

### Mutation Model

Use PowerSync's native **operation-based mutation model**. The server CRUD endpoint receives typed mutation envelopes and is responsible for:

1. Authenticating device/user
2. Verifying mutation ownership (user owns the entity or has household access)
3. Validating schema and business rules
4. Checking `base_version` for conflict detection
5. Applying or rejecting the mutation
6. Returning conflicts and canonical version to the client

### Conflict Resolution

**Default strategy: Last-write-wins (LWW) with conflict logging.**

When the server receives a mutation whose `base_version` doesn't match the current row version:

1. **Detect** — Compare `base_version` in mutation against current row version in Postgres
2. **Apply LWW** — Accept the mutation (last write wins by `occurred_at` timestamp)
3. **Log conflict** — Write both versions (incoming and overwritten) to a `sync_conflicts` table
4. **Notify** — Flag the conflict for user resolution via the notification system
5. **Resolve** — User reviews conflict in UI, picks a version or merges manually. Resolution marks the conflict as resolved.

**Per-domain considerations:**

| Domain | Strategy | Rationale |
|--------|----------|-----------|
| Guidance (quests, routines) | LWW + conflict log | Field-level changes rarely conflict; LWW is safe |
| Knowledge (notes) | LWW + conflict copy | Text conflicts risk data loss; create a conflict copy the user can merge |
| Tracking (items, quantities) | LWW + stricter validation | Quantities and reservations need business rule checks before accepting |
| Relations/Tags | LWW | Idempotent operations; conflicts are rare and low-stakes |

**Conflict copy behavior for Notes:** When a note's body is modified on two devices, the server creates a "conflict copy" note linked to the original rather than overwriting. The user sees both versions and can merge or discard.

### Delete Handling

Soft deletes with tombstones. Deleted rows remain queryable for sync reconciliation and undo support. Attachment cleanup is deferred until sync confirms all clients have received the deletion.

## Consequences

### Positive

- LWW is simple to implement and correct for the majority of mutations
- Conflict log satisfies "never silently lose data" — every conflict is preserved and surfaced
- Conflict copies for Notes prevent the highest-risk data loss scenario
- Operation-based model aligns natively with PowerSync's SDK
- Soft deletes support undo and safe cleanup

### Negative

- Conflict resolution UI must be built — this is user-facing work, not just backend
- Conflict copies for Notes create duplicate content the user must manage
- `sync_conflicts` table will grow; needs periodic cleanup of resolved conflicts
- Clock skew between devices could affect LWW ordering (mitigated by server-assigned canonical timestamps)

### Neutral

- Field-level conflict detection (merging non-conflicting field changes) is a future optimization, not required for v1
- CRDT-based text merging for Notes is explicitly deferred — conflict copies are simpler and sufficient for v1

## Exit Cost Assessment (Added 2026-04-12)

PowerSync is the only sync solution meeting all requirements (offline local writes + PostgreSQL + Web SDK + Android native SDK + self-hostable). This makes it the correct choice, but the lock-in should be documented.

### What's Proprietary

| Component | Lock-in Level | Detail |
|-----------|---------------|--------|
| **Sync rules** | High | YAML-based DSL for defining bucket partitioning and data scoping. No standard equivalent. |
| **Client SDKs** | High | PowerSync JS SDK (web) and Kotlin SDK (Android) manage the local SQLite database, outbox, and sync protocol. Deep integration into data layer. |
| **Connector pattern** | Medium | `PowerSyncBackendConnector` interface for auth token refresh and CRUD upload. Replaceable but shapes the data layer architecture. |
| **MongoDB dependency** | Medium | PowerSync service requires MongoDB as its internal metadata store. Adds ~200-400MB RAM and operational surface. |
| **Sync protocol** | High | Proprietary wire protocol between PowerSync service and client SDKs. Not interchangeable with other sync engines. |

### What a Migration Looks Like

Replacing PowerSync would require:

1. **New sync engine selection** — No drop-in replacement exists today. ElectricSQL dropped offline writes. Triplit lacks Android SDK. Custom sync is 3-6 months of infrastructure work.
2. **Client SDK replacement** — Both web and Android data layers would need rewriting. Room DAOs and SvelteKit stores are decoupled from PowerSync via repository pattern, but the sync plumbing (outbox, checkpoint, conflict detection) is PowerSync-specific.
3. **Server CRUD endpoint rewrite** — The mutation envelope format is PowerSync-specific. Server-side conflict detection logic is reusable but the transport layer changes.
4. **MongoDB removal** — Only needed for PowerSync. Removing it frees ~200-400MB RAM and simplifies the deployment stack.
5. **Sync rules translation** — Bucket partitioning and data scoping rules must be re-expressed in whatever the replacement uses.

### Mitigation

- Repository pattern in Android and web isolates domain logic from sync plumbing
- Server-side conflict resolution logic (LWW, conflict copies) is sync-engine-agnostic
- Domain models and DTOs are independent of PowerSync
- The exit path is expensive but bounded — it's a data layer rewrite, not an application rewrite
