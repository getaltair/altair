# Plan: Step 10 - Tracking Domain Backend

## Task Description

Implement the full Tracking domain backend for Altair: database tables, Rust service/handler modules, and API endpoints for locations, categories, items, item events, shopping lists, and shopping list items. This is a P0 backend feature that runs in parallel with Steps 8-9 (Guidance and Knowledge domains) and depends on Step 6 (core domain tables) and Step 5 (auth).

## Objective

When complete, the Altair server will expose a fully functional Tracking domain API with:

- Hierarchical location and category management
- Item CRUD with location/category assignment
- Append-only item event log that updates item quantities
- Low-stock detection query
- Shopping list management with check/uncheck line items
- Household-level authorization on all endpoints

## Problem Statement

The Tracking domain is one of Altair's three feature domains (alongside Guidance and Knowledge). It enables household inventory management, consumption tracking, and shopping list workflows. Without it, users cannot track physical/digital resources, monitor stock levels, or manage shopping lists. The backend tables and API must be in place before web/Android clients can build Tracking UI (Steps 12, 14, 15).

## Solution Approach

Follow the established server architecture patterns exactly as seen in `core/households/` and `auth/`:

- **Module structure**: `src/tracking/{sub_module}/mod.rs`, `models.rs`, `handlers.rs`, `service.rs`
- **Handler pattern**: Axum extractors (`AuthenticatedUser`, `State<PgPool>`, `Json`, `Path`), validator crate for input validation, delegate to service functions
- **Service pattern**: Pure async functions taking `&PgPool`, raw SQL via `sqlx::query_as`, transactions where multi-step
- **Model pattern**: DB models derive `Debug, Clone, Serialize, sqlx::FromRow`; request models derive `Debug, Deserialize, Validate`
- **Migration pattern**: SQL files in `apps/server/migrations/` with timestamp-ordered naming (next sequence: `20260326200001+`)
- **Router pattern**: Register all tracking routes in `api/mod.rs::create_router`

The Tracking domain decomposes into four sub-modules that can be built in parallel:

1. **Locations** - hierarchical CRUD (parent_location_id)
2. **Categories** - hierarchical CRUD (parent_category_id)
3. **Items** - CRUD + events + low-stock (references locations and categories by UUID FK)
4. **Shopping Lists** - container CRUD + line item CRUD + check toggle

## Relevant Files

Use these files to understand existing patterns and complete the task:

**Existing pattern references (READ these before writing any code):**

- `apps/server/src/core/households/handlers.rs` - Handler pattern: Axum extractors, validation, service delegation
- `apps/server/src/core/households/models.rs` - Model pattern: DB structs with sqlx::FromRow, request structs with Validate
- `apps/server/src/core/households/service.rs` - Service pattern: async fns with &PgPool, sqlx queries, transactions
- `apps/server/src/core/households/mod.rs` - Module re-exports
- `apps/server/src/core/mod.rs` - Domain module declaration
- `apps/server/src/auth/middleware.rs` - AuthenticatedUser extractor (use in all handlers)
- `apps/server/src/auth/service.rs` - `get_user_household_ids` for household authorization
- `apps/server/src/error.rs` - AppError enum (Database, NotFound, BadRequest, Unauthorized, Forbidden, Conflict, Internal)
- `apps/server/src/api/mod.rs` - Router registration pattern, AppState definition
- `apps/server/src/main.rs` - Module declarations (add `mod tracking;` here)
- `apps/server/Cargo.toml` - Dependencies (no new deps needed, existing sqlx/chrono/uuid/serde/validator suffice)

**Spec references:**

- `docs/altair-implementation-plan.md` (lines 775-830) - Step 10 requirements, table schemas, API endpoints, business rules, done criteria
- `docs/prd/altair-tracking-prd.md` - Tracking domain PRD
- `docs/schema/altair-schema-design-spec.md` - Schema overview including Tracking tables
- `docs/schema/altair-initial-schema.sql` - Reference SQL schema (update after migrations)

**Existing migrations (for naming sequence context):**

- `apps/server/migrations/20260326100004_create_entity_relations.sql` - Last existing migration

### New Files

**Migrations (8 files):**

