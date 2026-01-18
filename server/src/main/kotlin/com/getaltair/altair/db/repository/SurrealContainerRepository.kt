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
                    .query<Any>(
                        "SELECT * FROM container WHERE id = container:${id.value} AND user_id = user:${userId.value} AND deleted_at IS NONE",
                    ).mapLeft { ItemError.NotFound(id) }
                    .bind()
            parseContainer(result) ?: raise(ItemError.NotFound(id))
        }

    override suspend fun save(entity: Container): Either<ItemError, Container> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .execute(
                        """
                        UPDATE container:${entity.id.value} SET
                            name = '${entity.name.replace("'", "''")}',
                            description = ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            location_id = ${entity.locationId?.let { "location:${it.value}" } ?: "NONE"},
                            parent_container_id = ${entity.parentContainerId?.let {
                            "container:${it.value}"
                        } ?: "NONE"},
                            label = ${entity.label?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            updated_at = time::now()
                        WHERE user_id = user:${userId.value};
                        """.trimIndent(),
                    ).mapLeft { ItemError.NotFound(entity.id) }
                    .bind()
            } else {
                db
                    .execute(
                        """
                        CREATE container:${entity.id.value} CONTENT {
                            user_id: user:${userId.value},
                            name: '${entity.name.replace("'", "''")}',
                            description: ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            location_id: ${entity.locationId?.let { "location:${it.value}" } ?: "NONE"},
                            parent_container_id: ${entity.parentContainerId?.let { "container:${it.value}" } ?: "NONE"},
                            label: ${entity.label?.let { "'${it.replace("'", "''")}'" } ?: "NONE"}
                        };
                        """.trimIndent(),
                    ).mapLeft { ItemError.NotFound(entity.id) }
                    .bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<ItemError, Unit> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE container:${id.value} SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
        }

    override fun findAll(): Flow<List<Container>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM container WHERE user_id = user:${userId.value} AND deleted_at IS NONE ORDER BY name",
                )
            emit(result.fold({ emptyList() }, { parseContainers(it) }))
        }

    override fun findByLocation(locationId: Ulid): Flow<List<Container>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM container WHERE user_id = user:${userId.value} AND location_id = location:${locationId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseContainers(it) }))
        }

    override fun findByParentContainer(parentContainerId: Ulid): Flow<List<Container>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM container WHERE user_id = user:${userId.value} AND parent_container_id = container:${parentContainerId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseContainers(it) }))
        }

    override fun findRoots(): Flow<List<Container>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM container WHERE user_id = user:${userId.value} AND parent_container_id IS NONE AND deleted_at IS NONE ORDER BY name",
                )
            emit(result.fold({ emptyList() }, { parseContainers(it) }))
        }

    override suspend fun moveToLocation(
        id: Ulid,
        locationId: Ulid?,
    ): Either<ItemError, Container> =
        either {
            findById(id).bind()
            val locRef = locationId?.let { "location:${it.value}" } ?: "NONE"
            db
                .execute(
                    "UPDATE container:${id.value} SET location_id = $locRef, parent_container_id = NONE, updated_at = time::now() WHERE user_id = user:${userId.value};",
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
                .execute(
                    "UPDATE container:${id.value} SET parent_container_id = container:${parentContainerId.value}, updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
            findById(id).bind()
        }

    override suspend fun unnest(id: Ulid): Either<ItemError, Container> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE container:${id.value} SET parent_container_id = NONE, updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
            findById(id).bind()
        }

    override suspend fun searchByNameOrLabel(query: String): Either<ItemError, List<Container>> =
        either {
            val result =
                db
                    .query<Any>(
                        """
                        SELECT * FROM container WHERE user_id = user:${userId.value} AND deleted_at IS NONE
                        AND (string::lowercase(name) CONTAINS string::lowercase('${query.replace("'", "''")}')
                             OR string::lowercase(label) CONTAINS string::lowercase('${query.replace("'", "''")}'))
                        """.trimIndent(),
                    ).mapLeft { ItemError.NotFound(Ulid.generate()) }
                    .bind()
            parseContainers(result)
        }

    override suspend fun countItems(id: Ulid): Either<ItemError, Int> =
        either {
            findById(id).bind()
            val result =
                db
                    .query<Any>(
                        "SELECT count() FROM item WHERE user_id = user:${userId.value} AND container_id = container:${id.value} AND deleted_at IS NONE GROUP ALL",
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
