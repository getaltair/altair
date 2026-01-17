package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.NoteError
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.Flow

/**
 * Repository for Note entities.
 *
 * Notes are markdown content that support wiki-style linking,
 * hierarchical organization via Folders, and flat categorization via Tags.
 */
interface NoteRepository : Repository<Note, NoteError> {
    /**
     * Finds all notes in a specific folder.
     *
     * @param folderId The ULID of the folder (null for root-level notes)
     * @return A Flow emitting notes in the folder
     */
    fun findByFolder(folderId: Ulid?): Flow<List<Note>>

    /**
     * Finds all pinned notes.
     *
     * @return A Flow emitting pinned notes
     */
    fun findPinned(): Flow<List<Note>>

    /**
     * Finds notes associated with a specific initiative.
     *
     * @param initiativeId The ULID of the initiative
     * @return A Flow emitting notes for the initiative
     */
    fun findByInitiative(initiativeId: Ulid): Flow<List<Note>>

    /**
     * Searches notes by title and content (full-text search).
     *
     * @param query The search query
     * @return Either an error on failure, or matching notes
     */
    suspend fun search(query: String): Either<NoteError, List<Note>>

    /**
     * Finds notes that link TO the specified note (backlinks).
     *
     * @param noteId The ULID of the target note
     * @return A Flow emitting notes that contain links to this note
     */
    fun findBacklinks(noteId: Ulid): Flow<List<Note>>

    /**
     * Finds notes that the specified note links TO (forward links).
     *
     * @param noteId The ULID of the source note
     * @return A Flow emitting notes that this note links to
     */
    fun findForwardLinks(noteId: Ulid): Flow<List<Note>>

    /**
     * Toggles the pinned status of a note.
     *
     * @param id The ULID of the note
     * @return Either an error on failure, or the updated note
     */
    suspend fun togglePinned(id: Ulid): Either<NoteError, Note>

    /**
     * Moves a note to a different folder.
     *
     * @param id The ULID of the note
     * @param folderId The target folder ID (null for root)
     * @return Either an error on failure, or the updated note
     */
    suspend fun moveToFolder(
        id: Ulid,
        folderId: Ulid?,
    ): Either<NoteError, Note>
}
