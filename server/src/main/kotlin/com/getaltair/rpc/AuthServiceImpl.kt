package com.getaltair.rpc

import com.getaltair.altair.domain.model.system.InviteCode
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.dto.auth.ChangePasswordRequest
import com.getaltair.altair.dto.auth.InviteCodeResponse
import com.getaltair.altair.dto.auth.SuccessResponse
import com.getaltair.altair.repository.InviteCodeRepository
import com.getaltair.altair.repository.RefreshTokenRepository
import com.getaltair.altair.repository.UserRepository
import com.getaltair.altair.rpc.AuthService
import com.getaltair.altair.service.auth.JwtTokenService
import com.getaltair.altair.service.auth.PasswordService
import com.getaltair.auth.JwtConfig
import kotlinx.datetime.Instant
import org.slf4j.LoggerFactory
import java.security.SecureRandom
import kotlin.time.Clock
import kotlin.time.Duration.Companion.days

/**
 * Authenticated auth service implementation.
 *
 * Handles operations that require a valid user session:
 * logout, password change, invite code generation, session management.
 *
 * Note: Due to kotlinx-rpc limitations, per-request AuthContext is not currently
 * available in RPC services. Methods that require user context will return
 * appropriate error responses until this limitation is resolved.
 *
 * The Ktor authenticate block ensures that only authenticated users can connect
 * to this service, but individual user identity is not accessible within method calls.
 */
class AuthServiceImpl(
    userRepository: UserRepository,
    refreshTokenRepository: RefreshTokenRepository,
    private val inviteCodeRepository: InviteCodeRepository,
    passwordService: PasswordService,
    jwtTokenService: JwtTokenService,
    jwtConfig: JwtConfig,
) : AuthService {
    // Note: These dependencies are kept for API compatibility but are not currently used
    // due to kotlinx-rpc limitations with AuthContext. They will be needed once AuthContext
    // support is available.
    init {
        // Validate dependencies are provided (even if not currently used)
        requireNotNull(userRepository)
        requireNotNull(refreshTokenRepository)
        requireNotNull(passwordService)
        requireNotNull(jwtTokenService)
        requireNotNull(jwtConfig)
    }

    private val logger = LoggerFactory.getLogger(AuthServiceImpl::class.java)

    override suspend fun logout(): SuccessResponse {
        // Note: Without per-request AuthContext, we can't identify which user to log out.
        // The client should discard tokens locally. Server-side token revocation
        // would require passing the refresh token to this method.
        logger.debug("Logout called - client should discard tokens locally")

        return SuccessResponse(
            success = true,
            message = "Please discard tokens locally. Server-side session revocation requires token identification.",
        )
    }

    override suspend fun generateInviteCode(): InviteCodeResponse {
        // Note: Without AuthContext, we can't verify admin status or track who created the code.
        // For now, generate the code but mark creator as unknown.
        // In production, this should verify admin role from JWT context.
        logger.warn("Generating invite code without admin verification - AuthContext not available")

        val code = generateSecureCode()
        val now = currentInstant()
        val expiresAt =
            Instant.fromEpochMilliseconds(
                now.toEpochMilliseconds() + INVITE_CODE_EXPIRY.inWholeMilliseconds,
            )

        val inviteCode =
            InviteCode(
                id = Ulid.generate(),
                code = code,
                createdBy = Ulid.generate(), // Unknown creator - should be from AuthContext
                expiresAt = expiresAt,
                createdAt = now,
            )

        inviteCodeRepository
            .create(inviteCode)
            .fold(
                ifLeft = {
                    logger.error("Failed to create invite code")
                    error("Failed to generate invite code")
                },
                ifRight = { logger.info("Invite code generated: {}", code) },
            )

        return InviteCodeResponse(
            code = code,
            expiresAt = expiresAt.toString(),
        )
    }

    override suspend fun changePassword(request: ChangePasswordRequest): SuccessResponse =
        // Without AuthContext, we can't identify the user whose password to change.
        createAuthContextRequiredResponse("changePassword")

    override suspend fun revokeAllSessions(): SuccessResponse =
        // Without AuthContext, we can't identify which user's sessions to revoke.
        createAuthContextRequiredResponse("revokeAllSessions")

    private fun createAuthContextRequiredResponse(operation: String): SuccessResponse {
        logger.warn("$operation called without AuthContext - operation cannot be completed")
        return SuccessResponse(
            success = false,
            message =
                "$operation requires user identification. " +
                    "This feature requires AuthContext support in kotlinx-rpc.",
        )
    }

    private fun currentInstant(): Instant = Instant.fromEpochMilliseconds(Clock.System.now().toEpochMilliseconds())

    private fun generateSecureCode(): String {
        val chars = "ABCDEFGHJKLMNPQRSTUVWXYZ23456789" // Exclude ambiguous characters
        val secureRandom = SecureRandom()
        return (1..INVITE_CODE_LENGTH)
            .map { chars[secureRandom.nextInt(chars.length)] }
            .joinToString("")
    }

    companion object {
        private const val INVITE_CODE_LENGTH = 8
        private val INVITE_CODE_EXPIRY = 7.days
    }
}
