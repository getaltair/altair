package com.getaltair.altair.shared.repository

import arrow.core.Either
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.system.User

/**
 * Repository interface for User entity operations.
 *
 * Unlike other repositories in the system, User operations are NOT user-scoped
 * since this handles authentication before a user context is established.
 * User repositories operate at the system level and require special care
 * to avoid unintended data access.
 *
 * ## Authentication Context
 *
 * Most repository operations in Altair are automatically scoped to the
 * authenticated user (enforced via AuthContext). However, UserRepository
 * operates differently because:
 *
 * - [findByUsername] and [findByEmail] are used during login (no user context yet)
 * - [create] is used during registration (no user context yet)
 * - Password operations require verification of current credentials
 *
 * ## Security Considerations
 *
 * - Never expose password hashes to clients
 * - Always use Argon2 for password hashing (server-side only)
 * - Username and email lookups should be rate-limited to prevent enumeration
 * - Store refresh tokens hashed, not in plaintext
 *
 * ## Error Handling
 *
 * All methods return [Either]<[AltairError], T> for consistent error handling:
 *
 * - [AltairError.NotFoundError.UserNotFound] - User ID/username/email doesn't exist
 * - [AltairError.ConflictError.DuplicateEntity] - Username/email already exists
 * - [AltairError.ValidationError] - Invalid input data
 * - [AltairError.StorageError] - Database operation failed
 *
 * ## Example Usage
 *
 * ```kotlin
 * // Login flow
 * userRepository.findByUsername("alice").fold(
 *     ifLeft = { error -> /* handle error */ },
 *     ifRight = { user ->
 *         if (user == null) {
 *             // Username not found
 *         } else {
 *             // Verify password hash
 *         }
 *     }
 * )
 *
 * // Registration flow
 * if (userRepository.usernameExists("alice").getOrElse { false }) {
 *     // Username taken
 * } else {
 *     userRepository.create(newUser)
 * }
 * ```
 */
interface UserRepository {
    /**
     * Retrieve a user by their unique identifier.
     *
     * @param id Unique identifier of the user to retrieve
     * @return [Either] containing the [User] if found, or [AltairError.NotFoundError.UserNotFound]
     */
    suspend fun findById(id: String): Either<AltairError, User?>

    /**
     * Retrieve a user by their username.
     *
     * Used during login to find the user account for credential verification.
     * Usernames are unique and case-sensitive.
     *
     * @param username Username to search for (exact match, case-sensitive)
     * @return [Either] containing the [User] if found, null if not found, or error on failure
     */
    suspend fun findByUsername(username: String): Either<AltairError, User?>

    /**
     * Retrieve a user by their email address.
     *
     * Used for password recovery and alternate login methods.
     * Email addresses are unique and case-insensitive.
     *
     * @param email Email address to search for (case-insensitive)
     * @return [Either] containing the [User] if found, null if not found, or error on failure
     */
    suspend fun findByEmail(email: String): Either<AltairError, User?>

    /**
     * Create a new user account.
     *
     * The provided User entity should have:
     * - Unique username (will fail if already exists)
     * - Valid email format (if provided)
     * - Argon2-hashed password (never store plaintext)
     * - Valid role and status
     *
     * Timestamps (createdAt) are set automatically by the repository.
     *
     * @param user User entity to create
     * @return [Either] containing the created user with timestamps set, or error on failure
     */
    suspend fun create(user: User): Either<AltairError, User>

    /**
     * Update the last login timestamp for a user.
     *
     * Called after successful authentication to track user activity.
     * Sets lastLoginAt to the current server time.
     *
     * @param userId User identifier
     * @return [Either] containing Unit on success, or error on failure
     */
    suspend fun updateLastLogin(userId: String): Either<AltairError, Unit>

    /**
     * Update a user's password hash.
     *
     * Used for password change and password recovery flows.
     * The caller is responsible for:
     * - Verifying the current password (for password change)
     * - Hashing the new password with Argon2
     * - Invalidating existing sessions/tokens
     *
     * @param userId User identifier
     * @param newPasswordHash New Argon2-hashed password
     * @return [Either] containing Unit on success, or error on failure
     */
    suspend fun updatePassword(userId: String, newPasswordHash: String): Either<AltairError, Unit>

    /**
     * Check if a username already exists.
     *
     * Used during registration to validate username availability.
     * Usernames are unique and case-sensitive.
     *
     * @param username Username to check
     * @return [Either] containing true if username exists, false otherwise
     */
    suspend fun usernameExists(username: String): Either<AltairError, Boolean>

    /**
     * Check if an email address already exists.
     *
     * Used during registration to validate email availability.
     * Email addresses are unique and case-insensitive.
     *
     * @param email Email address to check
     * @return [Either] containing true if email exists, false otherwise
     */
    suspend fun emailExists(email: String): Either<AltairError, Boolean>
}