- `apps/server/migrations/20260326200001_create_tracking_locations.sql`
- `apps/server/migrations/20260326200002_create_tracking_categories.sql`
- `apps/server/migrations/20260326200003_create_tracking_items.sql`
- `apps/server/migrations/20260326200004_create_tracking_item_events.sql`
- `apps/server/migrations/20260326200005_create_tracking_shopping_lists.sql`
- `apps/server/migrations/20260326200006_create_tracking_shopping_list_items.sql`
- `apps/server/migrations/20260326200007_create_item_tags.sql`
- `apps/server/migrations/20260326200008_create_item_attachments.sql`

**Tracking module (16 Rust files):**

- `apps/server/src/tracking/mod.rs` - Declares sub-modules: locations, categories, items, shopping_lists
- `apps/server/src/tracking/locations/mod.rs` - Re-exports
- `apps/server/src/tracking/locations/models.rs` - TrackingLocation, CreateLocationRequest, UpdateLocationRequest
- `apps/server/src/tracking/locations/handlers.rs` - CRUD handlers
- `apps/server/src/tracking/locations/service.rs` - CRUD service functions
- `apps/server/src/tracking/categories/mod.rs` - Re-exports
- `apps/server/src/tracking/categories/models.rs` - TrackingCategory, CreateCategoryRequest, UpdateCategoryRequest
- `apps/server/src/tracking/categories/handlers.rs` - CRUD handlers
- `apps/server/src/tracking/categories/service.rs` - CRUD service functions
- `apps/server/src/tracking/items/mod.rs` - Re-exports
- `apps/server/src/tracking/items/models.rs` - TrackingItem, TrackingItemEvent, CreateItemRequest, UpdateItemRequest, CreateItemEventRequest
- `apps/server/src/tracking/items/handlers.rs` - CRUD + events + low-stock handlers
- `apps/server/src/tracking/items/service.rs` - CRUD + events + low-stock service functions
- `apps/server/src/tracking/shopping_lists/mod.rs` - Re-exports
- `apps/server/src/tracking/shopping_lists/models.rs` - TrackingShoppingList, TrackingShoppingListItem, Create/Update request types
- `apps/server/src/tracking/shopping_lists/handlers.rs` - CRUD + check toggle handlers
- `apps/server/src/tracking/shopping_lists/service.rs` - CRUD + check toggle service functions

**Modified files (3):**

- `apps/server/src/main.rs` - Add `mod tracking;` declaration
- `apps/server/src/api/mod.rs` - Import tracking handlers, register all tracking routes
- `docs/schema/altair-initial-schema.sql` - Append tracking table definitions to reference schema

## Implementation Phases

### Phase 1: Foundation (Migrations)

Create all 8 migration SQL files following the schema from the implementation plan. Key design decisions:

- All tables use `UUID PRIMARY KEY DEFAULT gen_random_uuid()`
- All tables include `created_at TIMESTAMPTZ NOT NULL DEFAULT now()`
- Mutable tables include `updated_at TIMESTAMPTZ NOT NULL DEFAULT now()`
- Foreign keys reference `users(id)`, `households(id)`, and tracking tables as specified
- CHECK constraints on status/enum columns (item status, event_type, shopping list status)
- Indexes on user_id, household_id, and foreign key columns used in joins/filters

**Table schemas (from implementation plan):**

`tracking_locations`:

```sql
CREATE TABLE tracking_locations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    household_id UUID NOT NULL REFERENCES households(id),
    name TEXT NOT NULL,
    description TEXT,
    parent_location_id UUID REFERENCES tracking_locations(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_locations_household ON tracking_locations(household_id);
CREATE INDEX idx_tracking_locations_parent ON tracking_locations(parent_location_id);
```

`tracking_categories`:

```sql
CREATE TABLE tracking_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    household_id UUID NOT NULL REFERENCES households(id),
    name TEXT NOT NULL,
    description TEXT,
    parent_category_id UUID REFERENCES tracking_categories(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_categories_household ON tracking_categories(household_id);
CREATE INDEX idx_tracking_categories_parent ON tracking_categories(parent_category_id);
```

`tracking_items`:

