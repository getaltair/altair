package com.getaltair.altair.db

import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * JVM-specific Koin module for SQLDelight database dependencies.
 *
 * Provides the JDBC SQLite driver and database instance.
 * Note: JVM clients (desktop) will typically use SurrealDB instead of SQLite.
 * This module is primarily for testing purposes.
 */
actual fun sqlDelightDatabaseModule(): Module =
    module {
        single<AltairDatabase> {
            createDatabase()
        }
    }
