package com.getaltair.altair.domain.model.knowledge

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A file associated with a Note or InboxItem.
 *
 * Attachments store file metadata; the actual file content is stored
 * separately in object storage. They can be images, documents, audio,
 * or any other file type.
 *
 * An Attachment must belong to at least one of Note or InboxItem, but can
 * belong to both simultaneously. This supports the workflow where an InboxItem
 * with attachments is triaged into a Note - the attachment maintains its
 * association with both entities for traceability.
 */
@Serializable
data class Attachment(
    val id: Ulid,
    val userId: Ulid,
    val noteId: Ulid?,
    val inboxItemId: Ulid?,
    val filename: String,
    val mimeType: String,
    val sizeBytes: Long,
    val storagePath: String,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped, SoftDeletable {
    init {
        require(filename.isNotBlank()) { "Attachment filename must not be blank" }
        require(filename.length <= 255) { "Attachment filename must be at most 255 characters" }
        require(mimeType.isNotBlank()) { "Attachment mimeType must not be blank" }
        require(sizeBytes >= 0) { "Attachment size must be non-negative" }
        require(storagePath.isNotBlank()) { "Attachment storage path must not be blank" }
        require(noteId != null || inboxItemId != null) {
            "Attachment must belong to either a Note or an InboxItem"
        }
    }
}
