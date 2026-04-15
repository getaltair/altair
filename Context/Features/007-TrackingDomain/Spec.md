# Feature 007: Tracking Domain

| Field | Value |
|---|---|
| **Feature** | 007-TrackingDomain |
| **Version** | 1.0 |
| **Status** | Draft |
| **Date** | 2026-04-15 |
| **Source Docs** | `docs/specs/01-PRD-004-tracking.md`, `docs/specs/03-invariants.md`, `docs/specs/10-PLAN-001-v1.md` (Step 7), migrations 019–024 |

---

## Overview

Feature 007 implements the Tracking domain — inventory and resource management for households. It delivers the server-side REST API (Rust/Axum) and the web client UI (SvelteKit/Svelte 5) on top of the six tracking tables already created by migrations 019–024: `tracking_locations`, `tracking_categories`, `tracking_items`, `tracking_item_events`, `tracking_shopping_lists`, and `tracking_shopping_list_items`.

All tracking resources are household-scoped: they are owned by a household and accessible only to its members.

---

## Problem Statement

The tracking tables exist in PostgreSQL but are unreachable by clients — there are no routes, service functions, or handlers. Clients cannot create, query, or manage inventory. Households need a reliable way to track physical resources (food, supplies, equipment) across members and over time. When multiple members change stock levels concurrently or offline, the system must record every change as an immutable event so that the ledger is always auditable and the current stock level is always derivable.

---

## User Stories

**US-01 — Location management**
As a household member, I can create and manage storage locations (pantry, fridge, garage, shelf) so that items can be placed at a known location.

**US-02 — Category management**
As a household member, I can create and manage categories (food, cleaning, health, tools) so that items are organised for browsing and filtering.

**US-03 — Item catalog**
As a household member, I can add, view, edit, and archive tracked items with a name, optional description, optional category, and optional location, so that the household has a catalog of things being tracked.

**US-04 — Stock events (ledger)**
As a household member, I can record quantity-change events (consume, restock, adjust, move, expire, loss) against an item so that every stock change is durably recorded and the current quantity is always derivable from the event history.

**US-05 — Current stock level**
As a household member, I can see the current quantity on hand for any item so that I know what is available.

**US-06 — Item event history**
As a household member, I can view the full chronological event history for an item so that I can understand how the stock level changed over time.

**US-07 — Shopping lists**
As a household member, I can create shopping lists and add items to them with quantities and a checked-off state so that household members can coordinate shopping runs.

**US-08 — Household access control**
As a household member, I can only see and modify tracking resources owned by my household; I cannot access another household's inventory data.

---

## Requirements

### Must Have

**Access control**
- All tracking resources are scoped to a household. Every API operation must verify the requesting user is an active member of the target household via `household_memberships`.
- Non-members must receive HTTP 403 for all endpoints on a household they do not belong to.
- Cross-household data access must be impossible at the service layer — not just at the route layer.

**Locations (`tracking_locations`)**
- CRUD: create, list, get by id, update name, soft-delete (`deleted_at`).
- A location has a `name` (required) and belongs to exactly one `household_id`.
- Soft-deleted locations are excluded from list results by default. Existing items referencing a soft-deleted location retain the reference; the location cannot be assigned to new or updated items.

**Categories (`tracking_categories`)**
- CRUD: create, list, get by id, update name, soft-delete (`deleted_at`).
- A category has a `name` (required) and belongs to exactly one `household_id`.
- Same soft-delete semantics as locations.

**Items (`tracking_items`)**
- CRUD: create, list, get by id, update, soft-delete (`deleted_at`).
- Required fields: `name`, `user_id` (creator), `household_id`.
- Optional fields: `description`, `barcode`, `location_id`, `category_id`, `initiative_id`, `expires_at`.
- The `quantity` column on `tracking_items` is the canonical stock level. It is updated transactionally when an item event is recorded (each event applies `quantity_change` to the stored quantity).
- A `location_id` must reference a location in the same household (invariant E-8). Cross-household location assignment must be rejected with 422.
- The client supplies the UUID for items (invariant E-2). Duplicate IDs must be rejected with 409.
- Soft-deleted items cannot receive new events.

**Item events (`tracking_item_events`)**
- Create and list events for an item.
- Required fields: `item_id`, `event_type`, `quantity_change`, `occurred_at`.
- Supported `event_type` values: `restock`, `consume`, `move`, `adjust`, `expire`, `loss`.
- `quantity_change` may be positive (stock increase) or negative (stock decrease). The sign convention is explicit and enforced server-side.
- Applying an event whose `quantity_change` would reduce the item's stored `quantity` below zero must be rejected (invariant E-7). The quantity check and event insert must execute in the same database transaction to prevent TOCTOU races.
- A `move` event may carry `from_location_id` and `to_location_id` to record movement between locations.
- Item events are **append-only** at the service layer: no update or delete endpoint exists for events (invariant D-5).
- The client supplies the UUID for each event (invariant E-2).

