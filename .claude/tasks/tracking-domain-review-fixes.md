# Plan: Tracking Domain Review Fixes

## Task Description

Address all findings from the comprehensive PR #23 review ("feat: Tracking domain backend (Step 10)"). The review identified 4 critical issues, 6 important issues, and 4 suggestions across security (authorization bypass), data integrity (missing enums, broken double-option semantics, negative quantities), error handling (missing FK violation mapping, transaction safety), code quality (duplicated functions, missing tests, incorrect comments), and API design (no pagination).

## Objective

Fix all critical and important issues, address all suggestions, and add missing tests so PR #23 is ready for merge with no known defects.

## Problem Statement

The Tracking domain backend has:

1. A **security hole**: shopping list item operations don't verify the item belongs to the specified list (cross-household data manipulation)
2. **Data integrity gaps**: `UpdateItemRequest` can't clear nullable fields to NULL; `status`/`event_type` fields accept arbitrary strings; quantity can go negative; transaction doesn't verify row update
3. **Poor error UX**: invalid status/event_type values and FK violations produce generic 500 errors instead of descriptive 400/409 responses
4. **Missing tests**: items and shopping_lists sub-modules have zero test coverage
5. **Code quality issues**: duplicated auth helper, incorrect comments, missing serde annotations

## Solution Approach

Follow the established codebase patterns exactly:

- Add `TrackingItemStatus`, `ShoppingListStatus`, `ItemEventType` enums to `contracts.rs` (following `InitiativeStatus` pattern)
- Apply `Option<Option<T>>` with `double_option` deserializer on all nullable update fields (following `UpdateInitiativeRequest` and `UpdateLocationRequest` patterns)
- Add targeted `is_foreign_key_violation()` / `is_unique_violation()` error mapping (following `create_category` pattern)
- Add DB migrations for CHECK constraints and ON DELETE behavior
- Extract shared `verify_household_membership` utility
- Add model validation tests matching locations/categories test patterns
- Add pagination support to list endpoints

## Relevant Files

### Existing Files to Modify

- `apps/server/src/contracts.rs` - Add 3 new enums (TrackingItemStatus, ShoppingListStatus, ItemEventType) with Display, as_str, and tests
- `apps/server/src/tracking/items/models.rs` - Fix UpdateItemRequest double-option, use enums for status/event_type, add validation tests
- `apps/server/src/tracking/items/service.rs` - Fix query builder for double-option, add rows_affected check in transaction, add FK/unique error mapping
- `apps/server/src/tracking/items/handlers.rs` - Use shared verify_household_membership, remove local copy
- `apps/server/src/tracking/shopping_lists/models.rs` - Use ShoppingListStatus enum, add validation to UpdateShoppingListItemRequest, fix unit double-option, add tests
- `apps/server/src/tracking/shopping_lists/service.rs` - Add shopping_list_id to WHERE clauses (auth fix), add FK error mapping, fix query builder for double-option
- `apps/server/src/tracking/shopping_lists/handlers.rs` - Pass shopping_list_id to service functions, use shared verify helper
- `apps/server/src/tracking/categories/models.rs` - Add serde annotations to parent_category_id on UpdateCategoryRequest
- `apps/server/src/tracking/categories/service.rs` - Add FK violation handling to delete_category
- `apps/server/src/tracking/categories/handlers.rs` - Use shared verify_household_membership
- `apps/server/src/tracking/locations/models.rs` - Add serde annotations to parent_location_id on UpdateLocationRequest
- `apps/server/src/tracking/locations/service.rs` - Add FK violation handling to delete_location
- `apps/server/src/tracking/locations/handlers.rs` - Use shared verify_household_membership
- `apps/server/src/tracking/mod.rs` - Add shared verify_household_membership utility function
- `apps/server/src/api/mod.rs` - Remove incorrect route-ordering NOTE comment
- `docs/schema/altair-initial-schema.sql` - Update reference schema with new constraints

