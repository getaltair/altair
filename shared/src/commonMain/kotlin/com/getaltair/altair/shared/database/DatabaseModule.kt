package com.getaltair.altair.shared.database

import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * Koin module for database dependencies.
 *
 * Provides:
 * - DatabaseDriverFactory (platform-specific, must be provided by platform modules)
 * - AltairDatabase (singleton)
 *
 * Platform modules must provide DatabaseDriverFactory implementation.
 */
val databaseModule = module {
    // DatabaseDriverFactory is provided by platform-specific modules
    // (androidDatabaseModule or iosDatabaseModule)

    // Provide AltairDatabase as singleton
    single<AltairDatabase> {
        createDatabase(get())
    }
}

/**
 * Platform-specific database modules must implement this function
 * to provide their DatabaseDriverFactory implementation.
 */
expect fun platformDatabaseModule(): Module
