package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.left
import arrow.core.raise.either
import arrow.core.right
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.model.system.InviteCode
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.InviteCodeRepository
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory

/**
 * SurrealDB implementation of InviteCodeRepository.
 */
class SurrealInviteCodeRepository(
    private val db: SurrealDbClient,
) : InviteCodeRepository {
    private val logger = LoggerFactory.getLogger(SurrealInviteCodeRepository::class.java)
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun create(inviteCode: InviteCode): Either<AuthError, InviteCode> =
        either {
            db
                .execute(
                    """
                    CREATE invite_code:${inviteCode.id.value} CONTENT {
                        code: '${inviteCode.code.replace("'", "''")}',
                        created_by: user:${inviteCode.createdBy.value},
                        used_by: NONE,
                        expires_at: d"${inviteCode.expiresAt}",
                        used_at: NONE
                    };
                    """.trimIndent(),
                ).mapLeft { AuthError.InvalidInvite(inviteCode.code) }
                .bind()

            inviteCode
        }

    override suspend fun findByCode(code: String): Either<AuthError, InviteCode> =
        either {
            val escapedCode = code.replace("'", "''")
            val query =
                "SELECT * FROM invite_code WHERE code = '$escapedCode' " +
                    "AND used_by IS NONE AND expires_at > time::now()"
            val result =
                db
                    .query<Any>(query)
                    .mapLeft { AuthError.InvalidInvite(code) }
                    .bind()

            logger.debug("findByCode query='{}' result='{}'", query, result)
            parseInviteCode(result) ?: raise(AuthError.InvalidInvite(code))
        }

    override suspend fun markUsed(
        id: Ulid,
        usedBy: Ulid,
    ): Either<AuthError, Unit> =
        either {
            db
                .execute(
                    """
                    UPDATE invite_code:${id.value} SET
                        used_by = user:${usedBy.value},
                        used_at = time::now();
                    """.trimIndent(),
                ).mapLeft { AuthError.InvalidInvite(id.value) }
                .bind()
        }

    override suspend fun findByCreator(createdBy: Ulid): Either<AuthError, List<InviteCode>> =
        db
            .query<Any>(
                "SELECT * FROM invite_code WHERE created_by = user:${createdBy.value} ORDER BY created_at DESC",
            ).fold(
                ifLeft = {
                    logger.error("Failed to find invite codes by creator: {}", createdBy.value)
                    emptyList<InviteCode>().right()
                },
                ifRight = { parseInviteCodes(it).right() },
            )

    override suspend fun deleteExpiredAndUsed(): Either<AuthError, Int> =
        db
            .execute(
                """
                DELETE invite_code WHERE expires_at < time::now() OR used_by IS NOT NONE;
                """.trimIndent(),
            ).fold(
                ifLeft = {
                    logger.error("Failed to delete expired invite codes")
                    AuthError.InvalidInvite("cleanup").left()
                },
                ifRight = { 0.right() },
            )

    private fun parseInviteCode(result: String): InviteCode? =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject ?: return null
            mapToInviteCode(obj)
        } catch (e: kotlinx.serialization.SerializationException) {
            logger.warn("Failed to parse invite code JSON: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse invite code data: ${e.message}", e)
            null
        }

    private fun parseInviteCodes(result: String): List<InviteCode> =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            array.mapNotNull { element ->
                try {
                    mapToInviteCode(element.jsonObject)
                } catch (e: IllegalStateException) {
                    logger.warn("Failed to parse invite code element: ${e.message}", e)
                    null
                }
            }
        } catch (e: kotlinx.serialization.SerializationException) {
            logger.warn("Failed to parse invite codes array: ${e.message}", e)
            emptyList()
        }

    private fun mapToInviteCode(obj: kotlinx.serialization.json.JsonObject): InviteCode {
        val id =
            obj["id"]?.jsonPrimitive?.content?.substringAfter(":")
                ?: error("Missing id")
        val createdBy =
            obj["created_by"]?.jsonPrimitive?.content?.substringAfter(":")
                ?: error("Missing created_by")
        val usedBy = obj["used_by"]?.jsonPrimitive?.content?.substringAfter(":")

        return InviteCode(
            id = Ulid(id),
            code = obj["code"]?.jsonPrimitive?.content ?: error("Missing code"),
            createdBy = Ulid(createdBy),
            usedBy = usedBy?.let { Ulid(it) },
            expiresAt = parseInstant(obj["expires_at"]?.jsonPrimitive?.content),
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            usedAt = obj["used_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
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