### New Files to Create

- `apps/server/migrations/20260326200009_add_quantity_check_constraint.sql` - Add CHECK (quantity >= 0) to tracking_items
- `apps/server/migrations/20260326200010_add_fk_on_delete_behaviors.sql` - Add ON DELETE SET NULL to category_id, location_id; add ON DELETE SET NULL to shopping_list_items.item_id

## Implementation Phases

### Phase 1: Foundation

Add the new enums to `contracts.rs`, create DB migrations, and extract the shared household membership verification utility. These are prerequisites for Phase 2.

### Phase 2: Core Fixes

Fix all 4 sub-modules in parallel, applying enum types, double-option semantics, error handling, authorization fixes, and service-layer safety checks. Each sub-module is an independent unit of work.

### Phase 3: Tests, Pagination & Polish

Add missing model tests for items and shopping_lists, add pagination to list endpoints, fix comments/docs, and run final validation.

## Team Orchestration

- You operate as the team lead and orchestrate the team to execute the plan.
- You're responsible for deploying the right team members with the right context to execute the plan.
- IMPORTANT: You NEVER operate directly on the codebase. You use `Task` and `Task*` tools to deploy team members to the building, validating, testing, deploying, and other tasks.
  - This is critical. Your job is to act as a high level director of the team, not a builder.
  - Your role is to validate all work is going well and make sure the team is on track to complete the plan.
  - You'll orchestrate this by using the Task\* Tools to manage coordination between the team members.
  - Communication is paramount. You'll use the Task\* Tools to communicate with the team members and ensure they're on track to complete the plan.
- Take note of the session id of each team member. This is how you'll reference them.

### Team Members

- Specialist
  - Name: builder-foundation
  - Role: Add enums to contracts.rs, create DB migrations, extract shared verify_household_membership utility
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-items
  - Role: Fix items sub-module (models, service, handlers) with double-option, enums, error handling, transaction safety
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-shopping
  - Role: Fix shopping_lists sub-module (models, service, handlers) with auth fix, enums, error handling, double-option
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-cats-locs
  - Role: Fix categories and locations sub-modules (serde annotations, FK error handling, shared verify helper)
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-tests-polish
  - Role: Add model tests for items and shopping_lists, add pagination to list endpoints, fix comments
  - Agent Type: backend-engineer
  - Resume: true

- Quality Engineer (Validator)
  - Name: validator
  - Role: Validate completed work against acceptance criteria (read-only inspection mode)
  - Agent Type: quality-engineer
  - Resume: false

## Step by Step Tasks

- IMPORTANT: Execute every step in order, top to bottom. Each task maps directly to a `TaskCreate` call.
- Before you start, run `TaskCreate` to create the initial task list that all team members can see and execute.

### 1. Add Tracking Domain Enums to contracts.rs

- **Task ID**: add-enums
- **Depends On**: none
- **Assigned To**: builder-foundation
- **Agent Type**: backend-engineer
- **Parallel**: false (must complete before Phase 2)
- Add `TrackingItemStatus` enum with variants: `Active`, `Archived`. Follow the exact `InitiativeStatus` pattern: `#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]`, `#[serde(rename_all = "snake_case")]`, `impl fmt::Display`, `fn as_str()`.
- Add `ShoppingListStatus` enum with variants: `Active`, `Completed`, `Archived`. Same pattern.
- Add `ItemEventType` enum with variants: `Consumed`, `Restocked`, `Moved`, `Adjusted`, `Expired`, `Donated`. Same pattern.
- Add serde roundtrip, unknown-string-fails, and as*str tests for each enum (matching `initiative_status*\*` test patterns in contracts.rs).
- File: `apps/server/src/contracts.rs`

### 2. Create Database Migrations

