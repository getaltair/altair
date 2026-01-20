package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.types.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable
import kotlin.time.Clock
import kotlin.time.Duration
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.minutes

/**
 * A refresh token for session management.
 *
 * Refresh tokens are long-lived tokens used to obtain new access tokens
 * without requiring the user to re-authenticate. They are stored in the
 * database with their SHA-256 hash (not the raw token) for security.
 *
 * Token rotation: Each time a refresh token is used, it is revoked and
 * a new one is issued. This limits the window of opportunity for token theft.
 *
 * Temporal invariants:
 * - `expiresAt` must be after `createdAt`
 * - `createdAt` must not be in the far future (allows up to [MAX_CLOCK_SKEW] tolerance)
 * - `expiresAt` must be within [MAX_EXPIRY_DURATION] of `createdAt`
 */
@Serializable
data class RefreshToken(
    val id: Ulid,
    val userId: Ulid,
    val tokenHash: String,
    val deviceName: String?,
    val expiresAt: Instant,
    val createdAt: Instant,
    val revokedAt: Instant? = null,
) {
    val isExpired: Boolean
        get() = Instant.fromEpochMilliseconds(Clock.System.now().toEpochMilliseconds()) > expiresAt

    val isRevoked: Boolean
        get() = revokedAt != null

    val isValid: Boolean
        get() = !isExpired && !isRevoked

    init {
        require(tokenHash.isNotBlank()) { "Token hash must not be blank" }
        require(expiresAt > createdAt) { "expiresAt must be after createdAt" }
        val now = Clock.System.now()
        require(createdAt <= now + MAX_CLOCK_SKEW) {
            "createdAt must not be in the far future (max $MAX_CLOCK_SKEW tolerance)"
        }
        require(expiresAt <= createdAt + MAX_EXPIRY_DURATION) {
            "expiresAt must be within $MAX_EXPIRY_DURATION of createdAt"
        }

        // Cross-field validation for revocation state
        if (revokedAt != null) {
            require(revokedAt >= createdAt) { "revokedAt must be after or equal to createdAt" }
        }
    }

    companion object {
        /** Maximum allowed clock skew for createdAt timestamps (5 minutes). */
        val MAX_CLOCK_SKEW: Duration = 5.minutes

        /** Maximum allowed expiry duration (90 days). */
        val MAX_EXPIRY_DURATION: Duration = 90.days

        /** Default expiry duration for new refresh tokens (30 days). */
        val DEFAULT_EXPIRY_DURATION: Duration = 30.days

        /**
         * Creates a new refresh token with duration-based expiration.
         *
         * @param id Unique identifier for the token
         * @param userId User ID this token belongs to
         * @param tokenHash SHA-256 hash of the raw token value
         * @param deviceName Optional device name for identification
         * @param expiresIn Duration until expiration (default: 30 days)
         * @param clock Clock to use for determining current time (default: system clock)
         * @return A new RefreshToken instance
         */
        @Suppress("LongParameterList")
        fun create(
            id: Ulid,
            userId: Ulid,
            tokenHash: String,
            deviceName: String? = null,
            expiresIn: Duration = DEFAULT_EXPIRY_DURATION,
            clock: Clock = Clock.System,
        ): RefreshToken {
            val now = clock.now()
            return RefreshToken(
                id = id,
                userId = userId,
                tokenHash = tokenHash,
                deviceName = deviceName,
                expiresAt = now + expiresIn,
                createdAt = now,
            )
        }
    }
}
