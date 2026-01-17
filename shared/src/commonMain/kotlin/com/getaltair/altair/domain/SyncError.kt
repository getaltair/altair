package com.getaltair.altair.domain

import com.getaltair.altair.domain.types.Ulid
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Errors related to client-server synchronization.
 *
 * These errors extend [DomainError] to enable exhaustive when-matching
 * for sync-specific error handling while maintaining compatibility
 * with generic error handlers.
 */
@Serializable
sealed interface SyncError : DomainError {
    /**
     * A conflict was detected between client and server versions of an entity.
     *
     * The client must resolve the conflict before the sync can complete.
     *
     * @property entityType The type of entity that has a conflict (e.g., "Quest", "Note")
     * @property entityId The ULID of the conflicting entity
     * @property clientVersion The version number on the client
     * @property serverVersion The version number on the server
     */
    @Serializable
    @SerialName("sync_conflict_detected")
    data class ConflictDetected(
        val entityType: String,
        val entityId: Ulid,
        val clientVersion: Long,
        val serverVersion: Long,
    ) : SyncError {
        override fun toUserMessage(): String =
            "A sync conflict was detected. Your changes and the server's changes need to be merged."
    }

    /**
     * The client's sync version is incompatible with the server.
     *
     * This typically means the client needs to perform a full re-sync.
     *
     * @property clientVersion The version the client is at
     * @property serverMinVersion The minimum version the server supports
     */
    @Serializable
    @SerialName("sync_version_mismatch")
    data class VersionMismatch(
        val clientVersion: Long,
        val serverMinVersion: Long,
    ) : SyncError {
        override fun toUserMessage(): String =
            "Your data is out of date. A full sync is required."
    }

    /**
     * The server could not be reached for synchronization.
     *
     * @property reason A technical description of why the server is unreachable
     */
    @Serializable
    @SerialName("sync_server_unreachable")
    data class ServerUnreachable(val reason: String) : SyncError {
        override fun toUserMessage(): String =
            "Unable to connect to the server. Please check your connection and try again."
    }

    /**
     * The change set sent by the client is invalid or corrupted.
     *
     * @property reason A description of what made the change set invalid
     */
    @Serializable
    @SerialName("sync_invalid_change_set")
    data class InvalidChangeSet(val reason: String) : SyncError {
        override fun toUserMessage(): String =
            "The sync data appears to be corrupted. Please try again."
    }

    /**
     * The sync operation timed out before completing.
     *
     * @property elapsedMs How long the sync ran before timing out
     */
    @Serializable
    @SerialName("sync_timeout")
    data class Timeout(val elapsedMs: Long) : SyncError {
        override fun toUserMessage(): String =
            "The sync took too long and was cancelled. Please try again with a better connection."
    }
}
