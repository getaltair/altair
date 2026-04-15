# ADR-018: Sync Create Allowlist and Household Membership Verification

## Status

Accepted

## Date

2026-04-14

## Context

The sync push endpoint (`POST /api/sync/push`) accepts `Operation::Create` mutations
for any `EntityType`. The `process_single_mutation` function explicitly skips
`check_ownership` for Create operations (ownership cannot exist yet). No code
restricts which entity types may be created through the sync endpoint.

Two classes of vulnerability result:

1. **Admin account creation**: `EntityType::User` and `EntityType::Household` are not
   in `is_user_scoped` and have no ownership check. An authenticated client can push
   `Create` for `EntityType::User` with `{"is_admin": true, "password_hash": "..."}`.
   The payload is written verbatim into the `users` table, creating an admin account.

2. **Household isolation bypass**: Household-scoped types (`TrackingLocation`,
   `TrackingCategory`, `TrackingShoppingList`, `TrackingItem`) accept a client-supplied
   `household_id` in the payload. No membership check verifies the client's user
   actually belongs to the claimed household. An authenticated client can write records
   into any household they know the UUID for.

Three strategies were considered:

1. **Opt-in allowlist per entity type**: Only entity types explicitly listed in
   `SYNC_CREATABLE_TYPES` may be created via push. All others return 400.

2. **Per-entity-type authorization hooks**: Each `EntityType` gets a `can_create`
   function that performs necessary checks. Flexible but verbose.

3. **Allowlist + household membership verification**: Combine (1) with an explicit
   membership check for household-scoped creates.

## Decision

Use option **3**: explicit `SYNC_CREATABLE_TYPES` allowlist combined with household
membership verification for household-scoped creates.

**SYNC_CREATABLE_TYPES allowlist**:
- User-scoped (only the authenticated user may create these):
  `KnowledgeNote`, `GuidanceQuest`, `GuidanceRoutine`, `GuidanceEpic`,
  `GuidanceFocusSession`, `GuidanceDailyCheckin`, `Tag`, `EntityRelation`
- Household-scoped (membership must be verified):
  `TrackingLocation`, `TrackingCategory`, `TrackingShoppingList`, `TrackingItem`,
  `TrackingItemEvent`, `TrackingShoppingListItem`
- **Excluded** (never creatable via sync): `User`, `Household`, `KnowledgeNoteSnapshot`

`KnowledgeNoteSnapshot` is excluded because it is created exclusively by the server
as the conflict-copy artifact; a client-supplied snapshot bypasses that invariant.
`User` and `Household` are excluded because their creation paths require server-side
logic (password hashing, owner bootstrap) that sync push cannot perform safely.

**Household membership verification**:
For household-scoped creates, after extracting `household_id` from the payload,
verify `SELECT 1 FROM household_members WHERE household_id = $1 AND user_id = $2`.
If no row exists, return `AppError::Forbidden`.

`household_id` is **not** taken from the client payload for inserts; it is instead
verified separately. The column is added from the verified value, not from the raw
payload, to prevent clients from constructing a payload where `household_id` passes
the allowlist key filter but carries a different UUID.

## Consequences

- Clients that attempt to Create `User`, `Household`, or `KnowledgeNoteSnapshot` via
  sync push receive `AppError::BadRequest`.
- Clients that attempt household-scoped creates without membership receive
  `AppError::Forbidden`.
- Any new entity type added to the contracts registry must be explicitly assigned to
  either `SYNC_CREATABLE_TYPES` or the excluded set before it can be synced. This
  makes authorization an explicit opt-in.
- `check_ownership` for Create is replaced by `check_create_allowed`, which handles
  both the allowlist check and household membership verification.

## Relates To

- P5-002 (review finding — authorization bypass)
- ADR-014 (RLS deferral — the allowlist is the interim authorization layer until RLS
  is enabled)
- FA-004, FA-005 (ownership and cross-user isolation assertions)
