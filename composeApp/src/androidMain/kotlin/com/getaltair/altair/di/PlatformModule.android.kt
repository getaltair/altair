package com.getaltair.altair.di

import com.getaltair.altair.api.TokenProvider
import com.getaltair.altair.auth.AndroidTokenProvider
import org.koin.android.ext.koin.androidContext
import org.koin.core.module.Module
import org.koin.dsl.module

/**
 * Android-specific Koin module.
 * Provides platform-specific implementations.
 */
actual val platformModule: Module = module {
    // TokenProvider using EncryptedSharedPreferences backed by Android Keystore
    single<TokenProvider> {
        AndroidTokenProvider(
            context = androidContext(),
            serverUrl = System.getenv("ALTAIR_SERVER_URL") ?: "http://10.0.2.2:8080"
        )
    }
}
