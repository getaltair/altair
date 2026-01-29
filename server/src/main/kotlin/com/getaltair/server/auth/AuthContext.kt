package com.getaltair.server.auth

import com.getaltair.altair.shared.domain.common.Ulid

/**
 * Provides authenticated user context for repository operations.
 *
 * All repository methods use this to enforce user-scoped data isolation.
 * The implementation is responsible for extracting the current user ID
 * from the authentication context (e.g., JWT token in Phase 5).
 */
interface AuthContext {
    /**
     * The ULID of the currently authenticated user.
     * All database queries are scoped to this user.
     */
    val currentUserId: Ulid

    /**
     * Whether the current context has a valid authenticated user.
     */
    val isAuthenticated: Boolean
}

/**
 * Static implementation of AuthContext for Phase 3 testing.
 *
 * This placeholder allows repository testing without the full
 * authentication system (implemented in Phase 5). In production,
 * this will be replaced with JwtAuthContext that extracts the
 * user ID from the JWT token in the request.
 *
 * @param currentUserId The fixed user ID to use for all operations
 */
class StaticAuthContext(override val currentUserId: Ulid) : AuthContext {
    override val isAuthenticated: Boolean = true
}
