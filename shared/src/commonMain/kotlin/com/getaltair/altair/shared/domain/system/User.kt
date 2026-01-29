package com.getaltair.altair.shared.domain.system

import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.common.UserRole
import com.getaltair.altair.shared.domain.common.UserStatus
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Represents a user account in the Altair system.
 *
 * Users are the primary actors in the system, owning all domain entities and data.
 * Each user has their own isolated workspace with storage quotas and role-based permissions.
 *
 * ## Authentication
 * - Password authentication via Argon2 hashing ([passwordHash])
 * - JWT-based session tokens (managed separately)
 *
 * ## Storage Management
 * - [storageUsed]: Current bytes consumed by user's data
 * - [storageQuota]: Maximum allowed bytes (null = unlimited)
 * - Quota enforcement happens at the service layer
 *
 * ## Soft Deletion
 * - Users are soft-deleted via [deletedAt] timestamp
 * - Deleted users retain data for recovery/audit
 * - Hard deletion is a separate administrative process
 *
 * @property id Unique identifier for this user
 * @property username Unique username (max 50 characters)
 * @property email Optional email address (must contain @)
 * @property passwordHash Argon2-hashed password
 * @property role User role determining permissions
 * @property status Current operational status
 * @property storageUsed Bytes of storage currently consumed
 * @property storageQuota Maximum allowed storage bytes (null = unlimited)
 * @property createdAt When this user account was created
 * @property lastLoginAt When this user last authenticated (null if never)
 * @property deletedAt When this user was soft-deleted (null if active)
 */
@Serializable
data class User(
    val id: Ulid,
    val username: String,
    val email: String?,
    val passwordHash: String,
    val role: UserRole,
    val status: UserStatus,
    val storageUsed: Long,
    val storageQuota: Long?,
    val createdAt: Instant,
    val lastLoginAt: Instant?,
    val deletedAt: Instant?
) {
    init {
        require(username.length <= 50) { "Username max 50 chars" }
        require(username.isNotBlank()) { "Username required" }
        email?.let { require(it.contains("@")) { "Invalid email format" } }
    }

    /**
     * Whether this user has been soft-deleted.
     */
    val isDeleted: Boolean get() = deletedAt != null

    /**
     * Whether this user has administrator privileges.
     */
    val isAdmin: Boolean get() = role == UserRole.ADMIN

    /**
     * Whether this user has a storage quota limit.
     */
    val hasQuota: Boolean get() = storageQuota != null

    /**
     * Remaining storage quota in bytes.
     * Returns null if user has no quota limit.
     */
    val quotaRemaining: Long? get() = storageQuota?.let { it - storageUsed }
}
