@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.knowledge.Tag
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.TagRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory
import kotlin.time.Clock

class SurrealTagRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : TagRepository {
    private val logger = LoggerFactory.getLogger(SurrealTagRepository::class.java)
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<DomainError, Tag> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM tag WHERE id = tag:${id.value} AND user_id = user:${userId.value} AND deleted_at IS NONE",
                    ).bind()
            parseTag(result) ?: raise(DomainError.NotFoundError("Tag", id.value))
        }

    override suspend fun save(entity: Tag): Either<DomainError, Tag> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .execute(
                        """
                        UPDATE tag:${entity.id.value} SET
                            name = '${entity.name.replace("'", "''")}',
                            color = ${entity.color?.let { "'$it'" } ?: "NONE"},
                            updated_at = time::now()
                        WHERE user_id = user:${userId.value};
                        """.trimIndent(),
                    ).bind()
            } else {
                db
                    .execute(
                        """
                        CREATE tag:${entity.id.value} CONTENT {
                            user_id: user:${userId.value},
                            name: '${entity.name.replace("'", "''")}',
                            color: ${entity.color?.let { "'$it'" } ?: "NONE"}
                        };
                        """.trimIndent(),
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE tag:${id.value} SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).bind()
        }

    override fun findAll(): Flow<List<Tag>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM tag WHERE user_id = user:${userId.value} AND deleted_at IS NONE ORDER BY name",
                )
            emit(result.fold({ emptyList() }, { parseTags(it) }))
        }

    override suspend fun findByName(name: String): Either<DomainError, Tag> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM tag WHERE user_id = user:${userId.value} AND name = '${name.replace(
                            "'",
                            "''",
                        )}' AND deleted_at IS NONE",
                    ).bind()
            parseTag(result) ?: raise(DomainError.NotFoundError("Tag", name))
        }

    override suspend fun findOrCreate(
        name: String,
        color: String?,
    ): Either<DomainError, Tag> =
        either {
            val existing = findByName(name)
            if (existing.isRight()) {
                existing.getOrNull()!!
            } else {
                val now = Clock.System.now()
                val tag =
                    Tag(
                        id = Ulid.generate(),
                        userId = userId,
                        name = name,
                        color = color,
                        createdAt = now,
                        updatedAt = now,
                    )
                save(tag).bind()
            }
        }

    override fun findByNote(noteId: Ulid): Flow<List<Tag>> =
        flow {
            val result =
                db.query<Any>(
                    """
                    SELECT tag.* FROM note_tag
                    INNER JOIN tag ON note_tag.tag_id = tag.id
                    WHERE note_tag.note_id = note:${noteId.value}
                        AND note_tag.user_id = user:${userId.value}
                        AND tag.user_id = user:${userId.value}
                        AND tag.deleted_at IS NONE
                    """.trimIndent(),
                )
            emit(result.fold({ emptyList() }, { parseTags(it) }))
        }

    override suspend fun tagNote(
        noteId: Ulid,
        tagId: Ulid,
    ): Either<DomainError, Unit> =
        either {
            db
                .execute(
                    """
                    CREATE note_tag CONTENT {
                        user_id: user:${userId.value},
                        note_id: note:${noteId.value},
                        tag_id: tag:${tagId.value}
                    };
                    """.trimIndent(),
                ).bind()
        }

    override suspend fun untagNote(
        noteId: Ulid,
        tagId: Ulid,
    ): Either<DomainError, Unit> =
        either {
            db
                .execute(
                    "DELETE note_tag WHERE user_id = user:${userId.value} AND note_id = note:${noteId.value} AND tag_id = tag:${tagId.value};",
                ).bind()
        }

    override fun findMostUsed(limit: Int): Flow<List<Pair<Tag, Int>>> =
        flow {
            val result =
                db.query<Any>(
                    """
                    SELECT tag.*, count(note_tag.id) AS usage_count FROM tag
                    LEFT JOIN note_tag ON tag.id = note_tag.tag_id
                    WHERE tag.user_id = user:${userId.value} AND tag.deleted_at IS NONE
                    GROUP BY tag.id
                    ORDER BY usage_count DESC
                    LIMIT $limit
                    """.trimIndent(),
                )
            emit(result.fold({ emptyList() }, { parseTagsWithCount(it) }))
        }

    private fun parseTag(result: String): Tag? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToTag(it) }
        } catch (e: Exception) {
            logger.warn("Failed to parse tag from result: ${e.message}", e)
            null
        }

    private fun parseTags(result: String): List<Tag> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToTag(it.jsonObject)
                } catch (e: Exception) {
                    logger.warn("Failed to parse individual tag: ${e.message}")
                    null
                }
            }
        } catch (e: Exception) {
            logger.warn("Failed to parse tags array: ${e.message}", e)
            emptyList()
        }

    private fun parseTagsWithCount(result: String): List<Pair<Tag, Int>> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull { element ->
                try {
                    val obj = element.jsonObject
                    val tag = mapToTag(obj)
                    val count = obj["usage_count"]?.jsonPrimitive?.content?.toIntOrNull() ?: 0
                    tag to count
                } catch (e: Exception) {
                    logger.warn("Failed to parse tag with count: ${e.message}")
                    null
                }
            }
        } catch (e: Exception) {
            logger.warn("Failed to parse tags with count array: ${e.message}", e)
            emptyList()
        }

    private fun mapToTag(obj: kotlinx.serialization.json.JsonObject): Tag {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return Tag(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            name = obj["name"]?.jsonPrimitive?.content ?: "",
            color = obj["color"]?.jsonPrimitive?.content,
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
            deletedAt = obj["deleted_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
        )
    }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: Exception) {
                logger.warn("Failed to parse instant '$value', using DISTANT_PAST: ${e.message}")
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
