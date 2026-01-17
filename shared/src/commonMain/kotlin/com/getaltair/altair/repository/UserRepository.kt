package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.UserError
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import kotlinx.coroutines.flow.Flow

/**
 * Repository for User entities.
 *
 * Users own all their data; every other entity references a userId for
 * multi-user isolation. This repository is primarily used by the server
 * for user management and authentication.
 *
 * All methods use [UserError] for consistent error handling across
 * both CRUD operations and auth-related scenarios.
 */
@Suppress("TooManyFunctions")
interface UserRepository {
    /**
     * Finds a user by their unique identifier.
     *
     * @param id The ULID of the user
     * @return Either [UserError.NotFound] if not found, or the user
     */
    suspend fun findById(id: Ulid): Either<UserError, User>

    /**
     * Finds a user by their email address.
     *
     * @param email The email address to search for
     * @return Either [UserError.EmailNotFound] if not found, or the user
     */
    suspend fun findByEmail(email: String): Either<UserError, User>

    /**
     * Creates a new user.
     *
     * @param user The user to create
     * @return Either [UserError.EmailAlreadyExists] if email taken, or the created user
     */
    suspend fun create(user: User): Either<UserError, User>

    /**
     * Updates an existing user.
     *
     * @param user The user to update
     * @return Either [UserError.NotFound] on failure, or the updated user
     */
    suspend fun update(user: User): Either<UserError, User>

    /**
     * Soft-deletes a user by setting their deletedAt timestamp.
     *
     * @param id The ULID of the user
     * @return Either [UserError.NotFound] on failure, or Unit on success
     */
    suspend fun delete(id: Ulid): Either<UserError, Unit>

    /**
     * Finds all users (admin only).
     *
     * @return A Flow emitting all users
     */
    fun findAll(): Flow<List<User>>

    /**
     * Finds users by role.
     *
     * @param role The role to filter by
     * @return A Flow emitting users with the specified role
     */
    fun findByRole(role: UserRole): Flow<List<User>>

    /**
     * Finds users by status.
     *
     * @param status The status to filter by
     * @return A Flow emitting users with the specified status
     */
    fun findByStatus(status: UserStatus): Flow<List<User>>

    /**
     * Updates a user's storage usage.
     *
     * @param id The ULID of the user
     * @param bytesUsed The new storage used value
     * @return Either [UserError.NotFound] or [UserError.StorageQuotaExceeded] on failure, or the updated user
     */
    suspend fun updateStorageUsed(
        id: Ulid,
        bytesUsed: Long,
    ): Either<UserError, User>

    /**
     * Checks if an email is available (not already registered).
     *
     * @param email The email to check
     * @return Either an error on failure, or true if available
     */
    suspend fun isEmailAvailable(email: String): Either<UserError, Boolean>

    /**
     * Counts all active users.
     *
     * @return Either an error on failure, or the count
     */
    suspend fun countActive(): Either<UserError, Int>
}
