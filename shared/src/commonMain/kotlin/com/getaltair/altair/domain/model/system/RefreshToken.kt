package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.types.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable
import kotlin.time.Clock

/**
 * A refresh token for session management.
 *
 * Refresh tokens are long-lived tokens used to obtain new access tokens
 * without requiring the user to re-authenticate. They are stored in the
 * database with their SHA-256 hash (not the raw token) for security.
 *
 * Token rotation: Each time a refresh token is used, it is revoked and
 * a new one is issued. This limits the window of opportunity for token theft.
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
    }
}
