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
                    .query<Any>(
                        "SELECT * FROM initiative:${id.value} WHERE user_id = user:${userId.value} AND deleted_at IS NONE",
                    ).bind()

            parseInitiative(result) ?: raise(DomainError.NotFoundError("Initiative", id.value))
        }

    override suspend fun save(entity: Initiative): Either<DomainError, Initiative> =
        either {
            val existing = findById(entity.id)

            if (existing.isRight()) {
                // Update
                db
                    .execute(
                        """
                        UPDATE initiative:${entity.id.value} SET
                            name = '${entity.name.replace("'", "''")}',
                            description = ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            color = ${entity.color?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            icon = ${entity.icon?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            status = '${entity.status.name.lowercase()}',
                            updated_at = time::now()
                        WHERE user_id = user:${userId.value};
                        """.trimIndent(),
                    ).bind()
            } else {
                // Insert
                db
                    .execute(
                        """
                        CREATE initiative:${entity.id.value} CONTENT {
                            user_id: user:${userId.value},
                            name: '${entity.name.replace("'", "''")}',
                            description: ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            color: ${entity.color?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            icon: ${entity.icon?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            status: '${entity.status.name.lowercase()}'
                        };
                        """.trimIndent(),
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
                        .query<Any>(
                            "SELECT * FROM initiative WHERE user_id = user:${userId.value} AND deleted_at IS NONE ORDER BY created_at DESC",
                        ).bind()
                parseInitiatives(result)
            }
        }

    override fun findByStatus(status: InitiativeStatus): Flow<List<Initiative>> =
        queryAsFlow {
            either {
                val result =
                    db
                        .query<Any>(
                            "SELECT * FROM initiative WHERE user_id = user:${userId.value} AND status = '${status.name.lowercase()}' AND deleted_at IS NONE",
                        ).bind()
                parseInitiatives(result)
            }
        }

    override suspend fun searchByName(query: String): Either<DomainError, List<Initiative>> =
        either {
            val result =
                db
                    .query<Any>(
                        """
                        SELECT * FROM initiative
                        WHERE user_id = user:${userId.value}
                        AND deleted_at IS NONE
                        AND string::lowercase(name) CONTAINS string::lowercase('${query.replace("'", "''")}')
                        """.trimIndent(),
                    ).bind()
            parseInitiatives(result)
        }

    override suspend fun countLinkedEntities(id: Ulid): Either<DomainError, Int> =
        either {
            val result =
                db
                    .query<Any>(
                        """
                        SELECT count() as total FROM (
                            SELECT id FROM quest WHERE user_id = user:${userId.value} AND initiative_id = initiative:${id.value} AND deleted_at IS NONE
                            UNION ALL
                            SELECT id FROM epic WHERE user_id = user:${userId.value} AND initiative_id = initiative:${id.value} AND deleted_at IS NONE
                            UNION ALL
                            SELECT id FROM note WHERE user_id = user:${userId.value} AND initiative_id = initiative:${id.value} AND deleted_at IS NONE
                            UNION ALL
                            SELECT id FROM item WHERE user_id = user:${userId.value} AND initiative_id = initiative:${id.value} AND deleted_at IS NONE
                        ) GROUP ALL;
                        """.trimIndent(),
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
