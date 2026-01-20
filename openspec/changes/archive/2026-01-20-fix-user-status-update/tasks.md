## 1. Investigation & Root Cause Analysis
- [x] 1.1 Create `SurrealUserRepositoryTest.kt` with a failing test that reproduces the bug
- [x] 1.2 Add debug logging to capture actual SurrealQL being executed
- [x] 1.3 Verify if UPDATE returns affected rows or updated record in SurrealDB

## 2. Implementation
- [x] 2.1 Add `RETURN AFTER` to the UPDATE statement in `SurrealUserRepository.update()`
- [x] 2.2 Change from `executeBind()` to `queryBind()` to capture the returned result
- [x] 2.3 Parse the returned record directly instead of calling `findById()`
- [x] 2.4 Add verification that the returned record matches expected values

## 3. Test Coverage
- [x] 3.1 Add test: `update() persists status change from ACTIVE to DISABLED`
- [x] 3.2 Add test: `update() persists status change from DISABLED to ACTIVE`
- [x] 3.3 Add test: `update() persists multiple field changes including status`
- [x] 3.4 Add test: `update() returns NotFound error for non-existent user`

## 4. Verification
- [x] 4.1 Run `./gradlew :server:test` to verify all tests pass
- [x] 4.2 Manually verify update behavior in local SurrealDB instance
