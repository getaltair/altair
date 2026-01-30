package com.getaltair.altair.di

import com.getaltair.altair.api.TokenProvider
import com.getaltair.altair.auth.IosTokenProvider
import org.koin.core.module.Module
import org.koin.dsl.module
import platform.Foundation.NSProcessInfo

/**
 * iOS-specific Koin module.
 * Provides platform-specific implementations.
 */
actual val platformModule: Module = module {
    // TokenProvider using NSUserDefaults (TODO: migrate to Keychain)
    single<TokenProvider> {
        val serverUrl = NSProcessInfo.processInfo.environment["ALTAIR_SERVER_URL"] as? String
            ?: "http://localhost:8080"
        IosTokenProvider(serverUrl = serverUrl)
    }
}
