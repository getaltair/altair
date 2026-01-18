# Design: Fix SurrealDB Repository findById Queries

## Context

SurrealDB provides two distinct patterns for retrieving records:

1. **Direct record access** (record link syntax):
   ```surql
   SELECT * FROM table:id
   ```
   Returns the specific record directly. Cannot be combined with WHERE filtering.

2. **Table query with ID filter**:
   ```surql
   SELECT * FROM table WHERE id = table:id AND user_id = user:xyz
   ```
   Queries the table with standard filtering. Allows combining multiple conditions.

Our repositories require user-scoping (`user_id` filter) and soft-delete filtering (`deleted_at IS NONE`) on all queries, making pattern 2 the correct approach.

## Goals / Non-Goals

**Goals:**
- Fix all `findById` queries to use correct SurrealDB syntax
- Maintain user-scoping and soft-delete filtering
- All 59 repository tests pass

**Non-Goals:**
- Refactoring repository architecture
- Adding new repository methods
- Changing the test structure

## Decisions

**Decision: Use table query with ID filter pattern**

Change from:
```kotlin
"SELECT * FROM attachment:${id.value} WHERE user_id = user:${userId.value} AND deleted_at IS NONE"
```

To:
```kotlin
"SELECT * FROM attachment WHERE id = attachment:${id.value} AND user_id = user:${userId.value} AND deleted_at IS NONE"
```

**Rationale:**
- Maintains consistent query pattern with other repository methods (findAll, findByX)
- Allows proper WHERE clause filtering for user-scoping and soft-delete
- Direct record access (`FROM table:id`) is designed for simple lookups without additional filtering

**Alternatives considered:**

1. **Use direct access, filter in Kotlin**: Query `SELECT * FROM table:id`, then validate `user_id` and `deleted_at` in application code.
   - Rejected: Security risk if validation logic has bugs; violates "filter at database" principle

2. **Use two queries**: First direct access, then validate ownership separately.
   - Rejected: Unnecessary complexity and performance overhead

## Risks / Trade-offs

**Risk: Query pattern change may affect performance**
- Mitigation: The `idx_*_user` indexes on all tables ensure efficient user-scoped queries
- The change from direct record access to filtered table query is negligible for single-record lookups

## Migration Plan

1. Update all `findById` methods with correct query syntax
2. Run integration tests to verify fixes
3. No data migration required - this is a query syntax fix only

## Open Questions

None - the fix is straightforward syntax correction.
