# Tasks: Implement Hybrid Database Layer

## 1. Dependencies and Build Configuration

- [ ] 1.1 Add surrealdb-java version to `libs.versions.toml`
- [ ] 1.2 Add SQLDelight plugin version to `libs.versions.toml`
- [ ] 1.3 Add SQLDelight runtime dependencies to `libs.versions.toml`
- [ ] 1.4 Configure surrealdb-java dependency in `server/build.gradle.kts`
- [ ] 1.5 Configure SQLDelight plugin in `shared/build.gradle.kts`
- [ ] 1.6 Configure SQLDelight Android driver in `shared/build.gradle.kts`
- [ ] 1.7 Configure SQLDelight iOS (native) driver in `shared/build.gradle.kts`
- [ ] 1.8 Configure surrealdb-java dependency in `composeApp/build.gradle.kts` for jvmMain

## 2. Server Database Layer

- [ ] 2.1 Create `DatabaseConfig` data class for connection settings
- [ ] 2.2 Create `SurrealDbClient` wrapper with connection pooling
- [ ] 2.3 Create `DatabaseModule` Koin module for DI
- [ ] 2.4 Implement schema migration runner for SurrealDB
- [ ] 2.5 Create initial schema migration (v1) with all tables
- [ ] 2.6 Implement `SurrealInitiativeRepository`
- [ ] 2.7 Implement `SurrealInboxRepository`
- [ ] 2.8 Implement `SurrealRoutineRepository`
- [ ] 2.9 Implement `SurrealQuestRepository`
- [ ] 2.10 Implement `SurrealEpicRepository`
- [ ] 2.11 Implement `SurrealCheckpointRepository`
- [ ] 2.12 Implement `SurrealEnergyBudgetRepository`
- [ ] 2.13 Implement `SurrealNoteRepository`
- [ ] 2.14 Implement `SurrealNoteLinkRepository`
- [ ] 2.15 Implement `SurrealFolderRepository`
- [ ] 2.16 Implement `SurrealTagRepository`
- [ ] 2.17 Implement `SurrealAttachmentRepository`
- [ ] 2.18 Implement `SurrealSourceDocumentRepository`
- [ ] 2.19 Implement `SurrealItemRepository`
- [ ] 2.20 Implement `SurrealLocationRepository`
- [ ] 2.21 Implement `SurrealContainerRepository`
- [ ] 2.22 Implement `SurrealItemTemplateRepository`
- [ ] 2.23 Implement `SurrealUserRepository`
- [ ] 2.24 Wire database initialization into Ktor Application startup

## 3. Mobile Database Layer (SQLDelight)

- [ ] 3.1 Configure SQLDelight database name and package
- [ ] 3.2 Create `User.sq` schema file
- [ ] 3.3 Create `Initiative.sq` schema file
- [ ] 3.4 Create `InboxItem.sq` schema file
- [ ] 3.5 Create `Routine.sq` schema file
- [ ] 3.6 Create `Epic.sq` schema file
- [ ] 3.7 Create `Quest.sq` schema file
- [ ] 3.8 Create `Checkpoint.sq` schema file
- [ ] 3.9 Create `EnergyBudget.sq` schema file
- [ ] 3.10 Create `Folder.sq` schema file
- [ ] 3.11 Create `Note.sq` schema file
- [ ] 3.12 Create `NoteLink.sq` schema file
- [ ] 3.13 Create `Tag.sq` schema file
- [ ] 3.14 Create `Attachment.sq` schema file
- [ ] 3.15 Create `SourceDocument.sq` schema file
- [ ] 3.16 Create `SourceAnnotation.sq` schema file
- [ ] 3.17 Create `Location.sq` schema file
- [ ] 3.18 Create `Container.sq` schema file
- [ ] 3.19 Create `Item.sq` schema file
- [ ] 3.20 Create `ItemTemplate.sq` schema file
- [ ] 3.21 Create Android driver factory in `androidMain`
- [ ] 3.22 Create iOS driver factory in `iosMain`
- [ ] 3.23 Create `SqlDelightDatabaseModule` Koin module

## 4. Desktop Database Layer (SurrealDB Embedded)

- [ ] 4.1 Create `DesktopDatabaseConfig` for embedded SurrealDB settings
- [ ] 4.2 Create `EmbeddedSurrealClient` for in-process database
- [ ] 4.3 Configure SurrealKV storage engine settings
- [ ] 4.4 Create desktop-specific Koin module for database
- [ ] 4.5 Reuse server repository implementations (shared code)
- [ ] 4.6 Wire database initialization into Desktop Application startup

## 5. Testing

- [ ] 5.1 Add test containers for SurrealDB integration tests
- [ ] 5.2 Write repository integration tests for server
- [ ] 5.3 Write SQLDelight query tests
- [ ] 5.4 Verify builds pass on all platforms
