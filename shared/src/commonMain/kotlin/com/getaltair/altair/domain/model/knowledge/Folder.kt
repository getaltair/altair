package com.getaltair.altair.domain.model.knowledge

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlinx.serialization.Serializable
import kotlin.time.Instant

/**
 * A hierarchical container for organizing Notes.
 *
 * Folders provide traditional hierarchical organization for notes.
 * They can be nested via parentId to create folder trees.
 */
@Serializable
data class Folder(
    val id: Ulid,
    val userId: Ulid,
    val name: String,
    val parentId: Ulid?,
    val sortOrder: Int,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(name.isNotBlank()) { "Folder name must not be blank" }
        require(name.length <= 100) { "Folder name must be at most 100 characters" }
        require(sortOrder >= 0) { "Sort order must be non-negative" }
        parentId?.let {
            require(it != id) { "Folder cannot be its own parent" }
        }
    }
}
