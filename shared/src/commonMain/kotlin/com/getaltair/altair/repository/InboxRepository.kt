package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.system.InboxItem
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.CaptureSource
import kotlinx.coroutines.flow.Flow

/**
 * Repository for InboxItem entities.
 *
 * InboxItems are quick-capture entries that haven't yet been triaged
 * into their final form (Quest, Note, Item, etc.).
 */
interface InboxRepository : Repository<InboxItem, DomainError> {
    /**
     * Captures a new item into the inbox.
     *
     * This is a convenience method that creates an InboxItem with
     * the given content and source.
     *
     * @param content The captured content text
     * @param source How the item was captured
     * @param attachmentIds Optional attachment IDs associated with this capture
     * @return Either an error on failure, or the created InboxItem
     */
    suspend fun capture(
        content: String,
        source: CaptureSource,
        attachmentIds: List<Ulid> = emptyList(),
    ): Either<DomainError, InboxItem>

    /**
     * Triages an inbox item by deleting it after it has been processed.
     *
     * This is typically called after the inbox item has been converted
     * to a Quest, Note, or other entity.
     *
     * @param id The ULID of the inbox item to triage
     * @return Either an error on failure, or Unit on success
     */
    suspend fun triage(id: Ulid): Either<DomainError, Unit>

    /**
     * Returns the count of items currently in the inbox.
     *
     * @return Either an error on failure, or the count
     */
    suspend fun count(): Either<DomainError, Int>

    /**
     * Finds all inbox items captured from a specific source.
     *
     * @param source The capture source to filter by
     * @return A Flow emitting inbox items from the specified source
     */
    fun findBySource(source: CaptureSource): Flow<List<InboxItem>>
}
