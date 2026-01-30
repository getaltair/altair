package com.getaltair.altair.persistence.repository

import arrow.core.Either
import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.persistence.DesktopSurrealDbClient
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.system.User
import com.getaltair.altair.shared.repository.UserRepository

/**
 * Desktop implementation of UserRepository using embedded SurrealDB.
 * Single-user context - no multi-tenant concerns.
 */
class DesktopUserRepository(
    private val db: DesktopSurrealDbClient
) : UserRepository {

    private val tableName = "user"

    override suspend fun findById(id: String): Either<AltairError, User?> {
        val sql = """
            SELECT * FROM $tableName
            WHERE id = ${'$'}id
        """.trimIndent()

        return db.queryOne(sql, mapOf("id" to "$tableName:$id"), User::class)
    }

    override suspend fun findByUsername(username: String): Either<AltairError, User?> {
        val sql = """
            SELECT * FROM $tableName
            WHERE username = ${'$'}username
            LIMIT 1
        """.trimIndent()

        return db.queryOne(sql, mapOf("username" to username), User::class)
    }

    override suspend fun findByEmail(email: String): Either<AltairError, User?> {
        val sql = """
            SELECT * FROM $tableName
            WHERE string::lowercase(email) = string::lowercase(${'$'}email)
            LIMIT 1
        """.trimIndent()

        return db.queryOne(sql, mapOf("email" to email), User::class)
    }

    override suspend fun create(user: User): Either<AltairError, User> {
        val sql = """
            CREATE $tableName CONTENT {
                id: ${'$'}id,
                username: ${'$'}username,
                email: ${'$'}email,
                password_hash: ${'$'}passwordHash,
                created_at: ${'$'}createdAt,
                last_login_at: ${'$'}lastLoginAt,
                role: ${'$'}role,
                status: ${'$'}status
            }
        """.trimIndent()

        val params = mapOf(
            "id" to "$tableName:${user.id}",
            "username" to user.username,
            "email" to user.email,
            "passwordHash" to user.passwordHash,
            "createdAt" to user.createdAt.toString(),
            "lastLoginAt" to user.lastLoginAt?.toString(),
            "role" to user.role.name,
            "status" to user.status.name
        )

        return db.queryOne(sql, params, User::class)
            .flatMap { result ->
                result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create user").left()
            }
    }

    override suspend fun updateLastLogin(userId: String): Either<AltairError, Unit> {
        val sql = """
            UPDATE ${'$'}id MERGE {
                last_login_at: time::now()
            }
        """.trimIndent()

        return db.query(sql, mapOf("id" to "$tableName:$userId"), User::class)
            .flatMap { results ->
                if (results.isNotEmpty()) Unit.right()
                else AltairError.NotFoundError.UserNotFound(userId).left()
            }
    }

    override suspend fun updatePassword(userId: String, newPasswordHash: String): Either<AltairError, Unit> {
        val sql = """
            UPDATE ${'$'}id MERGE {
                password_hash: ${'$'}passwordHash
            }
        """.trimIndent()

        val params = mapOf(
            "id" to "$tableName:$userId",
            "passwordHash" to newPasswordHash
        )

        return db.query(sql, params, User::class)
            .flatMap { results ->
                if (results.isNotEmpty()) Unit.right()
                else AltairError.NotFoundError.UserNotFound(userId).left()
            }
    }

    override suspend fun usernameExists(username: String): Either<AltairError, Boolean> {
        val sql = """
            SELECT VALUE count() FROM $tableName
            WHERE username = ${'$'}username
            LIMIT 1
        """.trimIndent()

        return db.query(sql, mapOf("username" to username), Int::class)
            .map { result ->
                // SurrealDB returns array of count results
                val count = result.firstOrNull() as? Number ?: 0
                count.toInt() > 0
            }
    }

    override suspend fun emailExists(email: String): Either<AltairError, Boolean> {
        val sql = """
            SELECT VALUE count() FROM $tableName
            WHERE string::lowercase(email) = string::lowercase(${'$'}email)
            LIMIT 1
        """.trimIndent()

        return db.query(sql, mapOf("email" to email), Int::class)
            .map { result ->
                val count = result.firstOrNull() as? Number ?: 0
                count.toInt() > 0
            }
    }
}
