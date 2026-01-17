package com.getaltair.altair.db

import org.koin.android.ext.koin.androidContext
import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * Android-specific Koin module for SQLDelight database dependencies.
 *
 * Provides the Android SQLite driver and database instance.
 */
actual fun sqlDelightDatabaseModule(): Module =
    module {
        single<AltairDatabase> {
            createDatabase(androidContext())
        }
    }