```sql
CREATE TABLE tracking_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    household_id UUID NOT NULL REFERENCES households(id),
    category_id UUID REFERENCES tracking_categories(id),
    location_id UUID REFERENCES tracking_locations(id),
    name TEXT NOT NULL,
    description TEXT,
    quantity INTEGER NOT NULL DEFAULT 0,
    unit TEXT,
    min_quantity INTEGER,
    barcode TEXT,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_items_household ON tracking_items(household_id);
CREATE INDEX idx_tracking_items_category ON tracking_items(category_id);
CREATE INDEX idx_tracking_items_location ON tracking_items(location_id);
CREATE INDEX idx_tracking_items_status ON tracking_items(status);
```

`tracking_item_events`:

```sql
CREATE TABLE tracking_item_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    item_id UUID NOT NULL REFERENCES tracking_items(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    event_type TEXT NOT NULL CHECK (event_type IN ('consumed', 'restocked', 'moved', 'adjusted', 'expired', 'donated')),
    quantity_change INTEGER NOT NULL,
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_item_events_item ON tracking_item_events(item_id);
CREATE INDEX idx_tracking_item_events_created ON tracking_item_events(created_at);
```

`tracking_shopping_lists`:

```sql
CREATE TABLE tracking_shopping_lists (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    household_id UUID NOT NULL REFERENCES households(id),
    name TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'completed', 'archived')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_shopping_lists_household ON tracking_shopping_lists(household_id);
```

`tracking_shopping_list_items`:

```sql
CREATE TABLE tracking_shopping_list_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    shopping_list_id UUID NOT NULL REFERENCES tracking_shopping_lists(id) ON DELETE CASCADE,
    item_id UUID REFERENCES tracking_items(id),
    name TEXT NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    unit TEXT,
    is_checked BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX idx_tracking_shopping_list_items_list ON tracking_shopping_list_items(shopping_list_id);
```

`item_tags`:

```sql
CREATE TABLE item_tags (
    item_id UUID NOT NULL REFERENCES tracking_items(id) ON DELETE CASCADE,
    tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (item_id, tag_id)
);
```

`item_attachments`:

```sql
CREATE TABLE item_attachments (
    item_id UUID NOT NULL REFERENCES tracking_items(id) ON DELETE CASCADE,
    attachment_id UUID NOT NULL REFERENCES attachments(id) ON DELETE CASCADE,
    PRIMARY KEY (item_id, attachment_id)
);
```

### Phase 2: Core Implementation (Rust Modules)

Build all four sub-modules following the exact patterns from `core/households/`:

**Locations module** (`src/tracking/locations/`):

- Models: `TrackingLocation` (FromRow), `CreateLocationRequest` (Validate: name 1-200 chars), `UpdateLocationRequest`
- Service: `create_location`, `list_locations` (filtered by household_id), `get_location`, `update_location`, `delete_location`
- Handlers: Full CRUD with `AuthenticatedUser`, household membership verification
- Hierarchy: `parent_location_id` nullable FK, service returns flat list (client handles tree rendering)

**Categories module** (`src/tracking/categories/`):

- Nearly identical structure to locations. Models: `TrackingCategory`, `CreateCategoryRequest`, `UpdateCategoryRequest`
- Same CRUD pattern with `parent_category_id` for hierarchy

**Items module** (`src/tracking/items/`):

- Models: `TrackingItem` (FromRow with all columns), `TrackingItemEvent` (FromRow), `CreateItemRequest`, `UpdateItemRequest`, `CreateItemEventRequest`
- Service: Full CRUD + `create_item_event` (transaction: insert event + update item quantity), `list_item_events` (sorted by created_at DESC), `list_low_stock_items` (WHERE quantity < min_quantity AND min_quantity IS NOT NULL)
- Handlers: CRUD routes + `POST /tracking/items/:id/events` + `GET /tracking/items/:id/events` + `GET /tracking/items/low-stock`
- Business rule: `create_item_event` must atomically insert the event and update the item's quantity within a transaction

**Shopping Lists module** (`src/tracking/shopping_lists/`):

- Models: `TrackingShoppingList`, `TrackingShoppingListItem`, `CreateShoppingListRequest`, `UpdateShoppingListRequest`, `CreateShoppingListItemRequest`
- Service: List CRUD + item CRUD + `toggle_check` (updates is_checked)
- Handlers: Nested resource routes for list items
- Business rule: Shopping list items can reference a `tracking_item` (item_id) or be freeform (just name/quantity/unit)

