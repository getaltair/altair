package com.getaltair.altair.domain

/**
 * Base sealed interface for all domain errors in Altair.
 * Use with Arrow's Either<DomainError, T> for functional error handling.
 *
 * Example usage:
 * ```kotlin
 * suspend fun getUser(id: String): Either<DomainError, User> = either {
 *     val cached = cache.get(id).bind()
 *     cached ?: api.fetchUser(id).bind()
 * }
 * ```
 *
 * This follows the architecture pattern outlined in CLAUDE.md:
 * - All operations that can fail return Either<DomainError, T>
 * - Never throw exceptions for expected failures
 * - Chain operations with flatMap, map, and recover with fold
 */
sealed interface DomainError {
    /**
     * Network-related errors (connectivity, timeouts, etc.)
     */
    data class NetworkError(
        val message: String,
        val cause: Throwable? = null,
    ) : DomainError

    /**
     * Validation errors (invalid input, constraints violated)
     */
    data class ValidationError(
        val field: String,
        val message: String,
    ) : DomainError

    /**
     * Resource not found errors
     */
    data class NotFoundError(
        val resource: String,
        val id: String,
    ) : DomainError

    /**
     * Unauthorized access errors
     */
    data class UnauthorizedError(
        val message: String = "Unauthorized access",
    ) : DomainError

    /**
     * Generic errors for unexpected situations
     */
    data class UnexpectedError(
        val message: String,
        val cause: Throwable? = null,
    ) : DomainError
}
