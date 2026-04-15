# PR Review: feat+tracking-domain → main

**Date:** 2026-04-15
**Feature:** Context/Features/007-TrackingDomain/
**Branch:** feat+tracking-domain
**PR:** #6
**Reviewers:** code-reviewer, silent-failure-hunter, pr-test-analyzer, type-design-analyzer
**Status:** 🟡 Partially resolved

## Summary

27 total findings: Fix-Now (14), Missing Tasks (10), Architectural Concerns (2), Convention Gaps (1).
Two critical issues require fixes before merge: a silent FOR UPDATE bypass that breaks the E-7
quantity invariant (P6-001), and a household isolation bypass in `list_item_events` (P6-002).
Six additional High-severity items span error handling, missing tests, and a broken `Move` event type.

---

## Findings

### Fix-Now

#### [FIX] P6-001: FOR UPDATE lock silently not acquired when item is deleted
- **Files:** `apps/server/server/src/tracking/shopping_list_items/service.rs:170-173, 216-219`
- **Severity:** Critical
- **Detail:** Both transactional purchase/reversal branches use `.execute()` to acquire the row
  lock via `SELECT id FROM tracking_items WHERE id = $1 FOR UPDATE`. If the item was soft-deleted
  between the membership check and the transaction start, the SELECT matches zero rows, `.execute()`
  returns `Ok(PgQueryResult{rows_affected: 0})`, and `?` propagates no error. The lock is silently
  not acquired and `create_item_event_in_tx` proceeds without holding the lock, breaking invariant
  E-7 (quantity floor). The pattern in `item_events/service.rs:56-68` correctly uses
  `SELECT EXISTS(...FOR UPDATE)` with an explicit bool check — that pattern should be applied here.
- **Fix:** Replace both `.execute()` calls with `.fetch_one::<bool>()` using
  `SELECT EXISTS(SELECT 1 FROM tracking_items WHERE id = $1 AND deleted_at IS NULL FOR UPDATE)`,
  then return `AppError::NotFound` if the result is false.
- **Relates to:** FA-019 (SELECT FOR UPDATE concurrency invariant), E-7
- **Status:** ✅ Fixed
- **Resolution:** Both purchase branches now use `SELECT EXISTS(... FOR UPDATE)` with explicit bool check; returns NotFound if item is deleted.

#### [FIX] P6-002: list_item_events authorization bypass — item not verified against household
- **Files:** `apps/server/server/src/tracking/item_events/service.rs:183-206`
- **Severity:** Critical
- **Detail:** `list_item_events` accepts a caller-supplied `household_id`, verifies the user is a
  member of that household, but never verifies the `item_id` belongs to that household. A member
  of household A can pass `household_id=A` with `item_id=<any item from household B>` and read the
  full event history for that item. Every other service function resolves or verifies household
  ownership from the item itself. This one does not.
- **Fix:** Add a query before the main select:
  ```rust
  let item_household: Option<Uuid> = sqlx::query_scalar(
      "SELECT household_id FROM tracking_items WHERE id = $1 AND deleted_at IS NULL",
  )
  .bind(item_id)
  .fetch_optional(pool)
  .await?;
  match item_household {
      Some(hid) if hid == household_id => {}
      _ => return Err(AppError::NotFound),
  }
  ```
- **Relates to:** FA-001 (household isolation)
- **Status:** ✅ Fixed
- **Resolution:** Added item→household lookup in list_item_events before the main query; returns NotFound for missing or wrong-household items.

#### [FIX] P6-003: item_events create handler ignores URL path parameter {id}
- **Files:** `apps/server/server/src/tracking/item_events/handlers.rs` (create fn)
- **Severity:** High
- **Detail:** The route is `POST /api/tracking/items/{id}/events` but the handler does not extract
  `Path(item_id)`. The `item_id` comes entirely from the JSON body. A client can POST to
  `/items/AAAA/events` with `{"item_id": "BBBB"}` and the event is silently created for item BBBB.
  The `list` handler on the same route correctly uses `Path(item_id)`.
