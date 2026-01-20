package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.left
import arrow.core.raise.either
import arrow.core.right
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.model.system.RefreshToken
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.RefreshTokenRepository
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory

/**
 * SurrealDB implementation of RefreshTokenRepository.
 */
class SurrealRefreshTokenRepository(
    private val db: SurrealDbClient,
) : RefreshTokenRepository {
    private val logger = LoggerFactory.getLogger(SurrealRefreshTokenRepository::class.java)
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun create(token: RefreshToken): Either<AuthError, RefreshToken> =
        either {
            db
                .executeBind(
                    """
                    CREATE refresh_token:${token.id.value} CONTENT {
                        user_id: user:${'$'}userId,
                        token_hash: ${'$'}tokenHash,
                        device_name: ${'$'}deviceName,
                        expires_at: d"${token.expiresAt}",
                        revoked_at: NONE
                    };
                    """.trimIndent(),
                    mapOf(
                        "userId" to token.userId.value,
                        "tokenHash" to token.tokenHash,
                        "deviceName" to token.deviceName,
                    ),
                ).mapLeft { AuthError.TokenInvalid("Failed to create refresh token") }
                .bind()

            token
        }

    override suspend fun findByHash(tokenHash: String): Either<AuthError, RefreshToken> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM refresh_token WHERE token_hash = ${'$'}tokenHash AND revoked_at IS NONE",
                        mapOf("tokenHash" to tokenHash),
                    ).mapLeft { AuthError.TokenInvalid("Failed to query refresh token") }
                    .bind()

            parseRefreshToken(result) ?: raise(AuthError.TokenInvalid("Refresh token not found"))
        }

    override suspend fun revoke(
        id: Ulid,
        userId: Ulid,
    ): Either<AuthError, Unit> =
        either {
            db
                .executeBind(
                    """
                    UPDATE refresh_token:${id.value} SET
                        revoked_at = time::now()
                    WHERE user_id = user:${'$'}userId;
                    """.trimIndent(),
                    mapOf("userId" to userId.value),
                ).mapLeft { AuthError.TokenInvalid("Failed to revoke refresh token") }
                .bind()
        }

    override suspend fun revokeAllForUser(userId: Ulid): Either<AuthError, Int> =
        db
            .executeBind(
                """
                UPDATE refresh_token SET revoked_at = time::now()
                WHERE user_id = user:${'$'}userId AND revoked_at IS NONE;
                """.trimIndent(),
                mapOf("userId" to userId.value),
            ).fold(
                ifLeft = {
                    logger.error("Failed to revoke all tokens for user: {}", userId.value)
                    AuthError.TokenInvalid("Failed to revoke tokens").left()
                },
                ifRight = {
                    // Execute returns Unit, so we can't count revoked tokens
                    // Return 0 as a placeholder (actual count not available)
                    0.right()
                },
            )

    override suspend fun deleteExpired(): Either<AuthError, Int> =
        db
            .execute(
                """
                DELETE refresh_token WHERE expires_at < time::now() OR revoked_at IS NOT NONE;
                """.trimIndent(),
            ).fold(
                ifLeft = {
                    logger.error("Failed to delete expired tokens")
                    AuthError.TokenInvalid("Failed to delete expired tokens").left()
                },
                ifRight = { 0.right() }, // SurrealDB doesn't return count for DELETE
            )

    private fun parseRefreshToken(result: String): RefreshToken? =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject ?: return null
            mapToRefreshToken(obj)
        } catch (e: kotlinx.serialization.SerializationException) {
            logger.warn("Failed to parse refresh token JSON: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse refresh token data: ${e.message}", e)
            null
        }

    private fun mapToRefreshToken(obj: kotlinx.serialization.json.JsonObject): RefreshToken? {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: return null
        val userId = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: return null

        return RefreshToken(
            id = Ulid(id),
            userId = Ulid(userId),
            tokenHash = obj["token_hash"]?.jsonPrimitive?.content ?: return null,
            deviceName = obj["device_name"]?.jsonPrimitive?.content,
            expiresAt = parseInstant(obj["expires_at"]?.jsonPrimitive?.content),
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            revokedAt = obj["revoked_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
        )
    }

    private fun parseInstant(value: String?): Instant =
        value?.takeIf { it != "null" && it.isNotBlank() }?.let {
            try {
                Instant.parse(it)
            } catch (e: IllegalArgumentException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