- **Task ID**: add-migrations
- **Depends On**: none
- **Assigned To**: builder-foundation
- **Agent Type**: backend-engineer
- **Parallel**: true (independent of enums)
- Create `apps/server/migrations/20260326200009_add_quantity_check_constraint.sql`:
  - `ALTER TABLE tracking_items ADD CONSTRAINT chk_tracking_items_quantity_non_negative CHECK (quantity >= 0);`
- Create `apps/server/migrations/20260326200010_add_fk_on_delete_behaviors.sql`:
  - Drop and re-add FK on `tracking_items.category_id` with `ON DELETE SET NULL`
  - Drop and re-add FK on `tracking_items.location_id` with `ON DELETE SET NULL`
  - Drop and re-add FK on `tracking_shopping_list_items.item_id` with `ON DELETE SET NULL`
- Update `docs/schema/altair-initial-schema.sql` reference schema to include CHECK constraint and ON DELETE SET NULL on the relevant columns.

### 3. Extract Shared verify_household_membership

- **Task ID**: extract-verify
- **Depends On**: none
- **Assigned To**: builder-foundation
- **Agent Type**: backend-engineer
- **Parallel**: true (independent of enums and migrations)
- Add a public `verify_household_membership` function in `apps/server/src/tracking/mod.rs`:

  ```rust
  use sqlx::PgPool;
  use uuid::Uuid;
  use crate::auth::service::get_user_household_ids;
  use crate::error::AppError;

  pub async fn verify_household_membership(
      pool: &PgPool,
      user_id: Uuid,
      household_id: Uuid,
  ) -> Result<(), AppError> {
      let household_ids = get_user_household_ids(pool, user_id).await?;
      if !household_ids.contains(&household_id) {
          return Err(AppError::Forbidden(
              "Not a member of this household".to_string(),
          ));
      }
      Ok(())
  }
  ```

- This function replaces the identical copies in categories/handlers.rs, locations/handlers.rs, and items/handlers.rs, and the inline logic in shopping_lists/handlers.rs.

### 4. Fix Items Sub-Module

- **Task ID**: fix-items
- **Depends On**: add-enums, extract-verify
- **Assigned To**: builder-items
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside fix-shopping, fix-cats-locs)
- **models.rs** changes:
  - Import `crate::contracts::{TrackingItemStatus, ItemEventType}`
  - In `CreateItemRequest`: change `status` field type (if it existed) or leave as-is (create uses DB default). Add `#[validate(range(min = 0))]` to `quantity`.
  - In `UpdateItemRequest`: change `description` to `Option<Option<String>>` with `#[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_util::double_option")]`. Do the same for `category_id` (Option<Option<Uuid>>), `location_id` (Option<Option<Uuid>>), `unit` (Option<Option<String>>), `min_quantity` (Option<Option<i32>>), `barcode` (Option<Option<String>>). Change `status` from `Option<String>` to `Option<TrackingItemStatus>`.
  - In `CreateItemEventRequest`: change `event_type` from `String` to `ItemEventType`. Remove the `Validate` derive since the enum handles validation via serde deserialization (or keep it and add `#[validate(length(min = 1))]` to `notes` if desired).
- **service.rs** changes:
  - In `create_item`: add FK/unique violation error mapping (match on `is_foreign_key_violation()` and `is_unique_violation()` like `create_category` does).
  - In `update_item`: change all `if let Some(...)` patterns to `match` arms supporting `Some(None)` (set NULL), `Some(Some(val))` (set value), `None` (skip). For `status`, bind `status.as_str()`. Pattern follows `update_category`/`update_location` in this PR.
  - In `update_item`: add FK violation error mapping on the `build_query_as` result.
  - In `create_item_event`: bind `req.event_type.as_str()` instead of `&req.event_type`. After the UPDATE query, check `rows_affected()` and return `AppError::NotFound` if 0.
  - In `create_item`: validate `quantity >= 0` at app layer or rely on DB CHECK.
- **handlers.rs** changes:
  - Remove local `verify_household_membership` function.
  - Import and use `crate::tracking::verify_household_membership` instead.
  - Add caller-responsibility doc comment to service functions that need it (matching categories/locations pattern).

