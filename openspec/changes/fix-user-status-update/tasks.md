## 1. Investigation & Root Cause Analysis
- [ ] 1.1 Create `SurrealUserRepositoryTest.kt` with a failing test that reproduces the bug
- [ ] 1.2 Add debug logging to capture actual SurrealQL being executed
- [ ] 1.3 Verify if UPDATE returns affected rows or updated record in SurrealDB

## 2. Implementation
- [ ] 2.1 Add `RETURN AFTER` to the UPDATE statement in `SurrealUserRepository.update()`
- [ ] 2.2 Change from `executeBind()` to `queryBind()` to capture the returned result
- [ ] 2.3 Parse the returned record directly instead of calling `findById()`
- [ ] 2.4 Add verification that the returned record matches expected values

## 3. Test Coverage
- [ ] 3.1 Add test: `update() persists status change from ACTIVE to DISABLED`
- [ ] 3.2 Add test: `update() persists status change from DISABLED to ACTIVE`
- [ ] 3.3 Add test: `update() persists multiple field changes including status`
- [ ] 3.4 Add test: `update() returns NotFound error for non-existent user`

## 4. Verification
- [ ] 4.1 Run `./gradlew :server:test` to verify all tests pass
- [ ] 4.2 Manually verify update behavior in local SurrealDB instance
