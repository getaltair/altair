@file:Suppress("detekt:MaxLineLength")

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
import kotlin.time.Instant
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
                    .queryBind(
                        "SELECT * FROM quest WHERE id = quest:\$id AND user_id = user:\$userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).mapLeft { QuestError.NotFound(id) }
                    .bind()

            parseQuest(result) ?: raise(QuestError.NotFound(id))
        }

    override suspend fun save(entity: Quest): Either<QuestError, Quest> =
        either {
            val existing = findById(entity.id)

            if (existing.isRight()) {
                updateQuest(entity).bind()
            } else {
                insertQuest(entity).bind()
            }

            findById(entity.id).bind()
        }

    private suspend fun updateQuest(entity: Quest): Either<QuestError, Unit> =
        db
            .executeBind(
                """
                UPDATE quest:${'$'}id SET
                    title = ${'$'}title,
                    description = ${'$'}description,
                    energy_cost = ${'$'}energyCost,
                    status = ${'$'}status,
                    epic_id = ${'$'}epicId,
                    routine_id = ${'$'}routineId,
                    initiative_id = ${'$'}initiativeId,
                    due_date = ${'$'}dueDate,
                    scheduled_date = ${'$'}scheduledDate,
                    started_at = ${'$'}startedAt,
                    completed_at = ${'$'}completedAt,
                    updated_at = time::now()
                WHERE user_id = user:${'$'}userId;
                """.trimIndent(),
                buildQuestParams(entity),
            ).mapLeft { QuestError.NotFound(entity.id) }

    private suspend fun insertQuest(entity: Quest): Either<QuestError, Unit> =
        db
            .executeBind(
                """
                CREATE quest:${'$'}id CONTENT {
                    user_id: user:${'$'}userId,
                    title: ${'$'}title,
                    description: ${'$'}description,
                    energy_cost: ${'$'}energyCost,
                    status: ${'$'}status,
                    epic_id: ${'$'}epicId,
                    routine_id: ${'$'}routineId,
                    initiative_id: ${'$'}initiativeId,
                    due_date: ${'$'}dueDate,
                    scheduled_date: ${'$'}scheduledDate,
                    started_at: ${'$'}startedAt,
                    completed_at: ${'$'}completedAt
                };
                """.trimIndent(),
                buildQuestParams(entity),
            ).mapLeft { QuestError.NotFound(entity.id) }

    private fun buildQuestParams(entity: Quest): Map<String, Any?> =
        mapOf(
            "id" to entity.id.value,
            "userId" to userId.value,
            "title" to entity.title,
            "description" to entity.description,
            "energyCost" to entity.energyCost,
            "status" to entity.status.name.lowercase(),
            "epicId" to entity.epicId?.let { "epic:${it.value}" },
            "routineId" to entity.routineId?.let { "routine:${it.value}" },
            "initiativeId" to entity.initiativeId?.let { "initiative:${it.value}" },
            "dueDate" to entity.dueDate?.toString(),
            "scheduledDate" to entity.scheduledDate?.toString(),
            "startedAt" to entity.startedAt?.toString(),
            "completedAt" to entity.completedAt?.toString(),
        )

    override suspend fun delete(id: Ulid): Either<QuestError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    """
                    UPDATE quest:${'$'}id SET
                        deleted_at = time::now(),
                        updated_at = time::now()
                    WHERE user_id = user:${'$'}userId;
                    """.trimIndent(),
                    mapOf("id" to id.value, "userId" to userId.value),
                ).mapLeft { QuestError.NotFound(id) }
                .bind()
        }

    override fun findAll(): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.queryBind(
                    "SELECT * FROM quest WHERE user_id = user:\$userId AND deleted_at IS NONE ORDER BY created_at DESC",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findByStatus(status: QuestStatus): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.queryBind(
                    "SELECT * FROM quest WHERE user_id = user:\$userId AND status = \$status AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "status" to status.name.lowercase()),
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findActive(): Flow<List<Quest>> = findByStatus(QuestStatus.ACTIVE)

    override fun findByScheduledDate(date: LocalDate): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.queryBind(
                    "SELECT * FROM quest WHERE user_id = user:\$userId AND scheduled_date = \$date AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "date" to date.toString()),
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findDueByDate(date: LocalDate): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.queryBind(
                    "SELECT * FROM quest WHERE user_id = user:\$userId AND due_date <= \$date AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "date" to date.toString()),
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findByEpic(epicId: Ulid): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.queryBind(
                    "SELECT * FROM quest WHERE user_id = user:\$userId AND epic_id = epic:\$epicId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "epicId" to epicId.value),
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findByRoutine(routineId: Ulid): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.queryBind(
                    "SELECT * FROM quest WHERE user_id = user:\$userId AND routine_id = routine:\$routineId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "routineId" to routineId.value),
                )
            emit(result.fold({ emptyList() }, { parseQuests(it) }))
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<Quest>> =
        kotlinx.coroutines.flow.flow {
            val result =
                db.queryBind(
                    "SELECT * FROM quest WHERE user_id = user:\$userId AND initiative_id = initiative:\$initiativeId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "initiativeId" to initiativeId.value),
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
                .executeBind(
                    """
                    UPDATE quest:${'$'}id SET
                        status = ${'$'}status,
                        ${if (statusUpdate.isNotEmpty()) "$statusUpdate," else ""}
                        updated_at = $now
                    WHERE user_id = user:${'$'}userId;
                    """.trimIndent(),
                    mapOf("id" to id.value, "status" to newStatus.name.lowercase(), "userId" to userId.value),
                ).mapLeft { QuestError.NotFound(id) }
                .bind()

            findById(id).bind()
        }

    override suspend fun countActive(): Either<QuestError, Int> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT count() FROM quest WHERE user_id = user:\$userId AND status = \$status AND deleted_at IS NONE GROUP ALL",
                        mapOf("userId" to userId.value, "status" to "active"),
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
