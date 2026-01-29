package com.getaltair.altair.api

import io.ktor.client.*
import io.ktor.client.plugins.*
import io.ktor.client.plugins.auth.*
import io.ktor.client.plugins.auth.providers.*
import io.ktor.client.plugins.contentnegotiation.*
import io.ktor.client.plugins.logging.*
import io.ktor.http.*
import io.ktor.serialization.kotlinx.json.*
import kotlinx.serialization.json.Json

/**
 * Provider interface for JWT tokens.
 * Platform implementations handle secure token storage (Keychain/Keystore/SecureStorage).
 */
interface TokenProvider {
    /**
     * The base URL of the Altair server.
     * Example: "http://localhost:8080"
     */
    val serverUrl: String

    /**
     * Get current stored tokens, or null if not logged in.
     */
    suspend fun getTokens(): TokenPair?

    /**
     * Refresh tokens using the stored refresh token.
     * Returns new tokens, or null if refresh failed.
     */
    suspend fun refresh(): TokenPair?

    /**
     * Store tokens after successful login.
     */
    suspend fun storeTokens(tokens: TokenPair)

    /**
     * Clear stored tokens on logout.
     */
    suspend fun clearTokens()
}

/**
 * Pair of access and refresh tokens.
 */
data class TokenPair(
    val accessToken: String,
    val refreshToken: String
)

/**
 * Creates configured HttpClient for Altair API communication.
 *
 * Features:
 * - JSON serialization via kotlinx.serialization
 * - Bearer token authentication with auto-refresh
 * - Request logging (debug builds)
 * - Default request configuration
 * - Timeout handling
 *
 * @param tokenProvider Platform-specific token storage and refresh implementation
 * @return Configured HttpClient instance
 */
fun createHttpClient(tokenProvider: TokenProvider): HttpClient {
    return HttpClient {
        // JSON serialization
        install(ContentNegotiation) {
            json(Json {
                ignoreUnknownKeys = true
                encodeDefaults = true
                isLenient = true
                prettyPrint = false
            })
        }

        // Bearer token authentication
        install(Auth) {
            bearer {
                loadTokens {
                    tokenProvider.getTokens()?.let {
                        BearerTokens(it.accessToken, it.refreshToken)
                    }
                }
                refreshTokens {
                    tokenProvider.refresh()?.let {
                        tokenProvider.storeTokens(it)
                        BearerTokens(it.accessToken, it.refreshToken)
                    }
                }
                sendWithoutRequest { request ->
                    // Don't send auth for login/register/refresh endpoints
                    val path = request.url.encodedPath
                    !path.endsWith("/login") &&
                    !path.endsWith("/register") &&
                    !path.endsWith("/refresh")
                }
            }
        }

        // Default request settings
        defaultRequest {
            url(tokenProvider.serverUrl)
            contentType(ContentType.Application.Json)
        }

        // Timeouts
        install(HttpTimeout) {
            requestTimeoutMillis = 30_000
            connectTimeoutMillis = 10_000
            socketTimeoutMillis = 30_000
        }

        // Logging (useful for debugging)
        install(Logging) {
            logger = Logger.DEFAULT
            level = LogLevel.NONE // Set to INFO or BODY for debugging
        }
    }
}