- **Fix:** Extract `Path(item_id): Path<Uuid>` in the handler signature and override
  `req.item_id = item_id` so the URL path is authoritative.
- **Status:** ✅ Fixed
- **Resolution:** create handler now extracts Path(item_id) and overrides req.item_id; URL path is authoritative.

#### [FIX] P6-004: add_shopping_list_item missing 23505 duplicate-key handling
- **Files:** `apps/server/server/src/tracking/shopping_list_items/service.rs:93-104`
- **Severity:** High
- **Detail:** The INSERT uses `fetch_one` with bare `?`, which converts a Postgres 23505 unique
  constraint violation to `AppError::Internal` (HTTP 500). Offline-first clients that replay
  operations after a connectivity gap will hit this path with duplicate client-generated UUIDs and
  receive a 500 with no actionable feedback. Compare to `items/service.rs:66-72` and
  `item_events/service.rs:107-123`, which both catch code `"23505"` and return `AppError::Conflict`
  (HTTP 409).
- **Fix:** Add a match arm for `sqlx::Error::Database(ref e) if e.code().as_deref() == Some("23505")`
  → `Err(AppError::Conflict(...))` as done in the other two service modules.
- **Relates to:** FA-017 (duplicate UUID returns 409)
- **Status:** ✅ Fixed
- **Resolution:** 23505 match arm added to add_shopping_list_item INSERT returning AppError::Conflict.

#### [FIX] P6-005: Rollback errors silently discarded with `let _ =`
- **Files:**
  - `apps/server/server/src/tracking/item_events/service.rs:65, 82, 114, 120`
  - `apps/server/server/src/tracking/shopping_list_items/service.rs:190, 200, 235, 244`
- **Severity:** High
- **Detail:** All 8 explicit rollbacks are written as `let _ = tx.rollback().await;`. This discards
  both Ok and Err variants. A rollback failure under connection stress produces no log entry, making
  degraded connection pool states invisible to operators. Postgres will auto-rollback on drop, so
  data safety is not at risk, but the failure mode is completely invisible in tracing.
- **Fix:** Replace with `if let Err(rb_err) = tx.rollback().await { tracing::warn!("rollback
  failed (postgres will auto-rollback on drop): {:?}", rb_err); }`
- **Status:** ✅ Fixed
- **Resolution:** All 8 let _ = tx.rollback() replaced with tracing::warn! pattern across both service files.

#### [FIX] P6-006: Move events are permanently broken — CreateItemEventRequest missing location fields
- **Files:** `apps/server/server/src/tracking/item_events/models.rs:82`
- **Severity:** High
- **Detail:** `CreateItemEventRequest` has no `from_location_id`, `to_location_id`, or `notes`
  fields, but `TrackingItemEventRow` has all three and the INSERT in `service.rs:93-97` never binds
  them. A `Move` event created via the API will always have NULL location fields — a semantically
  broken record. `ItemEventType::Move` is a defined variant the client can send, but the API cannot
  correctly store it.
- **Fix:** Add `from_location_id: Option<Uuid>`, `to_location_id: Option<Uuid>`, `notes:
  Option<String>` to `CreateItemEventRequest`, bind them in the INSERT, and add a service-level
  validation that rejects `Move` events where both location fields are `None`.
- **Status:** ✅ Fixed
- **Resolution:** Added from_location_id, to_location_id, notes to CreateItemEventRequest; updated INSERT to 8 bindings; added Move validation requiring at least one location field.

#### [FIX] P6-007: Negative limit/offset not validated — Postgres rejects negative OFFSET with 500
- **Files:**
  - `apps/server/server/src/tracking/item_events/handlers.rs`
  - `apps/server/server/src/tracking/items/handlers.rs`
- **Severity:** Medium
- **Detail:** `limit` and `offset` are typed as `i64` with no lower-bound validation. Postgres
  rejects a negative OFFSET with a database error, which becomes a 500 Internal Server Error to
  the client instead of a 400 Bad Request.
