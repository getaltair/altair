package com.getaltair.altair.shared.domain.tracking

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Represents a physical location where items can be stored.
 *
 * Locations form a hierarchical tree structure via parentId:
 * - Root locations: parentId = null (e.g., "Home", "Office")
 * - Child locations: parentId references parent (e.g., "Kitchen" under "Home")
 *
 * Examples:
 * - Home (root) → Kitchen → Pantry
 * - Office (root) → Storage Room → Shelf A
 *
 * Items can be placed directly at locations, or inside containers at locations.
 *
 * Invariants:
 * - Name is required and max 100 characters
 *
 * @property id Unique identifier for this location
 * @property userId Owner of this location
 * @property name Human-readable location name (required, max 100 chars)
 * @property description Optional detailed description
 * @property parentId Optional parent location for hierarchy (null = root)
 * @property createdAt When this location was created
 */
@Serializable
data class Location(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val parentId: Ulid?,
    val createdAt: Instant
) {
    init {
        require(name.length <= 100) { "Name max 100 chars" }
        require(name.isNotBlank()) { "Name required" }
    }

    /**
     * Returns true if this is a root location (no parent).
     */
    val isRoot: Boolean get() = parentId == null
}
