package com.getaltair.server.routes

import com.getaltair.altair.shared.dto.auth.ErrorResponse
import com.getaltair.altair.shared.dto.sync.*
import io.ktor.http.*
import io.ktor.server.application.*
import io.ktor.server.auth.*
import io.ktor.server.request.*
import io.ktor.server.response.*
import io.ktor.server.routing.*

/**
 * Registers sync-related routes under /api/sync.
 * All routes require JWT authentication.
 *
 * Endpoints:
 * - POST /pull - Pull changes since timestamp
 * - POST /push - Push local changes
 * - GET /status - Get sync status
 *
 * Note: Actual sync implementation is deferred to Phase 12.
 * These routes currently return NotImplemented responses.
 */
fun Route.syncRoutes() {
    route("/api/sync") {
        authenticate("jwt") {
            /**
             * POST /api/sync/pull
             * Pull changes from server since given timestamp.
             */
            post("/pull") {
                val userId = call.userId
                val request = call.receive<SyncPullRequest>()
                // TODO: Phase 12 will implement actual sync
                call.respond(HttpStatusCode.NotImplemented, ErrorResponse(
                    code = "SYNC_NOT_IMPLEMENTED",
                    message = "Sync will be implemented in Phase 12"
                ))
            }

            /**
             * POST /api/sync/push
             * Push local changes to server.
             */
            post("/push") {
                val userId = call.userId
                val request = call.receive<SyncPushRequest<Any>>()
                call.respond(HttpStatusCode.NotImplemented, ErrorResponse(
                    code = "SYNC_NOT_IMPLEMENTED",
                    message = "Sync will be implemented in Phase 12"
                ))
            }

            /**
             * GET /api/sync/status
             * Get current sync status for the user.
             */
            get("/status") {
                val userId = call.userId
                call.respond(SyncStatusResponse(
                    lastSyncTimestamp = null,
                    pendingChanges = 0,
                    syncState = SyncState.SYNCED
                ))
            }
        }
    }
}
