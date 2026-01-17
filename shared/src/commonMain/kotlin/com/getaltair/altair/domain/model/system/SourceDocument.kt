package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.ExtractionStatus
import com.getaltair.altair.domain.types.enums.SourceType
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * An imported external document for reference and annotation.
 *
 * SourceDocuments can be files (PDF, EPUB), web pages, or auto-imported from
 * watched folders. They support AI-powered extraction of content and embeddings
 * for semantic search, as well as user annotations.
 */
@Serializable
data class SourceDocument(
    val id: Ulid,
    val userId: Ulid,
    val title: String,
    val sourceType: SourceType,
    val sourcePath: String,
    val mimeType: String?,
    val fileSizeBytes: Long?,
    val pageCount: Int?,
    val extractionStatus: ExtractionStatus,
    val extractedText: String?,
    val watchedFolderId: Ulid?,
    val initiativeId: Ulid?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped, SoftDeletable {
    init {
        require(title.isNotBlank()) { "SourceDocument title must not be blank" }
        require(title.length <= 500) { "SourceDocument title must be at most 500 characters" }
        require(sourcePath.isNotBlank()) { "SourceDocument source path must not be blank" }
        fileSizeBytes?.let { require(it >= 0) { "File size must be non-negative" } }
        pageCount?.let { require(it >= 0) { "Page count must be non-negative" } }
    }
}
