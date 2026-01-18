# Tasks: Fix SurrealDB Repository findById Queries

## 1. Fix Repository Implementations

- [ ] 1.1 Fix `SurrealAttachmentRepository.findById()` query syntax
- [ ] 1.2 Fix `SurrealNoteRepository.findById()` query syntax
- [ ] 1.3 Fix `SurrealQuestRepository.findById()` query syntax
- [ ] 1.4 Fix `SurrealContainerRepository.findById()` query syntax
- [ ] 1.5 Fix `SurrealUserRepository.findById()` query syntax
- [ ] 1.6 Fix `SurrealSourceDocumentRepository.findById()` query syntax
- [ ] 1.7 Fix `SurrealInitiativeRepository.findById()` query syntax
- [ ] 1.8 Fix `SurrealLocationRepository.findById()` query syntax
- [ ] 1.9 Fix `SurrealItemRepository.findById()` query syntax
- [ ] 1.10 Fix `SurrealFolderRepository.findById()` query syntax
- [ ] 1.11 Fix `SurrealTagRepository.findById()` query syntax
- [ ] 1.12 Fix `SurrealEpicRepository.findById()` query syntax
- [ ] 1.13 Fix `SurrealRoutineRepository.findById()` query syntax
- [ ] 1.14 Fix `SurrealInboxRepository.findById()` query syntax
- [ ] 1.15 Fix `SurrealCheckpointRepository.findById()` query syntax
- [ ] 1.16 Fix `SurrealEnergyBudgetRepository.findById()` query syntax
- [ ] 1.17 Fix `SurrealNoteLinkRepository.findById()` query syntax
- [ ] 1.18 Fix `SurrealItemTemplateRepository.findById()` query syntax

## 2. Audit and Fix Related Queries

- [ ] 2.1 Audit all `UPDATE table:id WHERE ...` statements for same issue
- [ ] 2.2 Fix any similar pattern issues in update/delete queries

## 3. Verification

- [ ] 3.1 Run all repository integration tests
- [ ] 3.2 Verify all 59 tests pass
- [ ] 3.3 Run full test suite to check for regressions
