package com.getaltair.altair.di

import com.getaltair.altair.service.auth.DesktopSecureTokenStorage
import com.getaltair.altair.service.auth.NativeCredentialStoreFactory
import com.getaltair.altair.service.auth.NativeSecureTokenStorage
import com.getaltair.altair.service.auth.SecureTokenStorage
import org.koin.dsl.module
import org.slf4j.LoggerFactory

private val logger = LoggerFactory.getLogger("DesktopAuthModule")

/**
 * Desktop-specific Koin module for authentication dependencies.
 *
 * Provides SecureTokenStorage using native credential stores when available:
 * - macOS: Keychain Services
 * - Windows: Credential Manager
 * - Linux: Secret Service (libsecret / GNOME Keyring / KDE Wallet)
 *
 * Falls back to DesktopSecureTokenStorage (AES-GCM encrypted Java Preferences)
 * when native stores are unavailable.
 */
val desktopAuthModule =
    module {
        single<SecureTokenStorage> {
            val nativeProvider = NativeCredentialStoreFactory.create()
            if (nativeProvider != null) {
                logger.info("Using native credential store: ${nativeProvider.name}")
                NativeSecureTokenStorage(nativeProvider)
            } else {
                logger.info("Native credential store unavailable, using encrypted preferences fallback")
                DesktopSecureTokenStorage()
            }
        }
    }
