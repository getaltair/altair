# Tasks: Fix SurrealDB Repository findById Queries

## 1. Fix Repository Implementations

- [x] 1.1 Fix `SurrealAttachmentRepository.findById()` query syntax
- [x] 1.2 Fix `SurrealNoteRepository.findById()` query syntax
- [x] 1.3 Fix `SurrealQuestRepository.findById()` query syntax
- [x] 1.4 Fix `SurrealContainerRepository.findById()` query syntax
- [x] 1.5 Fix `SurrealUserRepository.findById()` query syntax
- [x] 1.6 Fix `SurrealSourceDocumentRepository.findById()` query syntax
- [x] 1.7 Fix `SurrealInitiativeRepository.findById()` query syntax
- [x] 1.8 Fix `SurrealLocationRepository.findById()` query syntax
- [x] 1.9 Fix `SurrealItemRepository.findById()` query syntax
- [x] 1.10 Fix `SurrealFolderRepository.findById()` query syntax
- [x] 1.11 Fix `SurrealTagRepository.findById()` query syntax
- [x] 1.12 Fix `SurrealEpicRepository.findById()` query syntax
- [x] 1.13 Fix `SurrealRoutineRepository.findById()` query syntax
- [x] 1.14 Fix `SurrealInboxRepository.findById()` query syntax
- [x] 1.15 Fix `SurrealCheckpointRepository.findById()` query syntax
- [x] 1.16 Fix `SurrealEnergyBudgetRepository.findById()` query syntax
- [x] 1.17 Fix `SurrealNoteLinkRepository.findById()` query syntax
- [x] 1.18 Fix `SurrealItemTemplateRepository.findById()` query syntax
- [x] 1.19 Fix `SurrealItemTemplateRepository.findFieldById()` query syntax (bonus fix)

## 2. Audit and Fix Related Queries

- [x] 2.1 Audit all `UPDATE table:id WHERE ...` statements for same issue
  - Note: UPDATE statements with `table:id WHERE ...` work correctly in SurrealDB (the WHERE clause acts as a conditional filter on the specific record, not as the primary selector). No changes needed.
- [x] 2.2 Fix any similar pattern issues in update/delete queries
  - No additional fixes needed.

## 3. Verification

- [x] 3.1 Run all repository integration tests
- [x] 3.2 Run full test suite to check for regressions
  - All tests pass: `BUILD SUCCESSFUL`
