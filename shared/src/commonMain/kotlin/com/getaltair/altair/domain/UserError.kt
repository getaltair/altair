package com.getaltair.altair.domain

import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Errors specific to User operations.
 *
 * These errors extend [DomainError] to enable exhaustive when-matching
 * for User-specific error handling while maintaining compatibility
 * with generic error handlers.
 *
 * UserError unifies errors that can occur during user management operations,
 * including both general CRUD operations and auth-specific scenarios.
 */
@Serializable
sealed interface UserError : DomainError {
    /**
     * The requested user could not be found.
     *
     * @property id The ULID of the user that was not found
     */
    @Serializable
    @SerialName("user_not_found")
    data class NotFound(
        val id: Ulid,
    ) : UserError {
        override fun toUserMessage(): String = "The requested user could not be found."
    }

    /**
     * The email address is already registered to another account.
     *
     * Note: The email is intentionally not stored in this error to prevent
     * potential information leakage in logs or error responses.
     */
    @Serializable
    @SerialName("user_email_already_exists")
    data object EmailAlreadyExists : UserError {
        override fun toUserMessage(): String = "An account with this email already exists."
    }

    /**
     * The email address could not be found.
     *
     * Used when looking up a user by email fails.
     */
    @Serializable
    @SerialName("user_email_not_found")
    data object EmailNotFound : UserError {
        override fun toUserMessage(): String = "No account found with this email address."
    }

    /**
     * The user's storage quota has been exceeded.
     *
     * @property currentUsage Current storage used in bytes
     * @property quota Maximum storage allowed in bytes
     */
    @Serializable
    @SerialName("user_storage_quota_exceeded")
    data class StorageQuotaExceeded(
        val currentUsage: Long,
        val quota: Long,
    ) : UserError {
        override fun toUserMessage(): String = "You have exceeded your storage quota. Please free up space or upgrade your plan."
    }

    /**
     * The operation is not permitted for the user's current role.
     *
     * @property requiredRole The role required to perform the operation
     */
    @Serializable
    @SerialName("user_insufficient_permissions")
    data class InsufficientPermissions(
        val requiredRole: UserRole,
    ) : UserError {
        override fun toUserMessage(): String = "You do not have permission to perform this action."
    }
}
