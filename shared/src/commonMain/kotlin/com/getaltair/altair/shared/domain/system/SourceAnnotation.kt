package com.getaltair.altair.shared.domain.system

import com.getaltair.altair.shared.domain.common.AnchorType
import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Fingerprint data for robust anchor point resolution across document changes.
 *
 * When a source document is updated, traditional anchors (page numbers, byte offsets)
 * can break. AnchorFingerprint provides multiple signals to re-locate the intended
 * anchor point even after the document structure changes.
 *
 * The fingerprint uses a combination of:
 * - Exact heading text (for quick matches when structure is stable)
 * - Simhash of heading (fuzzy matching if heading is slightly rephrased)
 * - Simhash of surrounding content (to locate the region even if heading changes)
 * - Structural hints (e.g., "2nd heading in section 3")
 *
 * @property headingText Exact text of the nearest heading to this anchor point
 * @property headingSimhash Locality-sensitive hash of the heading text for fuzzy matching
 * @property contentSimhash Locality-sensitive hash of content surrounding the anchor
 * @property structuralHint Human-readable structural position (e.g., "Chapter 3, Section 2")
 */
@Serializable
data class AnchorFingerprint(
    val headingText: String?,
    val headingSimhash: String?,
    val contentSimhash: String?,
    val structuralHint: String?
)

/**
 * Represents a user annotation (note, highlight, comment) attached to a specific
 * location within a SourceDocument.
 *
 * SourceAnnotations enable the core PKM workflow:
 * 1. Read source material (books, papers, web pages)
 * 2. Highlight important passages or add comments
 * 3. Link annotations to Quests, Notes, or Initiatives
 * 4. Retrieve annotations when revisiting source material
 *
 * Annotations use a robust anchoring system that survives document updates:
 * - AnchorType defines the granularity (document, page, heading, selection)
 * - AnchorValue stores the specific location (page number, text range, etc.)
 * - AnchorFingerprint provides fuzzy matching to re-locate anchors after changes
 *
 * Example use cases:
 * - Highlighting a paragraph in a PDF with a comment about its relevance
 * - Annotating a code snippet in documentation with implementation notes
 * - Marking a web article section with a link to a related Quest
 *
 * @property id Unique identifier for this annotation
 * @property userId Owner of this annotation
 * @property sourceDocumentId Reference to the SourceDocument being annotated
 * @property anchorType Type of anchor point (DOCUMENT, PAGE, HEADING, SELECTION)
 * @property anchorValue Specific anchor location (e.g., page number "42", text range "chars:120-450")
 * @property anchorFingerprint Fuzzy fingerprint for re-locating anchor after document changes
 * @property content Markdown-formatted annotation content
 * @property createdAt Timestamp when annotation was created
 * @property updatedAt Timestamp of last modification
 * @property deletedAt Soft-delete timestamp (null if not deleted)
 *
 * @throws IllegalArgumentException if content is blank
 */
@Serializable
data class SourceAnnotation(
    val id: Ulid,
    val userId: Ulid,
    val sourceDocumentId: Ulid,
    val anchorType: AnchorType,
    val anchorValue: String?,
    val anchorFingerprint: AnchorFingerprint?,
    val content: String,             // Markdown
    val createdAt: Instant,
    val updatedAt: Instant,
    val deletedAt: Instant?
) {
    init {
        require(content.isNotBlank()) { "Annotation content is required and cannot be blank" }
    }

    /**
     * Indicates whether this annotation has been soft-deleted.
     */
    val isDeleted: Boolean get() = deletedAt != null
}
