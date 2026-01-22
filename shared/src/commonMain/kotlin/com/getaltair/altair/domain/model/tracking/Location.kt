package com.getaltair.altair.domain.model.tracking

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlinx.serialization.Serializable
import kotlin.time.Instant

/**
 * A physical place where Items can be stored.
 *
 * Locations represent fixed physical spaces (rooms, buildings, etc.)
 * that can contain Items directly or Containers. They can be hierarchical
 * via parentId (e.g., Building > Floor > Room).
 */
@Serializable
data class Location(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val parentId: Ulid?,
    val address: String?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(name.isNotBlank()) { "Location name must not be blank" }
        require(name.length <= 100) { "Location name must be at most 100 characters" }
        parentId?.let {
            require(it != id) { "Location cannot be its own parent" }
        }
    }
}
