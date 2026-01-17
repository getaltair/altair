package com.getaltair.altair.db

import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * iOS-specific Koin module for SQLDelight database dependencies.
 *
 * Provides the Native SQLite driver and database instance.
 */
actual fun sqlDelightDatabaseModule(): Module =
    module {
        single<AltairDatabase> {
            createDatabase()
        }
    }