**Household authorization pattern** (apply to ALL handlers):

```rust
// Verify user is a member of the target household
let household_ids = auth_service::get_user_household_ids(&pool, auth.user_id).await?;
if !household_ids.contains(&body.household_id) {
    return Err(AppError::Forbidden("Not a member of this household".to_string()));
}
```

### Phase 3: Integration and Polish

- Register all tracking routes in `api/mod.rs::create_router`
- Add `mod tracking;` to `main.rs`
- Update `docs/schema/altair-initial-schema.sql` with all new table definitions
- Verify `cargo build` succeeds
- Verify `cargo test` passes (unit tests on models)

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
  - Name: builder-migrations
  - Role: Create all 8 migration SQL files and update the reference schema in docs/schema/altair-initial-schema.sql
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-locations-categories
  - Role: Build the locations and categories Rust modules (models, service, handlers) following exact patterns from core/households
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-items
  - Role: Build the items and item events Rust modules (models, service, handlers) including low-stock query and event transaction logic
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-shopping
  - Role: Build the shopping lists Rust module (models, service, handlers) including nested line items and check toggle
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-integration
  - Role: Wire all tracking modules into the router (api/mod.rs), add mod declaration to main.rs, create tracking/mod.rs, verify cargo build passes
  - Agent Type: backend-engineer
  - Resume: false

- Quality Engineer (Validator)
  - Name: validator
  - Role: Validate completed work against acceptance criteria (read-only inspection mode)
  - Agent Type: quality-engineer
  - Resume: false

## Step by Step Tasks

- IMPORTANT: Execute every step in order, top to bottom. Each task maps directly to a `TaskCreate` call.
- Before you start, run `TaskCreate` to create the initial task list that all team members can see and execute.

### 1. Create Database Migrations

- **Task ID**: create-migrations
- **Depends On**: none
- **Assigned To**: builder-migrations
- **Agent Type**: backend-engineer
- **Parallel**: false (must complete before Rust modules)
- Create all 8 migration SQL files in `apps/server/migrations/` using timestamp prefix `20260326200001` through `20260326200008`
- Follow the exact table schemas defined in Phase 1 of this plan
- Include CREATE INDEX statements for all foreign key columns and frequently-filtered columns
- Use CHECK constraints on status and event_type enum columns
- Update `docs/schema/altair-initial-schema.sql` by appending all 8 table definitions after the existing `entity_relations` table
- Read existing migration files (e.g. `20260326100001_create_initiatives.sql`) to match SQL style conventions

### 2. Build Locations and Categories Modules

- **Task ID**: build-locations-categories
- **Depends On**: create-migrations
- **Assigned To**: builder-locations-categories
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside build-items and build-shopping)
- Read `apps/server/src/core/households/models.rs`, `handlers.rs`, `service.rs` thoroughly as pattern reference
- Read `apps/server/src/auth/middleware.rs` for AuthenticatedUser extractor
- Read `apps/server/src/auth/service.rs` for `get_user_household_ids` function
- Read `apps/server/src/error.rs` for AppError variants
- Create `apps/server/src/tracking/locations/mod.rs`, `models.rs`, `handlers.rs`, `service.rs`
- Create `apps/server/src/tracking/categories/mod.rs`, `models.rs`, `handlers.rs`, `service.rs`
- Both modules follow identical patterns: hierarchical CRUD with parent_id (nullable self-reference)
- All handlers must verify household membership using `auth_service::get_user_household_ids`
- Models: DB struct derives `Debug, Clone, Serialize, sqlx::FromRow`; Request structs derive `Debug, Deserialize, Validate`
- Service functions: `create`, `list` (by household_id), `get` (by id), `update`, `delete`
- Include unit tests on model serialization (matching the pattern in `households/models.rs`)

### 3. Build Items and Item Events Module