### 5. Fix Shopping Lists Sub-Module

- **Task ID**: fix-shopping
- **Depends On**: add-enums, extract-verify
- **Assigned To**: builder-shopping
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside fix-items, fix-cats-locs)
- **CRITICAL AUTH FIX** in service.rs:
  - `update_list_item`: add `shopping_list_id: Uuid` parameter. Add `AND shopping_list_id = ` + `push_bind(shopping_list_id)` to the WHERE clause.
  - `remove_list_item`: add `shopping_list_id: Uuid` parameter. Change query to `DELETE FROM tracking_shopping_list_items WHERE id = $1 AND shopping_list_id = $2`.
  - `toggle_check`: add `shopping_list_id: Uuid` parameter. Add `AND shopping_list_id = ` to the WHERE clause.
- **handlers.rs** changes:
  - Pass `path.id` (the shopping list ID) to `update_list_item`, `remove_list_item`, and `toggle_check` service calls.
  - Replace inline household membership checks with `crate::tracking::verify_household_membership`. For list-level operations (create_list, list_lists, get_list, update_list, delete_list), use the shared function. For item-level operations, keep `verify_item_access` helper but have it call the shared function internally.
- **models.rs** changes:
  - Import `crate::contracts::ShoppingListStatus`.
  - In `UpdateShoppingListRequest`: change `status` from `Option<String>` to `Option<ShoppingListStatus>`.
  - In `UpdateShoppingListItemRequest`: add `#[validate(length(min = 1, max = 200))]` to `name`. Change `unit` to `Option<Option<String>>` with double_option deserializer.
  - In `CreateShoppingListItemRequest`: add `#[validate(range(min = 1))]` to `quantity`.
- **service.rs** additional changes:
  - In `create_list` and `add_list_item`: add FK violation error mapping.
  - In `update_list`: for `status` field, bind `status.as_str()`.
  - In `update_list_item`: handle `unit` with double-option match arms. Add comment explaining `SET id = id` no-op: `// No updated_at column on this table; use no-op SET to anchor the dynamic column list`

### 6. Fix Categories and Locations Sub-Modules

- **Task ID**: fix-cats-locs
- **Depends On**: extract-verify
- **Assigned To**: builder-cats-locs
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside fix-items, fix-shopping)
- **categories/models.rs**: Add `#[serde(default, skip_serializing_if = "Option::is_none", deserialize_with = "crate::serde_util::double_option")]` to `parent_category_id` on `UpdateCategoryRequest`. Update struct-level doc comment to mention both `description` and `parent_category_id` use double-option.
- **locations/models.rs**: Same change for `parent_location_id` on `UpdateLocationRequest`.
- **categories/service.rs**: In `delete_category`, add FK violation error mapping: `is_foreign_key_violation()` -> `AppError::Conflict("Cannot delete: items still reference this category".to_string())`.
- **locations/service.rs**: Same for `delete_location`: `AppError::Conflict("Cannot delete: items still reference this location".to_string())`.
- **categories/handlers.rs**: Remove local `verify_household_membership`, import and use `crate::tracking::verify_household_membership`.
- **locations/handlers.rs**: Same change.
- Note: The `double_option` deserializer in `serde_util.rs` is generic over `T: Deserialize`, so it works for both `String` and `Uuid` types without changes.

### 7. Add Items Model Tests

- **Task ID**: add-items-tests
- **Depends On**: fix-items
- **Assigned To**: builder-tests-polish
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside add-shopping-tests)
- Add `#[cfg(test)] mod tests` block to `apps/server/src/tracking/items/models.rs` matching the pattern in locations/models.rs:
  - `CreateItemRequest`: valid name passes, empty name fails, name over 200 fails, name exactly 200 passes
  - `UpdateItemRequest`: valid name passes, empty name fails, all-None passes
  - `UpdateItemRequest` double-option tests for description: absent is None, explicit null is Some(None), with value is Some(Some)
  - `UpdateItemRequest` double-option tests for category_id: absent is None, explicit null is Some(None)
  - `CreateItemEventRequest` with valid ItemEventType variant
  - `CreateItemEventRequest` serde rejects invalid event_type string
  - `UpdateItemRequest` with valid TrackingItemStatus variant
  - `UpdateItemRequest` serde rejects invalid status string

