package com.getaltair.altair.di

import com.getaltair.altair.service.auth.DesktopSecureTokenStorage
import com.getaltair.altair.service.auth.SecureTokenStorage
import org.koin.dsl.module

/**
 * Desktop-specific Koin module for authentication dependencies.
 *
 * Provides DesktopSecureTokenStorage which uses AES-GCM encryption
 * with PBKDF2 key derivation for secure token storage.
 */
val desktopAuthModule =
    module {
        single<SecureTokenStorage> { DesktopSecureTokenStorage() }
    }
