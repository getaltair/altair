package com.getaltair.altair.domain

/**
 * Base sealed interface for all domain errors in Altair.
 * Use with Arrow's Either<DomainError, T> for functional error handling.
 *
 * Example usage:
 * ```kotlin
 * suspend fun getUser(id: String): Either<DomainError, User> = either {
 *     val validated = validateId(id).bind()  // Short-circuits on Left
 *     fetchData(validated).bind()
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
     * Returns a user-friendly message suitable for display in the UI.
     * Technical details are hidden; use the error properties for logging.
     */
    fun toUserMessage(): String

    /**
     * Network-related errors (connectivity, timeouts, etc.)
     *
     * @property message Technical description of the network error
     * @property cause Optional underlying exception
     */
    data class NetworkError private constructor(
        val message: String,
        val cause: Throwable? = null,
    ) : DomainError {
        override fun toUserMessage(): String = "Unable to connect. Please check your internet connection and try again."

        companion object {
            /**
             * Creates a NetworkError with validation.
             * @throws IllegalArgumentException if message is blank
             */
            operator fun invoke(
                message: String,
                cause: Throwable? = null,
            ): NetworkError {
                require(message.isNotBlank()) { "NetworkError message must not be blank" }
                return NetworkError(message, cause)
            }
        }
    }

    /**
     * Validation errors (invalid input, constraints violated)
     *
     * @property field The field that failed validation
     * @property message Description of what validation failed
     */
    data class ValidationError private constructor(
        val field: String,
        val message: String,
    ) : DomainError {
        override fun toUserMessage(): String = message

        companion object {
            /**
             * Creates a ValidationError with validation.
             * @throws IllegalArgumentException if field or message is blank
             */
            operator fun invoke(
                field: String,
                message: String,
            ): ValidationError {
                require(field.isNotBlank()) { "ValidationError field must not be blank" }
                require(message.isNotBlank()) { "ValidationError message must not be blank" }
                return ValidationError(field, message)
            }
        }
    }

    /**
     * Resource not found errors
     *
     * @property resource The type of resource that was not found
     * @property id The identifier that was searched for
     */
    data class NotFoundError private constructor(
        val resource: String,
        val id: String,
    ) : DomainError {
        override fun toUserMessage(): String = "The requested $resource could not be found."

        companion object {
            /**
             * Creates a NotFoundError with validation.
             * @throws IllegalArgumentException if resource or id is blank
             */
            operator fun invoke(
                resource: String,
                id: String,
            ): NotFoundError {
                require(resource.isNotBlank()) { "NotFoundError resource must not be blank" }
                require(id.isNotBlank()) { "NotFoundError id must not be blank" }
                return NotFoundError(resource, id)
            }
        }
    }

    /**
     * Unauthorized access errors
     *
     * @property message Technical description of why access was denied
     */
    data class UnauthorizedError(
        val message: String = "Unauthorized access",
    ) : DomainError {
        override fun toUserMessage(): String = "You don't have permission to access this. Please sign in and try again."
    }

    /**
     * Generic errors for unexpected situations.
     * Use sparingly - prefer specific error types when the failure mode is known.
     *
     * @property message Technical description of what went wrong
     * @property cause Optional underlying exception
     */
    data class UnexpectedError private constructor(
        val message: String,
        val cause: Throwable? = null,
    ) : DomainError {
        override fun toUserMessage(): String = "Something went wrong. Please try again later."

        companion object {
            /**
             * Creates an UnexpectedError with validation.
             * @throws IllegalArgumentException if message is blank
             */
            operator fun invoke(
                message: String,
                cause: Throwable? = null,
            ): UnexpectedError {
                require(message.isNotBlank()) { "UnexpectedError message must not be blank" }
                return UnexpectedError(message, cause)
            }
        }
    }
}
