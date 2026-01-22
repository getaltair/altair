package com.getaltair.altair.domain.model.knowledge

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlinx.serialization.Serializable
import kotlin.time.Instant

/**
 * A piece of knowledge content in markdown format.
 *
 * Notes are the core entity of the Knowledge module. They support:
 * - Markdown content with wiki-style linking
 * - Hierarchical organization via Folders
 * - Flat categorization via Tags
 * - Bi-directional linking via NoteLinks
 * - File attachments
 */
@Serializable
data class Note(
    val id: Ulid,
    val userId: Ulid,
    val title: String,
    val content: String,
    val folderId: Ulid?,
    val initiativeId: Ulid?,
    val isPinned: Boolean,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(title.isNotBlank()) { "Note title must not be blank" }
        require(title.length <= 200) { "Note title must be at most 200 characters" }
    }
}
