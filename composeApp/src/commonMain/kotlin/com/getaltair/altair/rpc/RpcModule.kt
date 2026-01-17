package com.getaltair.altair.rpc

import io.ktor.client.HttpClient
import org.koin.dsl.module

/**
 * Koin module for RPC client dependencies.
 *
 * Provides:
 * - ServerConfig: Connection settings for the RPC server
 * - RpcClientFactory: Factory for obtaining typed RPC service stubs
 *
 * Usage:
 * ```kotlin
 * val factory: RpcClientFactory by inject()
 * val authService = factory.authService()
 * ```
 */
val rpcModule =
    module {
        single {
            ServerConfig(
                host = "localhost",
                port = 8080,
                secure = false,
            )
        }

        single {
            RpcClientFactory(
                config = get(),
                httpClientProvider = { get<HttpClient>() },
            )
        }
    }
