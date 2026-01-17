package com.getaltair.altair.domain

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

/**
 * Errors related to authentication and authorization.
 *
 * These errors extend [DomainError] to enable exhaustive when-matching
 * for auth-specific error handling while maintaining compatibility
 * with generic error handlers.
 */
@Serializable
sealed interface AuthError : DomainError {
    /**
     * The provided credentials (email/password) are incorrect.
     */
    @Serializable
    @SerialName("auth_invalid_credentials")
    data object InvalidCredentials : AuthError {
        override fun toUserMessage(): String =
            "Invalid email or password. Please check your credentials and try again."
    }

    /**
     * The authentication token has expired and the user must re-authenticate.
     *
     * @property expiredAt The timestamp when the token expired (epoch milliseconds)
     */
    @Serializable
    @SerialName("auth_token_expired")
    data class TokenExpired(val expiredAt: Long) : AuthError {
        override fun toUserMessage(): String =
            "Your session has expired. Please sign in again."
    }

    /**
     * The authentication token is malformed or has an invalid signature.
     *
     * @property reason Technical description of why the token is invalid
     */
    @Serializable
    @SerialName("auth_token_invalid")
    data class TokenInvalid(val reason: String) : AuthError {
        override fun toUserMessage(): String =
            "Your session is invalid. Please sign in again."
    }

    /**
     * The user's account has been locked due to security concerns.
     *
     * @property reason The reason the account was locked
     * @property lockedUntil When the lock expires (epoch milliseconds), null if permanent
     */
    @Serializable
    @SerialName("auth_account_locked")
    data class AccountLocked(
        val reason: String,
        val lockedUntil: Long?,
    ) : AuthError {
        override fun toUserMessage(): String =
            if (lockedUntil != null) {
                "Your account has been temporarily locked. Please try again later."
            } else {
                "Your account has been locked. Please contact support for assistance."
            }
    }

    /**
     * The server requires an invite code to create new accounts.
     */
    @Serializable
    @SerialName("auth_invite_required")
    data object InviteRequired : AuthError {
        override fun toUserMessage(): String =
            "An invite code is required to create an account on this server."
    }

    /**
     * The provided invite code is invalid or has already been used.
     *
     * @property code The invite code that was rejected
     */
    @Serializable
    @SerialName("auth_invalid_invite")
    data class InvalidInvite(val code: String) : AuthError {
        override fun toUserMessage(): String =
            "The invite code is invalid or has already been used."
    }

    /**
     * The email address is already registered to another account.
     *
     * @property email The email that already exists
     */
    @Serializable
    @SerialName("auth_email_already_exists")
    data class EmailAlreadyExists(val email: String) : AuthError {
        override fun toUserMessage(): String =
            "An account with this email already exists. Please sign in or use a different email."
    }
}
