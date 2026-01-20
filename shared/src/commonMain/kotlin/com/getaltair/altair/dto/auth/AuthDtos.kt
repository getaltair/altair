package com.getaltair.altair.dto.auth

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import kotlinx.serialization.Serializable

/**
 * Request to authenticate a user with email and password.
 *
 * Note: Constructor is public for RPC serialization. Use [create] for validated construction.
 */
@Serializable
data class AuthRequest(
    val email: String,
    val password: String,
) {
    companion object {
        /**
         * Creates an AuthRequest with validation.
         * Returns Left(ValidationError) if validation fails, Right(AuthRequest) otherwise.
         */
        fun create(
            email: String,
            password: String,
        ): Either<DomainError.ValidationError, AuthRequest> {
            if (email.isBlank()) {
                return DomainError.ValidationError("email", "Email must not be blank").left()
            }
            if (!email.contains("@")) {
                return DomainError.ValidationError("email", "Email must contain @").left()
            }
            if (password.isBlank()) {
                return DomainError.ValidationError("password", "Password must not be blank").left()
            }
            return AuthRequest(email, password).right()
        }
    }
}

/**
 * Response containing authentication tokens and user information.
 *
 * Note: Constructor is public for RPC serialization. Use [create] for validated construction.
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
    companion object {
        /**
         * Creates an AuthResponse with validation.
         * Returns Left(ValidationError) if validation fails, Right(AuthResponse) otherwise.
         */
        @Suppress("LongParameterList") // Factory method mirrors DTO structure
        fun create(
            accessToken: String,
            refreshToken: String,
            expiresIn: Long,
            userId: Ulid,
            displayName: String,
            role: UserRole,
        ): Either<DomainError.ValidationError, AuthResponse> {
            if (accessToken.isBlank()) {
                return DomainError.ValidationError("accessToken", "Access token must not be blank").left()
            }
            if (refreshToken.isBlank()) {
                return DomainError.ValidationError("refreshToken", "Refresh token must not be blank").left()
            }
            if (expiresIn <= 0) {
                return DomainError.ValidationError("expiresIn", "Token expiry must be positive").left()
            }
            if (displayName.isBlank()) {
                return DomainError.ValidationError("displayName", "Display name must not be blank").left()
            }
            return AuthResponse(accessToken, refreshToken, expiresIn, userId, displayName, role).right()
        }
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
 * Note: Constructor is public for RPC serialization. Use [create] for validated construction.
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
    companion object {
        /**
         * Creates a TokenRefreshResponse with validation.
         * Returns Left(ValidationError) if validation fails, Right(TokenRefreshResponse) otherwise.
         */
        fun create(
            accessToken: String,
            refreshToken: String,
            expiresIn: Long,
        ): Either<DomainError.ValidationError, TokenRefreshResponse> {
            if (accessToken.isBlank()) {
                return DomainError.ValidationError("accessToken", "Access token must not be blank").left()
            }
            if (refreshToken.isBlank()) {
                return DomainError.ValidationError("refreshToken", "Refresh token must not be blank").left()
            }
            if (expiresIn <= 0) {
                return DomainError.ValidationError("expiresIn", "Token expiry must be positive").left()
            }
            return TokenRefreshResponse(accessToken, refreshToken, expiresIn).right()
        }
    }
}

/**
 * Request to register a new user account.
 *
 * Note: Constructor is public for RPC serialization. Use [create] for validated construction.
 */
@Serializable
data class RegisterRequest(
    val email: String,
    val password: String,
    val displayName: String,
    val inviteCode: String? = null,
) {
    companion object {
        private val EMAIL_REGEX = Regex("^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$")
        private const val MIN_PASSWORD_LENGTH = 8
        private const val MAX_DISPLAY_NAME_LENGTH = 100

        /**
         * Creates a RegisterRequest with validation.
         * Returns Left(ValidationError) if validation fails, Right(RegisterRequest) otherwise.
         */
        fun create(
            email: String,
            password: String,
            displayName: String,
            inviteCode: String? = null,
        ): Either<DomainError.ValidationError, RegisterRequest> {
            if (email.isBlank()) {
                return DomainError.ValidationError("email", "Email must not be blank").left()
            }
            if (!email.matches(EMAIL_REGEX)) {
                return DomainError.ValidationError("email", "Invalid email format").left()
            }
            if (password.length < MIN_PASSWORD_LENGTH) {
                return DomainError
                    .ValidationError(
                        "password",
                        "Password must be at least $MIN_PASSWORD_LENGTH characters",
                    ).left()
            }
            if (displayName.isBlank()) {
                return DomainError.ValidationError("displayName", "Display name must not be blank").left()
            }
            if (displayName.length > MAX_DISPLAY_NAME_LENGTH) {
                return DomainError
                    .ValidationError(
                        "displayName",
                        "Display name too long (max $MAX_DISPLAY_NAME_LENGTH characters)",
                    ).left()
            }
            if (inviteCode != null && inviteCode.isBlank()) {
                return DomainError.ValidationError("inviteCode", "Invite code must not be blank if provided").left()
            }
            return RegisterRequest(email, password, displayName, inviteCode).right()
        }
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
