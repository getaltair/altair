# Plan: Address PR #15 Review Feedback

## Task Description

Address all critical and high-severity findings from the comprehensive PR #15 review. The review identified 3 critical issues (authorization bypasses, COALESCE null-clearing bug) and 5 high issues (stringly-typed anti-pattern, missing validation, confidence range, blanket error handling, zero service-layer tests). Medium issues already have GitHub issues (#16-#19) and are deferred.

## Objective

Fix all critical and high-severity review findings so PR #15 can be merged with confidence. When complete, the relations module will have proper authorization, nullable fields can be cleared, domain types use Rust enums instead of raw strings, all request types have validation, DB constraint violations return proper HTTP status codes, and service-layer logic has test coverage.

## Problem Statement

PR #15 introduces core domain modules (initiatives, tags, relations, attachments) with well-structured code but has security and correctness gaps:

1. **Security**: The relations module has two authorization bypasses where any authenticated user can read/modify any relation
2. **Data integrity**: COALESCE-based partial updates silently prevent clearing nullable fields (description, color)
3. **Type safety**: 6 domain concepts are stringly-typed despite the codebase having `HouseholdRole` as an enum precedent
4. **Validation gaps**: Relations request types lack `Validate` derives; confidence range unchecked in Rust
5. **Error UX**: All DB constraint violations surface as generic 500s instead of meaningful 400/409s
6. **Test coverage**: Zero service-layer tests for authorization, validation, or error paths

## Solution Approach

**Phase 1 (Foundation)**: Replace string constants in `contracts.rs` with proper Rust enums. This is the highest-leverage change because it eliminates manual string validation throughout the service layer, makes invalid states unrepresentable at the type level, and follows the existing `HouseholdRole` precedent. The enums get `Serialize`/`Deserialize` (automatic request validation) and `sqlx::Type` (direct DB mapping).

**Phase 2 (Critical fixes)**: Fix the two authorization bypasses in the relations module and the COALESCE null-clearing bug in initiatives/tags updates.

**Phase 3 (Validation and error handling)**: Add `Validate` derives to relation request types, add confidence range validation, and improve `map_err` closures to distinguish constraint violations from generic DB errors.

**Phase 4 (Tests)**: Add service-layer unit tests covering authorization, validation, and error paths.

## Relevant Files

**Files to modify:**

- `apps/server/src/contracts.rs` -- Replace const string arrays with enum definitions; remove `is_valid_*` functions
- `apps/server/src/core/initiatives/models.rs` -- Change `status: String` to `InitiativeStatus` enum on domain and request types
- `apps/server/src/core/initiatives/service.rs` -- Remove manual status validation; fix COALESCE update pattern
- `apps/server/src/core/initiatives/handlers.rs` -- No changes expected (validation moves to deserialization)
- `apps/server/src/core/tags/models.rs` -- No enum fields, but update tests if needed
- `apps/server/src/core/tags/service.rs` -- Fix COALESCE update pattern for `color` field
- `apps/server/src/core/relations/models.rs` -- Change string fields to enum types; add `Validate` derive; add confidence range validation
- `apps/server/src/core/relations/service.rs` -- Fix authorization bypass; remove manual string validation; improve error handling
- `apps/server/src/core/relations/handlers.rs` -- Use `auth.user_id`; add `.validate()` calls
- `apps/server/src/core/attachments/models.rs` -- Change `entity_type` and `processing_state` to enum types
- `apps/server/src/error.rs` -- No structural changes, but services will use `AppError::Conflict`/`AppError::BadRequest` more precisely

**Existing pattern to follow:**

- `apps/server/src/core/households/models.rs` -- `HouseholdRole` enum is the reference implementation for domain enums
- `apps/server/src/core/households/service.rs` -- `add_member` shows the correct `map_err` pattern for constraint violations (lines 72-77)

### New Files

- None. All changes are modifications to existing files.

## Implementation Phases

### Phase 1: Foundation -- Domain Enums

Replace the stringly-typed pattern with proper Rust enums. This is the highest-leverage change.

**Define enums in `contracts.rs`:**

