@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.system.Routine
import com.getaltair.altair.domain.types.Schedule
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.RoutineRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlin.time.Instant
import kotlinx.datetime.LocalDate
import kotlinx.datetime.LocalTime
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

/**
 * SurrealDB implementation of RoutineRepository.
 */
class SurrealRoutineRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : RoutineRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<DomainError, Routine> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM routine WHERE id = routine:\$id AND user_id = user:\$userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()
            parseRoutine(result) ?: raise(DomainError.NotFoundError("Routine", id.value))
        }

    override suspend fun save(entity: Routine): Either<DomainError, Routine> =
        either {
            val existing = findById(entity.id)

            if (existing.isRight()) {
                updateRoutine(entity).bind()
            } else {
                insertRoutine(entity).bind()
            }
            findById(entity.id).bind()
        }

    private suspend fun updateRoutine(entity: Routine): Either<DomainError, Unit> {
        val scheduleJson = json.encodeToString(Schedule.serializer(), entity.schedule)
        return db.executeBind(
            """
            UPDATE routine:${'$'}id SET
                title = ${'$'}title,
                description = ${'$'}description,
                energy_cost = ${'$'}energyCost,
                schedule = $scheduleJson,
                scheduled_time = ${'$'}scheduledTime,
                initiative_id = ${'$'}initiativeId,
                is_active = ${'$'}isActive,
                last_spawned_at = ${'$'}lastSpawnedAt,
                updated_at = time::now()
            WHERE user_id = user:${'$'}userId;
            """.trimIndent(),
            buildRoutineParams(entity, includeLastSpawned = true),
        )
    }

    private suspend fun insertRoutine(entity: Routine): Either<DomainError, Unit> {
        val scheduleJson = json.encodeToString(Schedule.serializer(), entity.schedule)
        return db.executeBind(
            """
            CREATE routine:${'$'}id CONTENT {
                user_id: user:${'$'}userId,
                title: ${'$'}title,
                description: ${'$'}description,
                energy_cost: ${'$'}energyCost,
                schedule: $scheduleJson,
                scheduled_time: ${'$'}scheduledTime,
                initiative_id: ${'$'}initiativeId,
                is_active: ${'$'}isActive,
                last_spawned_at: NONE
            };
            """.trimIndent(),
            buildRoutineParams(entity, includeLastSpawned = false),
        )
    }

    private fun buildRoutineParams(
        entity: Routine,
        includeLastSpawned: Boolean,
    ): Map<String, Any?> =
        buildMap {
            put("id", entity.id.value)
            put("userId", userId.value)
            put("title", entity.title)
            put("description", entity.description)
            put("energyCost", entity.energyCost)
            put("scheduledTime", entity.scheduledTime?.toString())
            put("initiativeId", entity.initiativeId?.let { "initiative:${it.value}" })
            put("isActive", entity.isActive)
            if (includeLastSpawned) {
                put("lastSpawnedAt", entity.lastSpawnedAt?.toString())
            }
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE routine:${'$'}id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${'$'}userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).bind()
        }

    override fun findAll(): Flow<List<Routine>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM routine WHERE user_id = user:\$userId AND deleted_at IS NONE ORDER BY created_at DESC",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseRoutines(it) }))
        }

    override fun findActive(): Flow<List<Routine>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM routine WHERE user_id = user:\$userId AND is_active = \$isActive AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "isActive" to true),
                )
            emit(result.fold({ emptyList() }, { parseRoutines(it) }))
        }

    override suspend fun findDueForDate(date: LocalDate): Either<DomainError, List<Routine>> =
        either {
            // For simplicity, return all active routines; schedule matching would need business logic
            val result =
                db
                    .queryBind(
                        "SELECT * FROM routine WHERE user_id = user:\$userId AND is_active = \$isActive AND deleted_at IS NONE",
                        mapOf("userId" to userId.value, "isActive" to true),
                    ).bind()
            parseRoutines(result)
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<Routine>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM routine WHERE user_id = user:\$userId AND initiative_id = initiative:\$initiativeId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "initiativeId" to initiativeId.value),
                )
            emit(result.fold({ emptyList() }, { parseRoutines(it) }))
        }

    override suspend fun updateLastSpawnedAt(
        id: Ulid,
        spawnedAt: Instant,
    ): Either<DomainError, Routine> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE routine:${'$'}id SET last_spawned_at = ${'$'}spawnedAt, updated_at = time::now() WHERE user_id = user:${'$'}userId;",
                    mapOf("id" to id.value, "spawnedAt" to spawnedAt.toString(), "userId" to userId.value),
                ).bind()
            findById(id).bind()
        }

    private fun parseRoutine(result: String): Routine? =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            array.firstOrNull()?.jsonObject?.let { mapToRoutine(it) }
        } catch (e: Exception) {
            null
        }

    private fun parseRoutines(result: String): List<Routine> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToRoutine(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
            emptyList()
        }

    private fun mapToRoutine(obj: kotlinx.serialization.json.JsonObject): Routine {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val scheduleObj = obj["schedule"]?.jsonObject
        val schedule =
            if (scheduleObj != null) {
                try {
                    json.decodeFromJsonElement(Schedule.serializer(), scheduleObj)
                } catch (
                    e: Exception,
                ) {
                    Schedule.Daily
                }
            } else {
                Schedule.Daily
            }

        return Routine(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            title = obj["title"]?.jsonPrimitive?.content ?: "",
            description = obj["description"]?.jsonPrimitive?.content,
            energyCost = obj["energy_cost"]?.jsonPrimitive?.content?.toIntOrNull() ?: 1,
            schedule = schedule,
            scheduledTime = obj["scheduled_time"]?.jsonPrimitive?.content?.let { LocalTime.parse(it) },
            initiativeId =
                obj["initiative_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            isActive = obj["is_active"]?.jsonPrimitive?.content?.toBooleanStrictOrNull() ?: true,
            lastSpawnedAt = obj["last_spawned_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
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
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
