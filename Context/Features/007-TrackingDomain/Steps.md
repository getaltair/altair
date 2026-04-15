# Implementation Steps: Tracking Domain

**Spec:** Context/Features/007-TrackingDomain/Spec.md
**Tech:** Context/Features/007-TrackingDomain/Tech.md

## Progress
- **Status:** Complete
- **Current task:** --
- **Last milestone:** Milestone 6 — Feature complete (2026-04-15)

## Team Orchestration

### Team Members

- **builder**
  - Role: Implements all Rust/Axum server code for the tracking domain
  - Agent Type: backend-engineer
  - Resume: false

- **validator**
  - Role: Read-only quality and integration validation
  - Agent Type: qa-rust
  - Resume: false

---

## Tasks

### Phase 1: Infrastructure

- [ ] S001: Add `impl From<sqlx::Error> for AppError` to `apps/server/server/src/error.rs`
  - Adds a single `From` impl so service functions can use `?` on sqlx queries without inline `.map_err`. Does not change any existing handlers or variants.
  - **Assigned:** builder
  - **Depends:** none
  - **Parallel:** false

- [ ] S001-T: Test `From<sqlx::Error>` conversion in error.rs (converts to `Internal` variant; status code is 500; error detail not leaked in response body)
  - **Assigned:** builder
  - **Depends:** S001
  - **Parallel:** false

---

🏁 MILESTONE 1: Infrastructure complete — `error.rs` has `From<sqlx::Error>`; all service functions can use `?` on sqlx queries
  **Contracts:**
  - `apps/server/server/src/error.rs` — AppError enum with all variants and From<sqlx::Error> impl

---

### Phase 2: Module Skeleton

- [ ] S002: Create the `src/tracking/` module skeleton
  - Create `apps/server/server/src/tracking/mod.rs` declaring all six sub-modules and a stub `router()` function that composes their routers under `/api/tracking`.
  - Create `apps/server/server/src/tracking/household.rs` with `assert_household_member(pool: &PgPool, user_id: Uuid, household_id: Uuid) -> Result<(), AppError>` — queries `household_memberships` and returns `AppError::Forbidden` if the user is not a member.
  - Create stub `mod.rs` for each sub-module: `locations/`, `categories/`, `items/`, `item_events/`, `shopping_lists/`, `shopping_list_items/`. Each stub declares the four child modules (`mod.rs`, `models.rs`, `service.rs`, `handlers.rs`) and exports a stub `router()`.
  - Add `pub mod tracking;` to `apps/server/server/src/lib.rs`.
  - Register `tracking::router()` in `apps/server/server/src/routes.rs`.
  - **Assigned:** builder
  - **Depends:** S001
  - **Parallel:** false

- [ ] S002-T: Test `assert_household_member` (member returns Ok; non-member returns Forbidden; unknown household returns Forbidden; wrong user_id returns Forbidden)
  - **Assigned:** builder
  - **Depends:** S002
  - **Parallel:** false

---

### Phase 3: Reference Entities (parallel)

- [ ] S003: Implement locations CRUD in `apps/server/server/src/tracking/locations/`
  - `models.rs`: `TrackingLocationRow` (`#[derive(sqlx::FromRow)]`), `TrackingLocation` (`#[derive(Serialize)]`, excludes `household_id` and `deleted_at`), `From<TrackingLocationRow> for TrackingLocation`, `CreateLocationRequest`, `UpdateLocationRequest`.
  - `service.rs`: `list_locations`, `get_location`, `create_location`, `update_location`, `delete_location` — all assert household membership, filter `deleted_at IS NULL` on reads, set `deleted_at = NOW()` on delete.
  - `handlers.rs`: thin Axum handlers for `GET/POST /api/tracking/locations` and `GET/PATCH/DELETE /api/tracking/locations/{id}`. Extract `AuthUser` via existing extractor; read `household_id` from query param.
  - `mod.rs`: re-exports + `router()`.
  - **Assigned:** builder
  - **Depends:** S002
  - **Parallel:** true

- [ ] S003-T: Integration tests for locations (FA-001: non-member gets 403; FA-002: member creates location, gets 201; FA-003: two households isolated in list; FA-015: soft-deleted location absent from list)
  - **Assigned:** builder
  - **Depends:** S003
  - **Parallel:** false

- [ ] S004: Implement categories CRUD in `apps/server/server/src/tracking/categories/`
  - Same structure as S003. `TrackingCategoryRow`, `TrackingCategory`, `CreateCategoryRequest`, `UpdateCategoryRequest`.
  - All service functions assert household membership. Soft-delete pattern identical to locations.
  - Routes: `GET/POST /api/tracking/categories` and `GET/PATCH/DELETE /api/tracking/categories/{id}`.
  - **Assigned:** builder
  - **Depends:** S002
  - **Parallel:** true

