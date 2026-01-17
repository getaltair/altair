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
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlin.time.Clock

class SurrealCheckpointRepository(
    private val db: SurrealDbClient,
    private val userId: String,
) : CheckpointRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<DomainError, Checkpoint> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM checkpoint:${id.value} WHERE user_id = user:$userId",
                    ).bind()
            parseCheckpoint(result) ?: raise(DomainError.NotFoundError("Checkpoint", id.value))
        }

    override suspend fun save(entity: Checkpoint): Either<DomainError, Checkpoint> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .execute(
                        """
                        UPDATE checkpoint:${entity.id.value} SET
                            title = '${entity.title.replace("'", "''")}',
                            sort_order = ${entity.sortOrder},
                            is_completed = ${entity.isCompleted},
                            completed_at = ${entity.completedAt?.let { "<datetime>'$it'" } ?: "NONE"},
                            updated_at = time::now()
                        WHERE user_id = user:$userId;
                        """.trimIndent(),
                    ).bind()
            } else {
                db
                    .execute(
                        """
                        CREATE checkpoint:${entity.id.value} CONTENT {
                            user_id: user:$userId,
                            quest_id: quest:${entity.questId.value},
                            title: '${entity.title.replace("'", "''")}',
                            sort_order: ${entity.sortOrder},
                            is_completed: ${entity.isCompleted},
                            completed_at: ${entity.completedAt?.let { "<datetime>'$it'" } ?: "NONE"}
                        };
                        """.trimIndent(),
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            db.execute("DELETE checkpoint:${id.value} WHERE user_id = user:$userId;").bind()
        }

    override fun findAll(): Flow<List<Checkpoint>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM checkpoint WHERE user_id = user:$userId ORDER BY sort_order",
                )
            emit(result.fold({ emptyList() }, { parseCheckpoints(it) }))
        }

    override fun findByQuest(questId: Ulid): Flow<List<Checkpoint>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM checkpoint WHERE user_id = user:$userId AND quest_id = quest:${questId.value} ORDER BY sort_order",
                )
            emit(result.fold({ emptyList() }, { parseCheckpoints(it) }))
        }

    override suspend fun toggleComplete(id: Ulid): Either<DomainError, Checkpoint> =
        either {
            val checkpoint = findById(id).bind()
            val newCompleted = !checkpoint.isCompleted
            val completedAt = if (newCompleted) Clock.System.now() else null
            db
                .execute(
                    """
                    UPDATE checkpoint:${id.value} SET
                        is_completed = $newCompleted,
                        completed_at = ${completedAt?.let { "<datetime>'$it'" } ?: "NONE"},
                        updated_at = time::now()
                    WHERE user_id = user:$userId;
                    """.trimIndent(),
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
                    .execute(
                        "UPDATE checkpoint:${id.value} SET sort_order = $index, updated_at = time::now() WHERE user_id = user:$userId AND quest_id = quest:${questId.value};",
                    ).bind()
            }
        }

    override suspend fun countByQuest(
        questId: Ulid,
        completed: Boolean?,
    ): Either<DomainError, Int> =
        either {
            val filter = completed?.let { "AND is_completed = $it" } ?: ""
            val result =
                db
                    .query<Any>(
                        "SELECT count() FROM checkpoint WHERE user_id = user:$userId AND quest_id = quest:${questId.value} $filter GROUP ALL",
                    ).bind()
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
        } catch (e: Exception) {
            null
        }

    private fun parseCheckpoints(result: String): List<Checkpoint> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToCheckpoint(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
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
