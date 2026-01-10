package com.getaltair.altair.data.entity.mobile

/**
 * Item entity for mobile platforms (Android/iOS).
 *
 * Represents an inventory item in the application.
 * Follows ADR-002 entity conventions with ULID identifiers and soft delete.
 *
 * @property id Unique identifier in ULID format
 * @property name Item name (required)
 * @property description Optional item description
 * @property locationId Optional location ID where item is stored
 * @property containerId Optional container ID that holds this item
 * @property createdAt ISO 8601 timestamp when item was created
 * @property updatedAt ISO 8601 timestamp when item was last updated
 * @property deletedAt ISO 8601 timestamp when item was soft-deleted (null if active)
 * @property syncVersion Version counter for sync tracking
 */
data class Item(
    val id: String,
    val name: String,
    val description: String? = null,
    val locationId: String? = null,
    val containerId: String? = null,
    val createdAt: String,
    val updatedAt: String,
    val deletedAt: String? = null,
    val syncVersion: Long = 0,
) {
    /**
     * Returns true if this item has been soft-deleted.
     */
    val isDeleted: Boolean
        get() = deletedAt != null

    /**
     * Returns true if this item is stored in a location.
     */
    val hasLocation: Boolean
        get() = locationId != null

    /**
     * Returns true if this item is stored in a container.
     */
    val hasContainer: Boolean
        get() = containerId != null
}

/**
 * Extension function to convert SQLDelight-generated Item to domain Item entity.
 */
fun com.getaltair.altair.database.Item.toDomain(): Item = Item(
    id = id,
    name = name,
    description = description,
    locationId = location_id,
    containerId = container_id,
    createdAt = created_at,
    updatedAt = updated_at,
    deletedAt = deleted_at,
    syncVersion = sync_version,
)
