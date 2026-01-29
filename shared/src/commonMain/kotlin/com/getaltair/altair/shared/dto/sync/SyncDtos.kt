package com.getaltair.altair.shared.dto.sync

import kotlinx.serialization.Serializable

/**
 * Request payload for pulling server-side entity changes since last sync.
 *
 * @property entityType The type of entity to sync (e.g., "Quest", "Note", "Item")
 * @property lastSyncTimestamp ISO-8601 timestamp of last successful sync (null for initial sync)
 */
@Serializable
data class SyncPullRequest(
    val entityType: String,
    val lastSyncTimestamp: String?
)

/**
 * Response payload containing entities modified on the server since last sync.
 *
 * @param T The entity type being synchronized
 * @property entities List of entities changed since lastSyncTimestamp
 * @property serverTimestamp Current server timestamp (ISO-8601) for next sync
 * @property hasMore True if more entities exist (pagination needed)
 */
@Serializable
data class SyncPullResponse<T>(
    val entities: List<T>,
    val serverTimestamp: String,
    val hasMore: Boolean
)

/**
 * Request payload for pushing client-side entity changes to server.
 *
 * @param T The entity type being synchronized
 * @property entities List of locally modified entities to push
 * @property clientTimestamp ISO-8601 timestamp of client sync request
 */
@Serializable
data class SyncPushRequest<T>(
    val entities: List<T>,
    val clientTimestamp: String
)

/**
 * Response payload indicating which pushed entities were accepted or conflicted.
 *
 * @property accepted List of entity IDs successfully accepted by server
 * @property conflicts List of conflicting entities requiring resolution
 */
@Serializable
data class SyncPushResponse(
    val accepted: List<String>,
    val conflicts: List<SyncConflict>
)

/**
 * Represents a synchronization conflict between client and server versions.
 *
 * @property entityId Unique identifier of conflicting entity
 * @property entityType Type name of conflicting entity
 * @property serverVersion Version number of server entity
 * @property clientVersion Version number of client entity
 * @property message Human-readable conflict description
 */
@Serializable
data class SyncConflict(
    val entityId: String,
    val entityType: String,
    val serverVersion: Long,
    val clientVersion: Long,
    val message: String
)

/**
 * Response payload describing current synchronization state.
 *
 * @property lastSyncTimestamp ISO-8601 timestamp of last successful sync (null if never synced)
 * @property pendingChanges Count of local changes awaiting upload
 * @property syncState Current operational sync state
 */
@Serializable
data class SyncStatusResponse(
    val lastSyncTimestamp: String?,
    val pendingChanges: Int,
    val syncState: SyncState
)

/**
 * Enumeration of possible synchronization states.
 */
@Serializable
enum class SyncState {
    /** All local and remote changes are synchronized */
    SYNCED,
    /** Local changes exist that need to be pushed */
    PENDING,
    /** Synchronization operation currently in progress */
    SYNCING,
    /** Conflicts detected requiring manual resolution */
    CONFLICT,
    /** No network connection available for sync */
    OFFLINE
}
