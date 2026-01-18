package com.getaltair.altair.di

import com.getaltair.altair.rpc.RpcClientFactory
import com.getaltair.altair.service.auth.AuthManager
import com.getaltair.altair.service.auth.SecureTokenStorage
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import org.koin.dsl.module

/**
 * Koin module for authentication dependencies.
 *
 * Note: SecureTokenStorage implementation must be provided by platform-specific modules:
 * - Android: AndroidSecureTokenStorage
 * - iOS: IosSecureTokenStorage
 * - Desktop: DesktopSecureTokenStorage
 *
 * This module expects SecureTokenStorage to be already defined in the DI graph.
 */
val authModule =
    module {
        // Application-level coroutine scope for auth operations
        single<CoroutineScope> {
            CoroutineScope(SupervisorJob() + Dispatchers.Default)
        }

        // AuthManager singleton
        single {
            val rpcFactory = get<RpcClientFactory>()
            AuthManager(
                tokenStorage = get<SecureTokenStorage>(),
                publicAuthService = kotlinx.coroutines.runBlocking { rpcFactory.publicAuthService() },
                scope = get<CoroutineScope>(),
            )
        }
    }
