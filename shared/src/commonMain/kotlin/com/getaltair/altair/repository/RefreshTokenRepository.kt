package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.model.system.RefreshToken
import com.getaltair.altair.domain.types.Ulid

/**
 * Repository for managing refresh tokens.
 *
 * Refresh tokens are used for session management and token rotation.
 * The repository stores SHA-256 hashes of tokens, not the raw tokens.
 */
interface RefreshTokenRepository {
    /**
     * Store a new refresh token.
     *
     * @param token The refresh token to store
     * @return Either an error or the stored token
     */
    suspend fun create(token: RefreshToken): Either<AuthError, RefreshToken>

    /**
     * Find a refresh token by its hash.
     *
     * @param tokenHash The SHA-256 hash of the token
     * @return Either [AuthError.TokenInvalid] if not found, or the token
     */
    suspend fun findByHash(tokenHash: String): Either<AuthError, RefreshToken>

    /**
     * Revoke a refresh token.
     *
     * @param id The token's ULID
     * @param userId The user's ULID (required for data isolation)
     * @return Either an error or Unit on success
     */
    suspend fun revoke(id: Ulid, userId: Ulid): Either<AuthError, Unit>

    /**
     * Revoke all refresh tokens for a user (logout all sessions).
     *
     * @param userId The user's ULID
     * @return Either an error or the count of revoked tokens
     */
    suspend fun revokeAllForUser(userId: Ulid): Either<AuthError, Int>

    /**
     * Delete expired tokens (cleanup job).
     *
     * @return Either an error or the count of deleted tokens
     */
    suspend fun deleteExpired(): Either<AuthError, Int>
}
