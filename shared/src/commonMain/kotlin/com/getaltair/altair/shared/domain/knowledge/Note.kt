package com.getaltair.altair.shared.domain.knowledge

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A note in the Knowledge domain (PKM system).
 *
 * Notes are the primary content unit in Altair's Personal Knowledge Management system.
 * Each note contains Markdown-formatted content and can be organized via folders,
 * linked to other notes through bidirectional links, and semantically searched via embeddings.
 *
 * ## Organizational Features
 * - Hierarchical folder organization via [folderId]
 * - Cross-module initiative linkage via [initiativeId]
 * - Semantic search via optional [embedding] vector
 *
 * ## Markdown Support
 * Content supports standard Markdown with:
 * - Wiki-style links: `[[Note Title]]`
 * - Tags: `#tag-name`
 * - Code blocks, tables, task lists
 *
 * ## Soft Deletion
 * Notes use soft-deletion via [deletedAt] timestamp to preserve links and history.
 *
 * @property id Unique identifier for this note
 * @property userId Owner of this note
 * @property title Human-readable title (max 200 characters, required)
 * @property content Markdown-formatted note body
 * @property folderId Optional parent folder for hierarchical organization
 * @property initiativeId Optional link to cross-module Initiative (Project/Area)
 * @property embedding Optional vector for semantic search (dimension TBD by embedding model)
 * @property createdAt When this note was originally created
 * @property updatedAt When this note was last modified
 * @property deletedAt When this note was soft-deleted (null if active)
 */
@Serializable
data class Note(
    val id: Ulid,
    val userId: Ulid,
    val title: String,
    val content: String,
    val folderId: Ulid?,
    val initiativeId: Ulid?,
    val embedding: List<Float>?,
    val createdAt: Instant,
    val updatedAt: Instant,
    val deletedAt: Instant?
) {
    init {
        require(title.length <= MAX_TITLE_LENGTH) {
            "Title max $MAX_TITLE_LENGTH chars, got ${title.length}"
        }
        require(title.isNotBlank()) { "Title required" }
    }

    /**
     * Whether this note has been soft-deleted.
     */
    val isDeleted: Boolean get() = deletedAt != null

    /**
     * Whether this note belongs to a folder.
     */
    val isInFolder: Boolean get() = folderId != null

    /**
     * Whether this note has an embedding vector for semantic search.
     */
    val hasEmbedding: Boolean get() = embedding != null

    companion object {
        /** Maximum allowed title length in characters */
        const val MAX_TITLE_LENGTH = 200
    }
}
