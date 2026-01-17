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
     * @property clientVersion The version number on the client (must be >= 0)
     * @property serverVersion The version number on the server (must be >= 0)
     */
    @Serializable
    @SerialName("sync_conflict_detected")
    data class ConflictDetected(
        val entityType: String,
        val entityId: Ulid,
        val clientVersion: Long,
        val serverVersion: Long,
    ) : SyncError {
        init {
            require(entityType.isNotBlank()) { "Entity type must not be blank" }
            require(clientVersion >= 0) { "Client version must be non-negative" }
            require(serverVersion >= 0) { "Server version must be non-negative" }
            require(clientVersion != serverVersion) { "Client and server versions must differ for a conflict" }
        }

        override fun toUserMessage(): String = "A sync conflict was detected. Your changes and the server's changes need to be merged."
    }

    /**
     * The client's sync version is incompatible with the server.
     *
     * This typically means the client needs to perform a full re-sync.
     *
     * @property clientVersion The version the client is at (must be >= 0)
     * @property serverMinVersion The minimum version the server supports (must be >= 0)
     */
    @Serializable
    @SerialName("sync_version_mismatch")
    data class VersionMismatch(
        val clientVersion: Long,
        val serverMinVersion: Long,
    ) : SyncError {
        init {
            require(clientVersion >= 0) { "Client version must be non-negative" }
            require(serverMinVersion >= 0) { "Server min version must be non-negative" }
        }

        override fun toUserMessage(): String = "Your data is out of date. A full sync is required."
    }

    /**
     * The server could not be reached for synchronization.
     *
     * **IMPORTANT**: The [reason] property contains technical details that are intentionally
     * hidden from users. Implementations returning this error SHOULD log the reason at an
     * appropriate level (e.g., WARN) before returning, as it helps diagnose connectivity issues.
     *
     * @property reason A technical description of why the server is unreachable (e.g.,
     *                  "connection refused", "DNS lookup failed", "TLS handshake timeout")
     */
    @Serializable
    @SerialName("sync_server_unreachable")
    data class ServerUnreachable(
        val reason: String,
    ) : SyncError {
        init {
            require(reason.isNotBlank()) { "Reason must not be blank" }
        }

        override fun toUserMessage(): String = "Unable to connect to the server. Please check your connection and try again."
    }

    /**
     * The change set sent by the client is invalid or corrupted.
     *
     * **IMPORTANT**: The [reason] property contains technical details that are intentionally
     * hidden from users. Implementations returning this error MUST log the reason at an
     * appropriate level (e.g., WARN or ERROR) before returning, as it is critical for
     * diagnosing sync corruption issues.
     *
     * @property reason A description of what made the change set invalid (e.g., "missing
     *                  required field: entityId", "checksum mismatch", "invalid entity version")
     */
    @Serializable
    @SerialName("sync_invalid_change_set")
    data class InvalidChangeSet(
        val reason: String,
    ) : SyncError {
        init {
            require(reason.isNotBlank()) { "Reason must not be blank" }
        }

        override fun toUserMessage(): String = "The sync data appears to be corrupted. Please try again."
    }

    /**
     * The sync operation timed out before completing.
     *
     * @property elapsedMs How long the sync ran before timing out (must be > 0)
     */
    @Serializable
    @SerialName("sync_timeout")
    data class Timeout(
        val elapsedMs: Long,
    ) : SyncError {
        init {
            require(elapsedMs > 0) { "Elapsed time must be positive" }
        }

        override fun toUserMessage(): String = "The sync took too long and was cancelled. Please try again with a better connection."
    }
}