**Shopping lists (`tracking_shopping_lists`)**
- CRUD: create, list, get by id, update name, soft-delete.
- Required fields: `name`, `household_id`.

**Shopping list items (`tracking_shopping_list_items`)**
- Add, list, update status, soft-delete items on a shopping list.
- Required fields: `shopping_list_id`, `name`, `quantity` (default 1), `status` (default `pending`).
- Optional: `item_id` linking to an inventory item. When `item_id` is supplied it must reference an item in the same household (invariant E-9).
- Valid `status` values: `pending`, `purchased`, `removed`.
- State machine: `pending → purchased`, `pending → removed`, `purchased → pending` (un-check). `removed` is terminal — no transitions out of `removed` are permitted.
- Completing a shopping list (marking all items purchased) does not automatically generate item events; restocking from a shopping list is a separate explicit user action.

**Server API (Rust/Axum)**
- Module structure follows the established codebase pattern: `mod.rs`, `models.rs`, `service.rs`, `handlers.rs` per domain resource group.
- Handlers are thin: all business logic lives in service functions.
- Standard error mapping: 403 forbidden, 404 not found, 409 conflict (duplicate UUID, terminal state transition), 422 unprocessable (invariant violations).
- All endpoints require authentication via the existing JWT middleware (invariant SEC-2).
- `impl From<sqlx::Error> for AppError` is used; no inline `.map_err` in service functions.

**Web client (SvelteKit/Svelte 5)**
- Routes: inventory list, item detail with event history, location management, category management, shopping list management.
- Uses Svelte 5 runes (`$state`, `$derived`, `$effect`, `$props`) — no legacy reactive syntax.
- Reads and writes via the server REST API.

### Should Have

- Pagination on list endpoints for items and item events (events grow unbounded per item).
- Filtering items by `category_id`, `location_id`, and archived/active status.
- A summary row per item showing current `quantity`, assigned location name, and assigned category name.

### Won't Have (this iteration)

- PowerSync / offline sync for tracking tables — deferred to the sync feature.
- Android client for the Tracking domain — deferred.
- Domain event publishing (`ItemQuantityChanged`) — no consumers exist yet; deferred to Step 11.
- Barcode scanning or external product catalog integration (PRD-004 G-T-7 is P1).
- Low-stock threshold alerts (PRD-004 G-T-10 is P1).
- Image attachments on items (PRD-004 G-T-8 is P1).
- Automated restock quest creation (PRD-004 G-T-12 is P2).
- Item reservation system (PRD-004 G-T-15 is P2).

---

## Testable Assertions

| ID     | Assertion | Verification |
|--------|-----------|--------------|
| FA-001 | A user who is not a member of a household receives 403 for any tracking endpoint on that household. | Integration test: user without membership attempts POST /tracking/locations for a foreign household. |
| FA-002 | Creating a location as a household member returns 201 with the created location body. | Integration test: happy-path create, assert 201 and response fields. |
| FA-003 | Listing locations for household X returns only locations belonging to household X. | Integration test: two households each with locations; list one, assert none from the other. |
| FA-004 | Creating an item with a caller-supplied `id` stores and returns that exact UUID. | Integration test: POST with explicit id field, assert response.id matches. |
| FA-005 | Creating an item with a `id` that already exists returns 409 Conflict. | Integration test: POST same id twice, assert 409 on second. |
| FA-006 | Creating an item with a `location_id` from a different household returns 422 (invariant E-8). | Integration test: location from household B assigned to item in household A. |
| FA-007 | Recording an event whose `quantity_change` would bring `quantity` below zero returns 422 (invariant E-7). | Integration test: item quantity=5, post consume event quantity_change=-6, assert 422. |
| FA-008 | Recording an event within available quantity succeeds and the item's `quantity` is updated. | Integration test: quantity=10, consume quantity_change=-3, assert quantity=7. |
| FA-009 | Item events are returned in chronological order (`occurred_at` ascending) and all events are present. | Integration test: create 5 events with varying occurred_at, list, assert all present and ordered. |
| FA-010 | No HTTP method exists to update or delete an item event — attempts return 404 or 405 (invariant D-5). | Route table review + integration test: attempt PATCH/DELETE on /tracking/items/{id}/events/{eid}. |
| FA-011 | Recording an event against a soft-deleted item returns 422. | Integration test: soft-delete item, POST event, assert error. |
| FA-012 | Creating a shopping list item with an `item_id` from a different household returns 422 (invariant E-9). | Integration test: item from household B referenced in shopping list of household A. |
| FA-013 | A shopping list item transitions from `pending` to `purchased` successfully. | Integration test: PATCH status to purchased, assert 200 and status=purchased. |
| FA-014 | A shopping list item transitions from `pending` to `removed` successfully, and any further PATCH on it returns 409. | Integration test: transition to removed, then attempt another PATCH, assert 409. |
| FA-015 | A shopping list item in `purchased` state transitions back to `pending` (un-check) successfully. | Integration test: purchased → pending, assert 200. |
| FA-016 | Completing a shopping list (all items purchased) does not generate any rows in `tracking_item_events`. | Integration test: purchase all items, assert item_events count unchanged. |
| FA-017 | Soft-deleted locations, categories, items, and shopping lists are excluded from default list responses. | Integration test: soft-delete each, list, assert absent. |
| FA-018 | All tracking list endpoints scope results to the requesting user's household — no other household's data appears. | Integration test: two households with full data sets; member of A queries all list endpoints, asserts no B data. |
| FA-019 | Concurrent events cannot together drive quantity below zero (atomic check-and-insert). | Concurrency integration test: two simultaneous consume requests each claiming the remaining stock; at most one succeeds, other returns 422. |
| FA-020 | PRD A-026: a `consume` event decrements the item's displayed quantity. | Integration test: quantity=10, consume 4, GET item, assert quantity=6. |
| FA-021 | PRD A-027: a shopping list created in a household is visible to all members of that household. | Integration test: member A creates list, member B lists shopping lists, assert list is present. |
| FA-022 | PRD A-028: items can be filtered by location and category. | Integration test: items with different locations/categories, filter by each, assert only matching items returned. |
| FA-023 | PRD A-029: item event history shows a complete chronological timeline of all changes. | Integration test: multiple event types created, all present and in chronological order. |

