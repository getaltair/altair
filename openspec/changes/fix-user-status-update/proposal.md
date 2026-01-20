# Change: Fix SurrealUserRepository.update() not persisting status changes

GitHub Issue: https://github.com/getaltair/altair/issues/22

## Why

The `SurrealUserRepository.update()` method does not persist `status` field changes. After calling update with a modified status, the returned user still has the original status. This breaks account suspension/deletion features.

The issue is that `SurrealDbClient.executeBind()` and `execute()` methods don't check if the UPDATE statement actually succeeded. SurrealDB may silently fail to update if:

1. The UPDATE query returns no results (record doesn't exist or WHERE clause fails)
2. The UPDATE succeeds but with 0 affected rows
3. A field name mismatch prevents the update

Currently, the repository pattern:
1. Calls `executeBind()` which runs the query and discards the result
2. Calls `findById()` to return the "updated" record
3. The stale record is returned without verification

## What Changes

1. **Modify `SurrealDbClient`** to add a `queryBindReturning()` method that returns the updated record(s) using SurrealDB's `RETURN AFTER` clause
2. **Update `SurrealUserRepository.update()`** to use `RETURN AFTER` and verify the update succeeded by comparing field values
3. **Add test coverage** for user status update operations

## Impact

- Affected specs: `database-server`
- Affected code:
  - `server/src/main/kotlin/com/getaltair/altair/db/SurrealDbClient.kt`
  - `server/src/main/kotlin/com/getaltair/altair/db/repository/SurrealUserRepository.kt`
  - `server/src/test/kotlin/com/getaltair/altair/db/repository/SurrealUserRepositoryTest.kt` (new)
- Risk: Low - isolated change to one repository with comprehensive test coverage
- Breaking changes: None
