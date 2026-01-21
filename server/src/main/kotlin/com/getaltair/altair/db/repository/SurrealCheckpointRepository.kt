@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.guidance.Checkpoint
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.CheckpointRepository
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

class SurrealCheckpointRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : CheckpointRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    companion object {
        private val logger = LoggerFactory.getLogger(SurrealCheckpointRepository::class.java)
    }

    override suspend fun findById(id: Ulid): Either<DomainError, Checkpoint> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM checkpoint WHERE id = checkpoint:\$id AND user_id = user:\$userId",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()
            parseCheckpoint(result) ?: raise(DomainError.NotFoundError("Checkpoint", id.value))
        }

    override suspend fun save(entity: Checkpoint): Either<DomainError, Checkpoint> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .executeBind(
                        """
                        UPDATE checkpoint:${'$'}id SET
                            title = ${'$'}title,
                            sort_order = ${'$'}sortOrder,
                            is_completed = ${'$'}isCompleted,
                            completed_at = ${'$'}completedAt,
                            updated_at = time::now()
                        WHERE user_id = user:${'$'}userId;
                        """.trimIndent(),
                        mapOf(
                            "id" to entity.id.value,
                            "title" to entity.title,
                            "sortOrder" to entity.sortOrder,
                            "isCompleted" to entity.isCompleted,
                            "completedAt" to entity.completedAt?.toString(),
                            "userId" to userId.value,
                        ),
                    ).bind()
            } else {
                db
                    .executeBind(
                        """
                        CREATE checkpoint:${'$'}id CONTENT {
                            user_id: user:${'$'}userId,
                            quest_id: quest:${'$'}questId,
                            title: ${'$'}title,
                            sort_order: ${'$'}sortOrder,
                            is_completed: ${'$'}isCompleted,
                            completed_at: ${'$'}completedAt
                        };
                        """.trimIndent(),
                        mapOf(
                            "id" to entity.id.value,
                            "userId" to userId.value,
                            "questId" to entity.questId.value,
                            "title" to entity.title,
                            "sortOrder" to entity.sortOrder,
                            "isCompleted" to entity.isCompleted,
                            "completedAt" to entity.completedAt?.toString(),
                        ),
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            db
                .executeBind(
                    "DELETE checkpoint:${'$'}id WHERE user_id = user:${'$'}userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).bind()
        }

    override fun findAll(): Flow<List<Checkpoint>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM checkpoint WHERE user_id = user:\$userId ORDER BY sort_order",
                    mapOf("userId" to userId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findAll: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findAll: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findAll: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findAll: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findAll: ERROR_MSG")

                            else -> logger.warn("Database error in findAll: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseCheckpoints(it) },
                ),
            )
        }

    override fun findByQuest(questId: Ulid): Flow<List<Checkpoint>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM checkpoint WHERE user_id = user:\$userId AND quest_id = quest:\$questId ORDER BY sort_order",
                    mapOf("userId" to userId.value, "questId" to questId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findByQuest: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findByQuest: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findByQuest: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findByQuest: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findByQuest: ERROR_MSG")

                            else -> logger.warn("Database error in findByQuest: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseCheckpoints(it) },
                ),
            )
        }

    override suspend fun toggleComplete(id: Ulid): Either<DomainError, Checkpoint> =
        either {
            val checkpoint = findById(id).bind()
            val newCompleted = !checkpoint.isCompleted
            val completedAt = if (newCompleted) Clock.System.now() else null
            db
                .executeBind(
                    """
                    UPDATE checkpoint:${'$'}id SET
                        is_completed = ${'$'}isCompleted,
                        completed_at = ${'$'}completedAt,
                        updated_at = time::now()
                    WHERE user_id = user:${'$'}userId;
                    """.trimIndent(),
                    mapOf(
                        "id" to id.value,
                        "isCompleted" to newCompleted,
                        "completedAt" to completedAt?.toString(),
                        "userId" to userId.value,
                    ),
                ).bind()
            findById(id).bind()
        }

    override suspend fun reorder(
        questId: Ulid,
        orderedIds: List<Ulid>,
    ): Either<DomainError, Unit> =
        either {
            orderedIds.forEachIndexed { index, id ->
                db
                    .executeBind(
                        "UPDATE checkpoint:${'$'}id SET sort_order = ${'$'}sortOrder, updated_at = time::now() WHERE user_id = user:${'$'}userId AND quest_id = quest:${'$'}questId;",
                        mapOf("id" to id.value, "sortOrder" to index, "userId" to userId.value, "questId" to questId.value),
                    ).bind()
            }
        }

    override suspend fun countByQuest(
        questId: Ulid,
        completed: Boolean?,
    ): Either<DomainError, Int> =
        either {
            val query =
                if (completed != null) {
                    "SELECT count() FROM checkpoint WHERE user_id = user:\$userId AND quest_id = quest:\$questId AND is_completed = \$completed GROUP ALL"
                } else {
                    "SELECT count() FROM checkpoint WHERE user_id = user:\$userId AND quest_id = quest:\$questId GROUP ALL"
                }
            val params = mutableMapOf<String, Any?>("userId" to userId.value, "questId" to questId.value)
            if (completed != null) {
                params["completed"] = completed
            }
            val result =
                db
                    .queryBind(query, params)
                    .bind()
            parseCount(result)
        }

    private fun parseCheckpoint(result: String): Checkpoint? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToCheckpoint(it) }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse checkpoint: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse checkpoint: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse checkpoint: ${e.message}", e)
            null
        }

    private fun parseCheckpoints(result: String): List<Checkpoint> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToCheckpoint(it.jsonObject)
                } catch (e: SerializationException) {
                    logger.warn("Failed to parse checkpoint element: ${e.message}", e)
                    null
                } catch (e: IllegalStateException) {
                    logger.warn("Failed to parse checkpoint element: ${e.message}", e)
                    null
                } catch (e: IllegalArgumentException) {
                    logger.warn("Failed to parse checkpoint element: ${e.message}", e)
                    null
                }
            }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse checkpoints array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse checkpoints array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse checkpoints array: ${e.message}", e)
            emptyList()
        }

    private fun mapToCheckpoint(obj: kotlinx.serialization.json.JsonObject): Checkpoint {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val questIdStr = obj["quest_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return Checkpoint(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            questId = Ulid(questIdStr),
            title = obj["title"]?.jsonPrimitive?.content ?: "",
            sortOrder = obj["sort_order"]?.jsonPrimitive?.content?.toIntOrNull() ?: 0,
            isCompleted = obj["is_completed"]?.jsonPrimitive?.content?.toBooleanStrictOrNull() ?: false,
            completedAt = obj["completed_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
        )
    }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: SerializationException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            } catch (e: IllegalStateException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            } catch (e: IllegalArgumentException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
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
