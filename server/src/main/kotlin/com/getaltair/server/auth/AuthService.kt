package com.getaltair.server.auth

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.common.UserRole
import com.getaltair.altair.shared.domain.common.UserStatus
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.system.User
import com.getaltair.altair.shared.dto.auth.*
import com.getaltair.altair.shared.repository.RefreshTokenRepository
import com.getaltair.altair.shared.repository.UserRepository
import kotlinx.datetime.Instant
import java.security.MessageDigest
import kotlin.time.Duration.Companion.seconds

/**
 * Service handling authentication business logic.
 *
 * Provides user authentication, registration, token management, and password operations.
 * All password operations use Argon2 hashing via [PasswordHasher].
 * All token operations use JWT via [JwtService].
 *
 * ## Security Considerations
 *
 * - Passwords are hashed with Argon2id before storage
 * - Refresh tokens are hashed (SHA-256) before storage
 * - Failed login attempts should be rate-limited (TODO: add rate limiting)
 * - Password change revokes all existing refresh tokens
 * - Invite code validation prevents unauthorized registration
 *
 * ## Error Handling
 *
 * All methods return [Either]<[AltairError], T> for functional error handling:
 *
 * - [AltairError.AuthError.InvalidCredentials] - Login failed
 * - [AltairError.ValidationError] - Invalid input data
 * - [AltairError.ConflictError.DuplicateEntity] - Username/email exists
 * - [AltairError.NotFoundError.UserNotFound] - User not found
 */
