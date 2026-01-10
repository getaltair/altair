package com.getaltair.altair.di

import org.koin.core.module.dsl.singleOf
import org.koin.dsl.module

/**
 * Koin module for shared business logic and data layer dependencies.
 *
 * This module provides:
 * - Repository implementations
 * - Use cases
 * - Domain services
 */
val sharedModule = module {
    // Repositories will be added here as they are implemented
    // Example pattern:
    // singleOf(::QuestRepositoryImpl) bind QuestRepository::class
    // singleOf(::NoteRepositoryImpl) bind NoteRepository::class
    // singleOf(::ItemRepositoryImpl) bind ItemRepository::class
}

/**
 * Data layer module for database-related dependencies.
 */
val dataModule = module {
    // Database drivers and connections
    // Example pattern:
    // single { DatabaseProvider.create(get()) }
}

/**
 * Returns all shared Koin modules for use in platform-specific initialization.
 */
fun sharedKoinModules() = listOf(sharedModule, dataModule)
