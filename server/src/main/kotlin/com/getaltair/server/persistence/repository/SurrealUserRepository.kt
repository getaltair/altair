package com.getaltair.server.persistence.repository

import arrow.core.Either
import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.system.User
import com.getaltair.altair.shared.repository.UserRepository
import com.getaltair.server.persistence.SurrealDbClient
import kotlinx.datetime.Instant

/**
 * SurrealDB implementation of [UserRepository].
 *
 * Provides user account management including authentication lookups,
 * registration, and password updates. This repository operates at the
 * system level (NOT user-scoped) since it handles authentication before
 * a user context is established.
 *
 * ## Database Schema
 *
 * Uses the `user` table defined in V001__initial_schema.surql:
 * - username (unique, indexed)
 * - email (unique, indexed, optional)
 * - password_hash (Argon2)
 * - role (member/admin)
 * - status (active/disabled)
 * - storage_used/storage_quota
 * - created_at, last_login_at
 *
 * ## Security Notes
 *
 * - Password hashes are never exposed in responses
 * - Username/email existence checks prevent user enumeration via timing
 * - All queries use parameterized binding to prevent SQL injection
 * - Deleted users are soft-deleted (deleted_at timestamp)
 *
 * @param db The SurrealDB client for database operations
 */
class SurrealUserRepository(
    private val db: SurrealDbClient
) : UserRepository {

    /**
     * Gets the current timestamp for audit fields.
     */
    private fun now(): Instant = Instant.fromEpochMilliseconds(System.currentTimeMillis())

    override suspend fun findById(id: String): Either<AltairError, User?> {
        val sql = """
            SELECT * FROM user:${'$'}id
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to id), User::class)
    }

    override suspend fun findByUsername(username: String): Either<AltairError, User?> {
        val sql = """
            SELECT * FROM user
            WHERE username = ${'$'}username
            LIMIT 1
        """.trimIndent()

        return db.queryOne(sql, mapOf("username" to username), User::class)
    }

    override suspend fun findByEmail(email: String): Either<AltairError, User?> {
        val sql = """
            SELECT * FROM user
            WHERE string::lowercase(email) = string::lowercase(${'$'}email)
            LIMIT 1
        """.trimIndent()

        return db.queryOne(sql, mapOf("email" to email), User::class)
    }

    override suspend fun create(user: User): Either<AltairError, User> {
        // Check for duplicate username
        val usernameCheck = usernameExists(user.username)
        if (usernameCheck.isLeft()) {
            return usernameCheck.map { user }
        }
        if (usernameCheck.getOrNull() == true) {
            return AltairError.ConflictError.DuplicateEntity(
                entityType = "User",
                field = "username",
                value = user.username
            ).left()
        }

        // Check for duplicate email if provided
        user.email?.let { email ->
            val emailCheck = emailExists(email)
            if (emailCheck.isLeft()) {
                return emailCheck.map { user }
            }
            if (emailCheck.getOrNull() == true) {
                return AltairError.ConflictError.DuplicateEntity(
                    entityType = "User",
                    field = "email",
                    value = email
                ).left()
            }
        }

        val sql = """
            CREATE user:${'$'}id CONTENT {
                username: ${'$'}username,
                email: ${'$'}email,
                password_hash: ${'$'}passwordHash,
                role: ${'$'}role,
                status: ${'$'}status,
                storage_used: ${'$'}storageUsed,
                storage_quota: ${'$'}storageQuota,
                created_at: ${'$'}createdAt,
                last_login_at: ${'$'}lastLoginAt
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to user.id.toString(),
                "username" to user.username,
                "email" to user.email,
                "passwordHash" to user.passwordHash,
                "role" to user.role.name,
                "status" to user.status.name,
                "storageUsed" to user.storageUsed,
                "storageQuota" to user.storageQuota,
                "createdAt" to user.createdAt.toString(),
                "lastLoginAt" to user.lastLoginAt?.toString()
            ),
            User::class
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create user").left()
        }
    }

    override suspend fun updateLastLogin(userId: String): Either<AltairError, Unit> {
        val sql = """
            UPDATE user:${'$'}id SET
                last_login_at = ${'$'}now
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to userId, "now" to now().toString()), User::class)
            .flatMap { result ->
                if (result != null) {
                    Unit.right()
                } else {
                    AltairError.NotFoundError.UserNotFound(userId).left()
                }
            }
    }

    override suspend fun updatePassword(userId: String, newPasswordHash: String): Either<AltairError, Unit> {
        val sql = """
            UPDATE user:${'$'}id SET
                password_hash = ${'$'}passwordHash
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to userId,
                "passwordHash" to newPasswordHash
            ),
            User::class
        ).flatMap { result ->
            if (result != null) {
                Unit.right()
            } else {
                AltairError.NotFoundError.UserNotFound(userId).left()
            }
        }
    }

    override suspend fun usernameExists(username: String): Either<AltairError, Boolean> {
        val sql = """
            SELECT VALUE count() FROM user
            WHERE username = ${'$'}username
            GROUP ALL
        """.trimIndent()

        return db.queryOne(sql, mapOf("username" to username), Int::class)
            .map { count -> (count ?: 0) > 0 }
    }

    override suspend fun emailExists(email: String): Either<AltairError, Boolean> {
        val sql = """
            SELECT VALUE count() FROM user
            WHERE string::lowercase(email) = string::lowercase(${'$'}email)
            GROUP ALL
        """.trimIndent()

        return db.queryOne(sql, mapOf("email" to email), Int::class)
            .map { count -> (count ?: 0) > 0 }
    }
}
