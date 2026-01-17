package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.NoteError
import com.getaltair.altair.domain.model.knowledge.NoteLink
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Repository for NoteLink entities.
 *
 * NoteLinks represent directional connections between notes,
 * enabling knowledge graph navigation and backlink discovery.
 */
interface NoteLinkRepository : Repository<NoteLink, NoteError> {
    /**
     * Finds all links originating from a specific note.
     *
     * @param sourceNoteId The ULID of the source note
     * @return A Flow emitting outgoing links
     */
    fun findBySource(sourceNoteId: Ulid): Flow<List<NoteLink>>

    /**
     * Finds all links pointing to a specific note.
     *
     * @param targetNoteId The ULID of the target note
     * @return A Flow emitting incoming links (backlinks)
     */
    fun findByTarget(targetNoteId: Ulid): Flow<List<NoteLink>>

    /**
     * Finds a link between two specific notes.
     *
     * @param sourceNoteId The ULID of the source note
     * @param targetNoteId The ULID of the target note
     * @return Either an error if not found, or the link
     */
    suspend fun findBySourceAndTarget(
        sourceNoteId: Ulid,
        targetNoteId: Ulid,
    ): Either<NoteError, NoteLink>

    /**
     * Creates or updates a link between two notes.
     *
     * If a link already exists between the notes, updates its context.
     * Otherwise, creates a new link.
     *
     * @param sourceNoteId The ULID of the source note
     * @param targetNoteId The ULID of the target note
     * @param context Optional context describing the link
     * @return Either an error on failure, or the created/updated link
     */
    suspend fun linkNotes(
        sourceNoteId: Ulid,
        targetNoteId: Ulid,
        context: String? = null,
    ): Either<NoteError, NoteLink>

    /**
     * Removes a link between two notes.
     *
     * @param sourceNoteId The ULID of the source note
     * @param targetNoteId The ULID of the target note
     * @return Either an error on failure, or Unit on success
     */
    suspend fun unlinkNotes(
        sourceNoteId: Ulid,
        targetNoteId: Ulid,
    ): Either<NoteError, Unit>

    /**
     * Syncs links for a note based on its wiki-link content.
     *
     * Parses the note content for [[wiki links]], resolves them to note IDs,
     * and creates/removes NoteLinks to match. Invalid links are ignored.
     *
     * @param noteId The ULID of the note to sync links for
     * @return Either an error on failure, or the list of resolved links
     */
    suspend fun syncLinksFromContent(noteId: Ulid): Either<NoteError, List<NoteLink>>
}
