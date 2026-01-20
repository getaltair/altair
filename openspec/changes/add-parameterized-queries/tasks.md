# Tasks: Add parameterized queries to SurrealDB repositories

## 1. SurrealDbClient Enhancement

- [x] 1.1 Add `queryBind` method to `SurrealDbClient` that accepts a `Map<String, Any?>` of parameters
- [x] 1.2 Add `queryBindAs` method for typed deserialization with parameters
- [x] 1.3 Add `executeBind` method for DDL operations with parameters
- [x] 1.4 Add unit tests for the new parameterized query methods

## 2. SurrealRepositoryBase Updates

- [x] 2.1 Update `softDelete` to use parameterized queries
- [x] 2.2 Update `hardDelete` to use parameterized queries
- [x] 2.3 Add helper method for building common parameter maps (userId, table, id) - Not needed; handled directly in repositories

## 3. Repository Migrations - Auth Domain

- [x] 3.1 Migrate `SurrealUserRepository` to parameterized queries
- [x] 3.2 Migrate `SurrealRefreshTokenRepository` to parameterized queries
- [x] 3.3 Migrate `SurrealInviteCodeRepository` to parameterized queries

## 4. Repository Migrations - Guidance Domain

- [x] 4.1 Migrate `SurrealQuestRepository` to parameterized queries
- [x] 4.2 Migrate `SurrealEpicRepository` to parameterized queries
- [x] 4.3 Migrate `SurrealInitiativeRepository` to parameterized queries
- [x] 4.4 Migrate `SurrealCheckpointRepository` to parameterized queries
- [x] 4.5 Migrate `SurrealRoutineRepository` to parameterized queries
- [x] 4.6 Migrate `SurrealEnergyBudgetRepository` to parameterized queries
- [x] 4.7 Migrate `SurrealInboxRepository` to parameterized queries

## 5. Repository Migrations - Knowledge Domain

- [x] 5.1 Migrate `SurrealNoteRepository` to parameterized queries
- [x] 5.2 Migrate `SurrealNoteLinkRepository` to parameterized queries
- [x] 5.3 Migrate `SurrealFolderRepository` to parameterized queries
- [x] 5.4 Migrate `SurrealTagRepository` to parameterized queries
- [x] 5.5 Migrate `SurrealAttachmentRepository` to parameterized queries
- [x] 5.6 Migrate `SurrealSourceDocumentRepository` to parameterized queries

## 6. Repository Migrations - Tracking Domain

- [x] 6.1 Migrate `SurrealItemRepository` to parameterized queries
- [x] 6.2 Migrate `SurrealLocationRepository` to parameterized queries
- [x] 6.3 Migrate `SurrealContainerRepository` to parameterized queries
- [x] 6.4 Migrate `SurrealItemTemplateRepository` to parameterized queries

## 7. Testing & Validation

- [x] 7.1 Add integration tests for SQL injection prevention scenarios
- [x] 7.2 Verify all existing repository tests pass
- [x] 7.3 Run full test suite to ensure no regressions
