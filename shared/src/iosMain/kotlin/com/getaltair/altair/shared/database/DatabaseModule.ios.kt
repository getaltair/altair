package com.getaltair.altair.shared.database

import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * iOS-specific database module.
 *
 * Provides DatabaseDriverFactory for iOS.
 */
actual fun platformDatabaseModule(): Module = module {
    single<DatabaseDriverFactory> {
        DatabaseDriverFactory()
    }
}