- **Task ID**: build-items
- **Depends On**: create-migrations
- **Assigned To**: builder-items
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside build-locations-categories and build-shopping)
- Read the same pattern reference files as task 2
- Create `apps/server/src/tracking/items/mod.rs`, `models.rs`, `handlers.rs`, `service.rs`
- Models to define:
  - `TrackingItem` (all columns from tracking_items table)
  - `TrackingItemEvent` (all columns from tracking_item_events table)
  - `CreateItemRequest` (name required, household_id required, category_id/location_id/quantity/unit/min_quantity/barcode optional)
  - `UpdateItemRequest` (all fields optional except id)
  - `CreateItemEventRequest` (event_type required with CHECK constraint values, quantity_change required, notes optional)
- Service functions:
  - `create_item`, `list_items` (by household_id), `get_item`, `update_item`, `delete_item`
  - `create_item_event` - MUST use a transaction: insert event row, then UPDATE tracking_items SET quantity = quantity + $quantity_change, updated_at = now()
  - `list_item_events` (by item_id, ORDER BY created_at DESC)
  - `list_low_stock_items` (by household_id, WHERE min_quantity IS NOT NULL AND quantity < min_quantity)
- Handlers: CRUD on `/tracking/items` + `POST /tracking/items/{id}/events` + `GET /tracking/items/{id}/events` + `GET /tracking/items/low-stock`
- Item events are append-only: no update or delete endpoints
- All handlers enforce household membership

### 4. Build Shopping Lists Module

- **Task ID**: build-shopping
- **Depends On**: create-migrations
- **Assigned To**: builder-shopping
- **Agent Type**: backend-engineer
- **Parallel**: true (can run alongside build-locations-categories and build-items)
- Read the same pattern reference files as task 2
- Create `apps/server/src/tracking/shopping_lists/mod.rs`, `models.rs`, `handlers.rs`, `service.rs`
- Models to define:
  - `TrackingShoppingList` (all columns from tracking_shopping_lists table)
  - `TrackingShoppingListItem` (all columns from tracking_shopping_list_items table)
  - `CreateShoppingListRequest` (name required, household_id required)
  - `UpdateShoppingListRequest` (name and/or status optional)
  - `CreateShoppingListItemRequest` (name required, item_id optional, quantity/unit optional)
- Service functions:
  - `create_list`, `list_lists` (by household_id), `get_list`, `update_list`, `delete_list`
  - `add_list_item`, `list_list_items` (by shopping_list_id), `update_list_item`, `remove_list_item`
  - `toggle_check` (flip is_checked boolean for a specific list item)
- Handlers: CRUD on `/tracking/shopping-lists` + CRUD on `/tracking/shopping-lists/{id}/items` + `POST /tracking/shopping-lists/{id}/items/{item_id}/check`
- Shopping list items can be freeform (just name) or linked to a tracking_item (item_id FK)
- All handlers enforce household membership

### 5. Wire Router Integration

- **Task ID**: wire-integration
- **Depends On**: build-locations-categories, build-items, build-shopping
- **Assigned To**: builder-integration
- **Agent Type**: backend-engineer
- **Parallel**: false
- Add `mod tracking;` to `apps/server/src/main.rs` (after `mod core;`)
- Create `apps/server/src/tracking/mod.rs` declaring all four sub-modules: `pub mod locations;`, `pub mod categories;`, `pub mod items;`, `pub mod shopping_lists;`
- Update `apps/server/src/api/mod.rs`:
  - Add imports for all tracking handler modules
  - Register all tracking routes in `create_router`:
    - `GET/POST /tracking/locations` (list + create)
    - `GET/PUT/DELETE /tracking/locations/{id}` (get + update + delete)
    - `GET/POST /tracking/categories` (list + create)
    - `GET/PUT/DELETE /tracking/categories/{id}` (get + update + delete)
    - `GET/POST /tracking/items` (list + create)
    - `GET/PUT/DELETE /tracking/items/{id}` (get + update + delete)
    - `POST /tracking/items/{id}/events` (create event)
    - `GET /tracking/items/{id}/events` (list events)
    - `GET /tracking/items/low-stock` (low stock query)
    - `GET/POST /tracking/shopping-lists` (list + create)
    - `GET/PUT/DELETE /tracking/shopping-lists/{id}` (get + update + delete)
    - `GET/POST /tracking/shopping-lists/{id}/items` (list + add item)
    - `PUT/DELETE /tracking/shopping-lists/{id}/items/{item_id}` (update + remove item)
    - `POST /tracking/shopping-lists/{id}/items/{item_id}/check` (toggle check)
