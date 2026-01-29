package com.getaltair.altair.shared.core.error

import arrow.core.Either
import arrow.core.left
import arrow.core.right

/**
 * Base interface for domain errors.
 *
 * @see com.getaltair.altair.shared.domain.error.AltairError for the comprehensive error hierarchy.
 * This interface is kept for backward compatibility with Phase 1 code.
 */
sealed interface DomainError {
    val message: String
}

/**
 * Errors related to Quest operations.
 */
sealed interface QuestError : DomainError {
    data object WipLimitExceeded : QuestError {
        override val message = "Cannot start quest: WIP limit of 1 already reached"
    }

    data class NotFound(val id: String) : QuestError {
        override val message = "Quest not found: $id"
    }

    data class ValidationFailed(override val message: String) : QuestError
}

/**
 * Errors related to Note operations.
 */
sealed interface NoteError : DomainError {
    data class NotFound(val id: String) : NoteError {
        override val message = "Note not found: $id"
    }
}

/**
 * Errors related to Item (inventory) operations.
 */
sealed interface ItemError : DomainError {
    data class NotFound(val id: String) : ItemError {
        override val message = "Item not found: $id"
    }
}

/**
 * Type alias for domain operations that can fail.
 */
typealias DomainResult<T> = Either<DomainError, T>

/**
 * Extension to convert nullable to Either with error.
 */
inline fun <E : DomainError, T> T?.toEither(error: () -> E): Either<E, T> =
    this?.right() ?: error().left()
