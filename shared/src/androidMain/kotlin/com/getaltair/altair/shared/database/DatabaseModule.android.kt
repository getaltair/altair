package com.getaltair.altair.shared.database

import android.content.Context
import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * Android-specific database module.
 *
 * Provides DatabaseDriverFactory that requires Android Context.
 */
actual fun platformDatabaseModule(): Module = module {
    single<DatabaseDriverFactory> {
        DatabaseDriverFactory(context = get<Context>())
    }
}
