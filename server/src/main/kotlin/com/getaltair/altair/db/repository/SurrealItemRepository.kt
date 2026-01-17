package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.ItemError
import com.getaltair.altair.domain.model.tracking.Item
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.ItemRepository
import com.getaltair.altair.repository.PageRequest
import com.getaltair.altair.repository.PageResult
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

class SurrealItemRepository(
    private val db: SurrealDbClient,
    private val userId: String,
) : ItemRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<ItemError, Item> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM item:${id.value} WHERE user_id = user:$userId AND deleted_at IS NONE",
                    ).mapLeft { ItemError.NotFound(id) }
                    .bind()
            parseItem(result) ?: raise(ItemError.NotFound(id))
        }

    override suspend fun save(entity: Item): Either<ItemError, Item> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .execute(
                        """
                        UPDATE item:${entity.id.value} SET
                            name = '${entity.name.replace("'", "''")}',
                            description = ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            template_id = ${entity.templateId?.let { "item_template:${it.value}" } ?: "NONE"},
                            location_id = ${entity.locationId?.let { "location:${it.value}" } ?: "NONE"},
                            container_id = ${entity.containerId?.let { "container:${it.value}" } ?: "NONE"},
                            quantity = ${entity.quantity},
                            photo_attachment_id = ${entity.photoAttachmentId?.let { "attachment:${it.value}" } ?: "NONE"},
                            initiative_id = ${entity.initiativeId?.let { "initiative:${it.value}" } ?: "NONE"},
                            updated_at = time::now()
                        WHERE user_id = user:$userId;
                        """.trimIndent(),
                    ).mapLeft { ItemError.NotFound(entity.id) }
                    .bind()
            } else {
                db
                    .execute(
                        """
                        CREATE item:${entity.id.value} CONTENT {
                            user_id: user:$userId,
                            name: '${entity.name.replace("'", "''")}',
                            description: ${entity.description?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            template_id: ${entity.templateId?.let { "item_template:${it.value}" } ?: "NONE"},
                            location_id: ${entity.locationId?.let { "location:${it.value}" } ?: "NONE"},
                            container_id: ${entity.containerId?.let { "container:${it.value}" } ?: "NONE"},
                            quantity: ${entity.quantity},
                            photo_attachment_id: ${entity.photoAttachmentId?.let { "attachment:${it.value}" } ?: "NONE"},
                            initiative_id: ${entity.initiativeId?.let { "initiative:${it.value}" } ?: "NONE"}
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
                    "UPDATE item:${id.value} SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:$userId;",
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
        }

    override fun findAll(): Flow<List<Item>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM item WHERE user_id = user:$userId AND deleted_at IS NONE ORDER BY name",
                )
            emit(result.fold({ emptyList() }, { parseItems(it) }))
        }

    override fun findByLocation(locationId: Ulid): Flow<List<Item>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM item WHERE user_id = user:$userId AND location_id = location:${locationId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseItems(it) }))
        }

    override fun findByContainer(containerId: Ulid): Flow<List<Item>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM item WHERE user_id = user:$userId AND container_id = container:${containerId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseItems(it) }))
        }

    override fun findByTemplate(templateId: Ulid): Flow<List<Item>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM item WHERE user_id = user:$userId AND template_id = item_template:${templateId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseItems(it) }))
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<Item>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM item WHERE user_id = user:$userId AND initiative_id = initiative:${initiativeId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseItems(it) }))
        }

    override suspend fun searchByName(query: String): Either<ItemError, List<Item>> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM item WHERE user_id = user:$userId AND string::lowercase(name) CONTAINS string::lowercase('${query.replace("'", "''")}') AND deleted_at IS NONE",
                    ).mapLeft { ItemError.NotFound(Ulid.generate()) }
                    .bind()
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
            val locId = locationId.value
            db
                .execute(
                    "UPDATE item:${id.value} SET location_id = location:$locId, " +
                        "container_id = NONE, updated_at = time::now() WHERE user_id = user:$userId;",
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
            findById(id).bind()
        }

    override suspend fun moveToContainer(
        id: Ulid,
        containerId: Ulid,
    ): Either<ItemError, Item> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE item:${id.value} SET container_id = container:${containerId.value}, updated_at = time::now() WHERE user_id = user:$userId;",
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
            findById(id).bind()
        }

    override suspend fun updateQuantity(
        id: Ulid,
        quantity: Int,
    ): Either<ItemError, Item> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE item:${id.value} SET quantity = $quantity, updated_at = time::now() WHERE user_id = user:$userId;",
                ).mapLeft { ItemError.NotFound(id) }
                .bind()
            findById(id).bind()
        }

    override fun findUnplaced(): Flow<List<Item>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM item WHERE user_id = user:$userId AND location_id IS NONE AND container_id IS NONE AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseItems(it) }))
        }

    private fun parseItem(result: String): Item? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToItem(it) }
        } catch (e: Exception) {
            null
        }

    private fun parseItems(result: String): List<Item> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToItem(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
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
            } catch (e: Exception) {
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
