# Tasks: Implement Hybrid Database Layer

## 1. Dependencies and Build Configuration

- [x] 1.1 Add surrealdb-java version to `libs.versions.toml`
- [x] 1.2 Add SQLDelight plugin version to `libs.versions.toml`
- [x] 1.3 Add SQLDelight runtime dependencies to `libs.versions.toml`
- [x] 1.4 Configure surrealdb-java dependency in `server/build.gradle.kts`
- [x] 1.5 Configure SQLDelight plugin in `shared/build.gradle.kts`
- [x] 1.6 Configure SQLDelight Android driver in `shared/build.gradle.kts`
- [x] 1.7 Configure SQLDelight iOS (native) driver in `shared/build.gradle.kts`
- [x] 1.8 Configure surrealdb-java dependency in `composeApp/build.gradle.kts` for jvmMain

## 2. Server Database Layer

- [x] 2.1 Create `DatabaseConfig` data class for connection settings
- [x] 2.2 Create `SurrealDbClient` wrapper with connection pooling
- [x] 2.3 Create `DatabaseModule` Koin module for DI
- [x] 2.4 Implement schema migration runner for SurrealDB
- [x] 2.5 Create initial schema migration (v1) with all tables
- [x] 2.6 Implement `SurrealInitiativeRepository`
- [x] 2.7 Implement `SurrealInboxRepository`
- [x] 2.8 Implement `SurrealRoutineRepository`
- [x] 2.9 Implement `SurrealQuestRepository`
- [x] 2.10 Implement `SurrealEpicRepository`
- [x] 2.11 Implement `SurrealCheckpointRepository`
- [x] 2.12 Implement `SurrealEnergyBudgetRepository`
- [x] 2.13 Implement `SurrealNoteRepository`
- [x] 2.14 Implement `SurrealNoteLinkRepository`
- [x] 2.15 Implement `SurrealFolderRepository`
- [x] 2.16 Implement `SurrealTagRepository`
- [x] 2.17 Implement `SurrealAttachmentRepository`
- [x] 2.18 Implement `SurrealSourceDocumentRepository`
- [x] 2.19 Implement `SurrealItemRepository`
- [x] 2.20 Implement `SurrealLocationRepository`
- [x] 2.21 Implement `SurrealContainerRepository`
- [x] 2.22 Implement `SurrealItemTemplateRepository`
- [x] 2.23 Implement `SurrealUserRepository`
- [x] 2.24 Wire database initialization into Ktor Application startup

## 3. Mobile Database Layer (SQLDelight)

- [x] 3.1 Configure SQLDelight database name and package
- [x] 3.2 Create `User.sq` schema file
- [x] 3.3 Create `Initiative.sq` schema file
- [x] 3.4 Create `InboxItem.sq` schema file
- [x] 3.5 Create `Routine.sq` schema file
- [x] 3.6 Create `Epic.sq` schema file
- [x] 3.7 Create `Quest.sq` schema file
- [x] 3.8 Create `Checkpoint.sq` schema file
- [x] 3.9 Create `EnergyBudget.sq` schema file
- [x] 3.10 Create `Folder.sq` schema file
- [x] 3.11 Create `Note.sq` schema file
- [x] 3.12 Create `NoteLink.sq` schema file
- [x] 3.13 Create `Tag.sq` schema file
- [x] 3.14 Create `Attachment.sq` schema file
- [x] 3.15 Create `SourceDocument.sq` schema file
- [x] 3.16 Create `SourceAnnotation.sq` schema file
- [x] 3.17 Create `Location.sq` schema file
- [x] 3.18 Create `Container.sq` schema file
- [x] 3.19 Create `Item.sq` schema file
- [x] 3.20 Create `ItemTemplate.sq` schema file
- [x] 3.21 Create Android driver factory in `androidMain`
- [x] 3.22 Create iOS driver factory in `iosMain`
- [x] 3.23 Create `SqlDelightDatabaseModule` Koin module

## 4. Desktop Database Layer (SurrealDB Embedded)

- [x] 4.1 Create `DesktopDatabaseConfig` for embedded SurrealDB settings
- [x] 4.2 Create `EmbeddedSurrealClient` for in-process database
- [x] 4.3 Configure SurrealKV storage engine settings
- [x] 4.4 Create desktop-specific Koin module for database
- [x] 4.5 Reuse server repository implementations (shared code)
- [x] 4.6 Wire database initialization into Desktop Application startup

## 5. Testing

- [x] 5.1 Add test containers for SurrealDB integration tests
- [x] 5.2 Write repository integration tests for server
- [x] 5.3 Write SQLDelight query tests
- [x] 5.4 Verify builds pass on all platforms
