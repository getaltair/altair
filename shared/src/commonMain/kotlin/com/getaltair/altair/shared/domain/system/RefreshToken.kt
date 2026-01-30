package com.getaltair.altair.shared.domain.system

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Clock
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Represents a refresh token for JWT authentication.
 *
 * Refresh tokens allow users to obtain new access tokens without re-entering
 * credentials. They are stored hashed in the database for security.
 *
 * ## Security Model
 *
 * - Tokens are hashed (SHA-256) before storage
 * - Expired tokens are periodically cleaned up
 * - Revoked tokens cannot be used for token refresh
 * - Users can revoke all tokens (e.g., on password change)
 *
 * ## Lifecycle
 *
 * 1. Created on successful login
 * 2. Used to refresh access tokens
 * 3. Revoked on logout or security events
 * 4. Automatically deleted after expiration
 *
 * @property id Unique identifier for this refresh token record
 * @property userId User who owns this token
 * @property tokenHash SHA-256 hash of the token (never store plaintext)
 * @property expiresAt When this token expires
 * @property createdAt When this token was created
 * @property revokedAt When this token was revoked (null if active)
 */
@Serializable
data class RefreshToken(
    val id: Ulid,
    val userId: Ulid,
    val tokenHash: String,
    val expiresAt: Instant,
    val createdAt: Instant,
    val revokedAt: Instant?
) {
    /**
     * Whether this token has been revoked.
     */
    val isRevoked: Boolean get() = revokedAt != null

    /**
     * Whether this token is expired.
     */
    fun isExpired(now: Instant): Boolean =
        expiresAt < now

    /**
     * Whether this token is valid (not revoked and not expired).
     */
    fun isValid(now: Instant): Boolean =
        !isRevoked && !isExpired(now)
}
