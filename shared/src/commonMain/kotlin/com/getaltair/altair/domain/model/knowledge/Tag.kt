package com.getaltair.altair.domain.model.knowledge

import com.getaltair.altair.domain.common.ColorValidation
import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A flat-namespace label for categorizing Notes.
 *
 * Tags provide non-hierarchical organization that complements Folders.
 * A Note can have multiple Tags, enabling cross-cutting categorization.
 */
@Serializable
data class Tag(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val color: String?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped, SoftDeletable {
    init {
        require(name.isNotBlank()) { "Tag name must not be blank" }
        require(name.length <= 50) { "Tag name must be at most 50 characters" }
        require(!name.contains(' ')) { "Tag name must not contain spaces" }
        ColorValidation.requireValidHexColor(color, "Tag color")
    }
}
