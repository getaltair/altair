package com.getaltair.altair.domain.model.knowledge

import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlin.time.Instant
import kotlinx.serialization.Serializable

/**
 * A directional connection between two Notes.
 *
 * NoteLinks represent explicit relationships between notes, enabling
 * knowledge graph navigation and backlink discovery. Links are directional
 * (from source to target) but the UI can show both directions.
 */
@Serializable
data class NoteLink(
    val id: Ulid,
    val userId: Ulid,
    val sourceNoteId: Ulid,
    val targetNoteId: Ulid,
    val context: String?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
) : Timestamped {
    init {
        require(sourceNoteId != targetNoteId) { "Note cannot link to itself" }
        context?.let {
            require(it.length <= 500) { "Link context must be at most 500 characters" }
        }
    }
}