### 8. Add Shopping Lists Model Tests

- **Task ID**: add-shopping-tests
- **Depends On**: fix-shopping
- **Assigned To**: builder-tests-polish
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside add-items-tests)
- Add `#[cfg(test)] mod tests` block to `apps/server/src/tracking/shopping_lists/models.rs`:
  - `CreateShoppingListRequest`: valid name passes, empty name fails, name over 200 fails, name exactly 200 passes
  - `CreateShoppingListItemRequest`: valid name passes, empty name fails
  - `UpdateShoppingListRequest`: all-None passes, valid ShoppingListStatus variant passes, invalid status string rejected
  - `UpdateShoppingListItemRequest`: all-None passes, empty name update fails, unit double-option tests (absent/null/value)

### 9. Add Pagination to List Endpoints

- **Task ID**: add-pagination
- **Depends On**: fix-items, fix-shopping, fix-cats-locs
- **Assigned To**: builder-tests-polish
- **Agent Type**: backend-engineer
- **Parallel**: false (depends on Phase 2 completion)
- Add a shared `PaginationParams` struct in `apps/server/src/tracking/mod.rs`:
  ```rust
  #[derive(Debug, Deserialize)]
  pub struct PaginationParams {
      pub limit: Option<i64>,
      pub offset: Option<i64>,
  }
  impl PaginationParams {
      pub fn limit_or_default(&self) -> i64 { self.limit.unwrap_or(100).min(500) }
      pub fn offset_or_default(&self) -> i64 { self.offset.unwrap_or(0).max(0) }
  }
  ```
- Update all list query structs to include `limit` and `offset` fields (flatten PaginationParams or add directly).
- Append `LIMIT $N OFFSET $M` to all list SQL queries. Use `push_bind` for the values.
- The item events endpoint (`list_item_events`) is the highest priority since it is an append-only log.

### 10. Fix Comments and Docs

- **Task ID**: fix-comments
- **Depends On**: fix-items, fix-shopping, fix-cats-locs
- **Assigned To**: builder-tests-polish
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside add-pagination)
- Remove the incorrect NOTE comment in `apps/server/src/api/mod.rs` line 103: `// NOTE: /low-stock must be registered before /{id} to avoid path conflict`. Axum's trie router resolves these unambiguously regardless of registration order.
- Add caller-responsibility doc comment (`/// The caller is responsible for verifying household membership...`) to `create_item` and all shopping_lists service functions, matching the pattern in categories/locations services.
- Add inline comment before the quantity UPDATE in `create_item_event`: `// Atomically adjust the item's running quantity to stay in sync with the event log`
- Update UpdateShoppingListRequest and UpdateShoppingListItemRequest doc comments to mention partial-update semantics (matching categories/locations pattern).
- Remove `skip_serializing_if` from deserialization-only structs (UpdateCategoryRequest, UpdateLocationRequest, UpdateItemRequest, etc.) since they derive Deserialize but not Serialize, making this attribute dead code.

### 11. Validate All Changes

