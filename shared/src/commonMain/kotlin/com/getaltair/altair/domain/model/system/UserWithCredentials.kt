package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import kotlinx.datetime.Instant

/**
 * Internal representation of a User with password credentials.
 *
 * This model is used only on the server for authentication operations.
 * It should NEVER be serialized or sent to clients - use [User] instead.
 *
 * The password hash is stored separately from the public User model
 * to prevent accidental exposure in API responses or logs.
 */
data class UserWithCredentials(
    val user: User,
    val passwordHash: String,
) {
    val id: Ulid get() = user.id
    val email: String get() = user.email
    val displayName: String get() = user.displayName
    val role: UserRole get() = user.role
    val status: UserStatus get() = user.status
    val createdAt: Instant get() = user.createdAt
    val updatedAt: Instant get() = user.updatedAt

    init {
        require(passwordHash.isNotBlank()) { "Password hash must not be blank" }
    }
}