- [ ] S004-T: Integration tests for categories (non-member gets 403; member creates category, gets 201; two households isolated; soft-deleted category absent from list)
  - **Assigned:** builder
  - **Depends:** S004
  - **Parallel:** false

---

🏁 MILESTONE 2: Reference entities complete — verify FA-001, FA-002, FA-003, FA-015 for locations/categories
  **Contracts:**
  - `apps/server/server/src/tracking/locations/models.rs` — TrackingLocationRow schema and TrackingLocation response type; items service validates location household via service call
  - `apps/server/server/src/tracking/categories/models.rs` — TrackingCategoryRow schema and TrackingCategory response type

---

### Phase 4: Item Layer

- [ ] S005: Implement items CRUD in `apps/server/server/src/tracking/items/`
  - `models.rs`: `TrackingItemRow`, `TrackingItem` (excludes `household_id`, `deleted_at`), `CreateItemRequest` (includes optional `id: Option<Uuid>` per invariant E-2), `UpdateItemRequest`.
  - `service.rs`:
    - `create_item`: if `req.id` is `Some`, use it; otherwise `gen_random_uuid()`. Detect duplicate key (`sqlx::Error::Database` code `23505`) → `AppError::Conflict`. Validate `location_id` belongs to same household (E-8): `SELECT household_id FROM tracking_locations WHERE id = $1 AND deleted_at IS NULL`; if different household → `AppError::UnprocessableEntity`.
    - `list_items`: accepts optional `category_id` and `location_id` filters; required `household_id`; offset/limit pagination (default `limit=50, offset=0`).
    - `get_item`, `update_item`, `delete_item`: standard pattern with household membership check.
  - `handlers.rs`: routes for `GET/POST /api/tracking/items` and `GET/PATCH/DELETE /api/tracking/items/{id}`.
  - **Assigned:** builder
  - **Depends:** S003, S004
  - **Parallel:** false

- [ ] S005-T: Integration tests for items (FA-004: location from different household rejected with 422; FA-005: duplicate UUID rejected with 409; member creates item with supplied UUID; item list filters by category; FA-015: soft-deleted item absent from list)
  - **Assigned:** builder
  - **Depends:** S005
  - **Parallel:** false

---

🏁 MILESTONE 3: Item layer complete — verify FA-004, FA-005, FA-015 for items
  **Contracts:**
  - `apps/server/server/src/tracking/items/models.rs` — TrackingItemRow schema; item_events and shopping_list_items services reference item household via item row

---

### Phase 5: Events and Shopping Lists (parallel)

- [ ] S006: Implement item events in `apps/server/server/src/tracking/item_events/`
  - `models.rs`: `TrackingItemEventRow`, `TrackingItemEvent` (excludes `household_id`), `CreateItemEventRequest` (includes `id: Option<Uuid>` per invariant E-2), `ItemEventType` enum (`#[derive(sqlx::Type)]` — `restock`, `consume`, `purchase`, `purchase_reversed`, `adjustment`).
  - `service.rs`:
    - `create_item_event`: wraps a sqlx transaction. Within the transaction: `SELECT id FROM tracking_items WHERE id = $1 AND household_id = $2 FOR UPDATE` (acquires row lock); `SELECT COALESCE(SUM(quantity_delta), 0) FROM tracking_item_events WHERE item_id = $1` (derives current quantity); if `current_qty + req.quantity_delta < 0` → rollback → `AppError::UnprocessableEntity` (invariant E-7). INSERT event, COMMIT.
    - `create_item_event_in_tx(tx: &mut Transaction<'_, Postgres>, ...)`: same logic but accepts a caller-supplied transaction — used by the shopping list items service for the purchased side effect. Public within the crate (`pub(crate)`).
    - `list_item_events`: ordered by `created_at ASC`; optional `limit`/`offset` (default limit=50). Never filters by `deleted_at` — events are permanent (invariant D-5).
    - No `delete_item_event` function. Handler for `DELETE /api/tracking/items/{id}/events/{event_id}` returns `AppError::BadRequest("item events are immutable")` mapped to 405 by the router.
  - `handlers.rs`: `GET/POST /api/tracking/items/{id}/events`. `DELETE` route registered and returns 405.
  - **Assigned:** builder
  - **Depends:** S005
  - **Parallel:** true

- [ ] S006-T: Integration tests for item events (FA-006: negative quantity rejected with 422; FA-007: duplicate event ID rejected with 409; FA-008: all N events returned in insertion order; FA-009: DELETE returns 405; concurrent drain test with SELECT FOR UPDATE)
  - **Assigned:** builder
  - **Depends:** S006
  - **Parallel:** false

