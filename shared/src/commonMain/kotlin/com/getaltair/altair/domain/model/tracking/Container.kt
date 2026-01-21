package com.getaltair.altair.domain.model.tracking

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlinx.serialization.Serializable
import kotlin.time.Instant

/**
 * A movable storage unit that can contain Items.
 *
 * Containers are physical objects (boxes, bags, drawers) that hold Items
 * and can be moved between Locations. Unlike Locations, Containers are
 * themselves movable and can be nested.
 */
@Serializable
data class Container(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val locationId: Ulid?,
    val parentContainerId: Ulid?,
    val label: String?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(name.isNotBlank()) { "Container name must not be blank" }
        require(name.length <= 100) { "Container name must be at most 100 characters" }
        parentContainerId?.let {
            require(it != id) { "Container cannot be its own parent" }
        }
    }
}
