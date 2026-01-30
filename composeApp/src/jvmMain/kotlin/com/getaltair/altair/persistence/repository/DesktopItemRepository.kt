package com.getaltair.altair.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.persistence.DesktopSurrealDbClient
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

/**
 * Desktop implementation of ItemRepository for the Tracking module.
 *
 * Manages items, locations, containers, templates, and custom fields.
 * Desktop version is single-user, so no user scoping needed.
 *
 * Table names:
 * - item: Physical inventory items
 * - location: Physical locations (hierarchical)
 * - container: Containers at locations (can be nested)
 * - item_template: Templates for consistent item creation
 * - field_definition: Field schemas for templates
 * - custom_field: Custom field values on items
 */
class DesktopItemRepository(
    db: DesktopSurrealDbClient
) : BaseDesktopRepository<Item>(db, "item", Item::class), ItemRepository {

    override fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.ItemNotFound(id.toString())

    // ========================================================================
    // Core CRUD Operations
    // ========================================================================

    override suspend fun getById(id: Ulid): AltairResult<Item> = findById(id)

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Item>> = findAll()

    override suspend fun create(item: Item): AltairResult<Item> {
        val sql = """
            CREATE item:${'$'}id CONTENT {
                user_id: ${'$'}userId,
                name: ${'$'}name,
                description: ${'$'}description,
                quantity: ${'$'}quantity,
                location_id: ${'$'}locationId,
                container_id: ${'$'}containerId,
                template_id: ${'$'}templateId,
                initiative_id: ${'$'}initiativeId,
                created_at: ${'$'}now,
                updated_at: ${'$'}now,
                deleted_at: NONE
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to item.id.toString(),
                "userId" to item.userId.toString(),
                "name" to item.name,
                "description" to item.description,
                "quantity" to item.quantity,
                "locationId" to item.locationId?.toString(),
                "containerId" to item.containerId?.toString(),
                "templateId" to item.templateId?.toString(),
                "initiativeId" to item.initiativeId?.toString(),
                "now" to now().toString()
            ),
            entityClass
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create item").left()
        }
    }

    override suspend fun update(item: Item): AltairResult<Item> {
        val sql = """
            UPDATE item:${'$'}id SET
                name = ${'$'}name,
                description = ${'$'}description,
                quantity = ${'$'}quantity,
                location_id = ${'$'}locationId,
                container_id = ${'$'}containerId,
                template_id = ${'$'}templateId,
                initiative_id = ${'$'}initiativeId,
                updated_at = ${'$'}now
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to item.id.toString(),
                "name" to item.name,
                "description" to item.description,
                "quantity" to item.quantity,
                "locationId" to item.locationId?.toString(),
                "containerId" to item.containerId?.toString(),
                "templateId" to item.templateId?.toString(),
                "initiativeId" to item.initiativeId?.toString(),
                "now" to now().toString()
            ),
            entityClass
        ).flatMap { result ->
            result?.right() ?: notFoundError(item.id).left()
        }
    }

    // softDelete inherited from BaseDesktopRepository

    override suspend fun restore(id: Ulid): AltairResult<Unit> {
        val sql = """
            UPDATE $tableName
            SET deleted_at = NONE, updated_at = ${'$'}now
            WHERE id = ${'$'}id
        """.trimIndent()

        return db.query(sql, mapOf("id" to "$tableName:$id", "now" to now().toString()), entityClass)
            .flatMap { results ->
                if (results.isNotEmpty()) Unit.right()
                else notFoundError(id).left()
            }
    }

    // ========================================================================
    // Query Operations
    // ========================================================================

    override suspend fun getByLocation(locationId: Ulid): AltairResult<List<Item>> {
        val sql = """
            SELECT * FROM item
            WHERE deleted_at IS NONE
            AND (location_id = ${'$'}locationId OR container_id IN (
                SELECT VALUE id FROM container WHERE location_id = ${'$'}locationId
            ))
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, mapOf("locationId" to "location:$locationId"), entityClass)
    }

    override suspend fun getByContainer(containerId: Ulid): AltairResult<List<Item>> {
        return findWhere(
            "container_id = \$containerId",
            mapOf("containerId" to "container:$containerId")
        )
    }

    override suspend fun getByTemplate(templateId: Ulid): AltairResult<List<Item>> {
        return findWhere(
            "template_id = \$templateId",
            mapOf("templateId" to "item_template:$templateId")
        )
    }

    override suspend fun getByInitiative(initiativeId: Ulid): AltairResult<List<Item>> {
        return findWhere(
            "initiative_id = \$initiativeId",
            mapOf("initiativeId" to "initiative:$initiativeId")
        )
    }

    // ========================================================================
    // Search Operations
    // ========================================================================

    override suspend fun search(userId: Ulid, query: String): AltairResult<List<Item>> {
        val sql = """
            SELECT * FROM item
            WHERE deleted_at IS NONE
            AND (name CONTAINS ${'$'}query OR description CONTAINS ${'$'}query)
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, mapOf("query" to query), entityClass)
    }

    override suspend fun getLowStock(userId: Ulid, threshold: Int): AltairResult<List<Item>> {
        val sql = """
            SELECT * FROM item
            WHERE deleted_at IS NONE
            AND quantity <= ${'$'}threshold
            ORDER BY quantity ASC
        """.trimIndent()

        return db.query(sql, mapOf("threshold" to threshold), entityClass)
    }

    // ========================================================================
    // Item Operations
    // ========================================================================

    override suspend fun updateQuantity(id: Ulid, quantity: Int): AltairResult<Item> {
        require(quantity >= 0) { "Quantity must be non-negative" }

        val sql = """
            UPDATE item:${'$'}id
            SET quantity = ${'$'}quantity, updated_at = ${'$'}now
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf("id" to id.toString(), "quantity" to quantity, "now" to now().toString()),
            entityClass
        ).flatMap { item: Item? ->
            item?.right() ?: notFoundError(id).left()
        }
    }

    override suspend fun move(id: Ulid, locationId: Ulid?, containerId: Ulid?): AltairResult<Item> {
        val sql = """
            UPDATE item:${'$'}id
            SET location_id = ${'$'}locationId,
                container_id = ${'$'}containerId,
                updated_at = ${'$'}now
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to id.toString(),
                "locationId" to locationId?.toString(),
                "containerId" to containerId?.toString(),
                "now" to now().toString()
            ),
            entityClass
        ).flatMap { item: Item? ->
            item?.right() ?: notFoundError(id).left()
        }
    }

    // ========================================================================
    // Custom Fields (Minimal implementations with TODOs)
    // ========================================================================

    override suspend fun getCustomFields(itemId: Ulid): AltairResult<List<CustomField>> {
        // TODO: Implement custom fields retrieval
        return emptyList<CustomField>().right()
    }

    override suspend fun setCustomField(field: CustomField): AltairResult<CustomField> {
        // TODO: Implement custom field setting
        return AltairError.StorageError.DatabaseError("Custom field setting not yet implemented").left()
    }

    override suspend fun deleteCustomField(id: Ulid): AltairResult<Unit> {
        // TODO: Implement custom field deletion
        return Unit.right()
    }

    // ========================================================================
    // Locations (Minimal implementations with TODOs)
    // ========================================================================

    override suspend fun getLocations(userId: Ulid): AltairResult<List<Location>> {
        // TODO: Implement locations retrieval
        return emptyList<Location>().right()
    }

    override suspend fun createLocation(location: Location): AltairResult<Location> {
        // TODO: Implement location creation
        return AltairError.StorageError.DatabaseError("Location creation not yet implemented").left()
    }

    override suspend fun updateLocation(location: Location): AltairResult<Location> {
        // TODO: Implement location update
        return AltairError.StorageError.DatabaseError("Location update not yet implemented").left()
    }

    override suspend fun deleteLocation(id: Ulid): AltairResult<Unit> {
        // TODO: Implement location deletion
        return Unit.right()
    }

    // ========================================================================
    // Containers (Minimal implementations with TODOs)
    // ========================================================================

    override suspend fun getContainers(userId: Ulid): AltairResult<List<Container>> {
        // TODO: Implement containers retrieval
        return emptyList<Container>().right()
    }

    override suspend fun createContainer(container: Container): AltairResult<Container> {
        // TODO: Implement container creation
        return AltairError.StorageError.DatabaseError("Container creation not yet implemented").left()
    }

    override suspend fun updateContainer(container: Container): AltairResult<Container> {
        // TODO: Implement container update
        return AltairError.StorageError.DatabaseError("Container update not yet implemented").left()
    }

    override suspend fun moveContainer(id: Ulid, locationId: Ulid?): AltairResult<Container> {
        // TODO: Implement container move
        return AltairError.StorageError.DatabaseError("Container move not yet implemented").left()
    }

    override suspend fun deleteContainer(id: Ulid): AltairResult<Unit> {
        // TODO: Implement container deletion
        return Unit.right()
    }

    // ========================================================================
    // Templates (Minimal implementations with TODOs)
    // ========================================================================

    override suspend fun getTemplates(userId: Ulid): AltairResult<List<ItemTemplate>> {
        // TODO: Implement templates retrieval
        return emptyList<ItemTemplate>().right()
    }

    override suspend fun createTemplate(template: ItemTemplate, fields: List<FieldDefinition>): AltairResult<ItemTemplate> {
        // TODO: Implement template creation
        return AltairError.StorageError.DatabaseError("Template creation not yet implemented").left()
    }

    override suspend fun deleteTemplate(id: Ulid): AltairResult<Unit> {
        // TODO: Implement template deletion
        return Unit.right()
    }
}
