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
                    .query<Any>(
                        "SELECT * FROM location:${id.value} WHERE user_id = user:${userId.value} AND deleted_at IS NONE",
                    ).bind()
            parseLocation(result) ?: raise(DomainError.NotFoundError("Location", id.value))
        }

    override suspend fun save(entity: Location): Either<DomainError, Location> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .execute(
                        """
                        UPDATE location:${entity.id.value} SET
                            name = '${entity.name.replace("'", "''")}',
                            description = ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            parent_id = ${entity.parentId?.let { "location:${it.value}" } ?: "NONE"},
                            address = ${entity.address?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            updated_at = time::now()
                        WHERE user_id = user:${userId.value};
                        """.trimIndent(),
                    ).bind()
            } else {
                db
                    .execute(
                        """
                        CREATE location:${entity.id.value} CONTENT {
                            user_id: user:${userId.value},
                            name: '${entity.name.replace("'", "''")}',
                            description: ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            parent_id: ${entity.parentId?.let { "location:${it.value}" } ?: "NONE"},
                            address: ${entity.address?.let { "'${it.replace("'", "''")}'" } ?: "NONE"}
                        };
                        """.trimIndent(),
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE location:${id.value} SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).bind()
        }

    override fun findAll(): Flow<List<Location>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM location WHERE user_id = user:${userId.value} AND deleted_at IS NONE ORDER BY name",
                )
            emit(result.fold({ emptyList() }, { parseLocations(it) }))
        }

    override fun findRoots(): Flow<List<Location>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM location WHERE user_id = user:${userId.value} AND parent_id IS NONE AND deleted_at IS NONE ORDER BY name",
                )
            emit(result.fold({ emptyList() }, { parseLocations(it) }))
        }

    override fun findByParent(parentId: Ulid): Flow<List<Location>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM location WHERE user_id = user:${userId.value} AND parent_id = location:${parentId.value} AND deleted_at IS NONE ORDER BY name",
                )
            emit(result.fold({ emptyList() }, { parseLocations(it) }))
        }

    override fun findTree(): Flow<List<LocationNode>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM location WHERE user_id = user:${userId.value} AND deleted_at IS NONE ORDER BY name",
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
            val parentRef = newParentId?.let { "location:${it.value}" } ?: "NONE"
            db
                .execute(
                    "UPDATE location:${id.value} SET parent_id = $parentRef, updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).bind()
            findById(id).bind()
        }

    override suspend fun searchByName(query: String): Either<DomainError, List<Location>> =
        either {
            val escapedQuery = query.replace("'", "''")
            val result =
                db
                    .query<Any>(
                        """
                        SELECT * FROM location WHERE user_id = user:${userId.value}
                        AND string::lowercase(name) CONTAINS string::lowercase('$escapedQuery')
                        AND deleted_at IS NONE
                        """.trimIndent(),
                    ).bind()
            parseLocations(result)
        }

    override suspend fun countContents(id: Ulid): Either<DomainError, LocationContents> =
        either {
            findById(id).bind()
            val itemsResult =
                db
                    .query<Any>(
                        "SELECT count() FROM item WHERE user_id = user:${userId.value} AND location_id = location:${id.value} AND deleted_at IS NONE GROUP ALL",
                    ).bind()
            val containersResult =
                db
                    .query<Any>(
                        "SELECT count() FROM container WHERE user_id = user:${userId.value} AND location_id = location:${id.value} AND deleted_at IS NONE GROUP ALL",
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
