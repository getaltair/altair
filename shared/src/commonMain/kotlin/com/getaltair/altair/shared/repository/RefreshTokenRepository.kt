package com.getaltair.altair.shared.repository

import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.system.RefreshToken
import kotlinx.datetime.Instant

/**
 * Repository interface for RefreshToken operations.
 *
 * Manages refresh tokens for JWT-based authentication. Refresh tokens
 * allow users to obtain new access tokens without re-entering credentials.
 *
 * ## Security Considerations
 *
 * - Store tokens hashed (SHA-256), never in plaintext
 * - Expired tokens should be periodically cleaned up
 * - Revoked tokens cannot be used for token refresh
 * - All tokens for a user should be revoked on password change
 *
 * ## Token Lifecycle
 *
 * 1. [storeToken] - Create on successful login
 * 2. [findByTokenHash] - Validate during token refresh
 * 3. [revokeToken] - Revoke on logout
 * 4. [revokeAllForUser] - Revoke all on password change/security event
 * 5. [deleteExpired] - Periodic cleanup of expired tokens
 *
 * ## Error Handling
 *
 * All methods return [AltairResult] with typed errors:
 *
 * - [AltairError.NotFoundError] - Token not found
 * - [AltairError.ValidationError] - Invalid token data
 * - [AltairError.StorageError] - Database operation failed
 *
 * ## Example Usage
 *
 * ```kotlin
 * // Store a new refresh token
 * val tokenHash = hashToken(rawToken)
 * refreshTokenRepository.storeToken(
 *     userId = userId,
 *     tokenHash = tokenHash,
 *     expiresAt = now.plus(30.days)
 * )
 *
 * // Validate and use a refresh token
 * refreshTokenRepository.findByTokenHash(tokenHash).fold(
 *     ifLeft = { error -> /* invalid token */ },
 *     ifRight = { token ->
 *         if (token == null || !token.isValid()) {
 *             // Invalid or revoked
 *         } else {
 *             // Generate new access token
 *         }
 *     }
 * )
 *
 * // Revoke all tokens on password change
 * refreshTokenRepository.revokeAllForUser(userId)
 * ```
 */
interface RefreshTokenRepository {
    /**
     * Store a new refresh token.
     *
     * Creates a new refresh token record in the database. The token should
     * already be hashed (SHA-256) before calling this method.
     *
     * @param userId User who owns this token
     * @param tokenHash SHA-256 hash of the token (never store plaintext)
     * @param expiresAt When this token expires
     * @return [AltairResult] containing the created [RefreshToken]
     */
    suspend fun storeToken(
        userId: String,
        tokenHash: String,
        expiresAt: Instant
    ): AltairResult<RefreshToken>

    /**
     * Find a refresh token by its hash.
     *
     * Used during token refresh to validate and retrieve the token.
     * Returns null if the token doesn't exist.
     *
     * @param tokenHash SHA-256 hash of the token to find
     * @return [AltairResult] containing the [RefreshToken] if found, null otherwise
     */
    suspend fun findByTokenHash(tokenHash: String): AltairResult<RefreshToken?>

    /**
     * Revoke a specific refresh token.
     *
     * Sets the revokedAt timestamp, preventing the token from being used.
     * Called during logout or when a token is explicitly invalidated.
     *
     * @param tokenHash SHA-256 hash of the token to revoke
     * @return [AltairResult] containing Unit on success
     */
    suspend fun revokeToken(tokenHash: String): AltairResult<Unit>

    /**
     * Revoke all refresh tokens for a user.
     *
     * Sets the revokedAt timestamp on all tokens belonging to the user.
     * Called during password change, account compromise, or security events.
     *
     * @param userId User identifier
     * @return [AltairResult] containing the number of tokens revoked
     */
    suspend fun revokeAllForUser(userId: String): AltairResult<Int>

    /**
     * Delete all expired refresh tokens.
     *
     * Permanently removes tokens that have passed their expiration date.
     * Should be called periodically (e.g., daily cron job) to clean up
     * expired tokens and prevent database bloat.
     *
     * @return [AltairResult] containing the number of tokens deleted
     */
    suspend fun deleteExpired(): AltairResult<Int>
}
