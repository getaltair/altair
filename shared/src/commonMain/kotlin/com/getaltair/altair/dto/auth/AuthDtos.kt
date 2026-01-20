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
)

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
)

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
)

/**
 * Request to register a new user account.
 */
@Serializable
data class RegisterRequest(
    val email: String,
    val password: String,
    val displayName: String,
    val inviteCode: String? = null,
)

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
