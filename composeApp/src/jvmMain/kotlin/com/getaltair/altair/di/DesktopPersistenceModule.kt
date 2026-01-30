package com.getaltair.altair.di

import com.getaltair.altair.persistence.DesktopDatabaseConfig
import com.getaltair.altair.persistence.DesktopSurrealDbClient
import com.getaltair.altair.persistence.repository.*
import com.getaltair.altair.shared.repository.*
import org.koin.dsl.module

/**
 * Koin module for desktop persistence layer.
 * Provides embedded SurrealDB client and all repository implementations.
 */
val desktopPersistenceModule = module {
    // Database configuration and client
    single { DesktopDatabaseConfig() }
    single { DesktopSurrealDbClient(get()) }

    // Repository bindings
    single<QuestRepository> { DesktopQuestRepository(get()) }
    single<InitiativeRepository> { DesktopInitiativeRepository(get()) }
    single<InboxRepository> { DesktopInboxRepository(get()) }
    single<NoteRepository> { DesktopNoteRepository(get()) }
    single<ItemRepository> { DesktopItemRepository(get()) }
    single<RoutineRepository> { DesktopRoutineRepository(get()) }
    single<EpicRepository> { DesktopEpicRepository(get()) }
    single<UserRepository> { DesktopUserRepository(get()) }
}
