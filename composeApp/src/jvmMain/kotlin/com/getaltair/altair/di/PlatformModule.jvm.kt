package com.getaltair.altair.di

import com.getaltair.altair.api.TokenProvider
import com.getaltair.altair.auth.DesktopTokenProvider
import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * Desktop/JVM-specific Koin module.
 * Provides platform-specific implementations.
 */
actual val platformModule: Module = module {
    includes(desktopPersistenceModule)

    // TokenProvider using file-based storage in config directory
    single<TokenProvider> {
        DesktopTokenProvider(
            serverUrl = System.getenv("ALTAIR_SERVER_URL") ?: "http://localhost:8080"
        )
    }
}
