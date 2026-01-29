package com.getaltair.altair.shared.repository

import arrow.core.Either
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.tracking.Container
import com.getaltair.altair.shared.domain.tracking.CustomField
import com.getaltair.altair.shared.domain.tracking.FieldDefinition
import com.getaltair.altair.shared.domain.tracking.Item
import com.getaltair.altair.shared.domain.tracking.ItemTemplate
import com.getaltair.altair.shared.domain.tracking.Location

/**
 * Result type for Tracking domain operations.
 */
typealias AltairResult<T> = Either<AltairError, T>

/**
 * Repository interface for Tracking module operations.
 *
 * Manages physical items, their locations, containers, and metadata.
 * Supports hierarchical organization (locations contain containers contain items)
 * and template-based item creation with custom fields.
 *
 * Key features:
 * - Hierarchical location/container system
 * - Item templates with custom field definitions
 * - Low stock alerting
 * - Full-text search across item metadata
 */
interface ItemRepository {
    /**
     * Retrieves an item by its unique identifier.
     *
     * @param id The item's ULID
     * @return The item if found, or an error
     */
    suspend fun getById(id: Ulid): AltairResult<Item>

    /**
     * Retrieves all items owned by a user.
     *
     * @param userId The user's ULID
     * @return List of all items, or an error
     */
    suspend fun getAllForUser(userId: Ulid): AltairResult<List<Item>>

    /**
     * Retrieves items at a specific location.
     *
     * @param locationId The location's ULID
     * @return List of items at the location, or an error
     */
    suspend fun getByLocation(locationId: Ulid): AltairResult<List<Item>>

    /**
     * Retrieves items in a specific container.
     *
     * @param containerId The container's ULID
     * @return List of items in the container, or an error
     */
    suspend fun getByContainer(containerId: Ulid): AltairResult<List<Item>>

    /**
     * Retrieves items created from a specific template.
     *
     * @param templateId The template's ULID
     * @return List of items based on this template, or an error
     */
    suspend fun getByTemplate(templateId: Ulid): AltairResult<List<Item>>

    /**
     * Retrieves items associated with an initiative.
     *
     * @param initiativeId The initiative's ULID
     * @return List of items linked to the initiative, or an error
     */
    suspend fun getByInitiative(initiativeId: Ulid): AltairResult<List<Item>>

    /**
     * Performs full-text search across item names and descriptions.
     *
     * @param userId The user's ULID (scopes search to user's items)
     * @param query The search query string
     * @return List of matching items, or an error
     */
    suspend fun search(userId: Ulid, query: String): AltairResult<List<Item>>

    /**
     * Retrieves items with quantity at or below the specified threshold.
     *
     * @param userId The user's ULID
     * @param threshold The low stock threshold
     * @return List of low-stock items, or an error
     */
    suspend fun getLowStock(userId: Ulid, threshold: Int): AltairResult<List<Item>>

    /**
     * Creates a new item.
     *
     * @param item The item to create (id must be generated)
     * @return The created item with timestamps, or an error
     */
    suspend fun create(item: Item): AltairResult<Item>

    /**
     * Updates an existing item.
     *
     * @param item The item with updated fields
     * @return The updated item, or an error
     */
    suspend fun update(item: Item): AltairResult<Item>

    /**
     * Updates an item's quantity.
     *
     * @param id The item's ULID
     * @param quantity The new quantity value
     * @return The updated item, or an error
     */
    suspend fun updateQuantity(id: Ulid, quantity: Int): AltairResult<Item>

    /**
     * Moves an item to a new location and/or container.
     *
     * @param id The item's ULID
     * @param locationId The target location's ULID (null to clear location)
     * @param containerId The target container's ULID (null to clear container)
     * @return The updated item, or an error
     */
    suspend fun move(id: Ulid, locationId: Ulid?, containerId: Ulid?): AltairResult<Item>

    // --- Custom Fields ---