class AuthService(
    private val userRepository: UserRepository,
    private val refreshTokenRepository: RefreshTokenRepository,
    private val jwtService: JwtService,
    private val passwordHasher: PasswordHasher,
    private val config: AuthConfig
) {
    /**
     * Gets the current timestamp for audit fields.
     */
    private fun now(): Instant = Instant.fromEpochMilliseconds(System.currentTimeMillis())
    /**
     * Authenticate a user with username and password.
     *
     * @param request Login credentials
     * @return [Either] containing [TokenResponse] on success, or error on failure
     */
    suspend fun login(request: LoginRequest): Either<AltairError, TokenResponse> {
        // Validate input
        if (request.username.isBlank()) {
            return AltairError.ValidationError.FieldRequired("username").left()
        }
        if (request.password.isBlank()) {
            return AltairError.ValidationError.FieldRequired("password").left()
        }

        // Find user by username
        val user = userRepository.findByUsername(request.username).fold(
            ifLeft = { error -> return error.left() },
            ifRight = { it ?: return AltairError.AuthError.InvalidCredentials.left() }
        )

        // Check account status
        if (user.status != UserStatus.ACTIVE) {
            return AltairError.AuthError.AccountDisabled("Account status: ${user.status}").left()
        }

        // Verify password
        if (!passwordHasher.verify(request.password, user.passwordHash)) {
            return AltairError.AuthError.InvalidCredentials.left()
        }

        // Generate token pair
        val tokenPair = jwtService.generateTokenPair(
            userId = user.id.toString(),
            username = user.username,
            role = user.role.name
        )

        // Store refresh token hash
        val tokenHash = hashToken(tokenPair.refreshToken)
        val expiresAt = now().plus(config.refreshTokenExpiry.seconds)

        refreshTokenRepository.storeToken(
            userId = user.id.toString(),
            tokenHash = tokenHash,
            expiresAt = expiresAt
        ).fold(
            ifLeft = { error -> return error.left() },
            ifRight = { /* token stored */ }
        )

        // Update last login timestamp
        userRepository.updateLastLogin(user.id.toString()).fold(
            ifLeft = { /* ignore error - not critical */ },
            ifRight = { /* updated */ }
        )

        return TokenResponse(
            accessToken = tokenPair.accessToken,
            refreshToken = tokenPair.refreshToken,
            expiresIn = tokenPair.expiresIn,
            tokenType = "Bearer"
        ).right()
    }

    /**
     * Register a new user account.
     *
     * @param request Registration details
     * @return [Either] containing [TokenResponse] on success, or error on failure
     */
    suspend fun register(request: RegisterRequest): Either<AltairError, TokenResponse> {
        // Validate invite code
        if (request.inviteCode != config.inviteCode) {
            return AltairError.ValidationError.FieldInvalid("inviteCode", "Invalid invite code").left()
        }

        // Validate username
        if (request.username.isBlank()) {
            return AltairError.ValidationError.FieldRequired("username").left()
        }
        if (request.username.length < 3) {
            return AltairError.ValidationError.FieldInvalid("username", "Must be at least 3 characters").left()
        }
        if (request.username.length > 50) {
            return AltairError.ValidationError.FieldInvalid("username", "Must be at most 50 characters").left()
        }

        // Validate password
        if (request.password.isBlank()) {
            return AltairError.ValidationError.FieldRequired("password").left()
        }
        if (request.password.length < 8) {
            return AltairError.ValidationError.FieldInvalid("password", "Must be at least 8 characters").left()
        }

        // Validate email if provided
        request.email?.let { email ->
            if (email.isBlank() || !email.contains("@")) {
                return AltairError.ValidationError.FieldInvalid("email", "Invalid email format").left()
            }
        }

        // Check username uniqueness
        val usernameExists = userRepository.usernameExists(request.username).fold(
            ifLeft = { error -> return error.left() },
            ifRight = { it }
        )
        if (usernameExists) {
            return AltairError.ConflictError.DuplicateEntity(
                entityType = "User",
                field = "username",
                value = request.username
            ).left()
        }

        // Check email uniqueness if provided
        request.email?.let { email ->
            val emailExists = userRepository.emailExists(email).fold(
                ifLeft = { error -> return error.left() },
                ifRight = { it }
            )
            if (emailExists) {
                return AltairError.ConflictError.DuplicateEntity(
                    entityType = "User",
                    field = "email",
                    value = email
                ).left()
            }
        }

        // Hash password
        val passwordHash = passwordHasher.hash(request.password)

        // Create user
        val newUser = User(
            id = Ulid.generate(),
            username = request.username,
            email = request.email,
            passwordHash = passwordHash,
            role = UserRole.MEMBER,
            status = UserStatus.ACTIVE,
            storageUsed = 0L,
            storageQuota = null,
            createdAt = now(),
            lastLoginAt = null,
            deletedAt = null
        )

        val createdUser = userRepository.create(newUser).fold(
            ifLeft = { error -> return error.left() },
            ifRight = { it }
        )

        // Generate token pair
        val tokenPair = jwtService.generateTokenPair(
            userId = createdUser.id.toString(),
            username = createdUser.username,
            role = createdUser.role.name
        )

        // Store refresh token hash
        val tokenHash = hashToken(tokenPair.refreshToken)
        val expiresAt = now().plus(config.refreshTokenExpiry.seconds)

        refreshTokenRepository.storeToken(
            userId = createdUser.id.toString(),
            tokenHash = tokenHash,
            expiresAt = expiresAt
        ).fold(
            ifLeft = { error -> return error.left() },
            ifRight = { /* token stored */ }
        )

        return TokenResponse(
            accessToken = tokenPair.accessToken,
            refreshToken = tokenPair.refreshToken,
            expiresIn = tokenPair.expiresIn,
            tokenType = "Bearer"
        ).right()
    }

    /**
     * Refresh access token using a valid refresh token.
     *
     * @param request Refresh token request
     * @return [Either] containing [TokenResponse] with new access token, or error on failure
     */
    suspend fun refresh(request: RefreshTokenRequest): Either<AltairError, TokenResponse> {
        // Validate JWT structure and signature
        val userId = jwtService.validateRefreshToken(request.refreshToken)
            ?: return AltairError.AuthError.TokenExpired.left()

        // Hash the token to look up in database
        val tokenHash = hashToken(request.refreshToken)

        // Find token in database
        val refreshToken = refreshTokenRepository.findByTokenHash(tokenHash).fold(
            ifLeft = { error -> return error.left() },
            ifRight = { it ?: return AltairError.AuthError.TokenExpired.left() }
        )

        // Check if token is valid
        if (!refreshToken.isValid(now())) {
            return AltairError.AuthError.TokenExpired.left()
        }

        // Verify user ID matches
        if (refreshToken.userId.toString() != userId) {
            return AltairError.AuthError.Unauthorized.left()
        }

        // Get user details
        val user = userRepository.findById(userId).fold(
            ifLeft = { error -> return error.left() },
            ifRight = { it ?: return AltairError.NotFoundError.UserNotFound(userId).left() }
        )

        // Check account status
        if (user.status != UserStatus.ACTIVE) {
            return AltairError.AuthError.AccountDisabled("Account status: ${user.status}").left()
        }

        // Generate new access token (reuse existing refresh token)
        val accessToken = jwtService.generateAccessToken(
            userId = user.id.toString(),
            username = user.username,
            role = user.role.name
        )

        return TokenResponse(
            accessToken = accessToken,
            refreshToken = request.refreshToken,
            expiresIn = config.accessTokenExpiry,
            tokenType = "Bearer"
        ).right()
    }

    /**
     * Logout user by revoking refresh token.
     *
     * @param userId User identifier from JWT
     * @param refreshToken Refresh token to revoke
     * @return [Either] containing Unit on success, or error on failure
     */
    suspend fun logout(userId: String, refreshToken: String): Either<AltairError, Unit> {
        val tokenHash = hashToken(refreshToken)

        return refreshTokenRepository.revokeToken(tokenHash).fold(
            ifLeft = { error -> error.left() },
            ifRight = { Unit.right() }
        )
    }

    /**
     * Change user password.
     *
     * Verifies current password, updates to new password hash, and revokes all refresh tokens.
     *
     * @param userId User identifier from JWT
     * @param request Password change request
     * @return [Either] containing Unit on success, or error on failure
     */
    suspend fun changePassword(userId: String, request: ChangePasswordRequest): Either<AltairError, Unit> {
        // Validate new password
        if (request.newPassword.isBlank()) {
            return AltairError.ValidationError.FieldRequired("newPassword").left()
        }
        if (request.newPassword.length < 8) {
            return AltairError.ValidationError.FieldInvalid("newPassword", "Must be at least 8 characters").left()
        }

        // Get user
        val user = userRepository.findById(userId).fold(
            ifLeft = { error -> return error.left() },
            ifRight = { it ?: return AltairError.NotFoundError.UserNotFound(userId).left() }
        )

        // Verify current password
        if (!passwordHasher.verify(request.currentPassword, user.passwordHash)) {
            return AltairError.AuthError.InvalidCredentials.left()
        }

        // Hash new password
        val newPasswordHash = passwordHasher.hash(request.newPassword)

        // Update password
        userRepository.updatePassword(userId, newPasswordHash).fold(
            ifLeft = { error -> return error.left() },
            ifRight = { /* updated */ }
        )

        // Revoke all refresh tokens for security
        refreshTokenRepository.revokeAllForUser(userId).fold(
            ifLeft = { /* ignore error - not critical */ },
            ifRight = { /* tokens revoked */ }
        )

        return Unit.right()
    }

    /**
     * Get user information.
     *
     * @param userId User identifier from JWT
     * @return [Either] containing [UserResponse], or error on failure
     */
    suspend fun getUser(userId: String): Either<AltairError, UserResponse> {
        val user = userRepository.findById(userId).fold(
            ifLeft = { error -> return error.left() },
            ifRight = { it ?: return AltairError.NotFoundError.UserNotFound(userId).left() }
        )

        return UserResponse(
            id = user.id.toString(),
            username = user.username,
            email = user.email,
            role = user.role,
            status = user.status,
            storageUsed = user.storageUsed,
            storageQuota = user.storageQuota,
            createdAt = user.createdAt.toString()
        ).right()
    }

    /**
     * Hash a token using SHA-256.
     *
     * @param token The token to hash
     * @return The hex-encoded hash
     */
    private fun hashToken(token: String): String =
        MessageDigest.getInstance("SHA-256")
            .digest(token.toByteArray())
            .fold("") { str, it -> str + "%02x".format(it) }
}
