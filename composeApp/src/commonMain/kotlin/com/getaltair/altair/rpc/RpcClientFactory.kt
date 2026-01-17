package com.getaltair.altair.rpc

import io.ktor.client.HttpClient
import io.ktor.client.plugins.websocket.WebSockets
import kotlinx.rpc.krpc.ktor.client.installKrpc
import kotlinx.rpc.krpc.ktor.client.rpc
import kotlinx.rpc.krpc.ktor.client.rpcConfig
import kotlinx.rpc.krpc.serialization.json.json
import kotlinx.rpc.withService

/**
 * Configuration for connecting to the Altair RPC server.
 */
data class ServerConfig(
    val host: String = "localhost",
    val port: Int = 8080,
    val secure: Boolean = false,
)

/**
 * Factory for creating RPC service clients.
 *
 * Creates a single HttpClient with WebSocket support and RPC configuration,
 * then provides typed service stubs for each RPC service.
 */
class RpcClientFactory(
    private val config: ServerConfig,
    private val httpClientProvider: () -> HttpClient,
) {
    private val httpClient: HttpClient by lazy {
        httpClientProvider().config {
            install(WebSockets)
            installKrpc()
        }
    }

    private val rpcClient by lazy {
        httpClient.rpc {
            url {
                host = config.host
                port = config.port
                protocol =
                    if (config.secure) {
                        io.ktor.http.URLProtocol.WSS
                    } else {
                        io.ktor.http.URLProtocol.WS
                    }
                pathSegments = listOf("rpc")
            }

            rpcConfig {
                serialization {
                    json()
                }
            }
        }
    }

    /**
     * Get the SyncService client stub.
     */
    suspend fun syncService(): SyncService = rpcClient.withService<SyncService>()

    /**
     * Get the AuthService client stub.
     */
    suspend fun authService(): AuthService = rpcClient.withService<AuthService>()

    /**
     * Get the AiService client stub.
     */
    suspend fun aiService(): AiService = rpcClient.withService<AiService>()

    /**
     * Close the underlying HTTP client and RPC connection.
     */
    fun close() {
        httpClient.close()
    }
}