- [ ] S007: Implement shopping lists CRUD in `apps/server/server/src/tracking/shopping_lists/`
  - `models.rs`: `TrackingShoppingListRow`, `TrackingShoppingList` (excludes `household_id`, `deleted_at`), `CreateShoppingListRequest`, `UpdateShoppingListRequest`.
  - `service.rs`: standard CRUD with household membership check. Soft-delete pattern.
  - `handlers.rs`: `GET/POST /api/tracking/shopping_lists` and `GET/PATCH/DELETE /api/tracking/shopping_lists/{id}`.
  - **Assigned:** builder
  - **Depends:** S002
  - **Parallel:** true

- [ ] S007-T: Integration tests for shopping lists (non-member gets 403; member creates list; household isolation; soft-deleted list absent from list endpoint)
  - **Assigned:** builder
  - **Depends:** S007
  - **Parallel:** false

---

🏁 MILESTONE 4: Events and shopping lists complete — verify FA-006–FA-009 and shopping list CRUD
  **Contracts:**
  - `apps/server/server/src/tracking/item_events/service.rs` — `create_item_event_in_tx` signature (pub crate); shopping list items service calls this for the purchased side effect
  - `apps/server/server/src/tracking/shopping_lists/models.rs` — TrackingShoppingListRow schema

---

### Phase 6: Shopping List Items

- [ ] S008: Implement shopping list items in `apps/server/server/src/tracking/shopping_list_items/`
  - `models.rs`: `TrackingShoppingListItemRow`, `TrackingShoppingListItem` (excludes `deleted_at`), `CreateShoppingListItemRequest` (includes `item_id: Option<Uuid>`), `UpdateShoppingListItemRequest` (contains `status: ShoppingListItemStatus`), `ShoppingListItemStatus` enum with `can_transition_to()` method per `06-state-machines.md` Rust pattern.
    Valid transitions: `Pending → Purchased`, `Pending → Removed`, `Purchased → Pending`. `Removed` is terminal.
  - `service.rs`:
    - `add_shopping_list_item`: validate `item_id` (if present) belongs to same household as list (invariant E-9): query `tracking_items WHERE id = $1 AND household_id = $2`; if not found → `AppError::UnprocessableEntity`.
    - `update_shopping_list_item`: load current item, call `can_transition_to()`, return `AppError::UnprocessableEntity` for invalid transitions.
      - If transitioning to `Purchased` and `item_id IS NOT NULL`: begin transaction, UPDATE shopping list item status, call `item_events::service::create_item_event_in_tx(tx, ...)` with `event_type = consume, quantity_delta = -1`. If quantity check fails (E-7), rollback entire transaction.
      - If transitioning `Purchased → Pending` and `item_id IS NOT NULL`: begin transaction, UPDATE status, call `create_item_event_in_tx(tx, ...)` with `event_type = purchase_reversed, quantity_delta = +1`. Compensating event cannot violate E-7 (adds quantity back), so no E-7 check needed — document with comment.
    - `remove_shopping_list_item`: sets `deleted_at = NOW()`.
    - `list_shopping_list_items`: filters `deleted_at IS NULL`.
  - `handlers.rs`: `GET/POST /api/tracking/shopping_lists/{id}/items` and `PATCH/DELETE /api/tracking/shopping_lists/{id}/items/{item_id}`.
  - **Assigned:** builder
  - **Depends:** S006, S007
  - **Parallel:** false

- [ ] S008-T: Integration tests for shopping list items (FA-010: item from different household rejected with 422; FA-011: pending → purchased creates item event in same transaction; FA-012: pending → removed succeeds; FA-013: removed → any returns 422; FA-014: purchased → pending inserts compensating event; ShoppingListItemStatus::can_transition_to exhaustive unit test for all state pairs)
  - **Assigned:** builder
  - **Depends:** S008
  - **Parallel:** false

---

🏁 MILESTONE 5: Shopping domain complete — verify FA-010–FA-014
  **Contracts:**
  - `apps/server/server/src/tracking/` — full module tree; consumed by Steps 8 (Android) and 9 (Web) for API contract reference

---

### Phase 7: Documentation and Validation

- [ ] S009-D: Update `CLAUDE.md` active work section — mark tracking domain server implementation in progress; note module path `src/tracking/`; update feature status from "Next" to "In progress"
  - **Assigned:** builder
  - **Depends:** S008
  - **Parallel:** false

- [x] S010: Full validation pass — run all tracking integration tests, clippy, fmt
  - **Assigned:** validator
  - **Depends:** S008-T, S009-D
  - **Parallel:** false

---

🏁 MILESTONE 6: Feature complete — verify all FA-001–FA-015; full drift check against Spec.md requirements; all acceptance criteria met

---

---

### Phase 8: Review-Driven Follow-Up Tasks