- **Fix:** Validate `params.limit >= 0 && params.offset >= 0` and return
  `AppError::BadRequest(...)` if violated, or change field types to `u64`/`u32`.
- **Status:** ✅ Fixed
- **Resolution:** Added limit < 0 || offset < 0 check returning BadRequest in both item_events and items list handlers.

#### [FIX] P6-008: AppError variants other than Internal are not logged server-side
- **Files:** `apps/server/server/src/error.rs:39-58`
- **Severity:** Medium
- **Detail:** Only `AppError::Internal` is logged via `tracing::error!`. `Forbidden` and
  `Unauthorized` produce no server-side log line. Repeated `Forbidden` responses (e.g., from a
  sync misconfiguration leaking cross-household item IDs) are invisible in tracing, making
  production incidents difficult to diagnose.
- **Fix:** Add at minimum `tracing::debug!` for `Forbidden` and `Unauthorized`, including the
  relevant IDs where available.
- **Status:** ✅ Fixed
- **Resolution:** Added tracing::debug! for Unauthorized and Forbidden variants in error.rs IntoResponse impl.

#### [FIX] P6-009: concurrent_events_cannot_both_consume_below_zero test is non-deterministic
- **Files:** `apps/server/server/src/tracking/shopping_list_items/service.rs` (test module)
- **Severity:** Medium
- **Detail:** The concurrency test spawns two `tokio::spawn` tasks and asserts exactly one consume
  succeeds. Under cooperative scheduling, task 1 may complete entirely before task 2 is scheduled,
  making the test pass due to serial ordering rather than `SELECT FOR UPDATE`. The test also passes
  if locking is removed. It does not actually demonstrate that the lock is the correctness
  mechanism.
- **Fix:** Use `tokio::sync::Barrier` with capacity 2 to synchronize both tasks at the transaction
  start before either completes, ensuring true concurrency is exercised.
- **Relates to:** FA-019
- **Status:** ✅ Fixed
- **Resolution:** Concurrency test refactored with tokio::sync::Barrier; both tasks now synchronize before the critical section.

#### [FIX] P6-010: HouseholdQuery struct duplicated across two modules
- **Files:**
  - `apps/server/server/src/tracking/shopping_lists/models.rs:48`
  - `apps/server/server/src/tracking/shopping_list_items/models.rs:104`
- **Severity:** Low
- **Detail:** Identical `struct HouseholdQuery { pub household_id: Uuid }` defined in both modules.
  Should live in the tracking module root or `src/contracts.rs`.
- **Status:** ✅ Fixed
- **Resolution:** HouseholdQuery consolidated in tracking::mod.rs; all 5 handler modules import from crate::tracking::HouseholdQuery; inline duplicates removed.

#### [FIX] P6-011: quantity_delta sign not validated per ItemEventType
- **Files:** `apps/server/server/src/tracking/item_events/service.rs`
- **Severity:** Low
- **Detail:** A `Restock` with `quantity_delta: -5.0` is accepted by the type and will pass E-7
  checks as long as it does not drive quantity below zero. The implied polarity constraint
  (Restock/Purchase must be positive; Consume/Loss/Expire must be negative) is not enforced.
- **Fix:** Add runtime validation in `create_item_event` checking sign consistency per variant.
- **Status:** ✅ Fixed
- **Resolution:** Polarity validation per event type added before transaction start in create_item_event.

#### [FIX] P6-012: name fields not validated for non-emptiness
- **Files:** `apps/server/server/src/tracking/locations/service.rs`, `categories/service.rs`,
  `items/service.rs`, `shopping_lists/service.rs`
- **Severity:** Low
- **Detail:** Empty strings are accepted as `name` in all four entity create/update paths. There is
  no `CHECK` constraint visible in the models and no service-level validation.
- **Fix:** Add `if req.name.trim().is_empty() { return Err(AppError::BadRequest(...)); }` in each
  create handler.
