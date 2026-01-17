package com.getaltair.rpc

import com.getaltair.altair.dto.sync.ChangeSet
import com.getaltair.altair.dto.sync.EntityChange
import com.getaltair.altair.dto.sync.SyncResponse
import com.getaltair.altair.rpc.PushResult
import com.getaltair.altair.rpc.SyncService
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow

/**
 * Stub implementation of SyncService for infrastructure validation.
 *
 * Returns mock data to verify RPC transport works correctly.
 * Real implementation will integrate with repositories in Phase 5+.
 */
class SyncServiceImpl : SyncService {
    override suspend fun pull(
        since: Long,
        entityTypes: Set<String>,
    ): SyncResponse {
        // Stub: Return empty changes with incremented version
        return SyncResponse(
            serverVersion = since + 1,
            changes = emptyList(),
            conflicts = emptyList(),
            acknowledged = emptyList(),
        )
    }

    override suspend fun push(changes: ChangeSet): PushResult {
        // Stub: Acknowledge all changes
        return PushResult(
            success = true,
            serverVersion = System.currentTimeMillis(),
            acknowledged = changes.changes.map { it.entityId },
            conflicts = emptyList(),
        )
    }

    override fun streamChanges(entityTypes: Set<String>): Flow<EntityChange> =
        flow {
            // Stub: Emit nothing, just keep connection alive
            while (true) {
                delay(HEARTBEAT_INTERVAL_MS)
            }
        }

    private companion object {
        const val HEARTBEAT_INTERVAL_MS = 30_000L
    }
}
