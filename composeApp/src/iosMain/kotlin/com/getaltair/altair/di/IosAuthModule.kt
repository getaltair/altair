package com.getaltair.altair.di

import com.getaltair.altair.service.auth.IosSecureTokenStorage
import com.getaltair.altair.service.auth.SecureTokenStorage
import org.koin.dsl.module

/**
 * iOS-specific Koin module for authentication dependencies.
 *
 * Provides IosSecureTokenStorage which uses Keychain Services
 * for secure token storage with device-only protection.
 */
val iosAuthModule =
    module {
        single<SecureTokenStorage> { IosSecureTokenStorage() }
    }
