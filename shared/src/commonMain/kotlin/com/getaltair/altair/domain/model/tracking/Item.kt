package com.getaltair.altair.domain.model.tracking

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A physical object being tracked in the system.
 *
 * Items are the core entity of the Tracking module. They represent
 * physical possessions with location tracking, custom attributes,
 * and optional photos. Items can be based on templates for consistent
 * field definitions.
 */
@Serializable
data class Item(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val description: String?,
    val templateId: Ulid?,
    val locationId: Ulid?,
    val containerId: Ulid?,
    val quantity: Int,
    val photoAttachmentId: Ulid?,
    val initiativeId: Ulid?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(name.isNotBlank()) { "Item name must not be blank" }
        require(name.length <= 200) { "Item name must be at most 200 characters" }
        require(quantity >= 0) { "Item quantity must be non-negative" }
    }
}
