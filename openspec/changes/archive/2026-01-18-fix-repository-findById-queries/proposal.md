# Change: Fix SurrealDB Repository findById Queries

## Why

Repository integration tests (42 of 59) are failing because `findById` queries use invalid SurrealDB syntax. The `save()` method successfully creates records but immediately fails when calling `findById()` to return the saved entity, resulting in `NotFoundError`.

GitHub Issue: https://github.com/getaltair/altair/issues/8

## What Changes

- Fix `findById` query syntax in all SurrealDB repository implementations
- Change from `SELECT * FROM table:id WHERE ...` to `SELECT * FROM table WHERE id = table:id AND ...`
- The current syntax `SELECT * FROM attachment:id WHERE ...` fails because SurrealDB's direct record access (`FROM table:id`) cannot be combined with WHERE clause filtering

## Root Cause

In SurrealDB, there are two ways to query records:
1. **Direct record access**: `SELECT * FROM table:id` - returns the specific record
2. **Table query with filter**: `SELECT * FROM table WHERE id = table:id` - queries table with filter

The repositories incorrectly combine pattern 1 with WHERE clauses. When you use `SELECT * FROM attachment:id WHERE user_id = ...`, SurrealDB attempts to filter the direct record access result, but the WHERE clause is not applied as expected, returning empty results.

## Impact

- **Affected code**: All 18 SurrealDB repository implementations in `server/src/main/kotlin/com/getaltair/altair/db/repository/`
- **Affected specs**: None - this is a bug fix implementing existing spec requirements correctly
- **Test impact**: Will fix 42 failing repository tests
