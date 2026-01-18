package com.getaltair

import com.getaltair.altair.rpc.AiService
import com.getaltair.altair.rpc.AuthService
import com.getaltair.altair.rpc.PublicAuthService
import com.getaltair.altair.rpc.SyncService
import com.getaltair.rpc.AiServiceImpl
import com.getaltair.rpc.SyncServiceImpl
import io.ktor.server.application.Application
import io.ktor.server.application.install
import io.ktor.server.auth.authenticate
import io.ktor.server.routing.routing
import io.ktor.server.websocket.WebSockets
import kotlinx.rpc.krpc.ktor.server.Krpc
import kotlinx.rpc.krpc.ktor.server.rpc
import kotlinx.rpc.krpc.serialization.json.json
import org.koin.ktor.ext.inject

/**
 * Configure kotlinx-rpc services over WebSocket transport.
 *
 * Services are split into two endpoints:
 * - `/rpc/auth` - Public endpoint for login, register, refresh (no authentication required)
 * - `/rpc` - Authenticated endpoint for all other services (requires valid JWT)
 *
 * Authentication is performed at the WebSocket connection level using JWT tokens
 * passed in the Authorization header during the WebSocket handshake.
 *
 * Note: Per-request AuthContext is not currently available in kotlinx-rpc services.
 * The authenticated endpoint relies on Ktor's authenticate block to reject
 * unauthenticated connections. AuthService operations that need user context
 * will need to be enhanced when kotlinx-rpc adds better context support.
 */
fun Application.configureRpc() {
    install(WebSockets)
    install(Krpc)

    // Get services from Koin
    val publicAuthService: PublicAuthService by inject()
    val authService: AuthService by inject()

    routing {
        // Public endpoint - no authentication required
        // Used for login, register, and token refresh
        rpc("/rpc/auth") {
            rpcConfig {
                serialization {
                    json()
                }
            }

            registerService<PublicAuthService> { publicAuthService }
        }

        // Authenticated endpoint - requires valid JWT
        // Ktor's authenticate block rejects connections without valid JWT
        authenticate("auth-jwt") {
            rpc("/rpc") {
                rpcConfig {
                    serialization {
                        json()
                    }
                }

                registerService<SyncService> { SyncServiceImpl() }
                registerService<AiService> { AiServiceImpl() }
                registerService<AuthService> { authService }
            }
        }
    }
}
