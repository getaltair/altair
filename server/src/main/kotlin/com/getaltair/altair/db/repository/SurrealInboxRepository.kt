@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.system.InboxItem
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.CaptureSource
import com.getaltair.altair.repository.InboxRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory
import kotlin.time.Clock
import kotlin.time.Instant

/**
 * SurrealDB implementation of InboxRepository.
 */
class SurrealInboxRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : InboxRepository {
    private val logger = LoggerFactory.getLogger(SurrealInboxRepository::class.java)
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<DomainError, InboxItem> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM inbox_item WHERE id = inbox_item:\$id AND user_id = user:\$userId",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()
            parseInboxItem(result) ?: raise(DomainError.NotFoundError("InboxItem", id.value))
        }

    override suspend fun save(entity: InboxItem): Either<DomainError, InboxItem> =
        either {
            val attachmentIds = entity.attachmentIds.map { it.value }
            db
                .executeBind(
                    """
                    CREATE inbox_item:${'$'}id CONTENT {
                        user_id: user:${'$'}userId,
                        content: ${'$'}content,
                        source: ${'$'}source,
                        attachment_ids: ${'$'}attachmentIds
                    };
                    """.trimIndent(),
                    mapOf(
                        "id" to entity.id.value,
                        "userId" to userId.value,
                        "content" to entity.content,
                        "source" to entity.source.name.lowercase(),
                        "attachmentIds" to attachmentIds,
                    ),
                ).bind()
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            db
                .executeBind(
                    "DELETE inbox_item:${'$'}id WHERE user_id = user:${'$'}userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).bind()
        }

    override fun findAll(): Flow<List<InboxItem>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM inbox_item WHERE user_id = user:\$userId ORDER BY created_at DESC",
                    mapOf("userId" to userId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findAll: ${error.message}")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findAll: ${error.message}")

                            is DomainError.NotFoundError -> logger.warn("Database error in findAll: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findAll: ${error.field} - ${error.message}")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findAll: ${error.message}")

                            else -> logger.warn("Database error in findAll: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseInboxItems(it) },
                ),
            )
        }

    override suspend fun capture(
        content: String,
        source: CaptureSource,
        attachmentIds: List<Ulid>,
    ): Either<DomainError, InboxItem> =
        either {
            val now = Clock.System.now()
            val item =
                InboxItem(
                    id = Ulid.generate(),
                    userId = userId,
                    content = content,
                    source = source,
                    attachmentIds = attachmentIds,
                    createdAt = now,
                    updatedAt = now,
                )
            save(item).bind()
        }

    override suspend fun triage(id: Ulid): Either<DomainError, Unit> = delete(id)

    override suspend fun count(): Either<DomainError, Int> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT count() FROM inbox_item WHERE user_id = user:\$userId GROUP ALL",
                        mapOf("userId" to userId.value),
                    ).bind()
            parseCount(result)
        }

    override fun findBySource(source: CaptureSource): Flow<List<InboxItem>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM inbox_item WHERE user_id = user:\$userId AND source = \$source",
                    mapOf("userId" to userId.value, "source" to source.name.lowercase()),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findBySource: ${error.message}")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findBySource: ${error.message}")

                            is DomainError.NotFoundError -> logger.warn("Database error in findBySource: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findBySource: ${error.field} - ${error.message}")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findBySource: ${error.message}")

                            else -> logger.warn("Database error in findBySource: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseInboxItems(it) },
                ),
            )
        }

    private fun parseInboxItem(result: String): InboxItem? {
        return try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject ?: return null
            mapToInboxItem(obj)
        } catch (e: SerializationException) {
            logger.warn("Failed to parse InboxItem: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse InboxItem: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse InboxItem: ${e.message}", e)
            null
        }
    }

    private fun parseInboxItems(result: String): List<InboxItem> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToInboxItem(it.jsonObject)
                } catch (e: SerializationException) {
                    logger.warn("Failed to parse InboxItem element: ${e.message}", e)
                    null
                } catch (e: IllegalStateException) {
                    logger.warn("Failed to parse InboxItem element: ${e.message}", e)
                    null
                } catch (e: IllegalArgumentException) {
                    logger.warn("Failed to parse InboxItem element: ${e.message}", e)
                    null
                }
            }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse InboxItem list: ${e.message}", e)
            emptyList()
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse InboxItem list: ${e.message}", e)
            emptyList()
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse InboxItem list: ${e.message}", e)
            emptyList()
        }

    private fun mapToInboxItem(obj: kotlinx.serialization.json.JsonObject): InboxItem {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val attachments =
            try {
                obj["attachment_ids"]?.jsonArray?.mapNotNull { it.jsonPrimitive.content.let { id -> Ulid(id) } }
                    ?: emptyList()
            } catch (e: SerializationException) {
                logger.warn("Failed to parse attachment_ids: ${e.message}", e)
                emptyList()
            } catch (e: IllegalStateException) {
                logger.warn("Failed to parse attachment_ids: ${e.message}", e)
                emptyList()
            } catch (e: IllegalArgumentException) {
                logger.warn("Failed to parse attachment_ids: ${e.message}", e)
                emptyList()
            }
        return InboxItem(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            content = obj["content"]?.jsonPrimitive?.content ?: "",
            source = CaptureSource.valueOf(obj["source"]?.jsonPrimitive?.content?.uppercase() ?: "MANUAL"),
            attachmentIds = attachments,
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
        )
    }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: SerializationException) {
                logger.warn("Failed to parse Instant '$it': ${e.message}", e)
                Instant.DISTANT_PAST
            } catch (e: IllegalStateException) {
                logger.warn("Failed to parse Instant '$it': ${e.message}", e)
                Instant.DISTANT_PAST
            } catch (e: IllegalArgumentException) {
                logger.warn("Failed to parse Instant '$it': ${e.message}", e)
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST

    private fun parseCount(result: String): Int =
        try {
            json
                .parseToJsonElement(
                    result,
                ).jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.get("count")
                ?.jsonPrimitive
                ?.content
                ?.toIntOrNull()
                ?: 0
        } catch (e: SerializationException) {
            logger.warn("Failed to parse count: ${e.message}", e)
            0
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse count: ${e.message}", e)
            0
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse count: ${e.message}", e)
            0
        }
}