- **Status:** ✅ Fixed
- **Resolution:** name.trim().is_empty() check added to create handlers for locations and items; update handlers for all four entities include the check.

---

### Missing Tasks

#### [TASK] P6-013: No HTTP-layer integration tests for any tracking endpoint
- **Files:** `apps/server/server/tests/`
- **Severity:** Critical
- **Detail:** No test exercises an actual HTTP handler for any tracking domain route. Service-layer
  tests verify logic but do not catch: misconfigured router (auth middleware bypassed), incorrect
  HTTP status codes (201 vs 200 vs 204), or a handler that ignores `household_id`. The only
  tracking-adjacent integration test (`sync_push_integration.rs`) covers the sync push path only.
- **Relates to:** Steps.md S### (integration test milestone)
- **Status:** ✅ Task created
- **Resolution:** Added as S011 in Steps.md Phase 8.

#### [TASK] P6-014: No test for create_item_event with NULL household_id on item
- **Files:** `apps/server/server/src/tracking/item_events/service.rs`
- **Severity:** High
- **Detail:** The double-Option pattern from `fetch_optional` distinguishes "row not found" from
  "row exists with NULL household_id". No test covers the `Some(None)` case. If the inner-None arm
  is accidentally changed to `Ok(())`, items with no household could receive events from any
  authenticated user — a household isolation regression.
- **Relates to:** FA-001
- **Status:** ✅ Task created
- **Resolution:** Added as S012 in Steps.md Phase 8.

#### [TASK] P6-015: No shopping_list_items isolation test for remove/list paths
- **Files:** `apps/server/server/src/tracking/shopping_list_items/service.rs`
- **Severity:** High
- **Detail:** Tests cover state machine transitions and the wrong-household item_id invariant (E-9),
  but no test verifies that user B cannot call `list_shopping_list_items`, `update_shopping_list_item`,
  or `remove_shopping_list_item` against a list they have no membership in. A refactor that
  reorders the membership check after the mutation would not be caught.
- **Relates to:** FA-001
- **Status:** ✅ Task created
- **Resolution:** Added as S013 in Steps.md Phase 8.

#### [TASK] P6-016: No test verifying rollback in purchase transition prevents status persistence
- **Files:** `apps/server/server/src/tracking/shopping_list_items/service.rs`
- **Severity:** High
- **Detail:** `update_shopping_list_item` (Pending→Purchased) calls `create_item_event_in_tx`
  after updating status; if the event creation fails (E-7 violation), the code calls
  `tx.rollback()`. No test verifies that the shopping list item status was NOT persisted after
  the rollback. A future refactor swapping commit/rollback order would silently corrupt state.
- **Relates to:** E-7, FA-013
- **Status:** ✅ Task created
- **Resolution:** Added as S014 in Steps.md Phase 8.

#### [TASK] P6-017: Derive sqlx::Type on ShoppingListItemStatus — replace str_to_status/status_to_str
- **Files:** `apps/server/server/src/tracking/shopping_list_items/models.rs:11`,
  `apps/server/server/src/tracking/shopping_list_items/service.rs:20, 32`
- **Severity:** High
- **Detail:** `ShoppingListItemStatus` does not derive `sqlx::Type`, forcing manual
  `str_to_status`/`status_to_str` conversion functions and keeping `status: String` in the row
  and response types. This means the type system cannot prevent unrecognized DB strings (which
  currently return `AppError::Internal`). `ItemEventType` already derives `sqlx::Type` and works
  correctly — the same pattern should be applied.
- **Fix:** Add `#[derive(sqlx::Type)] #[sqlx(type_name = "varchar", rename_all = "snake_case")]`
  to `ShoppingListItemStatus`; change `status: String` → `status: ShoppingListItemStatus` in both
  row and response types; delete `str_to_status`/`status_to_str`.
- **Status:** ✅ Task created
- **Resolution:** Added as S015 in Steps.md Phase 8.

