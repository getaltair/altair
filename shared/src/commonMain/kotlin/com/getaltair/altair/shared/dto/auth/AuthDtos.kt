package com.getaltair.altair.shared.dto.auth

import com.getaltair.altair.shared.domain.common.UserRole
import com.getaltair.altair.shared.domain.common.UserStatus
import kotlinx.serialization.Serializable

/**
 * Request payload for user authentication via username and password.
 *
 * @property username The unique username identifier
 * @property password The plain-text password (encrypted in transit via HTTPS)
 */
@Serializable
data class LoginRequest(
    val username: String,
    val password: String
)

/**
 * Request payload for new user registration.
 *
 * @property username Desired unique username (alphanumeric, 3-20 chars)
 * @property password Plain-text password (min 8 chars, encrypted in transit)
 * @property email Optional email address for recovery
 * @property inviteCode Required registration invite code (server-validated)
 */
@Serializable
data class RegisterRequest(
    val username: String,
    val password: String,
    val email: String?,
    val inviteCode: String
)

/**
 * Response payload containing JWT authentication tokens.
 *
 * @property accessToken Short-lived JWT for API authentication
 * @property refreshToken Long-lived token for obtaining new access tokens
 * @property expiresIn Access token lifetime in seconds
 * @property tokenType Token type identifier, always "Bearer"
 */
@Serializable
data class TokenResponse(
    val accessToken: String,
    val refreshToken: String,
    val expiresIn: Long,
    val tokenType: String = "Bearer"
)

/**
 * Request payload for refreshing an expired access token.
 *
 * @property refreshToken The long-lived refresh token
 */
@Serializable
data class RefreshTokenRequest(
    val refreshToken: String
)

/**
 * Response payload containing user account information.
 *
 * @property id Unique user identifier (UUID format)
 * @property username Unique username
 * @property email User email address (nullable)
 * @property role User permission role (ADMIN or MEMBER)
 * @property status User account operational status
 * @property storageUsed Current storage consumption in bytes
 * @property storageQuota Maximum allowed storage in bytes (null = unlimited)
 * @property createdAt Account creation timestamp (ISO-8601 format)
 */
@Serializable
data class UserResponse(
    val id: String,
    val username: String,
    val email: String?,
    val role: UserRole,
    val status: UserStatus,
    val storageUsed: Long,
    val storageQuota: Long?,
    val createdAt: String
)

/**
 * Request payload for changing user password.
 *
 * @property currentPassword Current password for verification
 * @property newPassword New password (min 8 chars, encrypted in transit)
 */
@Serializable
data class ChangePasswordRequest(
    val currentPassword: String,
    val newPassword: String
)

/**
 * Standardized error response payload for API failures.
 *
 * @property code Machine-readable error code (e.g., "INVALID_CREDENTIALS")
 * @property message Human-readable error message
 * @property details Optional map of field-specific error details
 */
@Serializable
data class ErrorResponse(
    val code: String,
    val message: String,
    val details: Map<String, String>? = null
)