- [ ] S011: Add HTTP-layer integration tests for all tracking endpoints
  - Add Axum `TestClient` (or equivalent) tests that drive the full handler → service → DB stack for all six tracking sub-modules: locations, categories, items, item_events, shopping_lists, shopping_list_items.
  - At minimum: 200/201 happy path, 400 for bad input, 403 for non-member, 404 for missing resource, 409 for duplicate.
  - These are end-to-end, not unit tests — they exercise routing, extractor, handler validation, service, and DB layers together.
  - **Assigned:** builder
  - **Depends:** S010
  - **Parallel:** false

- [ ] S012: Add test for create_item_event when item has NULL household_id
  - Verifies that `create_item_event` returns `AppError::NotFound` (not a panic or 500) when the target item exists but `household_id IS NULL` — exercising the household isolation check added in the P6-002 fix.
  - **Assigned:** builder
  - **Depends:** S006-T
  - **Parallel:** false

- [ ] S013: Add shopping_list_items isolation tests for remove and list paths
  - Verify that a user from household A cannot `remove` or `list` items in a shopping list owned by household B. Both paths must return 403.
  - **Assigned:** builder
  - **Depends:** S008-T
  - **Parallel:** false

- [ ] S014: Add test verifying transaction rollback prevents status persistence
  - For the `Pending → Purchased` path in `update_shopping_list_item`, simulate a failure in `create_item_event_in_tx` and verify the shopping list item status is NOT persisted (i.e., the transaction rolled back correctly). Guards against partial-commit bugs.
  - **Assigned:** builder
  - **Depends:** S008-T
  - **Parallel:** false

- [ ] S015: Derive `sqlx::Type` on `ShoppingListItemStatus` and eliminate the `str_to_status` helper
  - Add `#[derive(sqlx::Type)]` with `#[sqlx(type_name = "varchar", rename_all = "snake_case")]` so sqlx decodes the status column directly into the enum. Remove the manual `String` field and the `str_to_status` conversion in service.rs. Update `TrackingShoppingListItemRow.status` from `String` to `ShoppingListItemStatus`. Update `TrackingShoppingListItem.status` accordingly.
  - **Assigned:** builder
  - **Depends:** S008
  - **Parallel:** false

- [ ] S015-T: Add test for the `str_to_status` error path before removing it in S015
  - Verify that an unknown status string returns an error (not a panic). This test documents the current behavior and can be removed once S015 makes it obsolete.
  - **Assigned:** builder
  - **Depends:** S008-T
  - **Parallel:** false

- [ ] S016: Change `event_type: String` to `event_type: ItemEventType` in `TrackingItemEventRow` and `TrackingItemEvent`
  - Add `#[derive(sqlx::Type)]` with appropriate rename on `ItemEventType`. Update the row struct, response struct, and the `From` conversion. Remove any manual string-to-enum conversion in service.rs.
  - **Assigned:** builder
  - **Depends:** S006
  - **Parallel:** false

- [ ] S017: Add 403 tests for update, delete, and get on categories and shopping_lists for non-members
  - Extend `categories/service.rs` and `shopping_lists/service.rs` tests to include: non-member calling `update_*`, `delete_*`, and `get_*` each returns `AppError::Forbidden`.
  - **Assigned:** builder
  - **Depends:** S004-T, S007-T
  - **Parallel:** false

- [ ] S018: Add pagination tests for `list_item_events`
  - Seed N > 50 events for an item and verify: default limit returns 50, `limit=N` returns all, `offset=k` returns correct slice, results are ordered `created_at ASC`.
  - **Assigned:** builder
  - **Depends:** S006-T
  - **Parallel:** false

- [ ] S019: Add test for `update_item` with a cross-household `location_id`
  - Verify that `update_item` with a `location_id` belonging to a different household returns `AppError::UnprocessableEntity` (422). Guards invariant E-8 on the update path.
  - **Assigned:** builder
  - **Depends:** S005-T
  - **Parallel:** false

---

## Acceptance Criteria

- [ ] All 15 testable assertions (FA-001–FA-015) pass
- [ ] `cargo test -p server` passes with no failures
- [ ] `cargo clippy -p server -- -D warnings` produces no warnings
- [ ] `cargo fmt -p server -- --check` passes
- [ ] No `unwrap()` in production code paths (no `#[cfg(test)]` exclusion)
- [ ] No inline `.map_err(|e| AppError::Internal(...))` in tracking module
- [ ] All tracking API response types exclude `household_id` and `deleted_at`
- [ ] No TODO/FIXME stubs remaining

## Validation Commands

```bash
# From apps/server/
CARGO_TARGET_DIR=/tmp/cargo-target cargo test -p server -- --test-threads=4
CARGO_TARGET_DIR=/tmp/cargo-target cargo clippy -p server -- -D warnings
CARGO_TARGET_DIR=/tmp/cargo-target cargo fmt -p server -- --check

# Integration tests only (requires running DB):
CARGO_TARGET_DIR=/tmp/cargo-target cargo test -p server --test 'tracking*' -- --test-threads=1
```
