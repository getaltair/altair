package com.getaltair.altair.rpc

import com.getaltair.altair.dto.sync.ChangeSet
import com.getaltair.altair.dto.sync.EntityChange
import com.getaltair.altair.dto.sync.SyncResponse
import kotlinx.coroutines.flow.Flow
import kotlinx.rpc.annotations.Rpc

/**
 * RPC service for synchronizing data between clients and server.
 *
 * Provides pull/push sync operations and optional real-time streaming
 * for clients that support persistent connections.
 */
@Rpc
interface SyncService {
    /**
     * Pull changes from the server since the given version.
     *
     * @param since Server version to fetch changes from (0 for initial sync)
     * @param entityTypes Set of entity types to sync (empty for all types)
     * @return SyncResponse containing server changes and current version
     */
    suspend fun pull(
        since: Long,
        entityTypes: Set<String>,
    ): SyncResponse

    /**
     * Push local changes to the server.
     *
     * @param changes ChangeSet containing local modifications
     * @return PushResult indicating success/failure and any conflicts
     */
    suspend fun push(changes: ChangeSet): PushResult

    /**
     * Stream real-time changes from the server.
     *
     * This is an optional streaming endpoint for clients that support
     * persistent WebSocket connections. Mobile clients may prefer
     * polling via [pull] instead.
     *
     * @param entityTypes Set of entity types to watch (empty for all types)
     * @return Flow of entity changes as they occur on the server
     */
    fun streamChanges(entityTypes: Set<String>): Flow<EntityChange>
}

/**
 * Result of a push operation.
 */
@kotlinx.serialization.Serializable
data class PushResult(
    val success: Boolean,
    val serverVersion: Long,
    val acknowledged: List<String>,
    val conflicts: List<String> = emptyList(),
    val errorMessage: String? = null,
)