- **Task ID**: validate-all
- **Depends On**: add-items-tests, add-shopping-tests, add-pagination, fix-comments
- **Assigned To**: validator
- **Agent Type**: quality-engineer
- **Parallel**: false
- Run `cargo build` and verify zero errors
- Run `cargo test` and verify all tests pass (expect ~140+ tests now)
- Run `cargo clippy -- -D warnings` and verify no warnings
- Verify the auth fix: confirm all 3 shopping list item service functions (`update_list_item`, `remove_list_item`, `toggle_check`) include `shopping_list_id` in their WHERE clauses
- Verify enum usage: grep for `status: Option<String>` and `event_type: String` in tracking module -- should find zero occurrences
- Verify double-option: grep for `double_option` in items/models.rs -- should find occurrences for description, category_id, location_id, unit, min_quantity, barcode
- Verify shared verify: grep for `fn verify_household_membership` -- should appear exactly once (in tracking/mod.rs), not in individual handler files
- Verify migrations: confirm 10 migration files exist in `apps/server/migrations/`
- Verify no regression: reference schema in `docs/schema/altair-initial-schema.sql` matches migration changes
- Operate in validation mode: inspect and report only, do not modify files

## Acceptance Criteria

1. `cargo build` compiles with zero errors
2. `cargo test` passes with all tests green (expect 140+ total)
3. `cargo clippy -- -D warnings` produces zero warnings
4. **Security**: All shopping list item service functions include `shopping_list_id` in WHERE clauses
5. **Enums**: `TrackingItemStatus`, `ShoppingListStatus`, `ItemEventType` exist in contracts.rs with full test coverage
6. **Double-option**: All nullable fields on all Update\*Request types use `Option<Option<T>>` with serde annotations
7. **Error handling**: All create/update/delete service functions handle FK and unique violations with descriptive 400/409 errors
8. **Transaction safety**: `create_item_event` checks `rows_affected()` on the quantity UPDATE
9. **DB constraints**: `CHECK (quantity >= 0)` exists on tracking_items; ON DELETE SET NULL on category_id, location_id, shopping_list_items.item_id
10. **Shared utility**: Single `verify_household_membership` in tracking/mod.rs used by all handler files
11. **Tests**: items/models.rs and shopping_lists/models.rs both have comprehensive validation and serde tests
12. **Pagination**: All list endpoints accept `limit`/`offset` parameters with sensible defaults
13. **Comments**: Incorrect route-ordering NOTE removed; doc comments accurate and consistent

## Validation Commands

Execute these commands to validate the task is complete:

- `cd apps/server && cargo build 2>&1` - Verify the project builds without errors
- `cd apps/server && cargo test 2>&1` - Run the full test suite
- `cd apps/server && cargo clippy -- -D warnings 2>&1` - Run Clippy linter with strict warnings
- `grep -rn "fn verify_household_membership" apps/server/src/tracking/` - Should appear exactly once in mod.rs
- `grep -rn "status: Option<String>" apps/server/src/tracking/` - Should return zero results
- `grep -rn "event_type: String" apps/server/src/tracking/` - Should return zero results
- `grep -rn "double_option" apps/server/src/tracking/items/models.rs` - Should show annotations on nullable fields
- `grep -rn "shopping_list_id" apps/server/src/tracking/shopping_lists/service.rs` - Should appear in update_list_item, remove_list_item, toggle_check WHERE clauses
- `ls apps/server/migrations/20260326200*.sql | wc -l` - Should return 10
- `grep -c "#\[cfg(test)\]" apps/server/src/tracking/items/models.rs` - Should return 1
- `grep -c "#\[cfg(test)\]" apps/server/src/tracking/shopping_lists/models.rs` - Should return 1

## Notes

- The `double_option` deserializer in `serde_util.rs` is generic (`T: Deserialize`), so it works for String, Uuid, and i32 without modification.
- When adding `Option<Option<Uuid>>` fields with double_option, verify serde handles Uuid deserialization correctly. The existing test in serde_util.rs only tests String. Consider adding a Uuid test if time permits.
- The `TrackingItem.status` DB column type remains `String` (for sqlx::FromRow compatibility), but request types use the enum. The service layer calls `.as_str()` when binding to SQL.
- Shopping list item auth fix is the highest priority change since it is a security vulnerability. Deploy builder-shopping first if sequential execution is needed.
- The pagination implementation should be backward compatible -- existing clients that don't send limit/offset get the default (100 items).
