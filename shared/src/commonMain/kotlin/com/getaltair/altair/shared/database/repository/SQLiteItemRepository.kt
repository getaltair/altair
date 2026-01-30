package com.getaltair.altair.shared.database.repository

import com.getaltair.altair.shared.database.AltairDatabase
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.tracking.Container
import com.getaltair.altair.shared.domain.tracking.CustomField
import com.getaltair.altair.shared.domain.tracking.FieldDefinition
import com.getaltair.altair.shared.domain.tracking.Item
import com.getaltair.altair.shared.domain.tracking.ItemTemplate
import com.getaltair.altair.shared.domain.tracking.Location
import com.getaltair.altair.shared.repository.ItemRepository
import kotlinx.datetime.Clock

/**
 * SQLite implementation of ItemRepository for mobile platforms.
 *
 * Maps between SQLDelight generated Item table and domain Item entities,
 * with support for locations, containers, and custom fields.
 */
class SQLiteItemRepository(database: AltairDatabase) : SQLiteRepository(database), ItemRepository {

    private val queries = database.itemQueries
    private val locationQueries = database.locationQueries
    private val containerQueries = database.containerQueries
    private val templateQueries = database.item_templateQueries
    private val customFieldQueries = database.custom_fieldQueries

    override suspend fun getById(id: Ulid): AltairResult<Item> = dbOperation {
        val result = queries.selectById(id.value).executeAsOneOrNull()
        result?.toDomain() ?: throw NoSuchElementException("Item not found: ${id.value}")
    }.mapLeft { error ->
        if (error is AltairError.StorageError.DatabaseError &&
            error.message.contains("Item not found")) {
            AltairError.NotFoundError.ItemNotFound(id.value)
        } else {
            error
        }
    }

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Item>> = dbOperation {
        queries.selectByUserId(userId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getByLocation(locationId: Ulid): AltairResult<List<Item>> = dbOperation {
        queries.selectByLocation(locationId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getByContainer(containerId: Ulid): AltairResult<List<Item>> = dbOperation {
        queries.selectByContainer(containerId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getByTemplate(templateId: Ulid): AltairResult<List<Item>> = dbOperation {
        queries.selectByTemplate(templateId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getByInitiative(initiativeId: Ulid): AltairResult<List<Item>> = dbOperation {
        queries.selectByInitiative(initiativeId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun search(userId: Ulid, query: String): AltairResult<List<Item>> = dbOperation {
        // TODO: Implement full-text search when search query is added
        // For now, return all items for the user
        queries.selectByUserId(userId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getLowStock(userId: Ulid, threshold: Int): AltairResult<List<Item>> = dbOperation {
        // TODO: Add a specific query for low stock items
        // For now, filter in memory
        queries.selectByUserId(userId.value)
            .executeAsList()
            .map { it.toDomain() }
            .filter { it.quantity <= threshold }
    }

    override suspend fun create(item: Item): AltairResult<Item> = dbOperation {
        queries.insert(
            id = item.id.value,
            user_id = item.userId.value,
            name = item.name,
            description = item.description,
            quantity = item.quantity.toLong(),
            template_id = item.templateId?.value,
            location_id = item.locationId?.value,
            container_id = item.containerId?.value,
            initiative_id = item.initiativeId?.value,
            image_key = item.imageKey,
            created_at = item.createdAt.toLong(),
            updated_at = item.updatedAt.toLong(),
            deleted_at = item.deletedAt.toLongOrNull()
        )
        item
    }

    override suspend fun update(item: Item): AltairResult<Item> = dbOperation {
        queries.update(
            name = item.name,
            description = item.description,
            quantity = item.quantity.toLong(),
            location_id = item.locationId?.value,
            container_id = item.containerId?.value,
            initiative_id = item.initiativeId?.value,
            image_key = item.imageKey,
            updated_at = item.updatedAt.toLong(),
            id = item.id.value
        )
        item
    }

    override suspend fun updateQuantity(id: Ulid, quantity: Int): AltairResult<Item> = dbOperation {
        val now = Clock.System.now()
        queries.updateQuantity(
            quantity = quantity.toLong(),
            updated_at = now.toLong(),
            id = id.value
        )
        getById(id).getOrNull()!!
    }

    override suspend fun move(id: Ulid, locationId: Ulid?, containerId: Ulid?): AltairResult<Item> = dbOperation {
        val now = Clock.System.now()

        when {
            locationId != null && containerId != null -> {
                throw IllegalArgumentException("Item cannot have both location and container")
            }
            locationId != null -> {
                queries.moveToLocation(
                    location_id = locationId.value,
                    updated_at = now.toLong(),
                    id = id.value
                )
            }
            containerId != null -> {
                queries.moveToContainer(
                    container_id = containerId.value,
                    updated_at = now.toLong(),
                    id = id.value
                )
            }
            else -> {
                queries.removeLocation(
                    updated_at = now.toLong(),
                    id = id.value
                )
            }
        }

        getById(id).getOrNull()!!
    }

    override suspend fun getCustomFields(itemId: Ulid): AltairResult<List<CustomField>> = dbOperation {
        // TODO: Implement when CustomField queries are defined
        emptyList()
    }

    override suspend fun setCustomField(field: CustomField): AltairResult<CustomField> = dbOperation {
        // TODO: Implement when CustomField insert/update queries are defined
        field
    }

    override suspend fun deleteCustomField(id: Ulid): AltairResult<Unit> = dbOperation {
        // TODO: Implement when CustomField delete query is defined
        Unit
    }

    override suspend fun getLocations(userId: Ulid): AltairResult<List<Location>> = dbOperation {
        // TODO: Implement when Location queries are defined
        emptyList()
    }

    override suspend fun createLocation(location: Location): AltairResult<Location> = dbOperation {
        // TODO: Implement when Location insert query is defined
        location
    }

    override suspend fun updateLocation(location: Location): AltairResult<Location> = dbOperation {
        // TODO: Implement when Location update query is defined
        location
    }

    override suspend fun deleteLocation(id: Ulid): AltairResult<Unit> = dbOperation {
        // TODO: Implement when Location delete query is defined
        Unit
    }

    override suspend fun getContainers(userId: Ulid): AltairResult<List<Container>> = dbOperation {
        // TODO: Implement when Container queries are defined
        emptyList()
    }

    override suspend fun createContainer(container: Container): AltairResult<Container> = dbOperation {
        // TODO: Implement when Container insert query is defined
        container
    }

    override suspend fun updateContainer(container: Container): AltairResult<Container> = dbOperation {
        // TODO: Implement when Container update query is defined
        container
    }

    override suspend fun moveContainer(id: Ulid, locationId: Ulid?): AltairResult<Container> = dbOperation {
        // TODO: Implement when Container move query is defined
        // For now, return stub
        getContainers(Ulid("stub")).getOrNull()?.first() ?: throw NoSuchElementException()
    }

    override suspend fun deleteContainer(id: Ulid): AltairResult<Unit> = dbOperation {
        // TODO: Implement when Container delete query is defined
        Unit
    }

    override suspend fun getTemplates(userId: Ulid): AltairResult<List<ItemTemplate>> = dbOperation {
        // TODO: Implement when ItemTemplate queries are defined
        emptyList()
    }

    override suspend fun createTemplate(template: ItemTemplate, fields: List<FieldDefinition>): AltairResult<ItemTemplate> = dbOperation {
        // TODO: Implement when ItemTemplate insert query is defined
        template
    }

    override suspend fun deleteTemplate(id: Ulid): AltairResult<Unit> = dbOperation {
        // TODO: Implement when ItemTemplate delete query is defined
        Unit
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> = dbOperation {
        val now = Clock.System.now()
        queries.softDelete(
            deleted_at = now.toLong(),
            updated_at = now.toLong(),
            id = id.value
        )
    }

    override suspend fun restore(id: Ulid): AltairResult<Unit> = dbOperation {
        // TODO: Add restore query to Item.sq
        val item = queries.selectById(id.value).executeAsOneOrNull()?.toDomain()
            ?: throw NoSuchElementException("Item not found: ${id.value}")

        val restored = item.copy(
            deletedAt = null,
            updatedAt = Clock.System.now()
        )
        update(restored)
        Unit
    }

    // Mapper extension function
    private fun com.getaltair.altair.shared.database.Item.toDomain(): Item = Item(
        id = id.toUlid(),
        userId = user_id.toUlid(),
        name = name,
        description = description,
        quantity = quantity.toInt(),
        templateId = template_id?.toUlid(),
        locationId = location_id?.toUlid(),
        containerId = container_id?.toUlid(),
        initiativeId = initiative_id?.toUlid(),
        imageKey = image_key,
        createdAt = created_at.toInstant(),
        updatedAt = updated_at.toInstant(),
        deletedAt = deleted_at.toInstantOrNull()
    )
}
