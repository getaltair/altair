package com.getaltair.altair.shared.domain.knowledge

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A bidirectional link between two notes.
 *
 * NoteLinks represent wiki-style connections between notes, enabling:
 * - Knowledge graph visualization
 * - Backlink display (notes linking TO this note)
 * - Related note discovery
 * - Context-aware link previews
 *
 * ## Bidirectionality
 * When a link is created from Note A to Note B:
 * - Note A shows it as an outgoing link
 * - Note B shows it as a backlink (incoming link)
 * - Only one NoteLink record is needed (sourceId → targetId)
 *
 * ## Link Context
 * The optional [context] field captures surrounding text from the source note
 * to provide preview context when hovering/viewing the link.
 *
 * Example:
 * ```
 * Source: "I need to implement [[Authentication]] before deployment"
 * Context: "...implement [[Authentication]] before..."
 * ```
 *
 * @property id Unique identifier for this link
 * @property sourceId Note containing the link (the "from" note)
 * @property targetId Note being linked to (the "to" note)
 * @property context Optional surrounding text for preview (e.g., sentence containing the link)
 * @property createdAt When this link was created
 */
@Serializable
data class NoteLink(
    val id: Ulid,
    val sourceId: Ulid,
    val targetId: Ulid,
    val context: String?,
    val createdAt: Instant
) {
    /**
     * Whether this link points back to itself.
     *
     * Self-links are technically valid but may be filtered out in UI/search contexts.
     */
    val isSelfLink: Boolean get() = sourceId == targetId
}
