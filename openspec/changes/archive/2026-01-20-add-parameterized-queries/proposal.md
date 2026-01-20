# Change: Add parameterized queries to SurrealDB repositories

GitHub Issue: https://github.com/getaltair/altair/issues/20

## Why

Current SurrealDB repository implementations use string interpolation for query construction with manual escaping (`replace("'", "''")`). While the current implementation uses Ulid types which validate format, parameterized queries would:

1. Provide defense-in-depth against SQL/NoSQL injection attacks
2. Align with database access best practices
3. Reduce cognitive overhead from manual escaping patterns
4. Enable the database to cache and optimize query plans

## What Changes

1. **Extend SurrealDbClient** with a `queryBind` method that accepts query parameters using SurrealDB Java SDK's native parameter binding
2. **Update SurrealRepositoryBase** with helper methods for parameterized queries
3. **Migrate all 20 SurrealDB repository implementations** to use parameterized queries instead of string interpolation
4. **Add security tests** to validate injection prevention

## Impact

- **Affected specs**: `database-server`
- **Affected code**:
  - `server/src/main/kotlin/com/getaltair/altair/db/SurrealDbClient.kt`
  - `server/src/main/kotlin/com/getaltair/altair/db/repository/SurrealRepositoryBase.kt`
  - All 20 `Surreal*Repository.kt` files in `server/src/main/kotlin/com/getaltair/altair/db/repository/`
- **Risk**: Low - this is a refactoring that maintains existing behavior while improving security posture
- **Breaking changes**: None - internal implementation change only