---

## Open Questions

**OQ-01 — Quantity update strategy**
The `tracking_items.quantity` column is a stored value (updated per event). Should the service layer update `quantity` inside the same transaction as the event insert, or should a PostgreSQL trigger maintain it? A service-layer update is explicit and easier to test; a trigger is more robust against direct DB writes. Decision needed before Tech.md.

**OQ-02 — Decimal quantities**
The `quantity` column is `NUMERIC` (arbitrary precision). PRD OQ-T-1 raises: should decimal input be accepted (e.g., 2.5 liters)? The schema permits it. The API and web UI must decide whether to allow fractional quantities or enforce integer-only input. Decision needed before implementation.

**OQ-03 — Move event and location update**
A `move` event has `from_location_id` and `to_location_id`. Should recording a move event also update the item's assigned `location_id` to `to_location_id`? If yes, that update must be within the same transaction as the event insert. Decision needed before Tech.md.

**OQ-04 — Shopping list `status` field**
The `tracking_shopping_lists` migration has no `status` column. If shopping list status (`open`, `in_progress`, `completed`) is desired, it requires a new migration or must be derived from item statuses. Clarify scope before Tech.md.

**OQ-05 — Location/category edit permissions**
Should any household member be able to edit or soft-delete a shared location/category, or only the creating member? Defaulting to any member for simplicity. If stricter ownership is required, an ADR should be opened.

**OQ-06 — Pagination defaults**
No page size is specified in the PRD. Propose offset-based pagination with `limit=50` default for items and `limit=100` for events. Confirm during Tech phase.

---

## Invariants Enforced

| Invariant | Description | Enforcement point |
|-----------|-------------|-------------------|
| E-2 | Client-generated UUIDs for items and events | Service: accept client `id`; reject duplicate with 409 |
| E-7 | Item quantity must never go below zero from a consumption event | Service: check-and-insert in a single transaction |
| E-8 | Item `location_id` must reference a valid location in the same household | Service: household scope check before item insert/update |
| E-9 | Shopping list item `item_id` must reference a valid item in the same household | Service: household scope check before shopping_list_item insert |
| D-5 | Item events are append-only — no UPDATE or DELETE paths | Routes: no PATCH/PUT/DELETE for item events |
| SEC-1 | Per-household data isolation at every query path | Service: all queries include household_id scoping |
| SEC-2 | All API endpoints require authentication | Axum JWT middleware on all tracking routes |

---

## Dependencies

| Feature | Dependency |
|---------|------------|
| 001 | Monorepo scaffold, Rust/Axum project structure, Cargo workspace |
| 002 | Shared contracts — EntityType constants for tracking entities |
| 003 | Auth (JWT middleware, AppError, AppState), `household_memberships` table |
| 004 | Migrations 019–024 already applied; PowerSync sync rules already reference tracking tables |

---

## Revision History

| Version | Date       | Author          | Notes |
|---------|------------|-----------------|-------|
| 1.0     | 2026-04-15 | Robert Hamilton | Initial spec — server + web client scope, grounded in migrations 019–024 and PRD-004 |