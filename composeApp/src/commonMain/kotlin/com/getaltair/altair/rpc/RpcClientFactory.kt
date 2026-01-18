package com.getaltair.altair.rpc

import io.ktor.client.HttpClient
import io.ktor.client.plugins.websocket.WebSockets
import io.ktor.client.request.header
import io.ktor.http.HttpHeaders
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
 * Provides access tokens for authenticated RPC requests.
 */
fun interface TokenProvider {
    /**
     * Returns the current valid access token, or null if not authenticated.
     * Implementations should handle token refresh as needed.
     */
    suspend fun getAccessToken(): String?
}

/**
 * Factory for creating RPC service clients.
 *
 * Creates HttpClients with WebSocket support and RPC configuration,
 * then provides typed service stubs for each RPC service.
 *
 * The server exposes two RPC endpoints:
 * - `/rpc/auth` - Public (unauthenticated) operations like login, register, refresh
 * - `/rpc` - Authenticated operations requiring a valid JWT token
 *
 * @param config Server connection configuration
 * @param httpClientProvider Provider for the base HttpClient
 * @param tokenProvider Provider for access tokens (used for authenticated endpoints)
 */
class RpcClientFactory(
    private val config: ServerConfig,
    private val httpClientProvider: () -> HttpClient,
    private val tokenProvider: TokenProvider? = null,
) {
    private val httpClient: HttpClient by lazy {
        httpClientProvider().config {
            install(WebSockets)
            installKrpc()
        }
    }

    /**
     * RPC client for public (unauthenticated) endpoints at /rpc/auth.
     */
    private val publicRpcClient by lazy {
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
                pathSegments = listOf("rpc", "auth")
            }

            rpcConfig {
                serialization {
                    json()
                }
            }
        }
    }

    /**
     * Creates an authenticated RPC client with the current access token.
     * A new client is created each time to ensure fresh token.
     */
    private suspend fun createAuthenticatedRpcClient(): kotlinx.rpc.RpcClient {
        // Get token before entering the builder (suspend call)
        val accessToken = tokenProvider?.getAccessToken()

        return httpClient.rpc {
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

            // Add Authorization header if token is available
            accessToken?.let { token ->
                header(HttpHeaders.Authorization, "Bearer $token")
            }

            rpcConfig {
                serialization {
                    json()
                }
            }
        }
    }

    /**
     * Get the PublicAuthService client stub for login, register, and token refresh.
     * This endpoint does not require authentication.
     */
    suspend fun publicAuthService(): PublicAuthService = publicRpcClient.withService<PublicAuthService>()

    /**
     * Get the SyncService client stub.
     * Requires authentication - includes Authorization header with current token.
     */
    suspend fun syncService(): SyncService = createAuthenticatedRpcClient().withService<SyncService>()

    /**
     * Get the AuthService client stub for authenticated operations.
     * Requires authentication - includes Authorization header with current token.
     */
    suspend fun authService(): AuthService = createAuthenticatedRpcClient().withService<AuthService>()

    /**
     * Get the AiService client stub.
     * Requires authentication - includes Authorization header with current token.
     */
    suspend fun aiService(): AiService = createAuthenticatedRpcClient().withService<AiService>()

    /**
     * Close the underlying HTTP client and RPC connection.
     */
    fun close() {
        httpClient.close()
    }
}
