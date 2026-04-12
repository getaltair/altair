# Invariants

| Field | Value |
|---|---|
| **Document** | 03-invariants |
| **Version** | 1.0 |
| **Status** | Draft |
| **Last Updated** | 2026-04-12 |
| **Source Docs** | `docs/altair-architecture-spec.md`, `docs/altair-shared-contracts-spec.md`, `docs/altair-powersync-sync-spec.md` |

---

## Invariant Categories

- **S** — Sync invariants
- **SEC** — Security invariants
- **C** — Contract invariants
- **E** — Entity invariants
- **D** — Data integrity invariants

---

## Sync Invariants

| ID | Invariant | Enforcement | Assertions |
|---|---|---|---|
| S-1 | Sync conflicts must never silently lose data | Server conflict detection compares `base_version`; conflicting mutations return conflict response, never auto-overwrite | A-004 |
| S-2 | Client mutations are idempotent — replaying a mutation produces the same result | Server checks `mutation_id` for dedup; operations are applied at-most-once | A-003 |
| S-3 | Quantity fields (inventory) use stricter conflict checks than last-write-wins | Server compares base quantity; conflicting quantity changes require explicit resolution | A-031 |
| S-4 | Device checkpoints advance monotonically — a device never re-syncs already-acked data | Checkpoint stored per-device; pull requests include checkpoint; server returns only newer changes | — |
| S-5 | Attachment binaries never flow through the sync engine | Attachment metadata syncs via PowerSync; binary upload/download uses separate object storage API | A-007 |
| S-6 | Soft-deleted records remain queryable for sync reconciliation | `deleted_at` timestamp marks deletion; records are not physically removed until all devices have synced past the deletion checkpoint | — |
| S-7 | Every sync stream query must be filtered by real access boundaries (user_id, household membership, initiative ownership) | Stream queries use `auth.user_id()` and membership checks; subscription parameters alone are insufficient | A-002 |

---

## Security Invariants

| ID | Invariant | Enforcement | Assertions |
|---|---|---|---|
| SEC-1 | Per-user data isolation at every query path | All domain service queries include `user_id` or household membership filter; no query returns unscoped data | A-002 |
| SEC-2 | All API access is authenticated except bootstrap/setup endpoints | Axum auth middleware on all routes except `/auth/register`, `/auth/login`, `/health` | A-001 |
| SEC-3 | Passwords are hashed with Argon2id — never stored in plaintext | Auth service uses Argon2id with recommended parameters | — |
| SEC-4 | Attachment URLs are signed or gated — no direct public access to object storage | Attachment download endpoint validates user ownership before returning signed URL or streaming content | — |
| SEC-5 | Secrets are loaded from environment or secret store — never hardcoded | Config module reads from env vars; no secrets in source code, config files, or migrations | — |
| SEC-6 | CSRF protections for browser session flows | SvelteKit server-side hooks enforce CSRF tokens for state-changing requests | — |

---

## Contract Invariants

| ID | Invariant | Enforcement | Assertions |
|---|---|---|---|
| C-1 | Entity type identifiers must come from the canonical registry — no inline magic strings | Shared contracts package provides constants; backend write paths reject unknown entity types | A-005, A-020 |
| C-2 | Relation type identifiers must come from the canonical registry | Same enforcement as C-1; relation creation validates `relation_type` against registry | A-005 |
| C-3 | Sync stream names must reference canonical stream identifiers | PowerSync config and client subscriptions use constants from the contracts package | — |
| C-4 | Adding a new entity type, relation type, or stream name is an additive (minor version) change; renaming or removing is a breaking change | Versioning rules in shared contracts spec; CI tests validate registry against language bindings | — |
| C-5 | AI pipelines must map outputs to canonical registry values — AI cannot invent new entity types at write time | AI service output validation layer checks entity types against registry before persisting | — |

---

## Entity Invariants

| ID | Invariant | Enforcement | Assertions |
|---|---|---|---|
| E-1 | All entities require a UUID `id` primary key, `created_at`, and `updated_at` timestamps | Database schema constraints; `updated_at` auto-maintained via trigger | — |
| E-2 | All entity IDs are UUIDs generated client-side for offline-first creation | Client SDKs generate UUIDs; server accepts client-generated IDs | — |
| E-3 | A quest's `initiative_id` must reference a valid initiative owned by the same user or household | Foreign key constraint + ownership check in service layer | — |
| E-4 | Routine frequency must produce at least one scheduled occurrence per period | Validation in routine creation/update service; reject degenerate frequency configs | — |
| E-5 | Backlinks are symmetric: if note A references note B, note B's backlinks include note A | Query-time derivation from entity_relations; no separate backlink table needed | A-019 |
| E-6 | Note snapshots are immutable once created | No UPDATE path for note_snapshot rows; only INSERT | A-021 |
| E-7 | Item quantity must never go negative from a consumption event | Validation in item_event service; reject events that would produce negative quantity | A-026 |
| E-8 | An item's `location_id` must reference a valid location within the same household scope | Foreign key constraint + household scope check | A-028 |
| E-9 | Shopping list items with an `item_id` reference must point to a valid item in the same household | Foreign key constraint + household scope check | A-027 |

