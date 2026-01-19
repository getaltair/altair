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
        override fun toUserMessage(): String = "Invalid email or password. Please check your credentials and try again."
    }

    /**
     * The authentication token has expired and the user must re-authenticate.
     *
     * @property expiredAt The timestamp when the token expired (epoch milliseconds, must be > 0)
     */
    @Serializable
    @SerialName("auth_token_expired")
    data class TokenExpired(
        val expiredAt: Long,
    ) : AuthError {
        init {
            require(expiredAt > 0) { "Expired timestamp must be positive" }
        }

        override fun toUserMessage(): String = "Your session has expired. Please sign in again."
    }

    /**
     * The user's session has expired or is no longer valid.
     *
     * Use this when the expiration timestamp is unknown or unavailable (e.g., missing refresh token).
     * For cases where the expiration time is known, use [TokenExpired] instead.
     */
    @Serializable
    @SerialName("auth_session_expired")
    data object SessionExpired : AuthError {
        override fun toUserMessage(): String = "Your session has expired. Please sign in again."
    }

    /**
     * The authentication token is malformed or has an invalid signature.
     *
     * **IMPORTANT**: The [reason] property contains technical details that are intentionally
     * hidden from users for security. Implementations returning this error MUST log the reason
     * at an appropriate level (e.g., WARN or DEBUG) before returning, as it is not exposed
     * in the user message. This is essential for debugging authentication issues.
     *
     * @property reason Technical description of why the token is invalid (e.g., "signature
     *                  verification failed", "missing required claims", "token expired")
     */
    @Serializable
    @SerialName("auth_token_invalid")
    data class TokenInvalid(
        val reason: String,
    ) : AuthError {
        init {
            require(reason.isNotBlank()) { "Reason must not be blank" }
        }

        override fun toUserMessage(): String = "Your session is invalid. Please sign in again."
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
        init {
            require(reason.isNotBlank()) { "Reason must not be blank" }
            require(lockedUntil == null || lockedUntil > 0) { "Locked until timestamp must be positive if provided" }
        }

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
        override fun toUserMessage(): String = "An invite code is required to create an account on this server."
    }

    /**
     * The email address is already registered to another account.
     *
     * Note: The email is intentionally not stored in this error to prevent
     * potential information leakage in logs or error responses.
     */
    @Serializable
    @SerialName("auth_email_already_exists")
    data object EmailAlreadyExists : AuthError {
        override fun toUserMessage(): String = "An account with this email already exists. Please sign in or use a different email."
    }

    /**
     * The provided invite code is invalid or expired.
     */
    @Serializable
    @SerialName("auth_invalid_invite_code")
    data object InvalidInviteCode : AuthError {
        override fun toUserMessage(): String = "The invite code is invalid or has expired."
    }

    /**
     * The password does not meet security requirements.
     */
    @Serializable
    @SerialName("auth_weak_password")
    data object WeakPassword : AuthError {
        override fun toUserMessage(): String = "Password must be at least 8 characters long."
    }

    /**
     * Registration failed for an unspecified reason.
     *
     * @property reason Technical description of the failure
     */
    @Serializable
    @SerialName("auth_registration_failed")
    data class RegistrationFailed(
        val reason: String,
    ) : AuthError {
        override fun toUserMessage(): String = "Registration failed. Please try again later."
    }

    /**
     * Network or connectivity error during authentication.
     *
     * @property message Technical description of the network error
     */
    @Serializable
    @SerialName("auth_network_failure")
    data class NetworkFailure(
        val message: String,
    ) : AuthError {
        init {
            require(message.isNotBlank()) { "Message must not be blank" }
        }

        override fun toUserMessage(): String = "Unable to connect. Please check your internet connection and try again."
    }

    /**
     * Server returned an unexpected error during authentication.
     *
     * @property message Technical description of the server error
     */
    @Serializable
    @SerialName("auth_server_error")
    data class ServerError(
        val message: String,
    ) : AuthError {
        init {
            require(message.isNotBlank()) { "Message must not be blank" }
        }

        override fun toUserMessage(): String = "The server encountered an error. Please try again later."
    }
}
