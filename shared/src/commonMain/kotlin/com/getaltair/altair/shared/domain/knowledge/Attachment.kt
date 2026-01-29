package com.getaltair.altair.shared.domain.knowledge

import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A file attachment associated with a note or inbox item.
 *
 * Attachments represent binary files (images, PDFs, audio, etc.) that enhance notes
 * or capture content in the Universal Inbox.
 *
 * ## Storage Architecture
 * - Files are stored in object storage (S3-compatible) using [storageKey]
 * - Metadata (filename, size, type) stored in database
 * - Content hash enables deduplication (same file uploaded multiple times = one storage entry)
 *
 * ## Ownership
 * Attachments can belong to either:
 * - A Note ([noteId] set, [inboxId] null)
 * - An Inbox item ([inboxId] set, [noteId] null)
 * - Neither yet (both null) - orphaned, eligible for cleanup
 *
 * ## Size Limits
 * Maximum file size is [MAX_FILE_SIZE] (100 MB by default) to balance:
 * - User experience (reasonable upload times)
 * - Storage costs
 * - Memory constraints on mobile devices
 *
 * ## MIME Types
 * Common types include:
 * - Images: image/png, image/jpeg, image/gif
 * - Documents: application/pdf, text/plain
 * - Audio: audio/mpeg, audio/wav
 *
 * @property id Unique identifier for this attachment
 * @property userId Owner of this attachment
 * @property noteId Note this attachment belongs to (null if inbox or orphaned)
 * @property inboxId Inbox item this attachment belongs to (null if note or orphaned)
 * @property filename Original filename with extension
 * @property mimeType MIME type of the file (e.g., "image/png")
 * @property sizeBytes File size in bytes
 * @property storageKey Object storage key (e.g., S3 object key)
 * @property hash Content hash for deduplication (e.g., SHA-256 hex digest)
 * @property createdAt When this attachment was uploaded
 */
@Serializable
data class Attachment(
    val id: Ulid,
    val userId: Ulid,
    val noteId: Ulid?,
    val inboxId: Ulid?,
    val filename: String,
    val mimeType: String,
    val sizeBytes: Long,
    val storageKey: String,
    val hash: String,
    val createdAt: Instant
) {
    init {
        require(filename.isNotBlank()) { "Filename required" }
        require(sizeBytes > 0) { "Size must be positive, got $sizeBytes" }
        require(sizeBytes <= MAX_FILE_SIZE) {
            "File too large (max ${MAX_FILE_SIZE / 1_000_000}MB), got ${sizeBytes / 1_000_000}MB"
        }
    }

    /**
     * Whether this attachment is associated with a note.
     */
    val isForNote: Boolean get() = noteId != null

    /**
     * Whether this attachment is associated with an inbox item.
     */
    val isForInbox: Boolean get() = inboxId != null

    /**
     * Whether this attachment is orphaned (not associated with any parent entity).
     *
     * Orphaned attachments are candidates for garbage collection after a grace period.
     */
    val isOrphaned: Boolean get() = noteId == null && inboxId == null

    companion object {
        /** Maximum allowed file size in bytes (100 MB) */
        const val MAX_FILE_SIZE = 100_000_000L
    }
}
