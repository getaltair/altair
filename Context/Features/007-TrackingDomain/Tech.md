# Tech Plan: Tracking Domain

**Spec:** Context/Features/007-TrackingDomain/Spec.md
**Stacks involved:** Rust/Axum (server only)

## Architecture Overview

Feature 007 adds a `src/tracking/` top-level module to `apps/server/server/src/`, parallel to
`src/core/` and `src/guidance/`. The module exposes REST routes for six entities — locations,
categories, items, item events, shopping lists, and shopping list items — under the `/api/tracking/`
prefix.

All six database tables already exist (migrations 019–024). No new migrations are needed.

The tracking domain is the first household-scoped domain: all six tables are partitioned by
`household_id`, not `user_id`. Household membership is verified at the service layer via a helper
that queries `household_memberships`.

A single shared helper (`src/tracking/household.rs`) performs the membership check. All service
functions call this helper before accessing any tracking table.

---

## Key Decisions

### Decision 1: Module structure — flat vs. sub-module per entity

**Options considered:**
- **Flat** (`src/tracking/mod.rs`, `models.rs`, `service.rs`, `handlers.rs`): simpler, matches
  small modules in `src/core/` (tags, relations). Becomes unwieldy with 6 entities and 25+ service
  functions.
- **Sub-module per entity** (`src/tracking/locations/`, `src/tracking/items/`, etc.): each entity
  gets its own `mod.rs / models.rs / service.rs / handlers.rs`. Consistent with the `src/core/`
  pattern for larger entities. More files, but clear ownership boundaries.

**Chosen:** Sub-module per entity.

**Rationale:** Six entities with full CRUD plus nested resources (item events, shopping list items)
would produce a single file exceeding 800 lines in the flat layout. Sub-module per entity matches
the existing pattern for `initiatives/` and keeps each file reviewable. The top-level
`src/tracking/mod.rs` assembles the router from the six sub-routers.

**Related ADRs:** ADR-007 (monorepo structure, establishes module-per-domain precedent)

---

### Decision 2: Household membership check — per-request helper vs. middleware

**Options considered:**
- **Axum middleware / extractor**: check membership in a custom `FromRequestParts` extractor that
  reads `household_id` from the path. Clean ergonomically, but the extractor cannot parse nested
  paths like `/items/{item_id}/events` without knowing the item's household.
- **Per-request service helper**: a plain async function `assert_household_member(pool, user_id,
  household_id)` called at the top of each service function. Explicit, testable, requires no
  framework machinery.

**Chosen:** Per-request service helper.

