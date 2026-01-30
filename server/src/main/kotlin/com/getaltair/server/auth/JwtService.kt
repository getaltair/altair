package com.getaltair.server.auth

import com.auth0.jwt.JWT
import com.auth0.jwt.algorithms.Algorithm
import com.auth0.jwt.exceptions.JWTVerificationException
import java.time.Instant
import java.util.Date

/**
 * Service for generating and validating JWT tokens.
 *
 * Provides two types of tokens:
 * - Access tokens: Short-lived (15 min), include user claims for authorization
 * - Refresh tokens: Long-lived (7 days), used only to obtain new access tokens
 */
interface JwtService {
    /**
     * Generates a JWT access token for an authenticated user.
     *
     * @param userId The user's unique identifier (ULID)
     * @param username The user's username
     * @param role The user's role (e.g., "user", "admin")
     * @return A signed JWT token string
     */
    fun generateAccessToken(userId: String, username: String, role: String): String

    /**
     * Generates a JWT refresh token for maintaining long-term sessions.
     *
     * @param userId The user's unique identifier (ULID)
     * @return A signed JWT token string
     */
    fun generateRefreshToken(userId: String): String

    /**
     * Generates both access and refresh tokens atomically.
     *
     * @param userId The user's unique identifier (ULID)
     * @param username The user's username
     * @param role The user's role (e.g., "user", "admin")
     * @return TokenPairResult containing both tokens and expiry information
     */
    fun generateTokenPair(userId: String, username: String, role: String): TokenPairResult

    /**
     * Validates a refresh token and extracts the user ID.
     *
     * @param token The refresh token to validate
     * @return The userId if the token is valid, null if invalid or expired
     */
    fun validateRefreshToken(token: String): String?
}

/**
 * Result of generating a token pair.
 *
 * @property accessToken The short-lived access token
 * @property refreshToken The long-lived refresh token
 * @property expiresIn Time in seconds until the access token expires
 */
data class TokenPairResult(
    val accessToken: String,
    val refreshToken: String,
    val expiresIn: Long
)

/**
 * Default implementation of JwtService using HMAC256 algorithm.
 *
 * @param config Authentication configuration containing JWT settings
 */
class DefaultJwtService(private val config: AuthConfig) : JwtService {

    private val algorithm = Algorithm.HMAC256(config.jwtSecret)

    override fun generateAccessToken(userId: String, username: String, role: String): String {
        val now = Instant.now()
        val expiresAt = now.plusSeconds(config.accessTokenExpiry)

        return JWT.create()
            .withAudience(config.jwtAudience)
            .withIssuer(config.jwtIssuer)
            .withSubject(userId)
            .withClaim("username", username)
            .withClaim("role", role)
            .withIssuedAt(Date.from(now))
            .withExpiresAt(Date.from(expiresAt))
            .sign(algorithm)
    }

    override fun generateRefreshToken(userId: String): String {
        val now = Instant.now()
        val expiresAt = now.plusSeconds(config.refreshTokenExpiry)

        return JWT.create()
            .withAudience(config.jwtAudience)
            .withIssuer(config.jwtIssuer)
            .withSubject(userId)
            .withIssuedAt(Date.from(now))
            .withExpiresAt(Date.from(expiresAt))
            .sign(algorithm)
    }

    override fun generateTokenPair(userId: String, username: String, role: String): TokenPairResult {
        return TokenPairResult(
            accessToken = generateAccessToken(userId, username, role),
            refreshToken = generateRefreshToken(userId),
            expiresIn = config.accessTokenExpiry
        )
    }

    override fun validateRefreshToken(token: String): String? {
        return try {
            val verifier = JWT
                .require(algorithm)
                .withIssuer(config.jwtIssuer)
                .withAudience(config.jwtAudience)
                .build()

            val decodedJWT = verifier.verify(token)
            decodedJWT.subject
        } catch (e: JWTVerificationException) {
            null
        }
    }
}
