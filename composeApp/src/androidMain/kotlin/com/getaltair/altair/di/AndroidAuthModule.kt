package com.getaltair.altair.di

import com.getaltair.altair.service.auth.AndroidSecureTokenStorage
import com.getaltair.altair.service.auth.SecureTokenStorage
import org.koin.android.ext.koin.androidContext
import org.koin.dsl.module

/**
 * Android-specific Koin module for authentication dependencies.
 *
 * Provides AndroidSecureTokenStorage which uses EncryptedSharedPreferences
 * backed by Android Keystore for secure token storage.
 */
val androidAuthModule =
    module {
        single<SecureTokenStorage> { AndroidSecureTokenStorage(androidContext()) }
    }