```rust
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum InitiativeStatus {
    Active,
    Paused,
    Completed,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum EntityType {
    User,
    Household,
    Initiative,
    Tag,
    Attachment,
    GuidanceEpic,
    GuidanceQuest,
    GuidanceRoutine,
    GuidanceFocusSession,
    GuidanceDailyCheckin,
    KnowledgeNote,
    KnowledgeNoteSnapshot,
    TrackingLocation,
    TrackingCategory,
    TrackingItem,
    TrackingItemEvent,
    TrackingShoppingList,
    TrackingShoppingListItem,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum RelationType {
    References,
    Supports,
    Requires,
    RelatedTo,
    DependsOn,
    Duplicates,
    SimilarTo,
    GeneratedFrom,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum SourceType {
    User,
    Ai,
    Import,
    Rule,
    Migration,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum RelationStatus {
    Accepted,
    Suggested,
    Dismissed,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "text", rename_all = "snake_case")]
pub enum ProcessingState {
    Pending,
    Processing,
    Ready,
    Failed,
}
```

Each enum should also implement `Display` and `as_str()` following the `HouseholdRole` pattern.

**Update domain types:**

- `Initiative.status`: `String` -> `InitiativeStatus`
- `EntityRelation.from_entity_type`, `.to_entity_type`: `String` -> `EntityType`
- `EntityRelation.relation_type`: `String` -> `RelationType`
- `EntityRelation.source_type`: `String` -> `SourceType`
- `EntityRelation.status`: `String` -> `RelationStatus`
- `Attachment.entity_type`: `String` -> `EntityType`
- `Attachment.processing_state`: `String` -> `ProcessingState`

**Update request types:**

- `CreateInitiativeRequest.status`: `Option<String>` -> `Option<InitiativeStatus>`
- `UpdateInitiativeRequest.status`: `Option<String>` -> `Option<InitiativeStatus>`
- `CreateRelationRequest.from_entity_type`, `.to_entity_type`: `String` -> `EntityType`
- `CreateRelationRequest.relation_type`: `String` -> `RelationType`
- `CreateRelationRequest.source_type`: `Option<String>` -> `Option<SourceType>`
- `UpdateRelationStatusRequest.status`: `String` -> `RelationStatus`
- `RelationQuery` filter fields: `Option<String>` -> `Option<EntityType>`, `Option<RelationType>`, `Option<RelationStatus>`

**Remove from service layers:**

- `initiatives/service.rs`: Remove `use crate::contracts::INITIATIVE_STATUSES` and all manual status string checks
- `relations/service.rs`: Remove all `contracts::is_valid_*()` calls -- deserialization handles this

**Update tests in `contracts.rs`:**
Replace string validation tests with enum serialization/deserialization roundtrip tests.

**Important sqlx note:** If `sqlx::Type` with `type_name = "text"` does not work for TEXT columns, fall back to keeping domain types as `String` and using enums only on request types. In that case, add `.to_string()` / `as_str()` conversions at the service layer boundary. The builder should verify with `cargo build` early in this phase.

### Phase 2: Core Implementation -- Critical Fixes

**Fix 1: Relations authorization (`relations/service.rs` + `relations/handlers.rs`)**

`update_relation_status`:

- Rename `_user_id` to `user_id`
- Add `AND created_by_user_id = $4` to the UPDATE WHERE clause
- Bind `user_id` as the 4th parameter

`query_relations`:

- Add `user_id: Uuid` parameter to the service function
- Add `AND created_by_user_id = ?` condition to the dynamic WHERE builder
- This ensures users can only query relations they created

`handlers.rs`:

- `query_relations` handler: Change `_auth` to `auth`, pass `auth.user_id` to service
- `update_relation_status` handler: Already passes `auth.user_id` (no change needed)

**Fix 2: COALESCE null-clearing (`initiatives/service.rs` + `tags/service.rs`)**

Replace static COALESCE UPDATE queries with dynamic SET clause building using `QueryBuilder`:

