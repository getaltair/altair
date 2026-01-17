package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.knowledge.Attachment
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Repository for Attachment entities.
 *
 * Attachments store file metadata; the actual file content is stored
 * separately in object storage.
 */
interface AttachmentRepository : Repository<Attachment, DomainError> {
    /**
     * Finds all attachments for a specific note.
     *
     * @param noteId The ULID of the note
     * @return A Flow emitting attachments for the note
     */
    fun findByNote(noteId: Ulid): Flow<List<Attachment>>

    /**
     * Finds all attachments for a specific inbox item.
     *
     * @param inboxItemId The ULID of the inbox item
     * @return A Flow emitting attachments for the inbox item
     */
    fun findByInboxItem(inboxItemId: Ulid): Flow<List<Attachment>>

    /**
     * Associates an attachment with a note.
     *
     * This is typically called when triaging an inbox item with attachments.
     *
     * @param id The ULID of the attachment
     * @param noteId The ULID of the note
     * @return Either an error on failure, or the updated attachment
     */
    suspend fun associateWithNote(
        id: Ulid,
        noteId: Ulid,
    ): Either<DomainError, Attachment>

    /**
     * Calculates total storage used by all attachments.
     *
     * @return Either an error on failure, or the total size in bytes
     */
    suspend fun getTotalStorageUsed(): Either<DomainError, Long>

    /**
     * Finds attachments by MIME type pattern.
     *
     * @param mimeTypePrefix The MIME type prefix (e.g., "image/", "application/pdf")
     * @return A Flow emitting matching attachments
     */
    fun findByMimeType(mimeTypePrefix: String): Flow<List<Attachment>>

    /**
     * Finds orphaned attachments (not linked to any note or inbox item).
     *
     * These may occur if an inbox item is deleted without triaging.
     *
     * @return Either an error on failure, or orphaned attachments
     */
    suspend fun findOrphaned(): Either<DomainError, List<Attachment>>
}
