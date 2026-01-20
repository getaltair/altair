@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.EpicError
import com.getaltair.altair.domain.model.guidance.Epic
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.EpicStatus
import com.getaltair.altair.repository.EpicProgress
import com.getaltair.altair.repository.EpicRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.datetime.Instant
import kotlinx.datetime.LocalDate
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

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
                    ).mapLeft { EpicError.NotFound(id) }
                    .bind()
            parseEpic(result) ?: raise(EpicError.NotFound(id))
        }

    override suspend fun save(entity: Epic): Either<EpicError, Epic> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
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
                    ).mapLeft { EpicError.NotFound(entity.id) }
                    .bind()
            } else {
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
                    ).mapLeft { EpicError.NotFound(entity.id) }
                    .bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<EpicError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE epic:${'$'}id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${'$'}userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).mapLeft { EpicError.NotFound(id) }
                .bind()
        }

    override fun findAll(): Flow<List<Epic>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM epic WHERE user_id = user:\$userId AND deleted_at IS NONE ORDER BY created_at DESC",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseEpics(it) }))
        }

    override fun findByStatus(status: EpicStatus): Flow<List<Epic>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM epic WHERE user_id = user:\$userId AND status = \$status AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "status" to status.name.lowercase()),
                )
            emit(result.fold({ emptyList() }, { parseEpics(it) }))
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<Epic>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM epic WHERE user_id = user:\$userId AND initiative_id = initiative:\$initiativeId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "initiativeId" to initiativeId.value),
                )
            emit(result.fold({ emptyList() }, { parseEpics(it) }))
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
                    ).mapLeft { EpicError.NotFound(id) }
                    .bind()
            parseProgress(result)
        }

    override fun findAllWithProgress(): Flow<List<Pair<Epic, EpicProgress>>> =
        flow {
            val epicsResult =
                db.queryBind(
                    "SELECT * FROM epic WHERE user_id = user:\$userId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value),
                )
            val epics = epicsResult.fold({ emptyList() }, { parseEpics(it) })
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
        } catch (e: Exception) {
            null
        }

    private fun parseEpics(result: String): List<Epic> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToEpic(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
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
        } catch (e: Exception) {
            EpicProgress(0, 0, 0, 0)
        }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: Exception) {
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
