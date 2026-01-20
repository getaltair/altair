@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.tracking.Location
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.LocationContents
import com.getaltair.altair.repository.LocationNode
import com.getaltair.altair.repository.LocationRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

class SurrealLocationRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : LocationRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<DomainError, Location> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM location:${'$'}id WHERE user_id = user:${'$'}userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()
            parseLocation(result) ?: raise(DomainError.NotFoundError("Location", id.value))
        }

    override suspend fun save(entity: Location): Either<DomainError, Location> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .executeBind(
                        """
                        UPDATE location:${'$'}id SET
                            name = ${'$'}name,
                            description = ${'$'}description,
                            parent_id = ${entity.parentId?.let { "location:${'$'}parentId" } ?: "NONE"},
                            address = ${'$'}address,
                            updated_at = time::now()
                        WHERE user_id = user:${'$'}userId
                        """.trimIndent(),
                        buildMap {
                            put("id", entity.id.value)
                            put("name", entity.name)
                            put("description", entity.description)
                            entity.parentId?.let { put("parentId", it.value) }
                            put("address", entity.address)
                            put("userId", userId.value)
                        },
                    ).bind()
            } else {
                db
                    .executeBind(
                        """
                        CREATE location:${'$'}id CONTENT {
                            user_id: user:${'$'}userId,
                            name: ${'$'}name,
                            description: ${'$'}description,
                            parent_id: ${entity.parentId?.let { "location:${'$'}parentId" } ?: "NONE"},
                            address: ${'$'}address
                        }
                        """.trimIndent(),
                        buildMap {
                            put("id", entity.id.value)
                            put("userId", userId.value)
                            put("name", entity.name)
                            put("description", entity.description)
                            entity.parentId?.let { put("parentId", it.value) }
                            put("address", entity.address)
                        },
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE location:${'$'}id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).bind()
        }

    override fun findAll(): Flow<List<Location>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM location WHERE user_id = user:${'$'}userId AND deleted_at IS NONE ORDER BY name",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseLocations(it) }))
        }

    override fun findRoots(): Flow<List<Location>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM location WHERE user_id = user:${'$'}userId AND parent_id IS NONE AND deleted_at IS NONE ORDER BY name",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseLocations(it) }))
        }

    override fun findByParent(parentId: Ulid): Flow<List<Location>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM location WHERE user_id = user:${'$'}userId AND parent_id = location:${'$'}parentId AND deleted_at IS NONE ORDER BY name",
                    mapOf("userId" to userId.value, "parentId" to parentId.value),
                )
            emit(result.fold({ emptyList() }, { parseLocations(it) }))
        }

    override fun findTree(): Flow<List<LocationNode>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM location WHERE user_id = user:${'$'}userId AND deleted_at IS NONE ORDER BY name",
                    mapOf("userId" to userId.value),
                )
            val locations = result.fold({ emptyList() }, { parseLocations(it) })
            emit(buildTree(locations))
        }

    override suspend fun getPath(id: Ulid): Either<DomainError, List<Location>> =
        either {
            val path = mutableListOf<Location>()
            var current: Location? = findById(id).bind()
            while (current != null) {
                path.add(0, current)
                current = current.parentId?.let { findById(it).getOrNull() }
            }
            path
        }

    override suspend fun move(
        id: Ulid,
        newParentId: Ulid?,
    ): Either<DomainError, Location> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE location:${'$'}id SET parent_id = ${newParentId?.let { "location:${'$'}parentId" } ?: "NONE"}, updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    buildMap {
                        put("id", id.value)
                        newParentId?.let { put("parentId", it.value) }
                        put("userId", userId.value)
                    },
                ).bind()
            findById(id).bind()
        }

    override suspend fun searchByName(query: String): Either<DomainError, List<Location>> =
        either {
            val result =
                db
                    .queryBind(
                        """
                        SELECT * FROM location WHERE user_id = user:${'$'}userId
                        AND string::lowercase(name) CONTAINS string::lowercase(${'$'}query)
                        AND deleted_at IS NONE
                        """.trimIndent(),
                        mapOf("userId" to userId.value, "query" to query),
                    ).bind()
            parseLocations(result)
        }

    override suspend fun countContents(id: Ulid): Either<DomainError, LocationContents> =
        either {
            findById(id).bind()
            val itemsResult =
                db
                    .queryBind(
                        "SELECT count() FROM item WHERE user_id = user:${'$'}userId AND location_id = location:${'$'}locationId AND deleted_at IS NONE GROUP ALL",
                        mapOf("userId" to userId.value, "locationId" to id.value),
                    ).bind()
            val containersResult =
                db
                    .queryBind(
                        "SELECT count() FROM container WHERE user_id = user:${'$'}userId AND location_id = location:${'$'}locationId AND deleted_at IS NONE GROUP ALL",
                        mapOf("userId" to userId.value, "locationId" to id.value),
                    ).bind()
            LocationContents(
                itemCount = parseCount(itemsResult),
                containerCount = parseCount(containersResult),
            )
        }

    private fun buildTree(locations: List<Location>): List<LocationNode> {
        val byParent = locations.groupBy { it.parentId }

        fun buildNodes(parentId: Ulid?): List<LocationNode> =
            byParent[parentId]?.map { location ->
                LocationNode(location, buildNodes(location.id))
            } ?: emptyList()
        return buildNodes(null)
    }

    private fun parseLocation(result: String): Location? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToLocation(it) }
        } catch (e: Exception) {
            null
        }

    private fun parseLocations(result: String): List<Location> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToLocation(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
            emptyList()
        }

    private fun mapToLocation(obj: kotlinx.serialization.json.JsonObject): Location {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return Location(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            name = obj["name"]?.jsonPrimitive?.content ?: "",
            description = obj["description"]?.jsonPrimitive?.content,
            parentId =
                obj["parent_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            address = obj["address"]?.jsonPrimitive?.content,
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
            0
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
