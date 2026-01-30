package com.getaltair.altair.shared.database.repository

import com.getaltair.altair.shared.database.AltairDatabase
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.knowledge.Attachment
import com.getaltair.altair.shared.domain.knowledge.Folder
import com.getaltair.altair.shared.domain.knowledge.Note
import com.getaltair.altair.shared.domain.knowledge.NoteLink
import com.getaltair.altair.shared.domain.knowledge.Tag
import com.getaltair.altair.shared.repository.NoteRepository
import kotlinx.datetime.Clock

/**
 * SQLite implementation of NoteRepository for mobile platforms.
 *
 * Maps between SQLDelight generated Note table and domain Note entities,
 * with support for folders, tags, links, and attachments.
 */
class SQLiteNoteRepository(database: AltairDatabase) : SQLiteRepository(database), NoteRepository {

    private val queries = database.noteQueries
    private val folderQueries = database.folderQueries
    private val tagQueries = database.tagQueries
    private val linkQueries = database.noteLinkQueries
    private val attachmentQueries = database.attachmentQueries

    override suspend fun getById(id: Ulid): AltairResult<Note> = dbOperation {
        val result = queries.selectById(id.value).executeAsOneOrNull()
        result?.toDomain() ?: throw NoSuchElementException("Note not found: ${id.value}")
    }.mapLeft { error ->
        if (error is AltairError.StorageError.DatabaseError &&
            error.message.contains("Note not found")) {
            AltairError.NotFoundError.NoteNotFound(id.value)
        } else {
            error
        }
    }

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Note>> = dbOperation {
        queries.selectByUserId(userId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getByFolder(folderId: Ulid?): AltairResult<List<Note>> = dbOperation {
        queries.selectByFolder(folderId?.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getByInitiative(initiativeId: Ulid): AltairResult<List<Note>> = dbOperation {
        queries.selectByInitiative(initiativeId.value).executeAsList().map { it.toDomain() }
    }

    override suspend fun getByTag(tagId: Ulid): AltairResult<List<Note>> = dbOperation {
        // TODO: Implement when note-tag junction table is available
        emptyList()
    }

    override suspend fun search(userId: Ulid, query: String): AltairResult<List<Note>> = dbOperation {
        queries.searchByTitle(query).executeAsList().map { it.toDomain() }
    }

    override suspend fun semanticSearch(userId: Ulid, embedding: List<Float>, limit: Int): AltairResult<List<Note>> = dbOperation {
        // TODO: Implement semantic search when vector support is added
        // For now, return empty list
        emptyList()
    }

    override suspend fun create(note: Note): AltairResult<Note> = dbOperation {
        queries.insert(
            id = note.id.value,
            user_id = note.userId.value,
            title = note.title,
            content = note.content,
            folder_id = note.folderId?.value,
            initiative_id = note.initiativeId?.value,
            embedding = note.embedding?.serialize(),
            created_at = note.createdAt.toLong(),
            updated_at = note.updatedAt.toLong(),
            deleted_at = note.deletedAt.toLongOrNull()
        )
        note
    }

    override suspend fun update(note: Note): AltairResult<Note> = dbOperation {
        queries.update(
            title = note.title,
            content = note.content,
            folder_id = note.folderId?.value,
            initiative_id = note.initiativeId?.value,
            embedding = note.embedding?.serialize(),
            updated_at = note.updatedAt.toLong(),
            id = note.id.value
        )
        note
    }

    override suspend fun updateEmbedding(id: Ulid, embedding: List<Float>): AltairResult<Unit> = dbOperation {
        val note = queries.selectById(id.value).executeAsOneOrNull()?.toDomain()
            ?: throw NoSuchElementException("Note not found: ${id.value}")

        val updated = note.copy(
            embedding = embedding,
            updatedAt = Clock.System.now()
        )
        update(updated)
        Unit
    }

    override suspend fun getBacklinks(noteId: Ulid): AltairResult<List<NoteLink>> = dbOperation {
        // TODO: Implement when NoteLink queries are defined
        emptyList()
    }

    override suspend fun getOutgoingLinks(noteId: Ulid): AltairResult<List<NoteLink>> = dbOperation {
        // TODO: Implement when NoteLink queries are defined
        emptyList()
    }

    override suspend fun parseAndSaveLinks(noteId: Ulid, content: String): AltairResult<Unit> = dbOperation {
        // TODO: Implement wikilink parsing
        // For MVP, this is a stub
        Unit
    }

    override suspend fun getTags(noteId: Ulid): AltairResult<List<Tag>> = dbOperation {
        // TODO: Implement when note-tag junction table is available
        emptyList()
    }

    override suspend fun setTags(noteId: Ulid, tagIds: List<Ulid>): AltairResult<Unit> = dbOperation {
        // TODO: Implement when note-tag junction table is available
        Unit
    }

    override suspend fun getAllTags(userId: Ulid): AltairResult<List<Tag>> = dbOperation {
        // TODO: Implement when Tag queries are defined
        emptyList()
    }

    override suspend fun createTag(tag: Tag): AltairResult<Tag> = dbOperation {
        // TODO: Implement when Tag insert query is defined
        tag
    }

    override suspend fun deleteTag(id: Ulid): AltairResult<Unit> = dbOperation {
        // TODO: Implement when Tag delete query is defined
        Unit
    }

    override suspend fun getAttachments(noteId: Ulid): AltairResult<List<Attachment>> = dbOperation {
        // TODO: Implement when Attachment queries are defined
        emptyList()
    }

    override suspend fun addAttachment(attachment: Attachment): AltairResult<Attachment> = dbOperation {
        // TODO: Implement when Attachment insert query is defined
        attachment
    }

    override suspend fun deleteAttachment(id: Ulid): AltairResult<Unit> = dbOperation {
        // TODO: Implement when Attachment delete query is defined
        Unit
    }

    override suspend fun getFolders(userId: Ulid): AltairResult<List<Folder>> = dbOperation {
        // TODO: Implement when Folder queries are defined
        emptyList()
    }

    override suspend fun createFolder(folder: Folder): AltairResult<Folder> = dbOperation {
        // TODO: Implement when Folder insert query is defined
        folder
    }

    override suspend fun updateFolder(folder: Folder): AltairResult<Folder> = dbOperation {
        // TODO: Implement when Folder update query is defined
        folder
    }

    override suspend fun deleteFolder(id: Ulid): AltairResult<Unit> = dbOperation {
        // TODO: Implement when Folder delete query is defined
        Unit
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> = dbOperation {
        val now = Clock.System.now()
        queries.softDelete(
            deleted_at = now.toLong(),
            updated_at = now.toLong(),
            id = id.value
        )
    }

    override suspend fun restore(id: Ulid): AltairResult<Unit> = dbOperation {
        val now = Clock.System.now()
        queries.restore(
            updated_at = now.toLong(),
            id = id.value
        )
    }

    // Mapper extension function
    private fun com.getaltair.altair.shared.database.Note.toDomain(): Note = Note(
        id = id.toUlid(),
        userId = user_id.toUlid(),
        title = title,
        content = content,
        folderId = folder_id?.toUlid(),
        initiativeId = initiative_id?.toUlid(),
        embedding = embedding?.deserializeFloatList(),
        createdAt = created_at.toInstant(),
        updatedAt = updated_at.toInstant(),
        deletedAt = deleted_at.toInstantOrNull()
    )
}
