package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.ItemError
import com.getaltair.altair.domain.model.tracking.Item
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Repository for Item entities.
 *
 * Items are physical objects being tracked in the system. They can be
 * located in Locations or Containers, and can use ItemTemplates for
 * consistent field definitions.
 */
interface ItemRepository : Repository<Item, ItemError> {
    /**
     * Finds items at a specific location.
     *
     * @param locationId The ULID of the location
     * @return A Flow emitting items at the location
     */
    fun findByLocation(locationId: Ulid): Flow<List<Item>>

    /**
     * Finds items in a specific container.
     *
     * @param containerId The ULID of the container
     * @return A Flow emitting items in the container
     */
    fun findByContainer(containerId: Ulid): Flow<List<Item>>

    /**
     * Finds items using a specific template.
     *
     * @param templateId The ULID of the template
     * @return A Flow emitting items using the template
     */
    fun findByTemplate(templateId: Ulid): Flow<List<Item>>

    /**
     * Finds items associated with a specific initiative.
     *
     * @param initiativeId The ULID of the initiative
     * @return A Flow emitting items for the initiative
     */
    fun findByInitiative(initiativeId: Ulid): Flow<List<Item>>

    /**
     * Searches items by name (case-insensitive partial match).
     *
     * @param query The search query
     * @return Either an error on failure, or matching items
     */
    suspend fun searchByName(query: String): Either<ItemError, List<Item>>

    /**
     * Moves an item to a new location.
     *
     * Clears any container association when moving to a location.
     *
     * @param id The ULID of the item
     * @param locationId The ULID of the new location
     * @return Either an error on failure, or the updated item
     */
    suspend fun moveToLocation(
        id: Ulid,
        locationId: Ulid,
    ): Either<ItemError, Item>

    /**
     * Moves an item into a container.
     *
     * The item's location becomes the container's location (if any).
     *
     * @param id The ULID of the item
     * @param containerId The ULID of the container
     * @return Either an error on failure, or the updated item
     */
    suspend fun moveToContainer(
        id: Ulid,
        containerId: Ulid,
    ): Either<ItemError, Item>

    /**
     * Updates the quantity of an item.
     *
     * @param id The ULID of the item
     * @param quantity The new quantity
     * @return Either an error on invalid quantity, or the updated item
     */
    suspend fun updateQuantity(
        id: Ulid,
        quantity: Int,
    ): Either<ItemError, Item>

    /**
     * Finds items without a location or container (unplaced items).
     *
     * @return A Flow emitting unplaced items
     */
    fun findUnplaced(): Flow<List<Item>>
}
