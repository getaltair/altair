package com.getaltair.altair.shared.domain.tracking

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Represents a physical item in inventory tracking.
 *
 * Items are discrete countable units (tools, books, supplies) with quantity tracking.
 * Each item can be:
 * - Located at a Location (physical place)
 * - Stored in a Container (box, drawer, etc.)
 * - Created from an ItemTemplate (for consistency)
 * - Associated with an Initiative (project/area)
 * - Enhanced with CustomFields (metadata)
 * - Photographed (imageKey references S3 storage)
 *
 * Invariants:
 * - Item cannot have both locationId and containerId (one or neither)
 * - Quantity must be non-negative
 * - Name is required and max 200 characters
 *
 * @property id Unique identifier for this item
 * @property userId Owner of this item
 * @property name Human-readable item name (required, max 200 chars)
 * @property description Optional detailed description
 * @property quantity Current item count (≥0, 0 = out of stock)
 * @property templateId Optional reference to ItemTemplate for consistency
 * @property locationId Optional direct location (mutually exclusive with containerId)
 * @property containerId Optional container location (mutually exclusive with locationId)
 * @property initiativeId Optional association with Initiative (project/area)
 * @property imageKey Optional S3 key for primary photo
 * @property createdAt When this item was created
 * @property updatedAt Last modification timestamp
 * @property deletedAt Soft deletion timestamp (null = not deleted)
 */
@Serializable
data class Item(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val quantity: Int,
    val templateId: Ulid?,
    val locationId: Ulid?,
    val containerId: Ulid?,
    val initiativeId: Ulid?,
    val imageKey: String?,
    val createdAt: Instant,
    val updatedAt: Instant,
    val deletedAt: Instant?
) {
    init {
        require(name.length <= 200) { "Name max 200 chars" }
        require(name.isNotBlank()) { "Name required" }
        require(quantity >= 0) { "Quantity must be non-negative" }
        require(!(locationId != null && containerId != null)) {
            "Item cannot have both location and container"
        }
    }

    /**
     * Returns true if this item has zero quantity.
     */
    val isOutOfStock: Boolean get() = quantity == 0

    /**
     * Returns true if this item is stored in a container.
     */
    val isInContainer: Boolean get() = containerId != null

    /**
     * Returns true if this item is directly at a location.
     */
    val isAtLocation: Boolean get() = locationId != null

    /**
     * Returns true if this item has been soft-deleted.
     */
    val isDeleted: Boolean get() = deletedAt != null
}