```rust
// initiatives/service.rs - update_initiative
pub async fn update_initiative(...) -> Result<Initiative, AppError> {
    // Status validation (now type-safe via enum, but still need to check if provided)

    let mut qb: QueryBuilder<sqlx::Postgres> = QueryBuilder::new("UPDATE initiatives SET updated_at = now()");

    if let Some(ref name) = req.name {
        qb.push(", name = ");
        qb.push_bind(name.clone());
    }
    if req.description.is_some() {
        // description is Option<Option<String>> -- Some(None) means "set to null"
        // But currently it's Option<String>, so we need to change the type
        qb.push(", description = ");
        qb.push_bind(req.description.clone());
    }
    if let Some(ref status) = req.status {
        qb.push(", status = ");
        qb.push_bind(status.as_str());
    }

    qb.push(" WHERE id = ");
    qb.push_bind(id);
    qb.push(" AND user_id = ");
    qb.push_bind(user_id);
    qb.push(" RETURNING id, user_id, household_id, name, description, status, created_at, updated_at");

    qb.build_query_as::<Initiative>()
        .fetch_optional(pool)
        .await
        .map_err(AppError::Database)?
        .ok_or_else(|| AppError::NotFound("Initiative not found or you do not have permission".to_string()))
}
```

The key insight: change `UpdateInitiativeRequest.description` from `Option<String>` to a type that distinguishes "not provided" from "explicitly null". The simplest approach without a custom `Patch<T>` type: use `#[serde(default, deserialize_with = "...")]` or simply treat any `Some` value (including `Some(None)` if using `Option<Option<String>>`) as "set this field." With `Option<Option<String>>`:

- `None` (field absent from JSON) = don't touch
- `Some(None)` (field present as `null`) = set to NULL
- `Some(Some("value"))` (field present with value) = set to value

Apply the same pattern to `tags/service.rs` for the `color` field.

### Phase 3: Integration & Polish

**Validation on relation request types (`relations/models.rs` + `relations/handlers.rs`):**

Add `#[derive(Validate)]` to:

- `CreateRelationRequest` -- add `#[validate(range(min = 0.0, max = 1.0))]` on `confidence`
- `UpdateRelationStatusRequest`

Add `.validate().map_err(|e| AppError::BadRequest(e.to_string()))?` in:

- `create_relation` handler
- `update_relation_status` handler

**Improved DB error handling (all service files):**

Follow the `households::service::add_member` pattern (lines 72-77). In every `.map_err(AppError::Database)` on INSERT/UPDATE queries, match on constraint violations:

```rust
.map_err(|e| match &e {
    sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
        AppError::Conflict("Resource already exists".to_string())
    }
    sqlx::Error::Database(db_err) if db_err.is_check_violation() => {
        AppError::BadRequest("Invalid field value".to_string())
    }
    _ => AppError::Database(e),
})
```

Apply to:

- `initiatives/service.rs`: `create_initiative`, `update_initiative`
- `tags/service.rs`: `create_tag`, `update_tag`
- `relations/service.rs`: `create_relation`, `update_relation_status`

**Add `Clone` to domain types:**
Add `Clone` to the derive list on `Initiative`, `Tag`, `EntityRelation`, `Attachment` to match the existing `Household` pattern.

**Service-layer tests:**

Add tests in each service module covering:

For initiatives:

- `create_initiative` with invalid household_id (user not a member) returns Forbidden
- `get_initiative` when user is not owner and not household member returns Forbidden
- `update_initiative` for non-owner returns NotFound
- `soft_delete_initiative` for non-owner returns NotFound

For tags:

- `create_tag` with invalid household_id returns Forbidden
- `update_tag` for non-owner returns NotFound
- `delete_tag` for non-owner returns NotFound

For relations:

- `create_relation` stores the correct `created_by_user_id`
- `query_relations` only returns relations owned by the requesting user
- `update_relation_status` by non-owner returns NotFound
- `query_relations` with no entity IDs returns BadRequest

For contracts (update existing tests):

- Enum serialization roundtrips for all enum types
- Deserialization of invalid values produces errors

Note: Service-layer tests require a PgPool. Use `#[sqlx::test]` with a test database, or if that infrastructure is not yet set up, the builder should set up a minimal test harness using sqlx's test utilities. If DB-backed tests are not feasible in this PR, add tests for the synchronous validation branches only (the paths that return errors before any DB call).

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
  - Name: builder-enums
  - Role: Introduce domain enums in contracts.rs, update all model files and service files to use them, update existing tests
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-auth-fixes
  - Role: Fix authorization bypasses in relations module and COALESCE null-clearing bug in initiatives/tags updates
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-validation
  - Role: Add Validate derives to relation request types, confidence range validation, and improved DB constraint error handling across all services
  - Agent Type: backend-engineer
  - Resume: true