- Run `cargo build` in `apps/server/` to verify compilation
- Run `cargo test` in `apps/server/` to verify all tests pass

### 6. Final Validation

- **Task ID**: validate-all
- **Depends On**: wire-integration
- **Assigned To**: validator
- **Agent Type**: quality-engineer
- **Parallel**: false
- Run `cargo build` and verify zero errors
- Run `cargo test` and verify all tests pass
- Verify all 8 migration SQL files exist with correct table schemas
- Verify all 16 new Rust files exist with correct module structure
- Verify `main.rs` declares `mod tracking`
- Verify `api/mod.rs` registers all tracking routes listed in the implementation plan
- Verify all handlers use `AuthenticatedUser` extractor for authentication
- Verify all handlers check household membership for authorization
- Verify `create_item_event` service function uses a transaction
- Verify `list_low_stock_items` filters by `min_quantity IS NOT NULL AND quantity < min_quantity`
- Verify item events have no update/delete endpoints (append-only)
- Verify shopping list items support both freeform and item-linked modes
- Verify `docs/schema/altair-initial-schema.sql` includes all 8 new table definitions
- Cross-reference all acceptance criteria from the implementation plan Step 10 "Done when" checklist
- Operate in validation mode: inspect and report only, do not modify files

## Acceptance Criteria

From the implementation plan Step 10 "Done when":

- [ ] Location and category hierarchical CRUD works
- [ ] Item CRUD with location and category assignment works
- [ ] Item events create and update item quantity
- [ ] Low-stock query returns items below threshold
- [ ] Shopping list CRUD with check/uncheck works
- [ ] All endpoints enforce household-level authorization
- [ ] Item event history is queryable and sorted by date

Additional technical criteria:

- [ ] All 8 migration SQL files created with correct schemas and indexes
- [ ] All 16 new Rust source files follow existing patterns (handlers/models/service split)
- [ ] `cargo build` succeeds with zero errors
- [ ] `cargo test` passes all existing and new tests
- [ ] Reference schema SQL (`docs/schema/altair-initial-schema.sql`) updated
- [ ] No new dependencies required in Cargo.toml (existing deps suffice)

## Validation Commands

Execute these commands to validate the task is complete:

- `cd apps/server && cargo build 2>&1` - Verify the project builds without errors
- `cd apps/server && cargo test 2>&1` - Run the full test suite
- `ls apps/server/migrations/20260326200*.sql | wc -l` - Verify 8 migration files exist
- `find apps/server/src/tracking -name '*.rs' | wc -l` - Verify 16+ Rust files in tracking module
- `grep -c 'mod tracking' apps/server/src/main.rs` - Verify tracking module declared
- `grep -c 'tracking' apps/server/src/api/mod.rs` - Verify tracking routes registered
- `grep -c 'AuthenticatedUser' apps/server/src/tracking/*/handlers.rs` - Verify auth on all handlers
- `grep 'low.stock\|min_quantity' apps/server/src/tracking/items/service.rs` - Verify low-stock logic exists

## Notes

- **No new Cargo dependencies needed.** The existing deps (axum, sqlx, chrono, uuid, serde, validator, thiserror) cover all requirements.
- **Household authorization pattern:** Use `crate::auth::service::get_user_household_ids` to fetch the user's household IDs, then verify the target household_id is in that list. This matches the authorization approach established in the households module.
- **Event quantity_change semantics:** Positive values mean restocking, negative values mean consumption. The service function should apply `SET quantity = quantity + $quantity_change` so both directions work with a single UPDATE.
- **Low-stock route ordering:** Register `GET /tracking/items/low-stock` BEFORE `GET /tracking/items/{id}` in the router to avoid the path parameter capturing "low-stock" as an ID.
- **Shopping list item check toggle:** The `toggle_check` endpoint should flip the boolean (SET is_checked = NOT is_checked) rather than requiring the client to send the new value. This makes it idempotent-safe for offline sync.
- **Migration naming:** Using `20260326200001` through `20260326200008` to sort after existing migrations (`20260326100004` is the last).
