package com.getaltair.auth

import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole

/**
 * Authentication context for the current request.
 *
 * Provides access to the authenticated user's information extracted
 * from their JWT token during request processing.
 */
interface AuthContext {
    /**
     * The authenticated user's ULID.
     */
    val userId: Ulid

    /**
     * The authenticated user's email address.
     */
    val email: String

    /**
     * The authenticated user's role.
     */
    val role: UserRole

    /**
     * Check if the current user has admin privileges.
     */
    val isAdmin: Boolean
        get() = role == UserRole.ADMIN
}

/**
 * Implementation of AuthContext for Ktor requests.
 */
data class RequestAuthContext(
    override val userId: Ulid,
    override val email: String,
    override val role: UserRole,
) : AuthContext

/**
 * Test implementation of AuthContext for unit testing.
 */
data class TestAuthContext(
    override val userId: Ulid = Ulid.generate(),
    override val email: String = "test@example.com",
    override val role: UserRole = UserRole.MEMBER,
) : AuthContext