#### [TASK] P6-018: Change event_type: String to event_type: ItemEventType in row/response types
- **Files:** `apps/server/server/src/tracking/item_events/models.rs:29, 46`
- **Severity:** High
- **Detail:** `TrackingItemEventRow.event_type` and `TrackingItemEvent.event_type` are both
  `String`, even though `ItemEventType` already derives `sqlx::Type`. The type-safety won at
  the request boundary (`CreateItemEventRequest.event_type: ItemEventType`) is lost on the
  response path. `event_type_to_str` in `service.rs` would become dead code.
- **Fix:** Change both fields to `event_type: ItemEventType` and delete `event_type_to_str`.
- **Status:** ✅ Task created
- **Resolution:** Added as S016 in Steps.md Phase 8.

---

### Architectural Concerns

#### [ADR] P6-019: UpdateItemRequest COALESCE pattern cannot clear nullable fields
- **Files:** `apps/server/server/src/tracking/items/service.rs:139-146` (and analogous
  patterns in locations, categories, shopping_lists)
- **Severity:** Medium
- **Detail:** The COALESCE update pattern (`field = COALESCE($n, field)`) interprets `None` as
  "no change." There is no way to set `location_id`, `category_id`, `expires_at`, `description`,
  or `barcode` back to NULL once set. This is a functional gap for `expires_at` especially (a
  client may need to remove an expiry date). The standard Rust solution is a `Patch<T>` enum
  (`Missing` | `Null` | `Value(T)`) with a custom `Deserialize` impl, but this requires a design
  decision since it affects the API contract and all update endpoints.
- **Relates to:** Tech.md (update endpoint design)
- **Status:** ✅ ADR created
- **Resolution:** ADR-019 (COALESCE limitation accepted; clearing deferred until use case arises)

---

### Convention Gaps

#### [RULE] P6-020: `let _ = tx.rollback().await` pattern should always log on error
- **Files:** 8 call sites across `item_events/service.rs` and `shopping_list_items/service.rs`
- **Severity:** Medium
- **Detail:** The `let _ = tx.rollback().await` pattern appeared in 8 places across the new module.
  The current `rust-axum.md` convention rules do not address explicit rollback handling. This
  pattern was flagged as a silent failure risk: rollback errors are invisible in production logs.
  The pattern should be codified so future modules don't repeat it.
