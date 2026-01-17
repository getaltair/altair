package com.getaltair.rpc

import com.getaltair.altair.dto.sync.ChangeSet
import com.getaltair.altair.dto.sync.EntityChange
import com.getaltair.altair.dto.sync.SyncResponse
import com.getaltair.altair.rpc.PushResult
import com.getaltair.altair.rpc.SyncService
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import org.slf4j.LoggerFactory

/**
 * Stub implementation of SyncService for infrastructure validation.
 *
 * Returns mock data to verify RPC transport works correctly.
 * Real implementation will integrate with repositories in Phase 5+.
 */
class SyncServiceImpl : SyncService {
    private val logger = LoggerFactory.getLogger(SyncServiceImpl::class.java)

    override suspend fun pull(
        since: Long,
        entityTypes: Set<String>,
    ): SyncResponse {
        logger.debug("STUB: SyncService.pull() since={}, entityTypes={}", since, entityTypes)
        return SyncResponse(
            serverVersion = since + 1,
            changes = emptyList(),
            conflicts = emptyList(),
            acknowledged = emptyList(),
        )
    }

    override suspend fun push(changes: ChangeSet): PushResult {
        logger.debug("STUB: SyncService.push() acknowledging {} changes", changes.changes.size)
        return PushResult.Success(
            serverVersion = System.currentTimeMillis(),
            acknowledged = changes.changes.map { it.entityId },
        )
    }

    override fun streamChanges(entityTypes: Set<String>): Flow<EntityChange> =
        flow {
            logger.info("STUB: SyncService.streamChanges() starting for entityTypes={}", entityTypes)
            try {
                // Stub: Maintains WebSocket connection without emitting changes.
                // The infinite delay loop keeps the Flow active for testing client connections.
                // Real implementation will subscribe to server-side change events.
                while (true) {
                    delay(HEARTBEAT_INTERVAL_MS)
                    logger.trace("STUB: SyncService.streamChanges() heartbeat")
                }
            } catch (e: CancellationException) {
                logger.info("STUB: SyncService.streamChanges() cancelled")
                throw e
            }
        }

    private companion object {
        const val HEARTBEAT_INTERVAL_MS = 30_000L
    }
}