**Rationale:** Tracking routes carry `household_id` as a query parameter or resolve it from a parent
entity (e.g., item events must look up the item's household). A middleware extractor cannot handle
the second case without a DB lookup of its own. The helper keeps the check close to the data access
and is straightforward to test. Single function defined in `src/tracking/household.rs`.

```rust
pub async fn assert_household_member(
    pool: &PgPool,
    user_id: Uuid,
    household_id: Uuid,
) -> Result<(), AppError> {
    // ...
}
```

---

### Decision 3: Quantity validation — SELECT FOR UPDATE vs. serializable isolation

**Options considered:**
- **Serializable transaction**: set the transaction isolation level to `SERIALIZABLE`. Any
  concurrent write that conflicts causes a serialization failure, which the client must retry.
  Correct, but adds retry logic and can produce spurious aborts on unrelated rows.
- **SELECT FOR UPDATE**: within a single `BEGIN` / `COMMIT`, acquire a row lock on the item with
  `FOR UPDATE`, compute the derived quantity (sum of `quantity_delta`), validate the incoming
  delta, then INSERT the event. Lock is released on commit.

**Chosen:** SELECT FOR UPDATE.

**Rationale:** Only one row (the item) needs to be locked. SELECT FOR UPDATE acquires a precise
lock without row-version conflicts on unrelated tables. Simpler retry semantics — the lock waits
rather than aborting. The service function acquires a `sqlx::Transaction`, runs the check, inserts,
and commits. Invariant E-7 is enforced atomically.

**Implementation sketch:**

```
BEGIN
  SELECT id FROM tracking_items WHERE id = $1 AND household_id = $2 FOR UPDATE
  SELECT COALESCE(SUM(quantity_delta), 0) FROM tracking_item_events WHERE item_id = $1
  if current_qty + new_delta < 0 → ROLLBACK → AppError::UnprocessableEntity
  INSERT INTO tracking_item_events (...)
COMMIT
```

---

### Decision 4: Client-generated UUIDs for items

Items accept a caller-supplied `id` in the create request body (invariant E-2). If the client
omits `id`, the server generates one with `gen_random_uuid()`.

The create handler uses `INSERT INTO tracking_items (id, ...) VALUES ($1, ...)`. On duplicate key
violation (`sqlx::Error::Database` with code `23505`), return `AppError::Conflict`.

All other tracking entities (locations, categories, shopping lists, item events, shopping list
items) use server-generated UUIDs only; `id` is not accepted in their create requests.

---

### Decision 5: Shopping list item `purchased` side effect — atomic item event

Per `docs/specs/06-state-machines.md:222`, transitioning a shopping list item to `purchased`
creates a `purchase` item event if the shopping list item is linked to an inventory item
(`item_id IS NOT NULL`).

This means `PATCH /tracking/shopping_lists/{list_id}/items/{item_id}` with `status: purchased`
must, in a single transaction:

1. Verify household membership.
2. Load the shopping list item; validate the state transition.
3. UPDATE the shopping list item's `status`.
4. If `shopping_list_item.item_id IS NOT NULL`: run the quantity validation (Decision 3) and INSERT
   a `tracking_item_events` row with `event_type = 'purchase'` and `quantity_delta = -1` (one unit
   consumed). If this violates E-7, the whole transaction rolls back and the caller receives 422.
5. COMMIT.

The shopping list items service calls the item events service's internal quantity-check function
(not the HTTP handler) to reuse the validation logic without HTTP overhead. The internal function
accepts a `&mut sqlx::Transaction<'_, Postgres>` so the two writes share one transaction.

Conversely, transitioning `purchased → pending` must roll back the associated `purchase` event. Per
invariant D-5, item events are append-only — they cannot be deleted. Instead, the reverse
transition inserts a compensating event with `event_type = 'purchase_reversed'` and
`quantity_delta = +1`. Same single-transaction pattern.

---

### Decision 6: Row/response type separation (ADR-015 mandate)

Per ADR-015, tracking domain modules must define separate DB and API types from day one:

| Type | Derive | Location | Purpose |
|---|---|---|---|
| `TrackingLocationRow` | `sqlx::FromRow` | `service.rs` | Direct DB mapping |
| `TrackingLocation` | `serde::Serialize` | `models.rs` | API response — no `household_id`, no `deleted_at` |

A `From<TrackingLocationRow> for TrackingLocation` impl converts between them. This pattern
applies to all six entities.

The `household_id` field is present in every DB row but must NOT appear in API responses — callers
already know the household from the request context. The `deleted_at` field is excluded for the
same reason.

**Canonical precedent:** `auth/` module's `MeRow` / `UserProfile` pattern.

---

### Decision 7: `impl From<sqlx::Error> for AppError`

Per `.claude/rules/rust-axum.md`, inline `.map_err(|e| AppError::Internal(...))` on every sqlx
call is a maintenance hazard. One `impl From<sqlx::Error> for AppError` is added to `error.rs`;
all service functions then use `?` directly on sqlx queries.

The existing `initiatives/service.rs` uses the inline anti-pattern (`.to_string()` variant) — that
module is out of scope here. The tracking module must use the new `From` impl from day one.

---

## Stack-Specific Details

### Rust/Axum (`apps/server/`)

**Files to create:**

```
apps/server/server/src/tracking/
  mod.rs                    — router: assembles 6 sub-routers under /api/tracking
  household.rs              — assert_household_member() helper
  locations/
    mod.rs, models.rs, service.rs, handlers.rs
  categories/
    mod.rs, models.rs, service.rs, handlers.rs
  items/
    mod.rs, models.rs, service.rs, handlers.rs
  item_events/
    mod.rs, models.rs, service.rs, handlers.rs
  shopping_lists/
    mod.rs, models.rs, service.rs, handlers.rs
  shopping_list_items/
    mod.rs, models.rs, service.rs, handlers.rs
```

**Files to modify:**

| File | Change |
|---|---|
| `src/lib.rs` | Add `pub mod tracking;` |
| `src/routes.rs` | Add `tracking::router()` to the router |
| `src/error.rs` | Add `impl From<sqlx::Error> for AppError` |

**Route table:**

| Method | Path | Handler |
|---|---|---|
| GET/POST | `/api/tracking/locations` | list, create |
| GET/PATCH/DELETE | `/api/tracking/locations/{id}` | get, update, soft-delete |
| GET/POST | `/api/tracking/categories` | list, create |
| GET/PATCH/DELETE | `/api/tracking/categories/{id}` | get, update, soft-delete |
| GET/POST | `/api/tracking/items` | list, create |
| GET/PATCH/DELETE | `/api/tracking/items/{id}` | get, update, soft-delete |
| GET/POST | `/api/tracking/items/{id}/events` | list events, create event |
| GET/POST | `/api/tracking/shopping_lists` | list, create |
| GET/PATCH/DELETE | `/api/tracking/shopping_lists/{id}` | get, update, soft-delete |
| GET/POST | `/api/tracking/shopping_lists/{id}/items` | list items, add item |
| PATCH | `/api/tracking/shopping_lists/{id}/items/{item_id}` | update state |
| DELETE | `/api/tracking/shopping_lists/{id}/items/{item_id}` | remove item (soft) |

All list endpoints accept `?household_id=<uuid>` as a required query parameter. Items list also
accepts optional `?category_id=<uuid>` and `?location_id=<uuid>` filters.

Item events list accepts optional `?limit=N&offset=M` (default `limit=50`).

**Dependencies:** No new Cargo crates. `uuid`, `sqlx`, `serde`, `axum`, `anyhow` are all present.

**Patterns to follow:**
- Module layout: `.claude/rules/rust-axum.md` — `mod.rs / models.rs / service.rs / handlers.rs`
- State enum pattern: `06-state-machines.md` (Rust section) — `#[derive(sqlx::Type)]` + `can_transition_to` method
- Error handling: `error.rs` — `AppError` variants, `From<sqlx::Error>`
- Row/response separation: `auth/` module — `MeRow` / `UserProfile` pattern (ADR-015)

---

## Integration Points

| Component | Change | Notes |
|---|---|---|
| `src/lib.rs` | `pub mod tracking;` | Registers module |
| `src/routes.rs` | Merge tracking router | Same pattern as sync and core |
| `src/error.rs` | `impl From<sqlx::Error> for AppError` | Enables `?` on sqlx queries in all new service functions |
| PowerSync sync rules | Already includes all 6 tracking tables | Defined in Feature 004; no changes needed |

---

## State Machine Implementation

`ShoppingListItemStatus` is a string-backed enum with a `can_transition_to` method, following the
`QuestStatus` pattern from `06-state-machines.md`:

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar", rename_all = "snake_case")]
pub enum ShoppingListItemStatus {
    Pending,
    Purchased,
    Removed,
}

impl ShoppingListItemStatus {
    pub fn can_transition_to(&self, target: &Self) -> bool {
        matches!(
            (self, target),
            (Self::Pending, Self::Purchased)
                | (Self::Pending, Self::Removed)
                | (Self::Purchased, Self::Pending)
        )
    }
}
```

Transition validation is in `shopping_list_items/service.rs`, not in the handler.

---

## Risks & Unknowns

- **Risk:** `purchased → pending` compensating event breaks E-7 if item quantity is already 0.
  - **Mitigation:** The compensating event adds `quantity_delta = +1` (adds back), so it cannot
    violate the ≥0 invariant. No quantity check needed for the reverse direction. Document this
    explicitly in the service function comment.

- **Risk:** Household membership check adds an extra DB round-trip per request.
  - **Mitigation:** The `household_memberships` table has an index on `(user_id, household_id)` per
    Feature 003 migrations. The check is a single primary-key lookup and will be fast. No caching
    needed at this stage.

- **Risk:** Shopping list item delete vs. state transition semantics: does `removed` state mean
  the same as soft-delete?
  - **Mitigation:** They are different. `removed` is a list-level state ("I decided not to buy
    this"). Soft-delete (`deleted_at IS NOT NULL`) is a data-level tombstone. A `removed` item
    still appears in the shopping list history; a soft-deleted item is gone. The `DELETE` endpoint
    sets `deleted_at`; the state transition PATCH sets `status = removed`.

- **Unknown:** Does the `item_events` table's `household_id` column require denormalization (it
  can be derived from `items.household_id`)?
  - **Resolution:** Check migration 022 schema. If `household_id` is present, include it in
    inserts. If absent, derive from the item join. Either way is fine — the tracking sync rules
    already reference `tracking_item_events.household_id`, so it must exist in the table.

---

## Testing Strategy

- **Unit tests** (in `#[cfg(test)]` within each module): `ShoppingListItemStatus::can_transition_to`
  for all valid and invalid pairings. `From<XxxRow> for Xxx` mapping correctness.
- **Integration tests** (`tests/tracking_*.rs`): cover all 15 testable assertions in `Spec.md`
  (FA-001–FA-015). Use `sqlx::test` with transaction rollback for DB isolation. Focus areas:
  - Household isolation (FA-001, FA-003, FA-004, FA-010)
  - Quantity invariant under concurrency (FA-006) — create two tasks that concurrently attempt
    to drain the same item to test the FOR UPDATE lock
  - Append-only enforcement (FA-008, FA-009)
  - State machine transitions (FA-011–FA-014)
  - Purchased side effect atomicity (FA-011: verify item event is created in same transaction)

---

## ADRs

No new ADRs required. All decisions are governed by or consistent with:

- **ADR-011** — AppError variant taxonomy (UnprocessableEntity covers E-7/E-8/E-9/state-machine violations; Conflict covers duplicate UUID)
- **ADR-014** — RLS deferred; tracking module relies on application-layer household membership checks
- **ADR-015** — Separate DB/API row types; tracking domain implements this from day one
- **ADR-003** — LWW conflict resolution; tracking item events handled as described in Feature 004 Tech.md

---

## Revision History

| Date       | Change        |
|------------|---------------|
| 2026-04-15 | Initial draft |