- Specialist
  - Name: builder-tests
  - Role: Add service-layer tests for authorization, validation, and error paths across initiatives, tags, and relations modules
  - Agent Type: backend-engineer
  - Resume: true

- Quality Engineer (Validator)
  - Name: validator
  - Role: Validate completed work against acceptance criteria (read-only inspection mode)
  - Agent Type: quality-engineer
  - Resume: false

## Step by Step Tasks

### 1. Introduce Domain Enums

- **Task ID**: introduce-enums
- **Depends On**: none
- **Assigned To**: builder-enums
- **Agent Type**: backend-engineer
- **Parallel**: false (foundation -- everything else depends on this)
- Rewrite `apps/server/src/contracts.rs`: replace all `const &[&str]` arrays and `is_valid_*` functions with enum definitions (`InitiativeStatus`, `EntityType`, `RelationType`, `SourceType`, `RelationStatus`, `ProcessingState`). Each enum derives `Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type` with `#[serde(rename_all = "snake_case")]` and `#[sqlx(type_name = "text", rename_all = "snake_case")]`. Implement `Display` and `as_str()` for each, following the `HouseholdRole` pattern in `apps/server/src/core/households/models.rs`.
- Update `apps/server/src/core/initiatives/models.rs`: change `Initiative.status` from `String` to `InitiativeStatus`, `CreateInitiativeRequest.status` from `Option<String>` to `Option<InitiativeStatus>`, `UpdateInitiativeRequest.status` from `Option<String>` to `Option<InitiativeStatus>`. Update model tests.
- Update `apps/server/src/core/relations/models.rs`: change all string-typed fields to their enum equivalents on `EntityRelation`, `CreateRelationRequest`, `UpdateRelationStatusRequest`, and `RelationQuery`.
- Update `apps/server/src/core/attachments/models.rs`: change `entity_type` to `EntityType`, `processing_state` to `ProcessingState`.
- Update `apps/server/src/core/initiatives/service.rs`: remove `use crate::contracts::INITIATIVE_STATUSES` and all manual status string validation. Status defaults should use `InitiativeStatus::Active` instead of `"active"`. The soft-delete should use `InitiativeStatus::Archived`.
- Update `apps/server/src/core/relations/service.rs`: remove all `contracts::is_valid_*()` calls. Bind enum values directly in queries. The `source_type` default becomes `SourceType::User`.
- Add `Clone` to derive lists on `Initiative`, `Tag`, `EntityRelation`, `Attachment`.
- Replace string validation tests in `contracts.rs` with enum serialization/deserialization roundtrip tests.
- Run `cargo build` early to verify `sqlx::Type` works with TEXT columns. If it does not compile, fall back to using enums on request types only and `.to_string()` at the service boundary.
- Run `cargo test` to verify all existing tests still pass.

### 2. Fix Relations Authorization

- **Task ID**: fix-relations-auth
- **Depends On**: introduce-enums
- **Assigned To**: builder-auth-fixes
- **Agent Type**: backend-engineer
- **Parallel**: true (can run in parallel with tasks 3 and 4 after task 1 completes)
- In `apps/server/src/core/relations/service.rs`, `update_relation_status` function: rename `_user_id` to `user_id`, add `AND created_by_user_id = $4` to the UPDATE WHERE clause, bind `user_id` as the 4th parameter.
- In `apps/server/src/core/relations/service.rs`, `query_relations` function: add `user_id: Uuid` parameter, add `created_by_user_id = <bind>` as a mandatory WHERE condition (always applied, not optional).
- In `apps/server/src/core/relations/handlers.rs`: change `_auth: AuthenticatedUser` to `auth: AuthenticatedUser` in `query_relations` handler, pass `auth.user_id` to `service::query_relations`.
- Update the doc comment on `update_relation_status` to note that only the relation creator can update status.
- Run `cargo build` and `cargo test`.

### 3. Fix COALESCE Null-Clearing Bug