    /**
     * Retrieves all custom fields for an item.
     *
     * @param itemId The item's ULID
     * @return List of custom fields, or an error
     */
    suspend fun getCustomFields(itemId: Ulid): AltairResult<List<CustomField>>

    /**
     * Sets a custom field value for an item (creates or updates).
     *
     * @param field The custom field with value
     * @return The created/updated custom field, or an error
     */
    suspend fun setCustomField(field: CustomField): AltairResult<CustomField>

    /**
     * Deletes a custom field from an item.
     *
     * @param id The custom field's ULID
     * @return Unit on success, or an error
     */
    suspend fun deleteCustomField(id: Ulid): AltairResult<Unit>

    // --- Locations ---

    /**
     * Retrieves all locations for a user.
     *
     * @param userId The user's ULID
     * @return List of locations (hierarchical structure), or an error
     */
    suspend fun getLocations(userId: Ulid): AltairResult<List<Location>>

    /**
     * Creates a new location.
     *
     * @param location The location to create
     * @return The created location, or an error
     */
    suspend fun createLocation(location: Location): AltairResult<Location>

    /**
     * Updates an existing location.
     *
     * @param location The location with updated fields
     * @return The updated location, or an error
     */
    suspend fun updateLocation(location: Location): AltairResult<Location>

    /**
     * Deletes a location.
     *
     * Fails if location contains items or containers.
     *
     * @param id The location's ULID
     * @return Unit on success, or an error
     */
    suspend fun deleteLocation(id: Ulid): AltairResult<Unit>

    // --- Containers ---

    /**
     * Retrieves all containers for a user.
     *
     * @param userId The user's ULID
     * @return List of containers, or an error
     */
    suspend fun getContainers(userId: Ulid): AltairResult<List<Container>>

    /**
     * Creates a new container.
     *
     * @param container The container to create
     * @return The created container, or an error
     */
    suspend fun createContainer(container: Container): AltairResult<Container>

    /**
     * Updates an existing container.
     *
     * @param container The container with updated fields
     * @return The updated container, or an error
     */
    suspend fun updateContainer(container: Container): AltairResult<Container>

    /**
     * Moves a container to a different location.
     *
     * All contained items move with the container.
     *
     * @param id The container's ULID
     * @param locationId The target location's ULID (null to clear location)
     * @return The updated container, or an error
     */
    suspend fun moveContainer(id: Ulid, locationId: Ulid?): AltairResult<Container>

    /**
     * Deletes a container.
     *
     * Fails if container contains items.
     *
     * @param id The container's ULID
     * @return Unit on success, or an error
     */
    suspend fun deleteContainer(id: Ulid): AltairResult<Unit>

    // --- Templates ---

    /**
     * Retrieves all item templates for a user.
     *
     * @param userId The user's ULID
     * @return List of templates, or an error
     */
    suspend fun getTemplates(userId: Ulid): AltairResult<List<ItemTemplate>>

    /**
     * Creates a new item template with field definitions.
     *
     * @param template The template to create
     * @param fields List of field definitions for this template
     * @return The created template, or an error
     */
    suspend fun createTemplate(template: ItemTemplate, fields: List<FieldDefinition>): AltairResult<ItemTemplate>

    /**
     * Deletes an item template.
     *
     * Does not delete items created from this template.
     *
     * @param id The template's ULID
     * @return Unit on success, or an error
     */
    suspend fun deleteTemplate(id: Ulid): AltairResult<Unit>

    /**
     * Soft-deletes an item (marks deleted, but keeps data).
     *
     * @param id The item's ULID
     * @return Unit on success, or an error
     */
    suspend fun softDelete(id: Ulid): AltairResult<Unit>

    /**
     * Restores a soft-deleted item.
     *
     * @param id The item's ULID
     * @return Unit on success, or an error
     */
    suspend fun restore(id: Ulid): AltairResult<Unit>
}
