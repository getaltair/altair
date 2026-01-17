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
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlin.time.Clock

/**
 * SurrealDB implementation of InboxRepository.
 */
class SurrealInboxRepository(
    private val db: SurrealDbClient,
    private val userId: String,
) : InboxRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<DomainError, InboxItem> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM inbox_item:${id.value} WHERE user_id = user:$userId",
                    ).bind()
            parseInboxItem(result) ?: raise(DomainError.NotFoundError("InboxItem", id.value))
        }

    override suspend fun save(entity: InboxItem): Either<DomainError, InboxItem> =
        either {
            val attachmentList = entity.attachmentIds.joinToString(", ") { "'${it.value}'" }
            db
                .execute(
                    """
                    CREATE inbox_item:${entity.id.value} CONTENT {
                        user_id: user:$userId,
                        content: '${entity.content.replace("'", "''")}',
                        source: '${entity.source.name.lowercase()}',
                        attachment_ids: [$attachmentList]
                    };
                    """.trimIndent(),
                ).bind()
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            db.execute("DELETE inbox_item:${id.value} WHERE user_id = user:$userId;").bind()
        }

    override fun findAll(): Flow<List<InboxItem>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM inbox_item WHERE user_id = user:$userId ORDER BY created_at DESC",
                )
            emit(result.fold({ emptyList() }, { parseInboxItems(it) }))
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
                    userId = Ulid(userId),
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
                    .query<Any>(
                        "SELECT count() FROM inbox_item WHERE user_id = user:$userId GROUP ALL",
                    ).bind()
            parseCount(result)
        }

    override fun findBySource(source: CaptureSource): Flow<List<InboxItem>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM inbox_item WHERE user_id = user:$userId AND source = '${source.name.lowercase()}'",
                )
            emit(result.fold({ emptyList() }, { parseInboxItems(it) }))
        }

    private fun parseInboxItem(result: String): InboxItem? {
        return try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject ?: return null
            mapToInboxItem(obj)
        } catch (e: Exception) {
            null
        }
    }

    private fun parseInboxItems(result: String): List<InboxItem> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToInboxItem(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
            emptyList()
        }

    private fun mapToInboxItem(obj: kotlinx.serialization.json.JsonObject): InboxItem {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val attachments =
            try {
                obj["attachment_ids"]?.jsonArray?.mapNotNull { it.jsonPrimitive.content.let { id -> Ulid(id) } }
                    ?: emptyList()
            } catch (e: Exception) {
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
            } catch (e: Exception) {
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
        } catch (e: Exception) {
            0
        }
}
