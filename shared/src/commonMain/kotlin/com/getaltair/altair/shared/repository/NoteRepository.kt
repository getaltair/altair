package com.getaltair.altair.shared.repository

import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.knowledge.Attachment
import com.getaltair.altair.shared.domain.knowledge.Folder
import com.getaltair.altair.shared.domain.knowledge.Note
import com.getaltair.altair.shared.domain.knowledge.NoteLink
import com.getaltair.altair.shared.domain.knowledge.Tag

/**
 * Repository interface for Knowledge module operations.
 *
 * Manages notes, folders, tags, attachments, and bidirectional linking.
 * Supports both full-text and semantic search for note discovery.
 *
 * Notes are organized hierarchically via folders and horizontally via tags.
 * Markdown content is parsed to extract [[wikilinks]] for bidirectional navigation.
 */
interface NoteRepository {
    /**
     * Retrieves a note by its unique identifier.
     *
     * @param id The note's ULID
     * @return The note if found, or an error
     */
    suspend fun getById(id: Ulid): AltairResult<Note>

    /**
     * Retrieves all notes owned by a user.
     *
     * @param userId The user's ULID
     * @return List of all notes, or an error
     */
    suspend fun getAllForUser(userId: Ulid): AltairResult<List<Note>>

    /**
     * Retrieves notes in a specific folder.
     *
     * @param folderId The folder's ULID, or null for root-level notes
     * @return List of notes in the folder, or an error
     */
    suspend fun getByFolder(folderId: Ulid?): AltairResult<List<Note>>

    /**
     * Retrieves notes associated with an initiative.
     *
     * @param initiativeId The initiative's ULID
     * @return List of notes linked to the initiative, or an error
     */
    suspend fun getByInitiative(initiativeId: Ulid): AltairResult<List<Note>>

    /**
     * Retrieves notes with a specific tag.
     *
     * @param tagId The tag's ULID
     * @return List of tagged notes, or an error
     */
    suspend fun getByTag(tagId: Ulid): AltairResult<List<Note>>

    /**
     * Performs full-text search across note titles and content.
     *
     * @param userId The user's ULID (scopes search to user's notes)
     * @param query The search query string
     * @return List of matching notes, or an error
     */
    suspend fun search(userId: Ulid, query: String): AltairResult<List<Note>>

    /**
     * Performs semantic search using vector embeddings.
     *
     * @param userId The user's ULID (scopes search to user's notes)
     * @param embedding The query embedding vector
     * @param limit Maximum number of results to return
     * @return List of semantically similar notes, or an error
     */
    suspend fun semanticSearch(userId: Ulid, embedding: List<Float>, limit: Int): AltairResult<List<Note>>

    /**
     * Creates a new note.
     *
     * @param note The note to create (id must be generated)
     * @return The created note with timestamps, or an error
     */
    suspend fun create(note: Note): AltairResult<Note>

    /**
     * Updates an existing note.
     *
     * @param note The note with updated fields
     * @return The updated note, or an error
     */
    suspend fun update(note: Note): AltairResult<Note>

    /**
     * Updates a note's vector embedding for semantic search.
     *
     * @param id The note's ULID
     * @param embedding The new embedding vector
     * @return Unit on success, or an error
     */
    suspend fun updateEmbedding(id: Ulid, embedding: List<Float>): AltairResult<Unit>

    // --- Linking ---

    /**
     * Retrieves all links pointing to this note (backlinks).
     *
     * @param noteId The note's ULID
     * @return List of incoming links, or an error
     */
    suspend fun getBacklinks(noteId: Ulid): AltairResult<List<NoteLink>>

    /**
     * Retrieves all links from this note to other notes.
     *
     * @param noteId The note's ULID
     * @return List of outgoing links, or an error
     */
    suspend fun getOutgoingLinks(noteId: Ulid): AltairResult<List<NoteLink>>

    /**
     * Parses note content for [[wikilinks]] and persists them as NoteLink entities.
     *
     * Replaces all existing outgoing links for this note.
     *
     * @param noteId The note's ULID
     * @param content The markdown content to parse
     * @return Unit on success, or an error
     */
    suspend fun parseAndSaveLinks(noteId: Ulid, content: String): AltairResult<Unit>

    // --- Tags ---

    /**
     * Retrieves all tags applied to a note.
     *
     * @param noteId The note's ULID
     * @return List of tags, or an error
     */
    suspend fun getTags(noteId: Ulid): AltairResult<List<Tag>>

    /**
     * Sets the tags for a note, replacing any existing tags.
     *
     * @param noteId The note's ULID
     * @param tagIds List of tag ULIDs to apply
     * @return Unit on success, or an error
     */
    suspend fun setTags(noteId: Ulid, tagIds: List<Ulid>): AltairResult<Unit>

    /**
     * Retrieves all tags owned by a user.
     *
     * @param userId The user's ULID
     * @return List of all tags, or an error
     */
    suspend fun getAllTags(userId: Ulid): AltairResult<List<Tag>>

    /**
     * Creates a new tag.
     *
     * @param tag The tag to create
     * @return The created tag, or an error
     */
    suspend fun createTag(tag: Tag): AltairResult<Tag>

    /**
     * Deletes a tag and removes it from all notes.
     *
     * @param id The tag's ULID
     * @return Unit on success, or an error
     */
    suspend fun deleteTag(id: Ulid): AltairResult<Unit>

    // --- Attachments ---

    /**
     * Retrieves all attachments for a note.
     *
     * @param noteId The note's ULID
     * @return List of attachments, or an error
     */
    suspend fun getAttachments(noteId: Ulid): AltairResult<List<Attachment>>

    /**
     * Adds an attachment to a note.
     *
     * @param attachment The attachment to create
     * @return The created attachment with metadata, or an error
     */
    suspend fun addAttachment(attachment: Attachment): AltairResult<Attachment>

    /**
     * Deletes an attachment and its associated file.
     *
     * @param id The attachment's ULID
     * @return Unit on success, or an error
     */
    suspend fun deleteAttachment(id: Ulid): AltairResult<Unit>

    // --- Folders ---

    /**
     * Retrieves all folders for a user.
     *
     * @param userId The user's ULID
     * @return List of folders (hierarchical structure), or an error
     */
    suspend fun getFolders(userId: Ulid): AltairResult<List<Folder>>

    /**
     * Creates a new folder.
     *
     * @param folder The folder to create
     * @return The created folder, or an error
     */
    suspend fun createFolder(folder: Folder): AltairResult<Folder>

    /**
     * Updates an existing folder.
     *
     * @param folder The folder with updated fields
     * @return The updated folder, or an error
     */
    suspend fun updateFolder(folder: Folder): AltairResult<Folder>

    /**
     * Deletes a folder, moving its notes to the parent folder.
     *
     * @param id The folder's ULID
     * @return Unit on success, or an error
     */
    suspend fun deleteFolder(id: Ulid): AltairResult<Unit>

    /**
     * Soft-deletes a note (marks deleted, but keeps data).
     *
     * @param id The note's ULID
     * @return Unit on success, or an error
     */
    suspend fun softDelete(id: Ulid): AltairResult<Unit>

    /**
     * Restores a soft-deleted note.
     *
     * @param id The note's ULID
     * @return Unit on success, or an error
     */
    suspend fun restore(id: Ulid): AltairResult<Unit>
}
