package com.getaltair.altair.dto.sync

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement

/**
 * Request to synchronize changes between client and server.
 */
@Serializable
data class SyncRequest(
    val clientId: String,
    val lastSyncVersion: Long,
    val changes: List<EntityChange>,
)

/**
 * Response containing server changes and conflict information.
 */
@Serializable
data class SyncResponse(
    val serverVersion: Long,
    val changes: List<EntityChange>,
    val conflicts: List<ConflictInfo>,
    val acknowledged: List<String>,
)

/**
 * Represents a change to a single entity.
 */
@Serializable
data class EntityChange(
    val entityType: String,
    val entityId: String,
    val operation: ChangeOperation,
    val version: Long,
    val data: JsonElement? = null,
    val timestamp: Long,
)

/**
 * The type of change operation.
 */
@Serializable
enum class ChangeOperation {
    @SerialName("create")
    CREATE,

    @SerialName("update")
    UPDATE,

    @SerialName("delete")
    DELETE,
}

/**
 * Information about a sync conflict that needs resolution.
 */
@Serializable
data class ConflictInfo(
    val entityType: String,
    val entityId: String,
    val clientVersion: Long,
    val serverVersion: Long,
    val clientData: JsonElement,
    val serverData: JsonElement,
)

/**
 * Request to resolve a sync conflict.
 */
@Serializable
data class ConflictResolutionRequest(
    val entityType: String,
    val entityId: String,
    val resolution: ConflictResolution,
    val mergedData: JsonElement? = null,
)

/**
 * How to resolve a sync conflict.
 */
@Serializable
enum class ConflictResolution {
    @SerialName("keep_client")
    KEEP_CLIENT,

    @SerialName("keep_server")
    KEEP_SERVER,

    @SerialName("merge")
    MERGE,
}

/**
 * A batch of entity changes grouped for sync.
 */
@Serializable
data class ChangeSet(
    val changes: List<EntityChange>,
    val clientTimestamp: Long,
)
