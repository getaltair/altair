# Design: Parameterized Queries for SurrealDB Repositories

## Context

The current SurrealDB repository implementations use string interpolation for query construction:

```kotlin
// Current pattern - string interpolation with manual escaping
"SELECT * FROM user WHERE email = '${email.replace("'", "''")}' AND deleted_at IS NONE"
```

This pattern:
- Requires manual escaping for each string value
- Is error-prone if escaping is forgotten
- Prevents query plan caching by the database
- Doesn't align with security best practices

The SurrealDB Java SDK provides a `queryBind` method that supports native parameter binding:

```kotlin
// Proposed pattern - parameterized queries
driver.queryBind(
    "SELECT * FROM user WHERE email = \$email AND deleted_at IS NONE",
    mapOf("email" to email)
)
```

## Goals / Non-Goals

### Goals
- Replace all string interpolation in repository queries with parameterized binding
- Maintain existing API contracts for all repository interfaces
- Preserve all current functionality and behavior
- Improve security posture through defense-in-depth

### Non-Goals
- Changing the repository interface contracts
- Adding new repository methods or capabilities
- Modifying the domain model or DTOs
- Implementing query caching (future enhancement)

## Decisions

### Decision 1: Use SurrealDB Java SDK's `queryBind` Method

**What**: Use the SDK's native `queryBind(query, Map<String, Object>)` method for parameter binding.

**Why**:
- It's the official, supported way to pass parameters in the SurrealDB Java SDK
- Parameters are properly escaped and typed by the SDK
- No need to invent custom parameter binding logic

**Alternatives considered**:
- Custom parameter interpolation: Would duplicate SDK functionality and introduce risk
- Prepared statements: SurrealDB doesn't have a traditional prepared statement API

### Decision 2: Add New Methods to SurrealDbClient

**What**: Add `queryBind`, `queryBindAs`, and `executeBind` methods alongside existing methods.

**Why**:
- Maintains backward compatibility during migration
- Allows incremental migration of repositories
- Clear separation between legacy and new patterns

**New methods**:
```kotlin
suspend fun queryBind(query: String, params: Map<String, Any?>): Either<DomainError, String>
suspend fun <T> queryBindAs(query: String, params: Map<String, Any?>, deserializer: (String) -> T): Either<DomainError, T>
suspend fun executeBind(statement: String, params: Map<String, Any?>): Either<DomainError, Unit>
```

### Decision 3: Parameter Naming Convention

**What**: Use `$paramName` syntax in queries with camelCase parameter names.

**Why**:
- Matches SurrealQL parameter syntax
- Consistent with existing codebase naming conventions
- Clear distinction from table/column names

**Examples**:
```kotlin
// User ID parameter
"SELECT * FROM user WHERE id = user:\$userId"
mapOf("userId" to id.value)

// Multiple parameters
"SELECT * FROM note WHERE user_id = user:\$userId AND title = \$title"
mapOf("userId" to userId.value, "title" to title)
```

### Decision 4: Handle Ulid and Record ID Formatting

**What**: Keep the `table:ulid` formatting in the query string, pass raw ULID value as parameter.

**Why**:
- SurrealDB record IDs have format `table:id`
- The table prefix is static and safe to include in query
- Only the variable portion (the ULID) needs parameterization

**Example**:
```kotlin
// Query uses table prefix, parameter is just the ULID value
"SELECT * FROM user WHERE id = user:\$userId"
mapOf("userId" to userId.value)  // userId.value is "01ABC..."
```

### Decision 5: Migrate Repositories by Domain

**What**: Group repository migrations by domain (Auth, Guidance, Knowledge, Tracking).

**Why**:
- Allows focused testing per domain
- Easier to review changes
- Reduces risk of missing repositories

## Risks / Trade-offs

### Risk: SDK API Stability
The SurrealDB Java SDK is in beta (`1.0.0-beta.1`). The `queryBind` API may change.

**Mitigation**: Wrap SDK calls in our own `SurrealDbClient` methods. If the SDK API changes, we only need to update one file.

### Risk: Parameter Type Handling
SurrealDB may handle certain Kotlin types differently when passed as parameters vs interpolated strings.

**Mitigation**:
- Add comprehensive tests for all data types (String, Long, Boolean, Instant, Ulid)
- Test edge cases (null values, special characters, unicode)

### Trade-off: Verbose Parameter Maps
Parameterized queries require explicit parameter maps, which adds verbosity.

**Acceptance**: This verbosity is acceptable because:
- It makes query parameters explicit and auditable
- IDE tooling can validate parameter names
- It eliminates the risk of injection vulnerabilities

## Migration Plan

1. **Phase 1**: Add new methods to `SurrealDbClient` (non-breaking)
2. **Phase 2**: Update `SurrealRepositoryBase` helper methods
3. **Phase 3**: Migrate Auth domain repositories (SurrealUserRepository, SurrealRefreshTokenRepository, SurrealInviteCodeRepository)
4. **Phase 4**: Migrate Guidance domain repositories
5. **Phase 5**: Migrate Knowledge domain repositories
6. **Phase 6**: Migrate Tracking domain repositories
7. **Phase 7**: Add security tests, remove deprecated methods (if desired)

**Rollback**: Each phase can be rolled back independently since existing methods remain available.

## Open Questions

None - the approach is straightforward given the SDK's existing support for parameterized queries.
