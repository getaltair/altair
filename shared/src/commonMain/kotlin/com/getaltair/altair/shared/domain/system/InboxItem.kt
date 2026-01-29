package com.getaltair.altair.shared.domain.system

import com.getaltair.altair.shared.domain.common.CaptureSource
import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Universal Inbox capture item awaiting triage.
 *
 * The Universal Inbox is a type-agnostic capture system for quick entry of ideas,
 * tasks, notes, and items without requiring upfront classification.
 *
 * ## Capture Flow
 * 1. User captures content via any [source] (keyboard, voice, camera, widget, etc.)
 * 2. Item enters inbox with [content] and optional [attachmentIds]
 * 3. User triages item into Quest (Guidance), Note (Knowledge), or Item (Tracking)
 * 4. Original inbox item is soft-deleted after successful triage
 *
 * ## Attachments
 * - [attachmentIds] reference File entities stored separately
 * - Images, audio, documents, etc. are all supported
 * - Attachment lifecycle is managed independently
 *
 * ## Soft Deletion
 * - Triaged items are soft-deleted via [deletedAt]
 * - Provides audit trail and undo capability
 * - Hard deletion is a separate cleanup process
 *
 * @property id Unique identifier for this inbox item
 * @property userId Owner of this inbox item
 * @property content Text content captured by the user
 * @property source How this item was captured (keyboard, voice, camera, etc.)
 * @property attachmentIds List of file attachment IDs (empty if none)
 * @property createdAt When this item was captured
 * @property deletedAt When this item was triaged/deleted (null if pending)
 */
@Serializable
data class InboxItem(
    val id: Ulid,
    val userId: Ulid,
    val content: String,
    val source: CaptureSource,
    val attachmentIds: List<Ulid>,
    val createdAt: Instant,
    val deletedAt: Instant?
) {
    init {
        require(content.isNotBlank()) { "Content required" }
    }

    /**
     * Whether this item has been triaged and soft-deleted.
     */
    val isDeleted: Boolean get() = deletedAt != null

    /**
     * Whether this item has file attachments.
     */
    val hasAttachments: Boolean get() = attachmentIds.isNotEmpty()
}
