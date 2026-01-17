package com.getaltair.altair.db

import com.getaltair.altair.db.repository.SurrealAttachmentRepository
import com.getaltair.altair.db.repository.SurrealCheckpointRepository
import com.getaltair.altair.db.repository.SurrealContainerRepository
import com.getaltair.altair.db.repository.SurrealEnergyBudgetRepository
import com.getaltair.altair.db.repository.SurrealEpicRepository
import com.getaltair.altair.db.repository.SurrealFolderRepository
import com.getaltair.altair.db.repository.SurrealInboxRepository
import com.getaltair.altair.db.repository.SurrealInitiativeRepository
import com.getaltair.altair.db.repository.SurrealItemRepository
import com.getaltair.altair.db.repository.SurrealItemTemplateRepository
import com.getaltair.altair.db.repository.SurrealLocationRepository
import com.getaltair.altair.db.repository.SurrealNoteLinkRepository
import com.getaltair.altair.db.repository.SurrealNoteRepository
import com.getaltair.altair.db.repository.SurrealQuestRepository
import com.getaltair.altair.db.repository.SurrealRoutineRepository
import com.getaltair.altair.db.repository.SurrealSourceDocumentRepository
import com.getaltair.altair.db.repository.SurrealTagRepository
import com.getaltair.altair.db.repository.SurrealUserRepository
import com.getaltair.altair.repository.AttachmentRepository
import com.getaltair.altair.repository.CheckpointRepository
import com.getaltair.altair.repository.ContainerRepository
import com.getaltair.altair.repository.EnergyBudgetRepository
import com.getaltair.altair.repository.EpicRepository
import com.getaltair.altair.repository.FolderRepository
import com.getaltair.altair.repository.InboxRepository
import com.getaltair.altair.repository.InitiativeRepository
import com.getaltair.altair.repository.ItemRepository
import com.getaltair.altair.repository.ItemTemplateRepository
import com.getaltair.altair.repository.LocationRepository
import com.getaltair.altair.repository.NoteLinkRepository
import com.getaltair.altair.repository.NoteRepository
import com.getaltair.altair.repository.QuestRepository
import com.getaltair.altair.repository.RoutineRepository
import com.getaltair.altair.repository.SourceDocumentRepository
import com.getaltair.altair.repository.TagRepository
import com.getaltair.altair.repository.UserRepository
import com.getaltair.altair.domain.types.Ulid
import org.koin.dsl.module

/**
 * Koin module for database dependencies.
 *
 * Provides:
 * - DatabaseConfig (from environment)
 * - SurrealDbClient (singleton)
 * - MigrationRunner (singleton)
 * - All repository implementations (factory, user-scoped)
 */
val databaseModule =
    module {
        // Database configuration from environment
        single<DatabaseConfig> { DatabaseConfig.fromEnvironment() }

        // SurrealDB client (singleton)
        single { SurrealDbClient(get()) }

        // Migration runner
        single { MigrationRunner(get()) }

        // UserRepository is special - not user-scoped (admin only)
        single<UserRepository> { SurrealUserRepository(get()) }
    }

/**
 * Creates a user-scoped Koin module with repositories bound to a specific user.
 *
 * This should be called per-request to inject the authenticated user's ID
 * into all repository implementations.
 *
 * @param userId The authenticated user's ULID
 */
fun userScopedRepositoryModule(userId: Ulid) =
    module {
        factory<InitiativeRepository> { SurrealInitiativeRepository(get(), userId) }
        factory<InboxRepository> { SurrealInboxRepository(get(), userId) }
        factory<RoutineRepository> { SurrealRoutineRepository(get(), userId) }
        factory<QuestRepository> { SurrealQuestRepository(get(), userId) }
        factory<EpicRepository> { SurrealEpicRepository(get(), userId) }
        factory<CheckpointRepository> { SurrealCheckpointRepository(get(), userId) }
        factory<EnergyBudgetRepository> { SurrealEnergyBudgetRepository(get(), userId) }
        factory<NoteRepository> { SurrealNoteRepository(get(), userId) }
        factory<NoteLinkRepository> { SurrealNoteLinkRepository(get(), userId) }
        factory<FolderRepository> { SurrealFolderRepository(get(), userId) }
        factory<TagRepository> { SurrealTagRepository(get(), userId) }
        factory<AttachmentRepository> { SurrealAttachmentRepository(get(), userId) }
        factory<SourceDocumentRepository> { SurrealSourceDocumentRepository(get(), userId) }
        factory<ItemRepository> { SurrealItemRepository(get(), userId) }
        factory<LocationRepository> { SurrealLocationRepository(get(), userId) }
        factory<ContainerRepository> { SurrealContainerRepository(get(), userId) }
        factory<ItemTemplateRepository> { SurrealItemTemplateRepository(get(), userId) }
    }
