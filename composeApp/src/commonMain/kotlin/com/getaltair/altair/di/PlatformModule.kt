package com.getaltair.altair.di

import org.koin.core.module.Module

/**
 * Platform-specific Koin module declaration.
 * Each platform provides its own implementation with platform-specific dependencies.
 *
 * Currently provides:
 * - TokenProvider: Platform-specific secure token storage
 *
 * Implementations:
 * - Android: EncryptedSharedPreferences with Keystore
 * - iOS: NSUserDefaults (TODO: Keychain)
 * - Desktop: File-based storage in config directory
 */
expect val platformModule: Module
