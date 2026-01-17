package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.common.ColorValidation
import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.AnchorType
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A user annotation on a SourceDocument.
 *
 * Annotations allow users to highlight, comment on, or mark sections of
 * imported documents. They can be anchored to different levels of granularity
 * (document, page, heading, or text selection).
 */
@Serializable
data class SourceAnnotation(
    val id: Ulid,
    val userId: Ulid,
    val sourceDocumentId: Ulid,
    val anchorType: AnchorType,
    val anchorData: String?,
    val content: String,
    val highlightColor: String?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped,
    SoftDeletable {
    init {
        require(content.isNotBlank()) { "SourceAnnotation content must not be blank" }
        require(content.length <= 10_000) { "SourceAnnotation content must be at most 10000 characters" }
        ColorValidation.requireValidHexColor(highlightColor, "Highlight color")
    }
}