- **Task ID**: fix-coalesce
- **Depends On**: introduce-enums
- **Assigned To**: builder-auth-fixes
- **Agent Type**: backend-engineer
- **Parallel**: true (can run in parallel with tasks 2 and 4, but same agent as task 2 so sequential for that agent)
- In `apps/server/src/core/initiatives/models.rs`: change `UpdateInitiativeRequest.description` from `Option<String>` to `Option<Option<String>>` to distinguish "not provided" from "explicitly null". Add `#[serde(default, deserialize_with = "deserialize_optional_field")]` or use `#[serde(default)]` with `#[serde(deserialize_with = "double_option")]` pattern. Consider defining a small serde helper if needed.
- In `apps/server/src/core/initiatives/service.rs`, `update_initiative`: replace the static COALESCE UPDATE query with dynamic SET clause building using `QueryBuilder`. Only set fields that are `Some(...)`. For `description`, `Some(None)` means set to NULL, `Some(Some(val))` means set to val, `None` means don't touch.
- In `apps/server/src/core/tags/models.rs`: change `UpdateTagRequest.color` from `Option<String>` to `Option<Option<String>>` with the same serde treatment.
- In `apps/server/src/core/tags/service.rs`, `update_tag`: replace the static COALESCE UPDATE query with dynamic SET clause building.
- Update the doc comments on `update_initiative` and `update_tag` to accurately describe the new partial update behavior. Remove the misleading "Uses COALESCE" language.
- Update model tests to cover the new `Option<Option<String>>` deserialization behavior.
- Run `cargo build` and `cargo test`.

### 4. Add Validation and Improve Error Handling

- **Task ID**: add-validation
- **Depends On**: introduce-enums
- **Assigned To**: builder-validation
- **Agent Type**: backend-engineer
- **Parallel**: true (can run in parallel with tasks 2 and 3)
- In `apps/server/src/core/relations/models.rs`: add `#[derive(Validate)]` to `CreateRelationRequest` and `UpdateRelationStatusRequest`. Add `#[validate(range(min = 0.0, max = 1.0))]` on the `confidence` field of `CreateRelationRequest`. Add `use validator::Validate;` import.
- In `apps/server/src/core/relations/handlers.rs`: add `use validator::Validate;` import. Add `body.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;` before the service call in both `create_relation` and `update_relation_status` handlers.
- In all service files that use `.map_err(AppError::Database)` on INSERT/UPDATE queries, replace with a match that checks for constraint violations. Follow the `households::service::add_member` pattern at lines 72-77. Apply to:
  - `apps/server/src/core/initiatives/service.rs`: `create_initiative` (line 51), `update_initiative`
  - `apps/server/src/core/tags/service.rs`: `create_tag` (line 38), `update_tag`
  - `apps/server/src/core/relations/service.rs`: `create_relation`, `update_relation_status`
- Add model-level tests for confidence range validation (valid at 0.0, 0.5, 1.0; invalid at -0.1, 1.1).
- Run `cargo build` and `cargo test`.

### 5. Add Service-Layer Tests

- **Task ID**: add-service-tests
- **Depends On**: fix-relations-auth, fix-coalesce, add-validation
- **Assigned To**: builder-tests
- **Agent Type**: backend-engineer
- **Parallel**: false (depends on all implementation tasks)
- Check if `#[sqlx::test]` infrastructure exists in the project. If not, determine whether to set up DB-backed tests or test only synchronous validation paths.
- Add tests in `apps/server/src/core/relations/service.rs` (or a dedicated test module):
  - `query_relations` with no entity IDs returns `BadRequest`
  - `update_relation_status` with non-creator user_id returns `NotFound` (requires DB)
  - `create_relation` roundtrip stores correct `created_by_user_id` (requires DB)
- Add tests in `apps/server/src/core/initiatives/service.rs`:
  - `update_initiative` with description explicitly set to null clears the field (requires DB)
  - `soft_delete_initiative` sets status to Archived (requires DB)
- Add tests in `apps/server/src/contracts.rs`:
  - Enum serde roundtrip tests for all 6 enum types
  - Deserialization of unknown variant returns error
  - `as_str()` returns correct lowercase string for each variant
- If DB-backed tests are not feasible, focus on the contracts enum tests and model validation tests only, and document remaining test gaps as TODO comments.
- Run `cargo test` to verify all tests pass.

### 6. Final Build and Clippy Check

- **Task ID**: final-build
- **Depends On**: add-service-tests
- **Assigned To**: builder-tests
- **Agent Type**: backend-engineer
- **Parallel**: false
- Run `cargo build` to verify zero errors
- Run `cargo test` to verify all tests pass
- Run `cargo clippy -- -D warnings` to verify zero warnings
- Fix any issues found

