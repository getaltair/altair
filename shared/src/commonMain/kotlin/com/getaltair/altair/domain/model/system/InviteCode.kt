package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.types.Ulid
import kotlin.time.Instant
import kotlinx.serialization.Serializable
import kotlin.time.Clock
import kotlin.time.Duration
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.minutes

/**
 * An invite code for controlled user registration.
 *
 * Invite codes are created by admin users and can only be used once.
 * They have an expiration date after which they become invalid.
 *
 * Temporal invariants:
 * - `expiresAt` must be after `createdAt`
 * - `createdAt` must not be in the far future (allows up to [MAX_CLOCK_SKEW] tolerance)
 * - `expiresAt` must be within [MAX_EXPIRY_DURATION] of `createdAt`
 */
@Serializable
data class InviteCode(
    val id: Ulid,
    val code: String,
    val createdBy: Ulid,
    val usedBy: Ulid? = null,
    val expiresAt: Instant,
    val createdAt: Instant,
    val usedAt: Instant? = null,
) {
    val isExpired: Boolean
        get() = Instant.fromEpochMilliseconds(Clock.System.now().toEpochMilliseconds()) > expiresAt

    val isUsed: Boolean
        get() = usedBy != null

    val isValid: Boolean
        get() = !isExpired && !isUsed

    init {
        require(code.isNotBlank()) { "Invite code must not be blank" }
        require(code.length >= MIN_CODE_LENGTH) { "Invite code must be at least $MIN_CODE_LENGTH characters" }
        require(expiresAt > createdAt) { "expiresAt must be after createdAt" }
        val now = Clock.System.now()
        require(createdAt <= now + MAX_CLOCK_SKEW) {
            "createdAt must not be in the far future (max $MAX_CLOCK_SKEW tolerance)"
        }
        require(expiresAt <= createdAt + MAX_EXPIRY_DURATION) {
            "expiresAt must be within $MAX_EXPIRY_DURATION of createdAt"
        }

        // Cross-field validation for usage state
        if (usedBy != null) {
            require(usedAt != null) { "usedAt must be set when usedBy is set" }
            require(usedAt >= createdAt) { "usedAt must be after or equal to createdAt" }
        }
        if (usedAt != null) {
            require(usedBy != null) { "usedBy must be set when usedAt is set" }
        }
    }

    companion object {
        const val MIN_CODE_LENGTH = 8

        /** Maximum allowed clock skew for createdAt timestamps (5 minutes). */
        val MAX_CLOCK_SKEW: Duration = 5.minutes

        /** Maximum allowed expiry duration (90 days). */
        val MAX_EXPIRY_DURATION: Duration = 90.days

        /** Default expiry duration for new invite codes (7 days). */
        val DEFAULT_EXPIRY_DURATION: Duration = 7.days

        /**
         * Creates a new invite code with duration-based expiration.
         *
         * @param id Unique identifier for the invite code
         * @param code The invite code string
         * @param createdBy User ID of the admin who created this code
         * @param expiresIn Duration until expiration (default: 7 days)
         * @param clock Clock to use for determining current time (default: system clock)
         * @return A new InviteCode instance
         */
        fun create(
            id: Ulid,
            code: String,
            createdBy: Ulid,
            expiresIn: Duration = DEFAULT_EXPIRY_DURATION,
            clock: Clock = Clock.System,
        ): InviteCode {
            val now = clock.now()
            return InviteCode(
                id = id,
                code = code,
                createdBy = createdBy,
                expiresAt = now + expiresIn,
                createdAt = now,
            )
        }
    }
}
