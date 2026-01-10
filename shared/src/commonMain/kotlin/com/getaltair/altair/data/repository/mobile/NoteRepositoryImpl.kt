package com.getaltair.altair.data.repository.mobile

import com.getaltair.altair.data.entity.mobile.Note
import com.getaltair.altair.data.entity.mobile.toDomain
import com.getaltair.altair.data.repository.Repository
import com.getaltair.altair.data.util.Timestamps
import com.getaltair.altair.data.util.UlidGenerator
import com.getaltair.altair.database.AltairDatabase

/**
 * SQLDelight-based repository implementation for Note entities.
 *
 * Provides CRUD operations for Note entities on mobile platforms (Android/iOS).
 * Uses soft-delete pattern for data retention and sync support.
 *
 * @property database The SQLDelight database instance
 */
class NoteRepositoryImpl(private val database: AltairDatabase) : Repository<Note, String> {

    private val queries get() = database.noteQueries

    /**
     * Creates a new note in the database.
     *
     * Generates a ULID if the entity ID is empty and sets timestamps.
     *
     * @param entity The note to create
     * @return The created note with generated fields populated
     */
    override suspend fun create(entity: Note): Note {
        val now = Timestamps.now()
        val id = if (entity.id.isBlank()) UlidGenerator.generate() else entity.id

        queries.insert(
            id = id,
            title = entity.title,
            content = entity.content,
            folder_id = entity.folderId,
            created_at = now,
            updated_at = now,
            deleted_at = null,
            sync_version = 0,
        )

        return entity.copy(
            id = id,
            createdAt = now,
            updatedAt = now,
            syncVersion = 0,
        )
    }

    /**
     * Finds a note by its identifier.
     *
     * @param id The note identifier (ULID)
     * @return The note if found, null otherwise
     */
    override suspend fun findById(id: String): Note? = queries.selectById(id).executeAsOneOrNull()?.toDomain()

    /**
     * Updates an existing note.
     *
     * Updates the updatedAt timestamp and increments syncVersion.
     *
     * @param entity The note with updated values
     * @return The updated note
     */
    override suspend fun update(entity: Note): Note {
        val now = Timestamps.now()

        queries.updateById(
            title = entity.title,
            content = entity.content,
            folder_id = entity.folderId,
            updated_at = now,
            id = entity.id,
        )

        return entity.copy(
            updatedAt = now,
            syncVersion = entity.syncVersion + 1,
        )
    }

    /**
     * Soft-deletes a note by its identifier.
     *
     * Sets the deletedAt timestamp rather than removing the record.
     *
     * @param id The note identifier
     * @return true if the note was deleted, false if not found
     */
    override suspend fun delete(id: String): Boolean {
        val existing = findById(id) ?: return false
        if (existing.isDeleted) return false

        val now = Timestamps.now()
        queries.softDeleteById(
            deleted_at = now,
            updated_at = now,
            id = id,
        )
        return true
    }

    /**
     * Finds all notes that are not soft-deleted.
     *
     * @return List of active notes ordered by creation date (newest first)
     */
    override suspend fun findAll(): List<Note> = queries.selectAll().executeAsList().map { it.toDomain() }

    /**
     * Finds all notes in a specific folder.
     *
     * @param folderId The folder identifier
     * @return List of active notes in the folder ordered by creation date (newest first)
     */
    suspend fun findByFolderId(folderId: String): List<Note> =
        queries.selectByFolderId(folderId).executeAsList().map { it.toDomain() }
}