### 7. Validate All Work

- **Task ID**: validate-all
- **Depends On**: final-build
- **Assigned To**: validator
- **Agent Type**: quality-engineer
- **Parallel**: false
- Run all validation commands
- Verify acceptance criteria met
- Operate in validation mode: inspect and report only, do not modify files
- Specifically verify:
  - `contracts.rs` contains enum definitions, not const string arrays
  - `relations/service.rs` uses `user_id` (not `_user_id`) in `update_relation_status`
  - `relations/handlers.rs` uses `auth` (not `_auth`) in `query_relations`
  - `relations/handlers.rs` calls `.validate()` on request bodies
  - `initiatives/service.rs` and `tags/service.rs` do NOT use COALESCE for nullable fields
  - All INSERT/UPDATE `map_err` closures check for constraint violations
  - All domain types derive `Clone`
  - `cargo build`, `cargo test`, `cargo clippy -- -D warnings` all pass clean

## Acceptance Criteria

- [ ] `cargo build` completes with 0 errors
- [ ] `cargo test` passes all tests (existing + new)
- [ ] `cargo clippy -- -D warnings` produces 0 warnings
- [ ] `contracts.rs` defines 6 Rust enums replacing all const string arrays
- [ ] No `is_valid_*` functions remain in `contracts.rs`
- [ ] `Initiative.status` is `InitiativeStatus` enum, not `String`
- [ ] `EntityRelation` fields use `EntityType`, `RelationType`, `SourceType`, `RelationStatus` enums
- [ ] `update_relation_status` enforces `created_by_user_id` ownership
- [ ] `query_relations` scopes results by `created_by_user_id`
- [ ] `query_relations` handler uses `auth.user_id` (no underscore prefix)
- [ ] `UpdateInitiativeRequest.description` supports explicit null-clearing
- [ ] `UpdateTagRequest.color` supports explicit null-clearing
- [ ] `CreateRelationRequest` derives `Validate` with `confidence` range check
- [ ] `UpdateRelationStatusRequest` derives `Validate`
- [ ] Both relation handlers call `.validate()` before service calls
- [ ] INSERT/UPDATE queries in all services match on constraint violations (unique, check) instead of blanket `AppError::Database`
- [ ] All domain types (`Initiative`, `Tag`, `EntityRelation`, `Attachment`) derive `Clone`
- [ ] New service-layer or enum tests are present and passing

## Validation Commands

Execute these commands to validate the task is complete:

- `cargo build` - Verify the project builds without errors
- `cargo test` - Run the full test suite and verify all tests pass
- `cargo clippy -- -D warnings` - Verify no clippy warnings exist
- `grep -r "is_valid_entity_type\|is_valid_relation_type\|is_valid_source_type\|is_valid_relation_status" apps/server/src/` - Should return zero results (functions removed)
- `grep -r "INITIATIVE_STATUSES\|RELATION_STATUSES\|ENTITY_TYPES\|RELATION_TYPES\|SOURCE_TYPES\|ATTACHMENT_STATES" apps/server/src/` - Should only appear in test comments, not as const definitions
- `grep "_user_id" apps/server/src/core/relations/service.rs` - Should return zero results (no unused user_id params)
- `grep "_auth" apps/server/src/core/relations/handlers.rs` - Should return zero results (no unused auth params)
- `grep "COALESCE" apps/server/src/core/initiatives/service.rs apps/server/src/core/tags/service.rs` - Should return zero results in update functions

## Notes

- The enum `sqlx::Type` mapping with `type_name = "text"` is the preferred approach. If it does not work with PostgreSQL TEXT columns, the fallback is to use enums on request types only and convert with `.to_string()` / `as_str()` at the service boundary. The builder should verify early with `cargo build`.
- The `Option<Option<String>>` pattern for nullable field clearing requires serde to distinguish between "field absent" and "field present as null". With `#[serde(default)]` on the field, serde will produce `None` when the field is absent and `Some(None)` when the field is `null` in JSON. This is a well-known Rust/serde pattern.
- Medium-severity issues (pagination, CHECK constraints, filter validation, comment cleanup) are tracked as GitHub issues #16-#19 and excluded from this plan.
- The `Attachment` struct is dead code (handlers deferred to Step 16). We update its types for consistency but do not add handlers or tests for it.
