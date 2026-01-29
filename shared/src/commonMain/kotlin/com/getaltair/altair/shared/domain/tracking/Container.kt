package com.getaltair.altair.shared.domain.tracking

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Represents a container that can hold items.
 *
 * Containers enable organized storage:
 * - Can be placed at a Location (locationId)
 * - Can be nested inside another Container (parentId)
 * - Hold Items (items reference container via containerId)
 *
 * Examples:
 * - "Toolbox" at Location "Garage"
 * - "Small Parts Drawer" inside Container "Workbench Cabinet"
 * - "Storage Bin" at Location "Basement"
 *
 * Nesting allows for complex organization:
 * - Cabinet → Drawer → Organizer Tray
 *
 * Invariants:
 * - Name is required and max 100 characters
 *
 * @property id Unique identifier for this container
 * @property userId Owner of this container
 * @property name Human-readable container name (required, max 100 chars)
 * @property description Optional detailed description
 * @property locationId Optional location where container is placed
 * @property parentId Optional parent container for nesting
 * @property createdAt When this container was created
 */
@Serializable
data class Container(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val locationId: Ulid?,
    val parentId: Ulid?,
    val createdAt: Instant
) {
    init {
        require(name.length <= 100) { "Name max 100 chars" }
        require(name.isNotBlank()) { "Name required" }
    }

    /**
     * Returns true if this container is nested inside another container.
     */
    val isNested: Boolean get() = parentId != null

    /**
     * Returns true if this container is placed at a location.
     */
    val hasLocation: Boolean get() = locationId != null
}
