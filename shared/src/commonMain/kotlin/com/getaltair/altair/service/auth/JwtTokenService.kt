package com.getaltair.altair.service.auth

import arrow.core.Either
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.types.Ulid

/**
 * Token pair containing both access and refresh tokens.
 */
data class TokenPair(
    val accessToken: String,
    val refreshToken: String,
    val accessTokenExpiresIn: Long,
)

/**
 * Claims extracted from a validated JWT token.
 */
data class TokenClaims(
    val userId: Ulid,
    val email: String,
    val role: String,
)

/**
 * Service for JWT token generation and validation.
 *
 * Access tokens are short-lived (15 minutes) and used for API authentication.
 * Refresh tokens are long-lived (30 days) and used to obtain new access tokens.
 */
interface JwtTokenService {
    /**
     * Generate a new access/refresh token pair for a user.
     *
     * @param userId The user's ULID
     * @param email The user's email address
     * @param role The user's role (member, admin)
     * @return A token pair containing access and refresh tokens
     */
    fun generateTokens(
        userId: Ulid,
        email: String,
        role: String,
    ): TokenPair

    /**
     * Generate only a new access token (used during refresh).
     *
     * @param userId The user's ULID
     * @param email The user's email address
     * @param role The user's role
     * @return The access token string and its expiry time in seconds
     */
    fun generateAccessToken(
        userId: Ulid,
        email: String,
        role: String,
    ): Pair<String, Long>

    /**
     * Validate an access token and extract its claims.
     *
     * @param token The JWT access token to validate
     * @return Either an error if validation fails, or the extracted claims
     */
    fun validateAccessToken(token: String): Either<AuthError, TokenClaims>

    /**
     * Generate a secure random refresh token.
     *
     * @return A cryptographically secure random token string
     */
    fun generateRefreshToken(): String
}
