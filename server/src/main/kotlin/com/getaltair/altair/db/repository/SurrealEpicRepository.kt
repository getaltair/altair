@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.EpicError
import com.getaltair.altair.domain.model.guidance.Epic
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.EpicStatus
import com.getaltair.altair.repository.EpicProgress
import com.getaltair.altair.repository.EpicRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.datetime.LocalDate
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory
import kotlin.time.Instant

class SurrealEpicRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : EpicRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<EpicError, Epic> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM epic WHERE id = epic:\$id AND user_id = user:\$userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).mapLeft { error ->
                        logger.warn("Database error in findById for ${id.value}: ERROR_MSG_PH (converting to NotFound)")
                        EpicError.NotFound(id)
                    }.bind()
            parseEpic(result) ?: raise(EpicError.NotFound(id))
        }

    override suspend fun save(entity: Epic): Either<EpicError, Epic> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                updateEpic(entity).bind()
            } else {
                insertEpic(entity).bind()
            }
            findById(entity.id).bind()
        }

    private suspend fun updateEpic(entity: Epic): Either<EpicError, Unit> =
        db
            .executeBind(
                """
                UPDATE epic:${'$'}id SET
                    title = ${'$'}title,
                    description = ${'$'}description,
                    status = ${'$'}status,
                    initiative_id = ${'$'}initiativeId,
                    target_date = ${'$'}targetDate,
                    completed_at = ${'$'}completedAt,
                    updated_at = time::now()
                WHERE user_id = user:${'$'}userId;
                """.trimIndent(),
                mapOf(
                    "id" to entity.id.value,
                    "title" to entity.title,
                    "description" to entity.description,
                    "status" to entity.status.name.lowercase(),
                    "initiativeId" to entity.initiativeId?.let { "initiative:${it.value}" },
                    "targetDate" to entity.targetDate?.toString(),
                    "completedAt" to entity.completedAt?.toString(),
                    "userId" to userId.value,
                ),
            ).mapLeft { error ->
                logger.warn("Database error updating ${entity.id.value}: ERROR_MSG_PH (converting to NotFound)")
                EpicError.NotFound(entity.id)
            }

    private suspend fun insertEpic(entity: Epic): Either<EpicError, Unit> =
        db
            .executeBind(
                """
                CREATE epic:${'$'}id CONTENT {
                    user_id: user:${'$'}userId,
                    title: ${'$'}title,
                    description: ${'$'}description,
                    status: ${'$'}status,
                    initiative_id: ${'$'}initiativeId,
                    target_date: ${'$'}targetDate,
                    completed_at: NONE
                };
                """.trimIndent(),
                mapOf(
                    "id" to entity.id.value,
                    "userId" to userId.value,
                    "title" to entity.title,
                    "description" to entity.description,
                    "status" to entity.status.name.lowercase(),
                    "initiativeId" to entity.initiativeId?.let { "initiative:${it.value}" },
                    "targetDate" to entity.targetDate?.toString(),
                ),
            ).mapLeft { error ->
                logger.warn("Database error inserting ${entity.id.value}: ERROR_MSG_PH (converting to NotFound)")
                EpicError.NotFound(entity.id)
            }

    override suspend fun delete(id: Ulid): Either<EpicError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE epic:${'$'}id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${'$'}userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).mapLeft { error ->
                    logger.warn("Database error in delete for ${id.value}: ERROR_MSG_PH (converting to NotFound)")
                    EpicError.NotFound(id)
                }.bind()
        }

    override fun findAll(): Flow<List<Epic>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM epic WHERE user_id = user:\$userId AND deleted_at IS NONE ORDER BY created_at DESC",
                    mapOf("userId" to userId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error: ERROR_MSG_PH")

                            is DomainError.UnexpectedError -> logger.warn("Database error: ERROR_MSG_PH")

                            is DomainError.NotFoundError -> logger.warn("Database error: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error: ${error.field} - ERROR_MSG_PH")

                            is DomainError.UnauthorizedError -> logger.warn("Database error: ERROR_MSG_PH")

                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseEpics(it) },
                ),
            )
        }

    override fun findByStatus(status: EpicStatus): Flow<List<Epic>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM epic WHERE user_id = user:\$userId AND status = \$status AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "status" to status.name.lowercase()),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error: ERROR_MSG_PH")

                            is DomainError.UnexpectedError -> logger.warn("Database error: ERROR_MSG_PH")

                            is DomainError.NotFoundError -> logger.warn("Database error: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error: ${error.field} - ERROR_MSG_PH")

                            is DomainError.UnauthorizedError -> logger.warn("Database error: ERROR_MSG_PH")

                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseEpics(it) },
                ),
            )
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<Epic>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM epic WHERE user_id = user:\$userId AND initiative_id = initiative:\$initiativeId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "initiativeId" to initiativeId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error: ERROR_MSG_PH")

                            is DomainError.UnexpectedError -> logger.warn("Database error: ERROR_MSG_PH")

                            is DomainError.NotFoundError -> logger.warn("Database error: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error: ${error.field} - ERROR_MSG_PH")

                            is DomainError.UnauthorizedError -> logger.warn("Database error: ERROR_MSG_PH")

                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseEpics(it) },
                ),
            )
        }

    override suspend fun getProgress(id: Ulid): Either<EpicError, EpicProgress> =
        either {
            findById(id).bind()
            val result =
                db
                    .queryBind(
                        """
                        SELECT
                            count() AS total,
                            count(status = 'completed') AS completed,
                            math::sum(energy_cost) AS total_energy,
                            math::sum(IF status = 'completed' THEN energy_cost ELSE 0 END) AS spent_energy
                        FROM quest
                        WHERE epic_id = epic:${'$'}id AND deleted_at IS NONE
                        GROUP ALL;
                        """.trimIndent(),
                        mapOf("id" to id.value),
                    ).mapLeft { error ->
                        logger.warn("Database error in getProgress for ${id.value}: ERROR_MSG_PH (converting to NotFound)")
                        EpicError.NotFound(id)
                    }.bind()
            parseProgress(result)
        }

    override fun findAllWithProgress(): Flow<List<Pair<Epic, EpicProgress>>> =
        flow {
            val epicsResult =
                db.queryBind(
                    "SELECT * FROM epic WHERE user_id = user:\$userId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value),
                )
            val epics =
                epicsResult.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findAllWithProgress: ERROR_MSG_PH")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findAllWithProgress: ERROR_MSG_PH")

                            is DomainError.NotFoundError -> logger.warn("Database error in findAllWithProgress: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findAllWithProgress: ${error.field} - ERROR_MSG_PH")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findAllWithProgress: ERROR_MSG_PH")

                            else -> logger.warn("Database error in findAllWithProgress: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseEpics(it) },
                )
            val withProgress =
                epics.map { epic ->
                    val progress =
                        getProgress(epic.id).fold(
                            { EpicProgress(0, 0, 0, 0) },
                            { it },
                        )
                    epic to progress
                }
            emit(withProgress)
        }

    private fun parseEpic(result: String): Epic? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToEpic(it) }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse epic: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse epic: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse epic: ${e.message}", e)
            null
        }

    private fun parseEpics(result: String): List<Epic> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToEpic(it.jsonObject)
                } catch (e: SerializationException) {
                    logger.warn("Failed to parse epic element: ${e.message}", e)
                    null
                } catch (e: IllegalStateException) {
                    logger.warn("Failed to parse epic element: ${e.message}", e)
                    null
                } catch (e: IllegalArgumentException) {
                    logger.warn("Failed to parse epic element: ${e.message}", e)
                    null
                }
            }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse epics array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse epics array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse epics array: ${e.message}", e)
            emptyList()
        }

    private fun mapToEpic(obj: kotlinx.serialization.json.JsonObject): Epic {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return Epic(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            title = obj["title"]?.jsonPrimitive?.content ?: "",
            description = obj["description"]?.jsonPrimitive?.content,
            status = EpicStatus.valueOf(obj["status"]?.jsonPrimitive?.content?.uppercase() ?: "ACTIVE"),
            initiativeId =
                obj["initiative_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            targetDate = obj["target_date"]?.jsonPrimitive?.content?.let { LocalDate.parse(it) },
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
            completedAt = obj["completed_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
            deletedAt = obj["deleted_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
        )
    }

    private fun parseProgress(result: String): EpicProgress =
        try {
            val obj =
                json
                    .parseToJsonElement(result)
                    .jsonArray
                    .firstOrNull()
                    ?.jsonObject
            EpicProgress(
                totalQuests =
                    obj
                        ?.get("total")
                        ?.jsonPrimitive
                        ?.content
                        ?.toIntOrNull() ?: 0,
                completedQuests =
                    obj
                        ?.get("completed")
                        ?.jsonPrimitive
                        ?.content
                        ?.toIntOrNull() ?: 0,
                totalEnergy =
                    obj
                        ?.get("total_energy")
                        ?.jsonPrimitive
                        ?.content
                        ?.toIntOrNull() ?: 0,
                spentEnergy =
                    obj
                        ?.get("spent_energy")
                        ?.jsonPrimitive
                        ?.content
                        ?.toIntOrNull() ?: 0,
            )
        } catch (e: SerializationException) {
            logger.warn("Failed to parse epic progress: ${e.message}", e)
            EpicProgress(0, 0, 0, 0)
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse epic progress: ${e.message}", e)
            EpicProgress(0, 0, 0, 0)
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse epic progress: ${e.message}", e)
            EpicProgress(0, 0, 0, 0)
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

    companion object {
        private val logger = LoggerFactory.getLogger(SurrealEpicRepository::class.java)
    }
}
