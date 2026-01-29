package com.getaltair.server.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
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
import com.getaltair.server.auth.AuthContext
import com.getaltair.server.persistence.SurrealDbClient
import kotlinx.serialization.Serializable

/**
 * SurrealDB implementation of ItemRepository for the Tracking module.
 *
 * Manages items, locations, containers, templates, and custom fields
 * with full user-scoped data isolation.
 *
 * Table names:
 * - item: Physical inventory items
 * - location: Physical locations (hierarchical)
 * - container: Containers at locations (can be nested)
 * - item_template: Templates for consistent item creation
 * - field_definition: Field schemas for templates
 * - custom_field: Custom field values on items
 *
 * @param db The SurrealDB client for database operations
 * @param auth The authentication context providing current user ID
 */
class SurrealItemRepository(
    db: SurrealDbClient,
    auth: AuthContext
) : BaseSurrealRepository<Item>(db, auth, "item", Item::class), ItemRepository {

    // ========================================================================
    // Core CRUD Operations
    // ========================================================================

    override suspend fun getById(id: Ulid): AltairResult<Item> = findById(id)

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Item>> = findAllForUser()

    override suspend fun create(item: Item): AltairResult<Item> {
        return db.create(tableName, item)
    }

    override suspend fun update(item: Item): AltairResult<Item> {
        return db.update(tableName, item.id.toString(), item.copy(updatedAt = now()))
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> = softDeleteEntity(id)

    override suspend fun restore(id: Ulid): AltairResult<Unit> = restoreEntity(id)

    // ========================================================================
    // Query Operations
    // ========================================================================

    override suspend fun getByLocation(locationId: Ulid): AltairResult<List<Item>> {
        // Get items directly at location OR in containers at that location
        val sql = """
            SELECT * FROM item
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
            AND (location_id = ${'$'}locationId OR container_id IN (
                SELECT VALUE id FROM container WHERE location_id = ${'$'}locationId
            ))
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, params("locationId" to "location:$locationId"), Item::class)
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
        // Full-text search on name and description
        val sql = """
            SELECT * FROM item
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
            AND (
                name CONTAINS ${'$'}query
                OR description CONTAINS ${'$'}query
            )
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, params("query" to query), Item::class)
    }

    override suspend fun getLowStock(userId: Ulid, threshold: Int): AltairResult<List<Item>> {
        val sql = """
            SELECT * FROM item
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
            AND quantity <= ${'$'}threshold
            ORDER BY quantity ASC
        """.trimIndent()

        return db.query(sql, params("threshold" to threshold), Item::class)
    }

    // ========================================================================
    // Item Operations
    // ========================================================================

    override suspend fun updateQuantity(id: Ulid, quantity: Int): AltairResult<Item> {
        require(quantity >= 0) { "Quantity must be non-negative" }

        val sql = """
            UPDATE item
            SET quantity = ${'$'}quantity, updated_at = ${'$'}now
            WHERE id = ${'$'}id AND user_id = ${'$'}userId AND deleted_at IS NONE
            RETURN AFTER
        """.trimIndent()

        return db.queryOne(
            sql,
            params("id" to "$tableName:$id", "quantity" to quantity, "now" to now().toString()),
            Item::class
        ).flatMap { item: Item? ->
            item?.right() ?: notFoundError(id).left()
        }
    }

    override suspend fun move(id: Ulid, locationId: Ulid?, containerId: Ulid?): AltairResult<Item> {
        require(!(locationId != null && containerId != null)) {
            "Item cannot have both location and container"
        }

        val sql = """
            UPDATE item
            SET location_id = ${'$'}locationId, container_id = ${'$'}containerId, updated_at = ${'$'}now
            WHERE id = ${'$'}id AND user_id = ${'$'}userId AND deleted_at IS NONE
            RETURN AFTER
        """.trimIndent()

        return db.queryOne(
            sql,
            params(
                "id" to "$tableName:$id",
                "locationId" to locationId?.let { "location:$it" },
                "containerId" to containerId?.let { "container:$it" },
                "now" to now().toString()
            ),
            Item::class
        ).flatMap { item: Item? ->
            item?.right() ?: notFoundError(id).left()
        }
    }

    // ========================================================================
    // Custom Fields
    // ========================================================================

    override suspend fun getCustomFields(itemId: Ulid): AltairResult<List<CustomField>> {
        val sql = """
            SELECT * FROM custom_field
            WHERE item_id = ${'$'}itemId
            ORDER BY name ASC
        """.trimIndent()

        return db.query(sql, mapOf("itemId" to "item:$itemId"), CustomField::class)
    }

    override suspend fun setCustomField(field: CustomField): AltairResult<CustomField> {
        // Upsert: check if field exists for this item with same name
        val existingQuery = """
            SELECT * FROM custom_field
            WHERE item_id = ${'$'}itemId AND name = ${'$'}name
            LIMIT 1
        """.trimIndent()

        val existingResult = db.queryOne(
            existingQuery,
            mapOf("itemId" to "item:${field.itemId}", "name" to field.name),
            CustomField::class
        )

        val existing = existingResult.fold({ return it.left() }, { it })

        return if (existing != null) {
            // Update existing field
            db.update("custom_field", existing.id.toString(), field.copy(id = existing.id))
        } else {
            // Create new field
            db.create("custom_field", field)
        }
    }

    override suspend fun deleteCustomField(id: Ulid): AltairResult<Unit> {
        return db.delete("custom_field", id.toString())
    }

    // ========================================================================
    // Locations
    // ========================================================================

    override suspend fun getLocations(userId: Ulid): AltairResult<List<Location>> {
        val sql = """
            SELECT * FROM location
            WHERE user_id = ${'$'}userId
            ORDER BY name ASC
        """.trimIndent()

        return db.query(sql, baseParams(), Location::class)
    }

    override suspend fun createLocation(location: Location): AltairResult<Location> {
        return db.create("location", location)
    }

    override suspend fun updateLocation(location: Location): AltairResult<Location> {
        return db.update("location", location.id.toString(), location)
    }

    override suspend fun deleteLocation(id: Ulid): AltairResult<Unit> {
        // Check for items at this location
        val itemCountQuery = """
            SELECT count() as cnt FROM item
            WHERE location_id = ${'$'}locationId AND deleted_at IS NONE
            GROUP ALL
        """.trimIndent()

        val itemCountResult = db.queryOne(itemCountQuery, mapOf("locationId" to "location:$id"), CountResult::class)
        val itemCount = itemCountResult.fold({ return it.left() }, { it?.cnt ?: 0 })

        if (itemCount > 0) {
            return AltairError.ValidationError.ConstraintViolation(
                "Cannot delete location: $itemCount item(s) still at this location"
            ).left()
        }

        // Check for containers at this location
        val containerCountQuery = """
            SELECT count() as cnt FROM container
            WHERE location_id = ${'$'}locationId
            GROUP ALL
        """.trimIndent()

        val containerCountResult = db.queryOne(containerCountQuery, mapOf("locationId" to "location:$id"), CountResult::class)
        val containerCount = containerCountResult.fold({ return it.left() }, { it?.cnt ?: 0 })

        if (containerCount > 0) {
            return AltairError.ValidationError.ConstraintViolation(
                "Cannot delete location: $containerCount container(s) still at this location"
            ).left()
        }

        return db.delete("location", id.toString())
    }

    // ========================================================================
    // Containers
    // ========================================================================

    override suspend fun getContainers(userId: Ulid): AltairResult<List<Container>> {
        val sql = """
            SELECT * FROM container
            WHERE user_id = ${'$'}userId
            ORDER BY name ASC
        """.trimIndent()

        return db.query(sql, baseParams(), Container::class)
    }

    override suspend fun createContainer(container: Container): AltairResult<Container> {
        return db.create("container", container)
    }

    override suspend fun updateContainer(container: Container): AltairResult<Container> {
        return db.update("container", container.id.toString(), container)
    }

    override suspend fun moveContainer(id: Ulid, locationId: Ulid?): AltairResult<Container> {
        val sql = """
            UPDATE container
            SET location_id = ${'$'}locationId
            WHERE id = ${'$'}id AND user_id = ${'$'}userId
            RETURN AFTER
        """.trimIndent()

        return db.queryOne(
            sql,
            params(
                "id" to "container:$id",
                "locationId" to locationId?.let { "location:$it" }
            ),
            Container::class
        ).flatMap { container: Container? ->
            container?.right() ?: AltairError.NotFoundError.ContainerNotFound(id.toString()).left()
        }
    }

    override suspend fun deleteContainer(id: Ulid): AltairResult<Unit> {
        // Check for items in this container
        val itemCountQuery = """
            SELECT count() as cnt FROM item
            WHERE container_id = ${'$'}containerId AND deleted_at IS NONE
            GROUP ALL
        """.trimIndent()

        val itemCountResult = db.queryOne(itemCountQuery, mapOf("containerId" to "container:$id"), CountResult::class)
        val itemCount = itemCountResult.fold({ return it.left() }, { it?.cnt ?: 0 })

        if (itemCount > 0) {
            return AltairError.ValidationError.ConstraintViolation(
                "Cannot delete container: $itemCount item(s) still in this container"
            ).left()
        }

        return db.delete("container", id.toString())
    }

    // ========================================================================
    // Templates
    // ========================================================================

    override suspend fun getTemplates(userId: Ulid): AltairResult<List<ItemTemplate>> {
        val sql = """
            SELECT * FROM item_template
            WHERE user_id = ${'$'}userId
            ORDER BY name ASC
        """.trimIndent()

        return db.query(sql, baseParams(), ItemTemplate::class)
    }

    override suspend fun createTemplate(
        template: ItemTemplate,
        fields: List<FieldDefinition>
    ): AltairResult<ItemTemplate> {
        // Create the template first
        val templateResult = db.create("item_template", template)
        val createdTemplate = templateResult.fold({ return it.left() }, { it })

        // Create each field definition sequentially
        for (field in fields) {
            val result = db.create("field_definition", field.copy(templateId = createdTemplate.id))
            if (result.isLeft()) {
                // If any field creation fails, return the error
                // Note: In a production system, we might want to rollback the template
                return result.map { createdTemplate }
            }
        }

        return createdTemplate.right()
    }

    override suspend fun deleteTemplate(id: Ulid): AltairResult<Unit> {
        // Delete field definitions first
        val deleteFieldsSql = """
            DELETE field_definition WHERE template_id = ${'$'}templateId
        """.trimIndent()

        val deleteFieldsResult = db.query(deleteFieldsSql, mapOf("templateId" to "item_template:$id"), Void::class)
        deleteFieldsResult.fold({ return it.left() }, { /* Success, continue */ })

        // Delete the template (items are NOT deleted per spec)
        return db.delete("item_template", id.toString())
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    override fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.ItemNotFound(id.toString())
}

/**
 * Helper data class for count query results.
 */
@Serializable
private data class CountResult(val cnt: Int)
