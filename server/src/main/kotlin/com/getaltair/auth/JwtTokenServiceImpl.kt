package com.getaltair.auth

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import com.auth0.jwt.exceptions.JWTDecodeException
import com.auth0.jwt.exceptions.JWTVerificationException
import com.auth0.jwt.exceptions.TokenExpiredException
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.service.auth.JwtTokenService
import com.getaltair.altair.service.auth.TokenClaims
import com.getaltair.altair.service.auth.TokenPair
import org.slf4j.LoggerFactory
import java.security.SecureRandom
import java.util.Base64
import java.util.Date

/**
 * JWT implementation using Auth0 java-jwt library with HMAC-SHA256.
 */
class JwtTokenServiceImpl(
    private val config: JwtConfig,
) : JwtTokenService {
    private val logger = LoggerFactory.getLogger(JwtTokenServiceImpl::class.java)
    private val algorithm = Algorithm.HMAC256(config.secret)
    private val verifier =
        JWT
            .require(algorithm)
            .withIssuer(config.issuer)
            .withAudience(config.audience)
            .build()
    private val secureRandom = SecureRandom()

    override fun generateTokens(
        userId: Ulid,
        email: String,
        role: String,
    ): TokenPair {
        val (accessToken, expiresIn) = generateAccessToken(userId, email, role)
        val refreshToken = generateRefreshToken()

        return TokenPair(
            accessToken = accessToken,
            refreshToken = refreshToken,
            accessTokenExpiresIn = expiresIn,
        )
    }

    override fun generateAccessToken(
        userId: Ulid,
        email: String,
        role: String,
    ): Pair<String, Long> {
        val now = Date()
        val expiresAt = Date(now.time + config.accessTokenExpiration.inWholeMilliseconds)
        val expiresInSeconds = config.accessTokenExpiration.inWholeSeconds
        val jti = generateJti()

        val token =
            JWT
                .create()
                .withIssuer(config.issuer)
                .withAudience(config.audience)
                .withSubject(userId.value)
                .withJWTId(jti)
                .withClaim(CLAIM_EMAIL, email)
                .withClaim(CLAIM_ROLE, role)
                .withIssuedAt(now)
                .withExpiresAt(expiresAt)
                .sign(algorithm)

        return token to expiresInSeconds
    }

    private fun generateJti(): String {
        val bytes = ByteArray(JTI_BYTES)
        secureRandom.nextBytes(bytes)
        return Base64.getUrlEncoder().withoutPadding().encodeToString(bytes)
    }

    @Suppress("ReturnCount") // Multiple validation checks require early returns
    override fun validateAccessToken(token: String): Either<AuthError, TokenClaims> {
        return try {
            val decoded = verifier.verify(token)

            val userId =
                decoded.subject
                    ?: return AuthError
                        .TokenInvalid("Missing subject claim")
                        .also {
                            logger.warn("JWT validation failed: missing subject claim")
                        }.left()

            val email =
                decoded.getClaim(CLAIM_EMAIL).asString()
                    ?: return AuthError
                        .TokenInvalid("Missing email claim")
                        .also {
                            logger.warn("JWT validation failed: missing email claim")
                        }.left()

            val role =
                decoded.getClaim(CLAIM_ROLE).asString()
                    ?: return AuthError
                        .TokenInvalid("Missing role claim")
                        .also {
                            logger.warn("JWT validation failed: missing role claim")
                        }.left()

            TokenClaims(
                userId = Ulid(userId),
                email = email,
                role = role,
            ).right()
        } catch (e: TokenExpiredException) {
            logger.debug("JWT expired: ${e.message}")
            AuthError.TokenExpired(e.expiredOn?.toEpochMilli() ?: 0L).left()
        } catch (e: JWTDecodeException) {
            logger.warn("JWT decode failed: ${e.message}")
            AuthError.TokenInvalid("Token malformed").left()
        } catch (e: JWTVerificationException) {
            logger.warn("JWT verification failed: ${e.message}")
            AuthError.TokenInvalid("Verification failed").left()
        }
    }

    override fun generateRefreshToken(): String {
        val bytes = ByteArray(REFRESH_TOKEN_BYTES)
        secureRandom.nextBytes(bytes)
        return Base64.getUrlEncoder().withoutPadding().encodeToString(bytes)
    }

    companion object {
        private const val CLAIM_EMAIL = "email"
        private const val CLAIM_ROLE = "role"
        private const val REFRESH_TOKEN_BYTES = 32
        private const val JTI_BYTES = 16
    }
}
