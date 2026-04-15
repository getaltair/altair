# ADR-017: Payload Column Allowlist for Sync INSERT/UPDATE

## Status

Accepted

## Date

2026-04-14

## Context

The sync engine's `insert_from_payload` and `apply_update_to_table` functions build
SQL dynamically from JSON object keys supplied by the client. The keys are joined
directly into the query string as column names:

```rust
let col_list = columns.join(", ");
let sql = format!("INSERT INTO {table} ({col_list}) VALUES ...");
```

This violates postgres.md ("parameterized queries only; never interpolate user input
into SQL") and creates an SQL injection vector: an authenticated client can craft a
payload key containing SQL syntax and execute arbitrary statements.

The root cause is a design choice to make the sync engine generic across all entity
types. Two fix strategies were considered:

1. **Per-entity typed structs** — deserialize each entity type into a concrete Rust
   struct; derive the column list from struct field names. Eliminates the generic path
   entirely. Maximum type safety, but requires a separate struct per entity type (~10
   types) and re-serialization of every field on every sync operation.

2. **Per-entity column allowlist + regex guard** — maintain a static function
   `allowed_columns(entity_type: &EntityType) -> &'static [&'static str]` that
   returns the permitted column names for each type. Validate every key from the
   client payload against this allowlist before including it in the SQL. Add a second
   defence layer: reject any key that does not match `^[a-z_][a-z0-9_]*$` even if
   accidentally not caught by the allowlist.

## Decision

Use option **2**: per-entity column allowlist with a regex defence layer.

Reasons:

1. **Minimal structural change**: The generic INSERT/UPDATE path is otherwise sound;
   replacing it with 10 typed structs changes the entire service layer and is out of
   scope for a security fix.

2. **Allowlist is authoritative**: `allowed_columns` is the single source of truth for
   which fields clients may write via sync. Adding a new entity type forces the author
   to update the allowlist explicitly — an omission causes a `BadRequest` rather than
   silently writing the field.

3. **Regex as defence-in-depth**: The `^[a-z_][a-z0-9_]*$` guard catches SQL metachar-
   acters (spaces, semicolons, quotes, parentheses) regardless of allowlist coverage.
   Even if a developer inadvertently adds a dangerous string to the allowlist, the
   regex blocks it.

4. **Values remain parameterized**: Column names come from the allowlist (static
   strings in the binary); values are still bound as `$N` parameters. There is no
   mechanism for a client-supplied value to affect query structure.

**Allowlist contract**:
- `allowed_columns` is `pub(crate)` and lives in `service.rs`
- Each `EntityType` arm returns a `&'static [&'static str]` of lowercase snake_case
  column names that clients may write
- `id` and `user_id`/`household_id` ownership columns are **excluded** from the
  allowlist; they are set unconditionally from `envelope.entity_id` and
  `auth.user_id` respectively
- If a payload key is not in the allowlist, skip it silently (client may send a
  superset of fields; unknown extras are ignored rather than causing a 400)
- If **no** allowlisted key survives (empty column list), return
  `AppError::BadRequest("no valid payload columns")`

## Consequences

- SQL injection via crafted payload keys is eliminated.
- The allowlist must be updated whenever a new entity type or column is added to sync.
  This is a feature gate, not a burden.
- `apply_update_to_table` now applies payload fields (fixing P5-003 in the same change).
- Tests that send payloads with unknown keys must not regress — extras are silently
  dropped.

## Relates To

- P5-001 (review finding — SQL injection)
- P5-003 (review finding — update handler discards payload; fixed in the same change)
- postgres.md (parameterized queries mandate)
