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
 *
 * **Requires authentication.** All operations are scoped to the authenticated user.
 *
 * ## Error Handling
 *
 * RPC services use exception-based error handling at the transport layer.
 * Callers should wrap RPC calls with Arrow's `Either.catch {}` to convert
 * exceptions to typed errors at the repository layer.
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
 *
 * This sealed class ensures that success and failure states have appropriate fields:
 * - Success cannot have conflicts or error messages
 * - Failure must have an error message and may have conflicts
 */
@kotlinx.serialization.Serializable
sealed class PushResult {
    /** Server version after the push operation */
    abstract val serverVersion: Long

    /** Entity IDs that were successfully acknowledged */
    abstract val acknowledged: List<String>

    /** Whether the push operation succeeded */
    val success: Boolean get() = this is Success

    /**
     * Successful push result with all changes acknowledged.
     *
     * @property serverVersion Server version after applying changes
     * @property acknowledged Entity IDs that were successfully applied
     */
    @kotlinx.serialization.Serializable
    @kotlinx.serialization.SerialName("success")
    data class Success(
        override val serverVersion: Long,
        override val acknowledged: List<String>,
    ) : PushResult()

    /**
     * Failed push result with conflict information.
     *
     * @property serverVersion Server version at time of failure
     * @property acknowledged Entity IDs that were applied before failure
     * @property conflicts Entity IDs that had version conflicts
     * @property errorMessage Description of what went wrong
     */
    @kotlinx.serialization.Serializable
    @kotlinx.serialization.SerialName("failure")
    data class Failure(
        override val serverVersion: Long,
        override val acknowledged: List<String>,
        val conflicts: List<String>,
        val errorMessage: String,
    ) : PushResult()
}
