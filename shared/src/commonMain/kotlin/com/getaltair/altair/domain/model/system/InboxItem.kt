package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.CaptureSource
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A captured item awaiting triage in the Universal Inbox.
 *
 * InboxItems are quick-capture entries that haven't yet been processed into
 * their final form (Quest, Note, Item, etc.). They support multiple capture
 * methods and can include attachments for photos, voice memos, etc.
 *
 * InboxItems are not soft-deletable; they are processed and deleted when triaged.
 */
@Serializable
data class InboxItem(
    val id: Ulid,
    val userId: Ulid,
    val content: String,
    val source: CaptureSource,
    val attachmentIds: List<Ulid>,
    override val createdAt: Instant,
    override val updatedAt: Instant,
) : Timestamped {
    init {
        require(content.isNotBlank()) { "InboxItem content must not be blank" }
        require(content.length <= 5000) { "InboxItem content must be at most 5000 characters" }
    }
}