- **Suggested rule:** Add to `.claude/rules/rust-axum.md`: "Never write `let _ =
  tx.rollback().await`. Always log rollback errors at warn level before discarding:
  `if let Err(e) = tx.rollback().await { tracing::warn!(...) }`"
- **Status:** ✅ Rule updated
- **Resolution:** Added to .claude/rules/rust-axum.md Error Handling section.

---

### Additional Missing Tasks (from suggestions)

#### [TASK] P6-021: No test for str_to_status error path
- **Files:** `apps/server/server/src/tracking/shopping_list_items/service.rs:32`
- **Severity:** Low
- **Detail:** `str_to_status` returns `AppError::Internal` for an unrecognized DB string. The
  comment says "unreachable in practice" but a DB migration adding a new status before the server
  is updated, or a direct DB patch, could trigger it. No test covers this path.
- **Status:** ✅ Task created
- **Resolution:** Added as S015-T in Steps.md Phase 8.

#### [TASK] P6-022: No pagination tests for list_item_events
- **Files:** `apps/server/server/src/tracking/item_events/service.rs`
- **Severity:** Low
- **Detail:** The existing test uses `limit=50, offset=0`. No test verifies that `offset`
  correctly skips rows, that a large offset returns an empty list, or that `limit=0` is handled
  cleanly.
- **Status:** ✅ Task created
- **Resolution:** Added as S018 in Steps.md Phase 8.

#### [TASK] P6-023: No test for update_item with cross-household location_id
- **Files:** `apps/server/server/src/tracking/items/service.rs`
- **Severity:** Low
- **Detail:** `location_from_different_household_returns_unprocessable` tests the `create_item`
  path. The `update_item` path also calls `validate_location_household` when `location_id` is
  Some, but this wiring has no test.
- **Relates to:** E-8
- **Status:** ✅ Task created
- **Resolution:** Added as S019 in Steps.md Phase 8.

#### [TASK] P6-024: Categories and shopping_lists lacking 403 tests for update/delete/get
- **Files:** `apps/server/server/src/tracking/categories/service.rs`,
  `apps/server/server/src/tracking/shopping_lists/service.rs`
- **Severity:** Medium
- **Detail:** Both modules test the 403 path on create. However `get_category`,
  `update_category`, `delete_category`, `update_shopping_list`, and `delete_shopping_list` have
  no non-member 403 tests. The `assert_household_member` primitive is tested in isolation but its
  wiring into these individual operations is not exercised.
- **Relates to:** FA-001
- **Status:** ✅ Task created
- **Resolution:** Added as S017 in Steps.md Phase 8.

### Additional Fix-Now (from suggestions)

#### [FIX] P6-025: Shopping list item tests compare status as raw strings instead of typed enum
- **Files:** `apps/server/server/src/tracking/shopping_list_items/service.rs` (test module)
- **Severity:** Low
- **Detail:** Tests assert `assert_eq!(updated.status, "purchased")` against string literals.
  If the serialization or DB representation changes (e.g., to `"PURCHASED"`), tests fail with a
  confusing string mismatch rather than a type error. This is a test quality issue — once
  `ShoppingListItemStatus` derives `sqlx::Type` (P6-017), these assertions should be updated to
  compare the enum variant directly.
- **Status:** ⏸ Deferred
- **Resolution:** Blocked on P6-017 (S015) — will be updated when ShoppingListItemStatus derives sqlx::Type.

#### [FIX] P6-026: Empty-update request (all fields None) silently returns 200 as a no-op
- **Files:** `apps/server/server/src/tracking/locations/service.rs`,
  `categories/service.rs`, `items/service.rs`, `shopping_lists/service.rs`
- **Severity:** Low
- **Detail:** `UpdateXRequest` with all optional fields set to `None` executes a no-op UPDATE and
  returns 200. Callers get no indication that nothing changed. Should return 400 Bad Request when
  no fields are provided.
- **Status:** ✅ Fixed
- **Resolution:** All-None check added to update handlers for locations, categories, items, and shopping_lists; returns 400 BadRequest.

### Additional Architectural Concerns (from suggestions)

#### [ADR] P6-027: Consider typed ID newtypes (ItemId, HouseholdId, UserId)
- **Files:** All `apps/server/server/src/tracking/*/service.rs` function signatures
- **Severity:** Low
- **Detail:** All entity IDs are raw `uuid::Uuid`. Function signatures like
  `fn get_item(pool, user_id: Uuid, household_id: Uuid, item_id: Uuid)` allow any permutation
  of IDs to be passed without a compile error. Typed newtypes (e.g., `struct ItemId(Uuid)`) would
  make this impossible. Cost: `From`/`Into`, `sqlx::Type`, `Serialize`/`Deserialize` impls per
  type. Warrants a design decision before introducing more domain modules.
- **Status:** ✅ ADR created
- **Resolution:** ADR-020 (typed newtypes deferred; current uniform ordering pattern accepted)

---

## Resolution Checklist
- [x] All [FIX] findings resolved (13 fixed, 1 deferred pending P6-017)
- [x] All [TASK] findings added to Steps.md
- [x] All [ADR] findings have ADRs created or dismissed
- [x] All [RULE] findings applied or dismissed
- [ ] Review verified by review-verify agent

---

## Resolution Summary
**Resolved at:** 2026-04-15
**Session:** review-resolve for PR #6 (Tracking Domain)

| Category | Total | Resolved | Deferred |
|---|---|---|---|
| [FIX] | 14 | 13 | 1 (P6-025, blocked on P6-017) |
| [TASK] | 10 | 10 | 0 |
| [ADR] | 2 | 2 | 0 |
| [RULE] | 1 | 1 | 0 |
| **Total** | **27** | **26** | **1** |
