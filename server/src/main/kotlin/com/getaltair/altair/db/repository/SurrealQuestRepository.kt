package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.ensure
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.QuestError
import com.getaltair.altair.domain.model.guidance.Quest
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.QuestStatus
import com.getaltair.altair.repository.QuestRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory

/**
 * SurrealDB implementation of QuestRepository.
 */
class SurrealQuestRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : QuestRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    companion object {
        private const val DEFAULT_WIP_LIMIT = 5
        private val logger = LoggerFactory.getLogger(SurrealQuestRepository::class.java)
    }

    override suspend fun findById(id: Ulid): Either<QuestError, Quest> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM quest:${id.value} WHERE user_id = user:${userId.value} AND deleted_at IS NONE",
                    ).mapLeft { QuestError.NotFound(id) }
                    .bind()

            parseQuest(result) ?: raise(QuestError.NotFound(id))
        }

    override suspend fun save(entity: Quest): Either<QuestError, Quest> =
        either {
            val existing = findById(entity.id)

            if (existing.isRight()) {
                // Update
                db
                    .execute(
                        """
                        UPDATE quest:${entity.id.value} SET
                            title = '${entity.title.replace("'", "''")}',
                            description = ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            energy_cost = ${entity.energyCost},
                            status = '${entity.status.name.lowercase()}',
                            epic_id = ${entity.epicId?.let { "epic:${it.value}" } ?: "NONE"},
                            routine_id = ${entity.routineId?.let { "routine:${it.value}" } ?: "NONE"},
                            initiative_id = ${entity.initiativeId?.let { "initiative:${it.value}" } ?: "NONE"},
                            due_date = ${entity.dueDate?.let { "'$it'" } ?: "NONE"},
                            scheduled_date = ${entity.scheduledDate?.let { "'$it'" } ?: "NONE"},
                            started_at = ${entity.startedAt?.let { "<datetime>'$it'" } ?: "NONE"},
                            completed_at = ${entity.completedAt?.let { "<datetime>'$it'" } ?: "NONE"},
                            updated_at = time::now()
                        WHERE user_id = user:${userId.value};
                        """.trimIndent(),
                    ).mapLeft { QuestError.NotFound(entity.id) }
                    .bind()
            } else {
                // Insert
                db
                    .execute(
                        """
                        CREATE quest:${entity.id.value} CONTENT {
                            user_id: user:${userId.value},
                            title: '${entity.title.replace("'", "''")}',
                            description: ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            energy_cost: ${entity.energyCost},
                            status: '${entity.status.name.lowercase()}',
                            epic_id: ${entity.epicId?.let { "epic:${it.value}" } ?: "NONE"},
                            routine_id: ${entity.routineId?.let { "routine:${it.value}" } ?: "NONE"},
                            initiative_id: ${entity.initiativeId?.let { "initiative:${it.value}" } ?: "NONE"},
                            due_date: ${entity.dueDate?.let { "'$it'" } ?: "NONE"},
                            scheduled_date: ${entity.scheduledDate?.let { "'$it'" } ?: "NONE"},
                            started_at: ${entity.startedAt?.let { "<datetime>'$it'" } ?: "NONE"},
                            completed_at: ${entity.completedAt?.let { "<datetime>'$it'" } ?: "NONE"}
                        };
                        """.trimIndent(),
                    ).mapLeft { QuestError.NotFound(entity.id) }
                    .bind()
            }

            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<QuestError, Unit> =
        either {
            findById(id).bind()
            db
                .execute(
                    """
                    UPDATE quest:${id.value} SET
                        deleted_at = time::now(),
                        updated_at = time::now()
                    WHERE user_id = user:${userId.value};
                    """.trimIndent(),
                ).mapLeft { QuestError.NotFound(id) }
                .bind()
        }

    override fun findAll(): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM quest WHERE user_id = user:${userId.value} AND deleted_at IS NONE ORDER BY created_at DESC",
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findByStatus(status: QuestStatus): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM quest WHERE user_id = user:${userId.value} AND status = '${status.name.lowercase()}' AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findActive(): Flow<List<Quest>> = findByStatus(QuestStatus.ACTIVE)

    override fun findByScheduledDate(date: LocalDate): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM quest WHERE user_id = user:${userId.value} AND scheduled_date = '$date' AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findDueByDate(date: LocalDate): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM quest WHERE user_id = user:${userId.value} AND due_date <= '$date' AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findByEpic(epicId: Ulid): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM quest WHERE user_id = user:${userId.value} AND epic_id = epic:${epicId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findByRoutine(routineId: Ulid): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM quest WHERE user_id = user:${userId.value} AND routine_id = routine:${routineId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM quest WHERE user_id = user:${userId.value} AND initiative_id = initiative:${initiativeId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override suspend fun transitionStatus(
        id: Ulid,
        newStatus: QuestStatus,
    ): Either<QuestError, Quest> =
        either {
            val quest = findById(id).bind()

            // Validate transition
            val validTransitions =
                mapOf(
                    QuestStatus.BACKLOG to setOf(QuestStatus.ACTIVE),
                    QuestStatus.ACTIVE to setOf(QuestStatus.COMPLETED, QuestStatus.ABANDONED),
                )

            val allowed = validTransitions[quest.status] ?: emptySet()
            ensure(newStatus in allowed) {
                QuestError.InvalidStatusTransition(id, quest.status, newStatus)
            }

            // Check WIP limit when transitioning to ACTIVE
            if (newStatus == QuestStatus.ACTIVE) {
                val currentActive = countActive().bind()
                ensure(currentActive < DEFAULT_WIP_LIMIT) {
                    QuestError.WipLimitExceeded(currentActive, DEFAULT_WIP_LIMIT)
                }
            }

            // Update with appropriate timestamps
            val now = "time::now()"
            val statusUpdate =
                when (newStatus) {
                    QuestStatus.ACTIVE -> "started_at = $now"
                    QuestStatus.COMPLETED, QuestStatus.ABANDONED -> "completed_at = $now"
                    else -> ""
                }

            db
                .execute(
                    """
                    UPDATE quest:${id.value} SET
                        status = '${newStatus.name.lowercase()}',
                        ${if (statusUpdate.isNotEmpty()) "$statusUpdate," else ""}
                        updated_at = $now
                    WHERE user_id = user:${userId.value};
                    """.trimIndent(),
                ).mapLeft { QuestError.NotFound(id) }
                .bind()

            findById(id).bind()
        }

    override suspend fun countActive(): Either<QuestError, Int> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT count() FROM quest WHERE user_id = user:${userId.value} AND status = 'active' AND deleted_at IS NONE GROUP ALL",
                    ).mapLeft { QuestError.NotFound(Ulid.generate()) }
                    .bind()

            parseCount(result)
        }

    private fun parseQuest(result: String): Quest? {
        return try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject ?: return null
            mapToQuest(obj)
        } catch (e: Exception) {
            logger.warn("Failed to parse quest: ${e.message}", e)
            null
        }
    }

    private fun parseQuests(result: String): List<Quest> =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            array.mapNotNull { element ->
                try {
                    mapToQuest(element.jsonObject)
                } catch (e: Exception) {
                    logger.warn("Failed to parse quest element: ${e.message}", e)
                    null
                }
            }
        } catch (e: Exception) {
            logger.warn("Failed to parse quests array: ${e.message}", e)
            emptyList()
        }

    private fun mapToQuest(obj: kotlinx.serialization.json.JsonObject): Quest {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException("Missing id")
        val userIdStr =
            obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":")
                ?: throw IllegalStateException("Missing user_id")
        return Quest(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            title = obj["title"]?.jsonPrimitive?.content ?: throw IllegalStateException("Missing title"),
            description = obj["description"]?.jsonPrimitive?.content,
            energyCost = obj["energy_cost"]?.jsonPrimitive?.content?.toIntOrNull() ?: 1,
            status = QuestStatus.valueOf(obj["status"]?.jsonPrimitive?.content?.uppercase() ?: "BACKLOG"),
            epicId =
                obj["epic_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            routineId =
                obj["routine_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            initiativeId =
                obj["initiative_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            dueDate = obj["due_date"]?.jsonPrimitive?.content?.let { LocalDate.parse(it) },
            scheduledDate = obj["scheduled_date"]?.jsonPrimitive?.content?.let { LocalDate.parse(it) },
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
            startedAt = obj["started_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
            completedAt = obj["completed_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
            deletedAt = obj["deleted_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
        )
    }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: Exception) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST

    private fun parseCount(result: String): Int =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject
            obj
                ?.get("count")
                ?.jsonPrimitive
                ?.content
                ?.toIntOrNull() ?: 0
        } catch (e: Exception) {
            logger.warn("Failed to parse count: ${e.message}", e)
            0
        }
}
