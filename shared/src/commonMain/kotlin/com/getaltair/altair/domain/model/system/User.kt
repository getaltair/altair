package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * An authenticated person in the Altair system.
 *
 * Users own all their data; every other entity references a userId for multi-user isolation.
 * Storage tracking fields are used for quota enforcement on self-hosted servers.
 */
@Serializable
data class User(
    val id: Ulid,
    val email: String,
    val displayName: String,
    val role: UserRole,
    val status: UserStatus,
    val storageUsedBytes: Long,
    val storageQuotaBytes: Long,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped, SoftDeletable {
    val storageRemainingBytes: Long get() = (storageQuotaBytes - storageUsedBytes).coerceAtLeast(0)
    val isOverQuota: Boolean get() = storageUsedBytes > storageQuotaBytes

    init {
        require(email.isNotBlank()) { "User email must not be blank" }
        require(email.matches(EMAIL_REGEX)) { "User email must be a valid email address" }
        require(displayName.isNotBlank()) { "User display name must not be blank" }
        require(displayName.length <= 100) { "User display name must be at most 100 characters" }
        require(storageUsedBytes >= 0) { "Storage used must be non-negative" }
        require(storageQuotaBytes >= 0) { "Storage quota must be non-negative" }
    }

    companion object {
        /** Basic email validation regex: local@domain.tld */
        private val EMAIL_REGEX = Regex("^[^@\\s]+@[^@\\s]+\\.[^@\\s]+$")
    }
}
