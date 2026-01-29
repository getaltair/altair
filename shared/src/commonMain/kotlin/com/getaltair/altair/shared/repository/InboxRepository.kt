package com.getaltair.altair.shared.repository

import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.guidance.Quest
import com.getaltair.altair.shared.domain.knowledge.Note
import com.getaltair.altair.shared.domain.system.InboxItem
import com.getaltair.altair.shared.domain.system.SourceDocument
import com.getaltair.altair.shared.domain.tracking.Item

/**
 * Repository interface for managing the Universal Inbox.
 *
 * The Inbox is a type-agnostic capture system that holds unprocessed items until they
 * are triaged into their proper domains (Guidance/Quest, Knowledge/Note, Tracking/Item).
 * This supports ADHD-friendly quick capture without requiring upfront categorization.
 */
interface InboxRepository {
    /**
     * Retrieves an InboxItem by its unique identifier.
     *
     * @param id The unique identifier of the InboxItem
     * @return Success with the InboxItem, or Failure if not found
     */
    suspend fun getById(id: Ulid): AltairResult<InboxItem>

    /**
     * Retrieves all InboxItems for a specific user.
     * Returns items in reverse chronological order (newest first).
     *
     * @param userId The unique identifier of the user
     * @return Success with list of InboxItems (may be empty), or Failure on error
     */
    suspend fun getAllForUser(userId: Ulid): AltairResult<List<InboxItem>>

    /**
     * Creates a new InboxItem for quick capture.
     *
     * @param item The InboxItem to create (ID should be generated)
     * @return Success with the created InboxItem (with populated metadata), or Failure on error
     */
    suspend fun create(item: InboxItem): AltairResult<InboxItem>

    /**
     * Soft deletes an InboxItem by setting deletedAt timestamp.
     * Used when an item is discarded without triage.
     *
     * @param id The unique identifier of the InboxItem to delete
     * @return Success on deletion, or Failure if not found
     */
    suspend fun softDelete(id: Ulid): AltairResult<Unit>

    /**
     * Triages an InboxItem to a Quest in the Guidance module.
     * Creates the Quest and soft-deletes the InboxItem atomically.
     *
     * @param itemId The unique identifier of the InboxItem to triage
     * @param quest The Quest to create from the InboxItem
     * @return Success with the created Quest's ID, or Failure on error
     */
    suspend fun triageToQuest(itemId: Ulid, quest: Quest): AltairResult<Ulid>

    /**
     * Triages an InboxItem to a Note in the Knowledge module.
     * Creates the Note and soft-deletes the InboxItem atomically.
     *
     * @param itemId The unique identifier of the InboxItem to triage
     * @param note The Note to create from the InboxItem
     * @return Success with the created Note's ID, or Failure on error
     */
    suspend fun triageToNote(itemId: Ulid, note: Note): AltairResult<Ulid>

    /**
     * Triages an InboxItem to an Item in the Tracking module.
     * Creates the Item and soft-deletes the InboxItem atomically.
     *
     * @param itemId The unique identifier of the InboxItem to triage
     * @param item The Item to create from the InboxItem
     * @return Success with the created Item's ID, or Failure on error
     */
    suspend fun triageToItem(itemId: Ulid, item: Item): AltairResult<Ulid>

    /**
     * Triages an InboxItem to a SourceDocument in the Knowledge module.
     * Creates the SourceDocument and soft-deletes the InboxItem atomically.
     *
     * @param itemId The unique identifier of the InboxItem to triage
     * @param sourceDoc The SourceDocument to create from the InboxItem
     * @return Success with the created SourceDocument's ID, or Failure on error
     */
    suspend fun triageToSourceDocument(itemId: Ulid, sourceDoc: SourceDocument): AltairResult<Ulid>
}
