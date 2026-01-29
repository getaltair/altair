package com.getaltair.altair.shared.domain.error

import arrow.core.Either

/**
 * Root error type for all Altair domain operations.
 *
 * Provides a comprehensive hierarchy of domain-specific errors that can occur
 * during business logic operations.
 *
 * ## Error Categories
 *
 * - [NetworkError] - Network connectivity and server communication failures
 * - [AuthError] - Authentication and authorization failures
 * - [ValidationError] - Input validation and constraint violations
 * - [NotFoundError] - Entity lookup failures with specific entity types
 * - [ConflictError] - State conflicts, duplicates, and constraint violations
 * - [StorageError] - Database and file storage failures
 *
 * ## Usage with Arrow
 *
 * Use with Arrow's [Either] type for functional error handling:
 *
 * ```kotlin
 * fun findUser(id: String): AltairResult<User> =
 *     userRepository.findById(id)
 *         .toEither { AltairError.NotFoundError.UserNotFound(id) }
 * ```
 *
 * @see AltairResult
 */
sealed interface AltairError {
    val message: String

    /**
     * Network connectivity and server communication errors.
     *
     * These errors represent failures in network operations, including
     * connection failures, timeouts, and server-side errors.
     */
    sealed interface NetworkError : AltairError {
        /**
         * Failed to establish network connection.
         *
         * @property message Descriptive error message including connection details
         */
        data class ConnectionFailed(override val message: String) : NetworkError

        /**
         * Network operation timed out.
         *
         * @property message Descriptive error message including timeout duration
         */
        data class Timeout(override val message: String) : NetworkError

        /**
         * Server returned an error response.
         *
         * @property code HTTP status code or server error code
         * @property message Descriptive error message from server
         */
        data class ServerError(val code: Int, override val message: String) : NetworkError
    }

    /**
     * Authentication and authorization errors.
     *
     * These errors represent failures in user authentication, token validation,
     * and access control operations.
     */
    sealed interface AuthError : AltairError {
        /**
         * User credentials are invalid.
         */
        data object InvalidCredentials : AuthError {
            override val message = "Invalid credentials"
        }

        /**
         * Authentication token has expired.
         */
        data object TokenExpired : AuthError {
            override val message = "Token expired"
        }

        /**
         * User is not authorized to perform the requested operation.
         */
        data object Unauthorized : AuthError {
            override val message = "Unauthorized access"
        }

        /**
         * User account has been disabled.
         *
         * @property reason Explanation for why the account was disabled
         */
        data class AccountDisabled(val reason: String) : AuthError {
            override val message = "Account disabled: $reason"
        }
    }

    /**
     * Input validation and constraint violation errors.
     *
     * These errors represent failures in validating user input or domain
     * constraints during entity creation or updates.
     */
    sealed interface ValidationError : AltairError {
        /**
         * Required field is missing.
         *
         * @property field Name of the missing required field
         */
        data class FieldRequired(val field: String) : ValidationError {
            override val message = "Field required: $field"
        }

        /**
         * Field value is invalid.
         *
         * @property field Name of the invalid field
         * @property reason Explanation of why the value is invalid
         */
        data class FieldInvalid(val field: String, val reason: String) : ValidationError {
            override val message = "Field '$field' invalid: $reason"
        }

        /**
         * Domain constraint has been violated.
         *
         * @property message Descriptive error message explaining the constraint violation
         */
        data class ConstraintViolation(override val message: String) : ValidationError
    }

    /**
     * Entity lookup failures with specific entity types.
     *
     * These errors represent failures to find domain entities by their identifiers.
     * Each entity type has a specific error class for type-safe error handling.
     *
     * @property entityType Human-readable entity type name
     * @property entityId The identifier that was not found
     */
    sealed interface NotFoundError : AltairError {
        val entityType: String
        val entityId: String
        override val message: String get() = "$entityType not found: $entityId"

        /** User entity not found. */
        data class UserNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "User"
        }

        /** Quest entity not found. */
        data class QuestNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "Quest"
        }

        /** Note entity not found. */
        data class NoteNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "Note"
        }

        /** Item entity not found. */
        data class ItemNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "Item"
        }

        /** Initiative entity not found. */
        data class InitiativeNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "Initiative"
        }

        /** Epic entity not found. */
        data class EpicNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "Epic"
        }

        /** Routine entity not found. */
        data class RoutineNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "Routine"
        }

        /** Folder entity not found. */
        data class FolderNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "Folder"
        }

        /** Tag entity not found. */
        data class TagNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "Tag"
        }

        /** Location entity not found. */
        data class LocationNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "Location"
        }

        /** Container entity not found. */
        data class ContainerNotFound(override val entityId: String) : NotFoundError {
            override val entityType = "Container"
        }
    }

    /**
     * State conflicts, duplicates, and constraint violations.
     *
     * These errors represent failures due to conflicting state, duplicate entities,
     * or violations of domain-specific constraints like WIP limits.
     */
    sealed interface ConflictError : AltairError {
        /**
         * Duplicate entity detected.
         *
         * @property entityType Type of the duplicate entity
         * @property field Name of the field that caused the duplicate
         * @property value Value that was duplicated
         */
        data class DuplicateEntity(val entityType: String, val field: String, val value: String) : ConflictError {
            override val message = "Duplicate $entityType: $field='$value'"
        }

        /**
         * Optimistic locking version conflict.
         *
         * @property entityType Type of the entity with version conflict
         * @property entityId Identifier of the entity with conflict
         */
        data class VersionConflict(val entityType: String, val entityId: String) : ConflictError {
            override val message = "Version conflict on $entityType: $entityId"
        }

        /**
         * Work-in-progress limit exceeded.
         *
         * @property current Current number of WIP items
         * @property limit Maximum allowed WIP items
         */
        data class WipLimitExceeded(val current: Int, val limit: Int) : ConflictError {
            override val message = "WIP limit exceeded: $current/$limit"
        }
    }

    /**
     * Database and file storage errors.
     *
     * These errors represent failures in persistent storage operations,
     * including quota limits, file size limits, and database failures.
     */
    sealed interface StorageError : AltairError {
        /**
         * Storage quota exceeded.
         *
         * @property used Current storage usage in bytes
         * @property quota Maximum allowed storage in bytes
         */
        data class QuotaExceeded(val used: Long, val quota: Long) : StorageError {
            override val message = "Storage quota exceeded: $used/$quota bytes"
        }

        /**
         * File exceeds maximum size limit.
         *
         * @property size Actual file size in bytes
         * @property maxSize Maximum allowed size in bytes
         */
        data class FileTooLarge(val size: Long, val maxSize: Long) : StorageError {
            override val message = "File too large: $size bytes (max: $maxSize)"
        }

        /**
         * Database operation failed.
         *
         * @property message Descriptive error message from database
         */
        data class DatabaseError(override val message: String) : StorageError
    }
}

/**
 * Type alias for operations returning Either with AltairError.
 *
 * Use this for all domain operations that can fail with an [AltairError].
 * Provides a consistent return type across the domain layer.
 *
 * ## Example
 *
 * ```kotlin
 * fun createQuest(title: String): AltairResult<Quest> =
 *     if (title.isBlank())
 *         AltairError.ValidationError.FieldRequired("title").left()
 *     else
 *         Quest(title = title).right()
 * ```
 *
 * @param T Success value type
 */
typealias AltairResult<T> = Either<AltairError, T>