---

## Data Integrity Invariants

| ID | Invariant | Enforcement | Assertions |
|---|---|---|---|
| D-1 | Migrations must include rollback procedures | Migration review process; each migration file includes `down` counterpart | — |
| D-2 | A migration that has been applied to any environment must never be modified | CI check: migration checksums are compared against known-good hashes | — |
| D-3 | PowerSync schema must mirror the PostgreSQL table structure for synced tables | CI validation: compare PowerSync schema definition against Postgres migration output | — |
| D-4 | Column names in SQLite (Room) must match PostgreSQL column names exactly (snake_case) for PowerSync compatibility | Room entity annotation review; linting rule for `@ColumnInfo` names | — |
| D-5 | Item events are append-only — no UPDATE or DELETE on item_event rows | No update/delete DAO methods or service paths for item_events | A-029 |

---

## Summary Table

| ID | Category | Short Description | Priority |
|---|---|---|---|
| S-1 | Sync | No silent data loss in sync conflicts | Critical |
| S-2 | Sync | Mutation idempotency | Critical |
| S-3 | Sync | Strict quantity conflict checks | High |
| S-4 | Sync | Monotonic checkpoint advancement | High |
| S-5 | Sync | Attachment binaries excluded from sync engine | High |
| S-6 | Sync | Soft-deleted records available for sync reconciliation | High |
| S-7 | Sync | Access-boundary filtering on all sync streams | Critical |
| SEC-1 | Security | Per-user data isolation | Critical |
| SEC-2 | Security | All API access authenticated | Critical |
| SEC-3 | Security | Argon2id password hashing | Critical |
| SEC-4 | Security | Signed/gated attachment URLs | High |
| SEC-5 | Security | No hardcoded secrets | Critical |
| SEC-6 | Security | CSRF protections | High |
| C-1 | Contract | Entity types from canonical registry | High |
| C-2 | Contract | Relation types from canonical registry | High |
| C-3 | Contract | Sync stream names from canonical registry | Medium |
| C-4 | Contract | Additive vs breaking contract changes | Medium |
| C-5 | Contract | AI outputs validated against registry | Medium |
| E-1 | Entity | UUID + timestamps on all entities | High |
| E-2 | Entity | Client-generated UUIDs | High |
| E-3 | Entity | Quest initiative ownership scope | Medium |
| E-4 | Entity | Routine frequency validity | Medium |
| E-5 | Entity | Symmetric backlinks | Medium |
| E-6 | Entity | Immutable note snapshots | Medium |
| E-7 | Entity | No negative item quantities | High |
| E-8 | Entity | Item location household scope | Medium |
| E-9 | Entity | Shopping list item household scope | Medium |
| D-1 | Data | Migration rollback procedures | High |
| D-2 | Data | Immutable applied migrations | High |
| D-3 | Data | PowerSync-Postgres schema parity | High |
| D-4 | Data | SQLite-Postgres column name parity | High |
| D-5 | Data | Append-only item events | Medium |

---

## Invariant-to-Assertion Mapping

| Invariant | Assertions |
|---|---|
| S-1 | A-004, A-031 |
| S-2 | A-003 |
| S-3 | A-031 |
| S-5 | A-007 |
| S-7 | A-002 |
| SEC-1 | A-002 |
| SEC-2 | A-001 |
| C-1 | A-005, A-020 |
| C-2 | A-005 |
| E-3 | — |
| E-5 | A-019 |
| E-6 | A-021 |
| E-7 | A-026 |
| E-8 | A-028 |
| E-9 | A-027 |
| D-5 | A-029 |

---

## Test Case Patterns

### Sync conflict test
1. Device A and Device B both read quest Q at version V
2. Device A updates Q.title offline → mutation M1 (base_version=V)
3. Device B updates Q.title offline → mutation M2 (base_version=V)
4. Device A syncs M1 → accepted, Q now at version V+1
5. Device B syncs M2 → conflict detected (base_version V != current V+1)
6. Assert: Q has not been silently overwritten; conflict returned to Device B

### Quantity conflict test
1. Item has quantity=10
2. Device A consumes 3 (base_quantity=10) offline
3. Device B consumes 5 (base_quantity=10) offline
4. Both sync
5. Assert: server does not blindly apply both (which would give 10-3-5=2); conflict detected

### Data isolation test
1. User A creates a quest
2. User B queries quests
3. Assert: User B's result set does not include User A's quest
