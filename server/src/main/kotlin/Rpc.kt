package com.getaltair

import com.getaltair.altair.rpc.AiService
import com.getaltair.altair.rpc.AuthService
import com.getaltair.altair.rpc.SyncService
import com.getaltair.rpc.AiServiceImpl
import com.getaltair.rpc.AuthServiceImpl
import com.getaltair.rpc.SyncServiceImpl
import io.ktor.server.application.Application
import io.ktor.server.application.install
import io.ktor.server.routing.routing
import io.ktor.server.websocket.WebSockets
import kotlinx.rpc.krpc.ktor.server.Krpc
import kotlinx.rpc.krpc.ktor.server.rpc
import kotlinx.rpc.krpc.serialization.json.json

/**
 * Configure kotlinx-rpc services over WebSocket transport.
 *
 * Registers all RPC services at the /rpc endpoint using kRPC protocol.
 */
fun Application.configureRpc() {
    install(WebSockets)
    install(Krpc)

    routing {
        rpc("/rpc") {
            rpcConfig {
                serialization {
                    json()
                }
            }

            registerService<SyncService> { SyncServiceImpl() }
            registerService<AuthService> { AuthServiceImpl() }
            registerService<AiService> { AiServiceImpl() }
        }
    }
}
