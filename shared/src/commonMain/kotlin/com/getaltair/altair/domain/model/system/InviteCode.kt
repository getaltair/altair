package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.types.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable
import kotlin.time.Clock

/**
 * An invite code for controlled user registration.
 *
 * Invite codes are created by admin users and can only be used once.
 * They have an expiration date after which they become invalid.
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
    }

    companion object {
        const val MIN_CODE_LENGTH = 8
    }
}
