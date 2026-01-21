@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.ItemError
import com.getaltair.altair.domain.model.tracking.Item
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.ItemRepository
import com.getaltair.altair.repository.PageRequest
import com.getaltair.altair.repository.PageResult
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory
import kotlin.time.Instant

class SurrealItemRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : ItemRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    companion object {
        private val logger = LoggerFactory.getLogger(SurrealItemRepository::class.java)
    }

    override suspend fun findById(id: Ulid): Either<ItemError, Item> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM item:${'$'}id WHERE user_id = user:${'$'}userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).mapLeft { error ->
                        logger.warn("Database error for ${id.value}: ERROR_MSG (converting to NotFound)")
                        ItemError.NotFound(id)
                    }.bind()
            parseItem(result) ?: raise(ItemError.NotFound(id))
        }

    override suspend fun save(entity: Item): Either<ItemError, Item> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                updateItem(entity).bind()
            } else {
                insertItem(entity).bind()
            }
            findById(entity.id).bind()
        }

    private suspend fun updateItem(entity: Item): Either<ItemError, Unit> =
        db
            .executeBind(
                """
                UPDATE item:${'$'}id SET
                    name = ${'$'}name,
                    description = ${'$'}description,
                    template_id = ${entity.templateId?.let { "item_template:${'$'}templateId" } ?: "NONE"},
                    location_id = ${entity.locationId?.let { "location:${'$'}locationId" } ?: "NONE"},
                    container_id = ${entity.containerId?.let { "container:${'$'}containerId" } ?: "NONE"},
                    quantity = ${'$'}quantity,
                    photo_attachment_id = ${entity.photoAttachmentId?.let { "attachment:${'$'}photoAttachmentId" } ?: "NONE"},
                    initiative_id = ${entity.initiativeId?.let { "initiative:${'$'}initiativeId" } ?: "NONE"},
                    updated_at = time::now()
                WHERE user_id = user:${'$'}userId
                """.trimIndent(),
                buildItemParams(entity),
            ).mapLeft { error ->
                logger.warn("Database error for ${entity.id.value}: ERROR_MSG (converting to NotFound)")
                ItemError.NotFound(entity.id)
            }

    private suspend fun insertItem(entity: Item): Either<ItemError, Unit> =
        db
            .executeBind(
                """
                CREATE item:${'$'}id CONTENT {
                    user_id: user:${'$'}userId,
                    name: ${'$'}name,
                    description: ${'$'}description,
                    template_id: ${entity.templateId?.let { "item_template:${'$'}templateId" } ?: "NONE"},
                    location_id: ${entity.locationId?.let { "location:${'$'}locationId" } ?: "NONE"},
                    container_id: ${entity.containerId?.let { "container:${'$'}containerId" } ?: "NONE"},
                    quantity: ${'$'}quantity,
                    photo_attachment_id: ${entity.photoAttachmentId?.let { "attachment:${'$'}photoAttachmentId" } ?: "NONE"},
                    initiative_id: ${entity.initiativeId?.let { "initiative:${'$'}initiativeId" } ?: "NONE"}
                }
                """.trimIndent(),
                buildItemParams(entity),
            ).mapLeft { error ->
                logger.warn("Database error for ${entity.id.value}: ERROR_MSG (converting to NotFound)")
                ItemError.NotFound(entity.id)
            }

    private fun buildItemParams(entity: Item): Map<String, Any?> =
        buildMap {
            put("id", entity.id.value)
            put("userId", userId.value)
            put("name", entity.name)
            put("description", entity.description)
            entity.templateId?.let { put("templateId", it.value) }
            entity.locationId?.let { put("locationId", it.value) }
            entity.containerId?.let { put("containerId", it.value) }
            put("quantity", entity.quantity)
            entity.photoAttachmentId?.let { put("photoAttachmentId", it.value) }
            entity.initiativeId?.let { put("initiativeId", it.value) }
        }

    override suspend fun delete(id: Ulid): Either<ItemError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE item:${'$'}id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).mapLeft { error ->
                    logger.warn("Database error for ${id.value}: ERROR_MSG (converting to NotFound)")
                    ItemError.NotFound(id)
                }.bind()
        }

    override fun findAll(): Flow<List<Item>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM item WHERE user_id = user:${'$'}userId AND deleted_at IS NONE ORDER BY name",
                    mapOf("userId" to userId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findAll: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findAll: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findAll: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findAll: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findAll: ERROR_MSG")

                            else -> logger.warn("Database error in findAll: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseItems(it) },
                ),
            )
        }

    override fun findByLocation(locationId: Ulid): Flow<List<Item>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM item WHERE user_id = user:${'$'}userId AND location_id = location:${'$'}locationId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "locationId" to locationId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findByLocation: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findByLocation: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findByLocation: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findByLocation: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findByLocation: ERROR_MSG")

                            else -> logger.warn("Database error in findByLocation: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseItems(it) },
                ),
            )
        }

    override fun findByContainer(containerId: Ulid): Flow<List<Item>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM item WHERE user_id = user:${'$'}userId AND container_id = container:${'$'}containerId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "containerId" to containerId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findByContainer: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findByContainer: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findByContainer: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findByContainer: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findByContainer: ERROR_MSG")

                            else -> logger.warn("Database error in findByContainer: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseItems(it) },
                ),
            )
        }

    override fun findByTemplate(templateId: Ulid): Flow<List<Item>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM item WHERE user_id = user:${'$'}userId AND template_id = item_template:${'$'}templateId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "templateId" to templateId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findByTemplate: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findByTemplate: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findByTemplate: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findByTemplate: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findByTemplate: ERROR_MSG")

                            else -> logger.warn("Database error in findByTemplate: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseItems(it) },
                ),
            )
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<Item>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM item WHERE user_id = user:${'$'}userId AND initiative_id = initiative:${'$'}initiativeId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "initiativeId" to initiativeId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findByInitiative: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findByInitiative: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findByInitiative: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findByInitiative: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findByInitiative: ERROR_MSG")

                            else -> logger.warn("Database error in findByInitiative: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseItems(it) },
                ),
            )
        }

    override suspend fun searchByName(query: String): Either<ItemError, List<Item>> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM item WHERE user_id = user:${'$'}userId AND string::lowercase(name) CONTAINS string::lowercase(${'$'}query) AND deleted_at IS NONE",
                        mapOf("userId" to userId.value, "query" to query),
                    ).mapLeft { error ->
                        logger.warn("Database error in search: ERROR_MSG (converting to NotFound)")
                        ItemError.NotFound(Ulid.generate())
                    }.bind()
            parseItems(result)
        }

    override suspend fun searchByNamePaged(
        query: String,
        page: PageRequest,
    ): Either<ItemError, PageResult<Item>> =
        either {
            val items = searchByName(query).bind()
            val paged = items.drop(page.offset).take(page.limit)
            PageResult(items = paged, totalCount = items.size, hasMore = page.offset + page.limit < items.size)
        }

    override suspend fun moveToLocation(
        id: Ulid,
        locationId: Ulid,
    ): Either<ItemError, Item> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE item:${'$'}id SET location_id = location:${'$'}locationId, container_id = NONE, updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    mapOf("id" to id.value, "locationId" to locationId.value, "userId" to userId.value),
                ).mapLeft { error ->
                    logger.warn("Database error for ${id.value}: ERROR_MSG (converting to NotFound)")
                    ItemError.NotFound(id)
                }.bind()
            findById(id).bind()
        }

    override suspend fun moveToContainer(
        id: Ulid,
        containerId: Ulid,
    ): Either<ItemError, Item> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE item:${'$'}id SET container_id = container:${'$'}containerId, updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    mapOf("id" to id.value, "containerId" to containerId.value, "userId" to userId.value),
                ).mapLeft { error ->
                    logger.warn("Database error for ${id.value}: ERROR_MSG (converting to NotFound)")
                    ItemError.NotFound(id)
                }.bind()
            findById(id).bind()
        }

    override suspend fun updateQuantity(
        id: Ulid,
        quantity: Int,
    ): Either<ItemError, Item> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE item:${'$'}id SET quantity = ${'$'}quantity, updated_at = time::now() WHERE user_id = user:${'$'}userId",
                    mapOf("id" to id.value, "quantity" to quantity, "userId" to userId.value),
                ).mapLeft { error ->
                    logger.warn("Database error for ${id.value}: ERROR_MSG (converting to NotFound)")
                    ItemError.NotFound(id)
                }.bind()
            findById(id).bind()
        }

    override fun findUnplaced(): Flow<List<Item>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM item WHERE user_id = user:${'$'}userId AND location_id IS NONE AND container_id IS NONE AND deleted_at IS NONE",
                    mapOf("userId" to userId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findUnplaced: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findUnplaced: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findUnplaced: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findUnplaced: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findUnplaced: ERROR_MSG")

                            else -> logger.warn("Database error in findUnplaced: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseItems(it) },
                ),
            )
        }

    private fun parseItem(result: String): Item? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToItem(it) }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse item: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse item: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse item: ${e.message}", e)
            null
        }

    private fun parseItems(result: String): List<Item> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToItem(it.jsonObject)
                } catch (e: SerializationException) {
                    logger.warn("Failed to parse item element: ${e.message}", e)
                    null
                } catch (e: IllegalStateException) {
                    logger.warn("Failed to parse item element: ${e.message}", e)
                    null
                } catch (e: IllegalArgumentException) {
                    logger.warn("Failed to parse item element: ${e.message}", e)
                    null
                }
            }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse items array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse items array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse items array: ${e.message}", e)
            emptyList()
        }

    private fun mapToItem(obj: kotlinx.serialization.json.JsonObject): Item {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return Item(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            name = obj["name"]?.jsonPrimitive?.content ?: "",
            description = obj["description"]?.jsonPrimitive?.content,
            templateId =
                obj["template_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            locationId =
                obj["location_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            containerId =
                obj["container_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            quantity = obj["quantity"]?.jsonPrimitive?.content?.toIntOrNull() ?: 1,
            photoAttachmentId =
                obj["photo_attachment_id"]
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
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
            deletedAt = obj["deleted_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
        )
    }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: IllegalArgumentException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
