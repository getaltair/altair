@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.ItemError
import com.getaltair.altair.domain.model.tracking.Container
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.ContainerRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory

class SurrealContainerRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : ContainerRepository {
    private val logger = LoggerFactory.getLogger(SurrealContainerRepository::class.java)
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<ItemError, Container> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM container:${'$'}id WHERE user_id = user:${'$'}userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).mapLeft { ItemError.NotFound(id) }
                    .bind()
            parseContainer(result) ?: raise(ItemError.NotFound(id))
        }

    override suspend fun save(entity: Container): Either<ItemError, Container> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .executeBind(
                        """
                        UPDATE container:${'$'}id SET
                            name = ${'$'}name,
                            description = ${'$'}description,
                            location_id = ${entity.locationId?.let { "location:${'$'}locationId" } ?: "NONE"},
                            parent_container_id = ${entity.parentContainerId?.let { "container:${'$'}parentContainerId" } ?: "NONE"},
                            label = ${'$'}label,
                            updated_at = time::now()
                        WHERE user_id = user:${'$'}userId
                        """.trimIndent(),
                        buildMap {
                            put("id", entity.id.value)
                            put("name", entity.name)
                            put("description", entity.description)
                            entity.locationId?.let { put("locationId", it.value) }
                            entity.parentContainerId?.let { put("parentContainerId", it.value) }
                            put("label", entity.label)
                            put("userId", userId.value)
                        },
                    ).mapLeft { ItemError.NotFound(entity.id) }
                    .bind()
            } else {
                db
                    .executeBind(
                        """
                        CREATE container:${'$'}id CONTENT {
                            user_id: user:${'$'}userId,
                            name: ${'$'}name,
                            description: ${'$'}description,
                            location_id: ${entity.locationId?.let { "location:${'$'}locationId" } ?: "NONE"},
                            parent_container_id: ${entity.parentContainerId?.let { "container:${'$'}parentContainerId" } ?: "NONE"},
                            label: ${'$'}label
                        }
                        """.trimIndent(),
                        buildMap {
                            put("id", entity.id.value)
                            put("userId", userId.value)
                            put("name", entity.name)
                            put("description", entity.description)
                            entity.locationId?.let { put("locationId", it.value) }
                            entity.parentContainerId?.let { put("parentContainerId", it.value) }
                            put("label", entity.label)
                        },
                    ).mapLeft { ItemError.NotFound(entity.id) }
                    .bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<ItemError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE container:${'$'}id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
        }

    override fun findAll(): Flow<List<Container>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM container WHERE user_id = user:${'$'}userId AND deleted_at IS NONE ORDER BY name",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseContainers(it) }))
        }

    override fun findByLocation(locationId: Ulid): Flow<List<Container>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM container WHERE user_id = user:${'$'}userId AND location_id = location:${'$'}locationId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "locationId" to locationId.value),
                )
            emit(result.fold({ emptyList() }, { parseContainers(it) }))
        }

    override fun findByParentContainer(parentContainerId: Ulid): Flow<List<Container>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM container WHERE user_id = user:${'$'}userId AND parent_container_id = container:${'$'}parentContainerId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "parentContainerId" to parentContainerId.value),
                )
            emit(result.fold({ emptyList() }, { parseContainers(it) }))
        }

    override fun findRoots(): Flow<List<Container>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM container WHERE user_id = user:${'$'}userId AND parent_container_id IS NONE AND deleted_at IS NONE ORDER BY name",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseContainers(it) }))
        }

    override suspend fun moveToLocation(
        id: Ulid,
        locationId: Ulid?,
    ): Either<ItemError, Container> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE container:${'$'}id SET location_id = ${locationId?.let { "location:${'$'}locationId" } ?: "NONE"}, parent_container_id = NONE, updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    buildMap {
                        put("id", id.value)
                        locationId?.let { put("locationId", it.value) }
                        put("userId", userId.value)
                    },
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
            findById(id).bind()
        }

    override suspend fun nestInContainer(
        id: Ulid,
        parentContainerId: Ulid,
    ): Either<ItemError, Container> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE container:${'$'}id SET parent_container_id = container:${'$'}parentContainerId, updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    mapOf("id" to id.value, "parentContainerId" to parentContainerId.value, "userId" to userId.value),
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
            findById(id).bind()
        }

    override suspend fun unnest(id: Ulid): Either<ItemError, Container> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE container:${'$'}id SET parent_container_id = NONE, updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
            findById(id).bind()
        }

    override suspend fun searchByNameOrLabel(query: String): Either<ItemError, List<Container>> =
        either {
            val result =
                db
                    .queryBind(
                        """
                        SELECT * FROM container WHERE user_id = user:${'$'}userId AND deleted_at IS NONE
                        AND (string::lowercase(name) CONTAINS string::lowercase(${'$'}query)
                             OR string::lowercase(label) CONTAINS string::lowercase(${'$'}query))
                        """.trimIndent(),
                        mapOf("userId" to userId.value, "query" to query),
                    ).mapLeft { ItemError.NotFound(Ulid.generate()) }
                    .bind()
            parseContainers(result)
        }

    override suspend fun countItems(id: Ulid): Either<ItemError, Int> =
        either {
            findById(id).bind()
            val result =
                db
                    .queryBind(
                        "SELECT count() FROM item WHERE user_id = user:${'$'}userId AND container_id = container:${'$'}containerId AND deleted_at IS NONE GROUP ALL",
                        mapOf("userId" to userId.value, "containerId" to id.value),
                    ).mapLeft { ItemError.NotFound(id) }
                    .bind()
            parseCount(result)
        }

    override suspend fun getPath(id: Ulid): Either<ItemError, List<Container>> =
        either {
            val path = mutableListOf<Container>()
            var current: Container? = findById(id).bind()
            while (current != null) {
                path.add(0, current)
                current = current.parentContainerId?.let { findById(it).getOrNull() }
            }
            path
        }

    private fun parseContainer(result: String): Container? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToContainer(it) }
        } catch (e: Exception) {
            logger.warn("Failed to parse Container: ${e.message}", e)
            null
        }

    private fun parseContainers(result: String): List<Container> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToContainer(it.jsonObject)
                } catch (e: Exception) {
                    logger.warn("Failed to parse Container element: ${e.message}", e)
                    null
                }
            }
        } catch (e: Exception) {
            logger.warn("Failed to parse Container list: ${e.message}", e)
            emptyList()
        }

    private fun mapToContainer(obj: kotlinx.serialization.json.JsonObject): Container {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return Container(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            name = obj["name"]?.jsonPrimitive?.content ?: "",
            description = obj["description"]?.jsonPrimitive?.content,
            locationId =
                obj["location_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            parentContainerId =
                obj["parent_container_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(
                        ":",
                    )?.let { Ulid(it) },
            label = obj["label"]?.jsonPrimitive?.content,
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
            deletedAt = obj["deleted_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
        )
    }

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
            logger.warn("Failed to parse count: ${e.message}", e)
            0
        }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: Exception) {
                logger.warn("Failed to parse Instant '$it': ${e.message}", e)
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
