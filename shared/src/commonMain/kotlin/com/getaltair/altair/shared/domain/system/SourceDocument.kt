package com.getaltair.altair.shared.domain.system

import com.getaltair.altair.shared.domain.common.ExtractionStatus
import com.getaltair.altair.shared.domain.common.SourceType
import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Represents a source document tracked for knowledge extraction in Altair's Knowledge module.
 *
 * SourceDocuments serve as the foundation for the Personal Knowledge Management system,
 * representing external files, web pages, or other content sources that can be:
 * - Extracted and processed for text content
 * - Semantically indexed via embeddings
 * - Annotated with user notes
 * - Watched for changes and re-processed
 *
 * The extraction lifecycle:
 * 1. Document is registered (status = PENDING)
 * 2. Content is extracted and hashed (status = PROCESSED)
 * 3. If source changes, document becomes STALE
 * 4. Re-extraction updates content and embeddings
 *
 * @property id Unique identifier for this source document
 * @property userId Owner of this document
 * @property title Human-readable document title (max 200 characters)
 * @property sourceType Type of source (FILE, URI, WATCHED)
 * @property sourcePath Platform-specific path or URI to the source
 * @property mimeType MIME type of the source content
 * @property contentHash SHA-256 hash of source content for change detection
 * @property extractedText Extracted plain text from the document (null if not yet processed)
 * @property embedding Vector representation for semantic search (null if not yet generated)
 * @property status Current extraction processing status
 * @property errorMessage Error details if extraction failed (null otherwise)
 * @property initiativeId Optional link to an Initiative (Project/Area) for organization
 * @property watchedFolderId If this document came from a watched folder, reference to that folder
 * @property lastSyncedAt Last time the document was checked for changes (null if never synced)
 * @property createdAt Timestamp when this document was first registered
 * @property updatedAt Timestamp of last modification to this record
 * @property deletedAt Soft-delete timestamp (null if not deleted)
 *
 * @throws IllegalArgumentException if title is blank or exceeds 200 characters
 */
@Serializable
data class SourceDocument(
    val id: Ulid,
    val userId: Ulid,
    val title: String,
    val sourceType: SourceType,
    val sourcePath: String,
    val mimeType: String,
    val contentHash: String,         // SHA-256
    val extractedText: String?,
    val embedding: List<Float>?,     // vector for semantic search
    val status: ExtractionStatus,
    val errorMessage: String?,
    val initiativeId: Ulid?,
    val watchedFolderId: Ulid?,
    val lastSyncedAt: Instant?,
    val createdAt: Instant,
    val updatedAt: Instant,
    val deletedAt: Instant?
) {
    init {
        require(title.length <= 200) { "Title must not exceed 200 characters, got ${title.length}" }
        require(title.isNotBlank()) { "Title is required and cannot be blank" }
    }

    /**
     * Indicates whether this document has been soft-deleted.
     */
    val isDeleted: Boolean get() = deletedAt != null

    /**
     * Indicates whether extraction has successfully completed.
     */
    val isProcessed: Boolean get() = status == ExtractionStatus.PROCESSED

    /**
     * Indicates whether the document needs re-processing due to source changes.
     */
    val needsReprocessing: Boolean get() = status == ExtractionStatus.STALE
}
