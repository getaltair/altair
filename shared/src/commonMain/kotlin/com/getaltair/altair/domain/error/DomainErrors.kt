package com.getaltair.altair.domain.error

import arrow.core.Either
import arrow.core.raise.Raise
import arrow.core.raise.either

/**
 * Base sealed interface for all domain errors.
 * Each module defines its own error hierarchy extending this.
 */
sealed interface DomainError {
    val message: String
}

/**
 * Guidance module errors (Quests, Epics, Checkpoints).
 */
sealed interface QuestError : DomainError {
    data class NotFound(val id: String) : QuestError {
        override val message = "Quest not found: $id"
    }
    data class ValidationFailed(override val message: String) : QuestError
    data object WipLimitExceeded : QuestError {
        override val message = "WIP limit exceeded: only one quest can be active at a time"
    }
    data class DatabaseError(val cause: Throwable) : QuestError {
        override val message = "Database error: ${cause.message}"
    }
}

/**
 * Knowledge module errors (Notes, Folders, Tags).
 */
sealed interface NoteError : DomainError {
    data class NotFound(val id: String) : NoteError {
        override val message = "Note not found: $id"
    }
    data class DuplicateTitle(val title: String, val folderId: String?) : NoteError {
        override val message = "Note with title '$title' already exists in folder"
    }
    data class ValidationFailed(override val message: String) : NoteError
    data class DatabaseError(val cause: Throwable) : NoteError {
        override val message = "Database error: ${cause.message}"
    }
}

/**
 * Tracking module errors (Items, Locations, Containers).
 */
sealed interface ItemError : DomainError {
    data class NotFound(val id: String) : ItemError {
        override val message = "Item not found: $id"
    }
    data class ValidationFailed(override val message: String) : ItemError
    data class LocationConflict(override val message: String) : ItemError
    data class DatabaseError(val cause: Throwable) : ItemError {
        override val message = "Database error: ${cause.message}"
    }
}

/**
 * Sync errors for client-server synchronization.
 */
sealed interface SyncError : DomainError {
    data class NetworkError(val cause: Throwable) : SyncError {
        override val message = "Network error: ${cause.message}"
    }
    data class ConflictError(val entityId: String, val localVersion: Long, val serverVersion: Long) : SyncError {
        override val message = "Version conflict for $entityId: local=$localVersion, server=$serverVersion"
    }
    data class AuthenticationError(override val message: String) : SyncError
    data class ServerError(val statusCode: Int, override val message: String) : SyncError
}

/**
 * Type alias for repository operations returning Either.
 * Note: E should be a subtype of DomainError (enforced by usage context, not compiler).
 */
typealias DomainResult<E, T> = Either<E, T>

/**
 * Extension to convert exceptions to domain errors within Raise context.
 */
inline fun <E : DomainError, T> Raise<E>.catching(onError: (Throwable) -> E, block: () -> T): T = try {
    block()
} catch (e: Throwable) {
    raise(onError(e))
}

/**
 * Execute a block in Either context for domain operations.
 */
inline fun <E : DomainError, T> domainEither(block: Raise<E>.() -> T): Either<E, T> = either(block)
