package com.getaltair.server.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.system.RefreshToken
import com.getaltair.altair.shared.repository.RefreshTokenRepository
import com.getaltair.server.persistence.SurrealDbClient
import kotlinx.datetime.Instant

/**
 * SurrealDB implementation of [RefreshTokenRepository].
 *
 * Provides refresh token management for JWT-based authentication.
 * Tokens are stored hashed (SHA-256) for security and can be revoked
 * individually or in bulk.
 *
 * ## Database Schema
 *
 * Uses the `refresh_token` table defined in V002__refresh_tokens.surql:
 * - token_hash (unique, indexed)
 * - user_id (indexed)
 * - expires_at
 * - created_at
 * - revoked_at (optional)
 *
 * ## Performance Notes
 *
 * - Token lookups are fast due to unique index on token_hash
 * - User-based operations use index on user_id
 * - Expired token cleanup is optimized with index on expires_at
 *
 * @param db The SurrealDB client for database operations
 */
class SurrealRefreshTokenRepository(
    private val db: SurrealDbClient
) : RefreshTokenRepository {

    /**
     * Gets the current timestamp for audit fields.
     */
    private fun now(): Instant = Instant.fromEpochMilliseconds(System.currentTimeMillis())

    override suspend fun storeToken(
        userId: String,
        tokenHash: String,
        expiresAt: Instant
    ): AltairResult<RefreshToken> {
        val id = Ulid.generate()
        val sql = """
            CREATE refresh_token:${'$'}id CONTENT {
                user_id: ${'$'}userId,
                token_hash: ${'$'}tokenHash,
                expires_at: ${'$'}expiresAt,
                created_at: ${'$'}createdAt,
                revoked_at: NONE
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to id.toString(),
                "userId" to "user:$userId",
                "tokenHash" to tokenHash,
                "expiresAt" to expiresAt.toString(),
                "createdAt" to now().toString()
            ),
            RefreshToken::class
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create refresh token").left()
        }
    }

    override suspend fun findByTokenHash(tokenHash: String): AltairResult<RefreshToken?> {
        val sql = """
            SELECT * FROM refresh_token
            WHERE token_hash = ${'$'}tokenHash
            LIMIT 1
        """.trimIndent()

        return db.queryOne(sql, mapOf("tokenHash" to tokenHash), RefreshToken::class)
    }

    override suspend fun revokeToken(tokenHash: String): AltairResult<Unit> {
        val sql = """
            UPDATE refresh_token SET
                revoked_at = ${'$'}now
            WHERE token_hash = ${'$'}tokenHash
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "tokenHash" to tokenHash,
                "now" to now().toString()
            ),
            RefreshToken::class
        ).flatMap { result ->
            if (result != null) {
                Unit.right()
            } else {
                AltairError.NotFoundError.UserNotFound("Token not found").left()
            }
        }
    }

    override suspend fun revokeAllForUser(userId: String): AltairResult<Int> {
        val sql = """
            UPDATE refresh_token SET
                revoked_at = ${'$'}now
            WHERE user_id = ${'$'}userId AND revoked_at IS NONE
        """.trimIndent()

        return db.query(
            sql,
            mapOf(
                "userId" to "user:$userId",
                "now" to now().toString()
            ),
            RefreshToken::class
        ).map { results -> results.size }
    }

    override suspend fun deleteExpired(): AltairResult<Int> {
        val sql = """
            DELETE FROM refresh_token
            WHERE expires_at < ${'$'}now
        """.trimIndent()

        return db.query(sql, mapOf("now" to now().toString()), RefreshToken::class)
            .map { results -> results.size }
    }
}
