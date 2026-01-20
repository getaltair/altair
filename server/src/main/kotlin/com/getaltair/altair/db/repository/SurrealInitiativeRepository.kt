@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.system.Initiative
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.InitiativeStatus
import com.getaltair.altair.repository.InitiativeRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.datetime.Instant
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

/**
 * SurrealDB implementation of InitiativeRepository.
 */
class SurrealInitiativeRepository(
    db: SurrealDbClient,
    userId: Ulid,
) : SurrealRepositoryBase(db, userId),
    InitiativeRepository {
    override suspend fun findById(id: Ulid): Either<DomainError, Initiative> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM initiative WHERE id = initiative:\$id AND user_id = user:\$userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()

            parseInitiative(result) ?: raise(DomainError.NotFoundError("Initiative", id.value))
        }

    override suspend fun save(entity: Initiative): Either<DomainError, Initiative> =
        either {
            val existing = findById(entity.id)

            if (existing.isRight()) {
                // Update
                db
                    .executeBind(
                        """
                        UPDATE initiative:${'$'}id SET
                            name = ${'$'}name,
                            description = ${'$'}description,
                            color = ${'$'}color,
                            icon = ${'$'}icon,
                            status = ${'$'}status,
                            updated_at = time::now()
                        WHERE user_id = user:${'$'}userId;
                        """.trimIndent(),
                        mapOf(
                            "id" to entity.id.value,
                            "name" to entity.name,
                            "description" to entity.description,
                            "color" to entity.color,
                            "icon" to entity.icon,
                            "status" to entity.status.name.lowercase(),
                            "userId" to userId.value,
                        ),
                    ).bind()
            } else {
                // Insert
                db
                    .executeBind(
                        """
                        CREATE initiative:${'$'}id CONTENT {
                            user_id: user:${'$'}userId,
                            name: ${'$'}name,
                            description: ${'$'}description,
                            color: ${'$'}color,
                            icon: ${'$'}icon,
                            status: ${'$'}status
                        };
                        """.trimIndent(),
                        mapOf(
                            "id" to entity.id.value,
                            "userId" to userId.value,
                            "name" to entity.name,
                            "description" to entity.description,
                            "color" to entity.color,
                            "icon" to entity.icon,
                            "status" to entity.status.name.lowercase(),
                        ),
                    ).bind()
            }

            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            softDelete("initiative", id.value).bind()
        }

    override fun findAll(): Flow<List<Initiative>> =
        queryAsFlow {
            either {
                val result =
                    db
                        .queryBind(
                            "SELECT * FROM initiative WHERE user_id = user:\$userId AND deleted_at IS NONE ORDER BY created_at DESC",
                            mapOf("userId" to userId.value),
                        ).bind()
                parseInitiatives(result)
            }
        }

    override fun findByStatus(status: InitiativeStatus): Flow<List<Initiative>> =
        queryAsFlow {
            either {
                val result =
                    db
                        .queryBind(
                            "SELECT * FROM initiative WHERE user_id = user:\$userId AND status = \$status AND deleted_at IS NONE",
                            mapOf("userId" to userId.value, "status" to status.name.lowercase()),
                        ).bind()
                parseInitiatives(result)
            }
        }

    override suspend fun searchByName(query: String): Either<DomainError, List<Initiative>> =
        either {
            val result =
                db
                    .queryBind(
                        """
                        SELECT * FROM initiative
                        WHERE user_id = user:${'$'}userId
                        AND deleted_at IS NONE
                        AND string::lowercase(name) CONTAINS string::lowercase(${'$'}query)
                        """.trimIndent(),
                        mapOf("userId" to userId.value, "query" to query),
                    ).bind()
            parseInitiatives(result)
        }

    override suspend fun countLinkedEntities(id: Ulid): Either<DomainError, Int> =
        either {
            val result =
                db
                    .queryBind(
                        """
                        SELECT count() as total FROM (
                            SELECT id FROM quest WHERE user_id = user:${'$'}userId AND initiative_id = initiative:${'$'}id AND deleted_at IS NONE
                            UNION ALL
                            SELECT id FROM epic WHERE user_id = user:${'$'}userId AND initiative_id = initiative:${'$'}id AND deleted_at IS NONE
                            UNION ALL
                            SELECT id FROM note WHERE user_id = user:${'$'}userId AND initiative_id = initiative:${'$'}id AND deleted_at IS NONE
                            UNION ALL
                            SELECT id FROM item WHERE user_id = user:${'$'}userId AND initiative_id = initiative:${'$'}id AND deleted_at IS NONE
                        ) GROUP ALL;
                        """.trimIndent(),
                        mapOf("userId" to userId.value, "id" to id.value),
                    ).bind()

            parseCount(result)
        }

    private fun parseInitiative(result: String): Initiative? {
        return try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject ?: return null
            mapToInitiative(obj)
        } catch (e: Exception) {
            logger.warn("Failed to parse Initiative: ${e.message}", e)
            null
        }
    }

    private fun parseInitiatives(result: String): List<Initiative> =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            array.mapNotNull { element ->
                try {
                    mapToInitiative(element.jsonObject)
                } catch (e: Exception) {
                    logger.warn("Failed to parse Initiative element: ${e.message}", e)
                    null
                }
            }
        } catch (e: Exception) {
            logger.warn("Failed to parse Initiative list: ${e.message}", e)
            emptyList()
        }

    private fun mapToInitiative(obj: kotlinx.serialization.json.JsonObject): Initiative {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException("Missing id")
        val userIdStr =
            obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":")
                ?: throw IllegalStateException("Missing user_id")
        return Initiative(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            name = obj["name"]?.jsonPrimitive?.content ?: throw IllegalStateException("Missing name"),
            description = obj["description"]?.jsonPrimitive?.content,
            color = obj["color"]?.jsonPrimitive?.content,
            icon = obj["icon"]?.jsonPrimitive?.content,
            status = InitiativeStatus.valueOf(obj["status"]?.jsonPrimitive?.content?.uppercase() ?: "ACTIVE"),
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
                logger.warn("Failed to parse Instant '$it': ${e.message}", e)
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST

    private fun parseCount(result: String): Int =
        try {
            val array = json.parseToJsonElement(result).jsonArray
            val obj = array.firstOrNull()?.jsonObject
            obj
                ?.get("total")
                ?.jsonPrimitive
                ?.content
                ?.toIntOrNull() ?: 0
        } catch (e: Exception) {
            logger.warn("Failed to parse count: ${e.message}", e)
            0
        }
}
