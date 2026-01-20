package com.getaltair.altair.dto.auth

import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import kotlinx.serialization.Serializable

/**
 * Request to authenticate a user with email and password.
 */
@Serializable
data class AuthRequest(
    val email: String,
    val password: String,
) {
    init {
        require(email.isNotBlank()) { "Email must not be blank" }
        require(email.contains("@")) { "Email must contain @" }
        require(password.isNotBlank()) { "Password must not be blank" }
    }
}

/**
 * Response containing authentication tokens and user information.
 *
 * @property accessToken JWT access token for API authentication
 * @property refreshToken JWT refresh token for obtaining new access tokens
 * @property expiresIn Token validity duration in seconds
 * @property userId Unique identifier for the authenticated user
 * @property displayName User's display name
 * @property role User's permission level (admin or member)
 */
@Serializable
data class AuthResponse(
    val accessToken: String,
    val refreshToken: String,
    val expiresIn: Long,
    val userId: Ulid,
    val displayName: String,
    val role: UserRole,
) {
    init {
        require(accessToken.isNotBlank()) { "Access token must not be blank" }
        require(refreshToken.isNotBlank()) { "Refresh token must not be blank" }
        require(expiresIn > 0) { "Token expiry must be positive" }
        require(displayName.isNotBlank()) { "Display name must not be blank" }
    }
}

/**
 * Request to refresh an expired access token.
 */
@Serializable
data class TokenRefreshRequest(
    val refreshToken: String,
)

/**
 * Response containing new access and refresh tokens.
 *
 * Implements refresh token rotation: each refresh invalidates the old
 * refresh token and issues a new one to prevent token replay attacks.
 */
@Serializable
data class TokenRefreshResponse(
    val accessToken: String,
    val refreshToken: String,
    val expiresIn: Long,
) {
    init {
        require(accessToken.isNotBlank()) { "Access token must not be blank" }
        require(refreshToken.isNotBlank()) { "Refresh token must not be blank" }
        require(expiresIn > 0) { "Token expiry must be positive" }
    }
}

/**
 * Request to register a new user account.
 */
@Serializable
data class RegisterRequest(
    val email: String,
    val password: String,
    val displayName: String,
    val inviteCode: String? = null,
) {
    init {
        require(email.isNotBlank()) { "Email must not be blank" }
        require(email.matches(EMAIL_REGEX)) { "Invalid email format" }
        require(password.length >= MIN_PASSWORD_LENGTH) { "Password must be at least $MIN_PASSWORD_LENGTH characters" }
        require(displayName.isNotBlank()) { "Display name must not be blank" }
        require(displayName.length <= MAX_DISPLAY_NAME_LENGTH) { "Display name too long (max $MAX_DISPLAY_NAME_LENGTH characters)" }
        require(inviteCode?.isNotBlank() != false) { "Invite code must not be blank if provided" }
    }

    companion object {
        private val EMAIL_REGEX = Regex("^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$")
        private const val MIN_PASSWORD_LENGTH = 8
        private const val MAX_DISPLAY_NAME_LENGTH = 100
    }
}

/**
 * Request to change user password.
 */
@Serializable
data class ChangePasswordRequest(
    val currentPassword: String,
    val newPassword: String,
)

/**
 * Request to initiate password reset.
 */
@Serializable
data class ForgotPasswordRequest(
    val email: String,
)

/**
 * Request to complete password reset with token.
 */
@Serializable
data class ResetPasswordRequest(
    val token: String,
    val newPassword: String,
)

/**
 * Response containing a newly generated invite code.
 */
@Serializable
data class InviteCodeResponse(
    val code: String,
    val expiresAt: String,
)

/**
 * Simple success/failure response.
 */
@Serializable
data class SuccessResponse(
    val success: Boolean,
    val message: String? = null,
)
