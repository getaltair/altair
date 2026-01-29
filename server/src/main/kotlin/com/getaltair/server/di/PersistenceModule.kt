package com.getaltair.server.di

import com.getaltair.server.persistence.DatabaseConfig
import com.getaltair.server.persistence.SurrealDbClient
import com.getaltair.server.persistence.SurrealDbClientImpl
import com.getaltair.server.persistence.repository.*
import com.getaltair.altair.shared.repository.*
import org.koin.dsl.module

/**
 * Koin module for persistence layer dependencies.
 *
 * Registers the database client and all repository implementations,
 * binding them to their shared interface types for proper abstraction.
 */
val persistenceModule = module {
    // Configuration
    single { DatabaseConfig() }

    // Database client
    single<SurrealDbClient> { SurrealDbClientImpl(get()) }

    // Repositories (bound to shared interfaces)
    single<InitiativeRepository> { SurrealInitiativeRepository(get(), get()) }
    single<InboxRepository> { SurrealInboxRepository(get(), get()) }
    single<RoutineRepository> { SurrealRoutineRepository(get(), get()) }
    single<QuestRepository> { SurrealQuestRepository(get(), get()) }
    single<EpicRepository> { SurrealEpicRepository(get(), get()) }
    single<NoteRepository> { SurrealNoteRepository(get(), get()) }
    single<ItemRepository> { SurrealItemRepository(get(), get()) }
}
