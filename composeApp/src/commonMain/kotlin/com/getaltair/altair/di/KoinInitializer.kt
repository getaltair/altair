package com.getaltair.altair.di

import org.koin.core.context.startKoin
import org.koin.dsl.KoinAppDeclaration
import org.koin.mp.KoinPlatform

/**
 * Initializes Koin dependency injection.
 * Call this once at application startup before any DI usage.
 *
 * This function is safe to call multiple times - subsequent calls are ignored
 * if Koin is already initialized.
 *
 * @param config Optional platform-specific configuration (e.g., androidContext)
 * @return true if Koin was initialized, false if it was already running
 */
fun initKoin(config: KoinAppDeclaration? = null): Boolean {
    // Check if Koin is already started to avoid KoinApplicationAlreadyStartedException
    if (KoinPlatform.getKoinOrNull() != null) {
        return false
    }

    startKoin {
        config?.invoke(this)
        modules(appModule)
    }
    return true
}
