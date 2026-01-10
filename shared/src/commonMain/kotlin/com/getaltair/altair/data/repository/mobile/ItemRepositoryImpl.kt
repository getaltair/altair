package com.getaltair.altair.data.repository.mobile

import com.getaltair.altair.data.entity.mobile.Item
import com.getaltair.altair.data.entity.mobile.toDomain
import com.getaltair.altair.data.repository.Repository
import com.getaltair.altair.data.util.Timestamps
import com.getaltair.altair.data.util.UlidGenerator
import com.getaltair.altair.database.AltairDatabase

/**
 * SQLDelight-based repository implementation for Item entities.
 *
 * Provides CRUD operations for Item entities on mobile platforms (Android/iOS).
 * Uses soft-delete pattern for data retention and sync support.
 *
 * @property database The SQLDelight database instance
 */
class ItemRepositoryImpl(private val database: AltairDatabase) : Repository<Item, String> {

    private val queries get() = database.itemQueries

    /**
     * Creates a new item in the database.
     *
     * Generates a ULID if the entity ID is empty and sets timestamps.
     *
     * @param entity The item to create
     * @return The created item with generated fields populated
     */
    override suspend fun create(entity: Item): Item {
        val now = Timestamps.now()
        val id = if (entity.id.isBlank()) UlidGenerator.generate() else entity.id

        queries.insert(
            id = id,
            name = entity.name,
            description = entity.description,
            location_id = entity.locationId,
            container_id = entity.containerId,
            created_at = now,
            updated_at = now,
            deleted_at = null,
            sync_version = 0,
        )

        return entity.copy(
            id = id,
            createdAt = now,
            updatedAt = now,
            syncVersion = 0,
        )
    }

    /**
     * Finds an item by its identifier.
     *
     * @param id The item identifier (ULID)
     * @return The item if found, null otherwise
     */
    override suspend fun findById(id: String): Item? = queries.selectById(id).executeAsOneOrNull()?.toDomain()

    /**
     * Updates an existing item.
     *
     * Updates the updatedAt timestamp and increments syncVersion.
     *
     * @param entity The item with updated values
     * @return The updated item
     */
    override suspend fun update(entity: Item): Item {
        val now = Timestamps.now()

        queries.updateById(
            name = entity.name,
            description = entity.description,
            location_id = entity.locationId,
            container_id = entity.containerId,
            updated_at = now,
            id = entity.id,
        )

        return entity.copy(
            updatedAt = now,
            syncVersion = entity.syncVersion + 1,
        )
    }

    /**
     * Soft-deletes an item by its identifier.
     *
     * Sets the deletedAt timestamp rather than removing the record.
     *
     * @param id The item identifier
     * @return true if the item was deleted, false if not found
     */
    override suspend fun delete(id: String): Boolean {
        val existing = findById(id) ?: return false
        if (existing.isDeleted) return false

        val now = Timestamps.now()
        queries.softDeleteById(
            deleted_at = now,
            updated_at = now,
            id = id,
        )
        return true
    }

    /**
     * Finds all items that are not soft-deleted.
     *
     * @return List of active items ordered by creation date (newest first)
     */
    override suspend fun findAll(): List<Item> = queries.selectAll().executeAsList().map { it.toDomain() }

    /**
     * Finds all items in a specific location.
     *
     * @param locationId The location identifier
     * @return List of active items in the location ordered by creation date (newest first)
     */
    suspend fun findByLocationId(locationId: String): List<Item> =
        queries.selectByLocationId(locationId).executeAsList().map { it.toDomain() }

    /**
     * Finds all items in a specific container.
     *
     * @param containerId The container identifier
     * @return List of active items in the container ordered by creation date (newest first)
     */
    suspend fun findByContainerId(containerId: String): List<Item> =
        queries.selectByContainerId(containerId).executeAsList().map { it.toDomain() }
}
