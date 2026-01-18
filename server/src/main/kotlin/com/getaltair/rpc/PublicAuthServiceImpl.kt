package com.getaltair.rpc

import com.getaltair.altair.domain.model.system.RefreshToken
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import com.getaltair.altair.repository.InviteCodeRepository
import com.getaltair.altair.repository.RefreshTokenRepository
import com.getaltair.altair.repository.UserRepository
import com.getaltair.altair.rpc.PublicAuthService
import com.getaltair.altair.service.auth.JwtTokenService
import com.getaltair.altair.service.auth.PasswordService
import com.getaltair.auth.JwtConfig
import com.getaltair.auth.TokenHasher
import kotlinx.datetime.Instant
import org.slf4j.LoggerFactory
import kotlin.time.Clock

/**
 * Public authentication service implementation.
 *
 * Handles login, registration, and token refresh operations that
 * don't require an existing authenticated session.
 */
@Suppress("TooManyFunctions") // Auth service requires multiple validation helpers
class PublicAuthServiceImpl(
    private val userRepository: UserRepository,
    private val refreshTokenRepository: RefreshTokenRepository,
    private val inviteCodeRepository: InviteCodeRepository,
    private val passwordService: PasswordService,
    private val jwtTokenService: JwtTokenService,
    private val jwtConfig: JwtConfig,
) : PublicAuthService {
    private val logger = LoggerFactory.getLogger(PublicAuthServiceImpl::class.java)

    override suspend fun login(request: AuthRequest): AuthResponse {
        logger.debug("Login attempt for email: {}", request.email)

        // Find user by email with credentials
        val userWithCredentials =
            userRepository
                .findByEmailWithCredentials(request.email)
                .fold(
                    ifLeft = {
                        logger.warn("Login failed: user not found for email {}", request.email)
                        throw IllegalArgumentException("Invalid email or password")
                    },
                    ifRight = { it },
                )

        // Verify password
        if (!passwordService.verify(request.password, userWithCredentials.passwordHash)) {
            logger.warn("Login failed: invalid password for email {}", request.email)
            throw IllegalArgumentException("Invalid email or password")
        }

        // Check user status
        if (userWithCredentials.status != UserStatus.ACTIVE) {
            logger.warn("Login failed: account not active for email {}", request.email)
            throw IllegalArgumentException("Account is not active")
        }

        // Generate tokens
        val user = userWithCredentials.user
        val tokenPair =
            jwtTokenService.generateTokens(
                userId = user.id,
                email = user.email,
                role = user.role.name.lowercase(),
            )

        // Store refresh token
        storeRefreshToken(user.id, tokenPair.refreshToken, null)

        logger.info("Login successful for user: {}", user.id.value)

        return AuthResponse(
            accessToken = tokenPair.accessToken,
            refreshToken = tokenPair.refreshToken,
            expiresIn = tokenPair.accessTokenExpiresIn,
            userId = user.id.value,
            displayName = user.displayName,
            role = user.role.name.lowercase(),
        )
    }

    override suspend fun refresh(refreshToken: String): TokenRefreshResponse {
        logger.debug("Token refresh attempt")

        // Find and validate refresh token
        val tokenHash = TokenHasher.hash(refreshToken)
        val storedToken =
            refreshTokenRepository
                .findByHash(tokenHash)
                .fold(
                    ifLeft = {
                        logger.warn("Token refresh failed: token not found")
                        throw IllegalArgumentException("Invalid refresh token")
                    },
                    ifRight = { it },
                )

        // Check if token is valid
        if (!storedToken.isValid) {
            logger.warn("Token refresh failed: token expired or revoked")
            throw IllegalArgumentException("Refresh token is expired or revoked")
        }

        // Revoke the old refresh token (rotation)
        refreshTokenRepository.revoke(storedToken.id, storedToken.userId).fold(
            ifLeft = { error ->
                logger.warn("Failed to revoke old refresh token {}: {}", storedToken.id.value, error)
                // Continue with refresh - security is degraded but user shouldn't be blocked
            },
            ifRight = { logger.debug("Old refresh token {} revoked", storedToken.id.value) },
        )

        // Get user info for new access token
        val user =
            userRepository
                .findById(storedToken.userId)
                .fold(
                    ifLeft = {
                        logger.error("Token refresh failed: user not found for valid token")
                        throw IllegalArgumentException("User not found")
                    },
                    ifRight = { it },
                )

        // Generate new tokens (rotation: new access + new refresh token)
        val tokenPair =
            jwtTokenService.generateTokens(
                userId = user.id,
                email = user.email,
                role = user.role.name.lowercase(),
            )

        // Store new refresh token
        storeRefreshToken(user.id, tokenPair.refreshToken, storedToken.deviceName)

        logger.debug("Token refresh successful for user: {}", user.id.value)

        return TokenRefreshResponse(
            accessToken = tokenPair.accessToken,
            refreshToken = tokenPair.refreshToken,
            expiresIn = tokenPair.accessTokenExpiresIn,
        )
    }

    override suspend fun register(request: RegisterRequest): AuthResponse {
        logger.debug("Registration attempt for email: {}", request.email)

        validatePassword(request.password)
        val isFirstUser = checkIsFirstUser()

        validateInviteCodeIfRequired(isFirstUser, request.inviteCode)
        validateEmailAvailable(request.email)

        val userId = Ulid.generate()
        val role = if (isFirstUser) UserRole.ADMIN else UserRole.MEMBER

        createUser(request, userId, role)
        markInviteCodeUsedIfApplicable(isFirstUser, request.inviteCode, userId)

        return generateAndStoreTokens(userId, request, role, isFirstUser)
    }

    private suspend fun checkIsFirstUser(): Boolean =
        userRepository
            .countActive()
            .fold(
                ifLeft = { error ->
                    logger.warn("Failed to check user count, assuming not first user: {}", error)
                    false
                },
                ifRight = { it == 0 },
            )

    @Suppress("ThrowsCount") // Validation function with multiple failure conditions
    private suspend fun validateInviteCodeIfRequired(
        isFirstUser: Boolean,
        inviteCode: String?,
    ) {
        if (isFirstUser) return

        if (inviteCode.isNullOrBlank()) {
            logger.warn("Registration failed: invite code required")
            throw IllegalArgumentException("Invite code is required")
        }

        val invite =
            inviteCodeRepository.findByCode(inviteCode).fold(
                ifLeft = {
                    logger.warn("Registration failed: invalid invite code")
                    throw IllegalArgumentException("Invalid or expired invite code")
                },
                ifRight = { it },
            )

        if (!invite.isValid) {
            logger.warn("Registration failed: invite code expired or used")
            throw IllegalArgumentException("Invalid or expired invite code")
        }
    }

    private suspend fun validateEmailAvailable(email: String) {
        userRepository.isEmailAvailable(email).fold(
            ifLeft = { error ->
                logger.error("Failed to check email availability: {}", error)
                error("Unable to verify email availability. Please try again.")
            },
            ifRight = { isAvailable ->
                if (!isAvailable) {
                    logger.warn("Registration failed: email already exists")
                    throw IllegalArgumentException("Email is already registered")
                }
            },
        )
    }

    private suspend fun createUser(
        request: RegisterRequest,
        userId: Ulid,
        role: UserRole,
    ) {
        val passwordHash = passwordService.hash(request.password)
        val now = currentInstant()

        val user =
            User(
                id = userId,
                email = request.email,
                displayName = request.displayName,
                role = role,
                status = UserStatus.ACTIVE,
                storageUsedBytes = 0L,
                storageQuotaBytes = DEFAULT_STORAGE_QUOTA,
                createdAt = now,
                updatedAt = now,
            )

        userRepository.createWithPassword(user, passwordHash).fold(
            ifLeft = {
                logger.error("Registration failed: could not create user")
                throw IllegalArgumentException("Failed to create user")
            },
            ifRight = { it },
        )
    }

    private suspend fun markInviteCodeUsedIfApplicable(
        isFirstUser: Boolean,
        inviteCode: String?,
        userId: Ulid,
    ) {
        if (!isFirstUser && !inviteCode.isNullOrBlank()) {
            val invite = inviteCodeRepository.findByCode(inviteCode).getOrNull()
            if (invite != null) {
                inviteCodeRepository.markUsed(invite.id, userId).fold(
                    ifLeft = { error ->
                        logger.warn("Failed to mark invite code as used: {}", error)
                        // Continue registration - code validation already passed
                    },
                    ifRight = { logger.debug("Invite code {} marked as used by {}", inviteCode, userId.value) },
                )
            }
        }
    }

    private suspend fun generateAndStoreTokens(
        userId: Ulid,
        request: RegisterRequest,
        role: UserRole,
        isFirstUser: Boolean,
    ): AuthResponse {
        val tokenPair =
            jwtTokenService.generateTokens(
                userId = userId,
                email = request.email,
                role = role.name.lowercase(),
            )

        storeRefreshToken(userId, tokenPair.refreshToken, null)
        logger.info("Registration successful for user: {} (first user: {})", userId.value, isFirstUser)

        return AuthResponse(
            accessToken = tokenPair.accessToken,
            refreshToken = tokenPair.refreshToken,
            expiresIn = tokenPair.accessTokenExpiresIn,
            userId = userId.value,
            displayName = request.displayName,
            role = role.name.lowercase(),
        )
    }

    private suspend fun storeRefreshToken(
        userId: Ulid,
        refreshToken: String,
        deviceName: String?,
    ) {
        val tokenHash = TokenHasher.hash(refreshToken)
        val now = currentInstant()
        val expiresAt =
            Instant.fromEpochMilliseconds(
                now.toEpochMilliseconds() + jwtConfig.refreshTokenExpiration.inWholeMilliseconds,
            )

        val token =
            RefreshToken(
                id = Ulid.generate(),
                userId = userId,
                tokenHash = tokenHash,
                deviceName = deviceName,
                expiresAt = expiresAt,
                createdAt = now,
            )

        refreshTokenRepository
            .create(token)
            .fold(
                ifLeft = { error ->
                    logger.error("Failed to store refresh token for user {}: {}", userId.value, error)
                    error("Failed to establish session. Please try again.")
                },
                ifRight = { logger.debug("Refresh token stored for user: {}", userId.value) },
            )
    }

    private fun currentInstant(): Instant = Instant.fromEpochMilliseconds(Clock.System.now().toEpochMilliseconds())

    private fun validatePassword(password: String) {
        require(password.length >= MIN_PASSWORD_LENGTH) {
            "Password must be at least $MIN_PASSWORD_LENGTH characters"
        }
    }

    companion object {
        private const val MIN_PASSWORD_LENGTH = 8
        private const val DEFAULT_STORAGE_QUOTA = 10_737_418_240L // 10 GB
    }
}
