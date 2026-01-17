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
 *
 * @property host Hostname or IP address of the server (must not be blank)
 * @property port Port number (must be in range 1-65535)
 * @property secure Whether to use secure WebSocket (WSS) connection
 */
data class ServerConfig(
    val host: String = "localhost",
    val port: Int = 8080,
    val secure: Boolean = false,
) {
    init {
        require(host.isNotBlank()) { "Host must not be blank" }
        require(port in 1..MAX_PORT) { "Port must be in range 1-$MAX_PORT" }
    }

    private companion object {
        const val MAX_PORT = 65_535
    }
}

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
