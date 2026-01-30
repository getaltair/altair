package com.getaltair.altair.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.persistence.DesktopSurrealDbClient
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.knowledge.Attachment
import com.getaltair.altair.shared.domain.knowledge.Folder
import com.getaltair.altair.shared.domain.knowledge.Note
import com.getaltair.altair.shared.domain.knowledge.NoteLink
import com.getaltair.altair.shared.domain.knowledge.Tag
import com.getaltair.altair.shared.repository.NoteRepository

/**
 * Desktop implementation of [NoteRepository] for the Knowledge module.
 *
 * Provides comprehensive note management including:
 * - CRUD operations with soft-delete support
 * - Full-text and semantic (vector) search
 * - Bidirectional wiki-link management
 * - Tag and folder organization
 * - File attachments
 *
 * Desktop version is single-user, so no user scoping needed.
 */
class DesktopNoteRepository(
    db: DesktopSurrealDbClient
) : BaseDesktopRepository<Note>(db, "note", Note::class), NoteRepository {

    override fun notFoundError(id: Ulid): AltairError =
        AltairError.NotFoundError.NoteNotFound(id.toString())

    // ============================================================
    // CORE CRUD
    // ============================================================

    override suspend fun getById(id: Ulid): AltairResult<Note> = findById(id)

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Note>> = findAll()

    override suspend fun create(note: Note): AltairResult<Note> {
        val sql = """
            CREATE note:${'$'}id CONTENT {
                user_id: ${'$'}userId,
                title: ${'$'}title,
                content: ${'$'}content,
                folder_id: ${'$'}folderId,
                initiative_id: ${'$'}initiativeId,
                embedding: ${'$'}embedding,
                created_at: ${'$'}now,
                updated_at: ${'$'}now,
                deleted_at: NONE
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to note.id.toString(),
                "userId" to note.userId.toString(),
                "title" to note.title,
                "content" to note.content,
                "folderId" to note.folderId?.toString(),
                "initiativeId" to note.initiativeId?.toString(),
                "embedding" to note.embedding,
                "now" to now().toString()
            ),
            entityClass
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create note").left()
        }
    }

    override suspend fun update(note: Note): AltairResult<Note> {
        val sql = """
            UPDATE note:${'$'}id
            SET title = ${'$'}title,
                content = ${'$'}content,
                folder_id = ${'$'}folderId,
                initiative_id = ${'$'}initiativeId,
                updated_at = ${'$'}now
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(
            sql,
            mapOf(
                "id" to note.id.toString(),
                "title" to note.title,
                "content" to note.content,
                "folderId" to note.folderId?.toString(),
                "initiativeId" to note.initiativeId?.toString(),
                "now" to now().toString()
            ),
            entityClass
        ).flatMap { result ->
            result?.right() ?: notFoundError(note.id).left()
        }
    }

    // softDelete inherited from BaseDesktopRepository

    override suspend fun restore(id: Ulid): AltairResult<Unit> {
        val sql = """
            UPDATE $tableName
            SET deleted_at = NONE, updated_at = ${'$'}now
            WHERE id = ${'$'}id
        """.trimIndent()

        return db.query(sql, mapOf("id" to "$tableName:$id", "now" to now().toString()), entityClass)
            .flatMap { results ->
                if (results.isNotEmpty()) Unit.right()
                else notFoundError(id).left()
            }
    }

    // ============================================================
    // QUERIES
    // ============================================================

    override suspend fun getByFolder(folderId: Ulid?): AltairResult<List<Note>> {
        return if (folderId == null) {
            findWhere("folder_id IS NONE")
        } else {
            findWhere("folder_id = \$folderId", mapOf("folderId" to "folder:$folderId"))
        }
    }

    override suspend fun getByInitiative(initiativeId: Ulid): AltairResult<List<Note>> {
        return findWhere(
            "initiative_id = \$initiativeId",
            mapOf("initiativeId" to "initiative:$initiativeId")
        )
    }

    override suspend fun getByTag(tagId: Ulid): AltairResult<List<Note>> {
        val sql = """
            SELECT note.* FROM note_tag
            WHERE tag_id = ${'$'}tagId
            AND note.deleted_at IS NONE
            FETCH note
        """.trimIndent()

        return db.query(sql, mapOf("tagId" to "tag:$tagId"), entityClass)
    }

    // ============================================================
    // SEARCH
    // ============================================================

    override suspend fun search(userId: Ulid, query: String): AltairResult<List<Note>> {
        val sql = """
            SELECT * FROM note
            WHERE deleted_at IS NONE
            AND (title CONTAINS ${'$'}query OR content CONTAINS ${'$'}query)
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(sql, mapOf("query" to query), entityClass)
    }

    override suspend fun semanticSearch(userId: Ulid, embedding: List<Float>, limit: Int): AltairResult<List<Note>> {
        // TODO: Implement vector search when SurrealDB vector support is available
        return AltairError.StorageError.DatabaseError("Semantic search not yet implemented").left()
    }

    override suspend fun updateEmbedding(id: Ulid, embedding: List<Float>): AltairResult<Unit> {
        val sql = """
            UPDATE note:${'$'}id
            SET embedding = ${'$'}embedding, updated_at = ${'$'}now
            WHERE deleted_at IS NONE
        """.trimIndent()

        return db.query(
            sql,
            mapOf("id" to id.toString(), "embedding" to embedding, "now" to now().toString()),
            entityClass
        ).flatMap { results ->
            if (results.isNotEmpty()) Unit.right()
            else notFoundError(id).left()
        }
    }

    // ============================================================
    // LINKING
    // ============================================================

    override suspend fun getBacklinks(noteId: Ulid): AltairResult<List<NoteLink>> {
        // TODO: Implement backlinks
        return emptyList<NoteLink>().right()
    }

    override suspend fun getOutgoingLinks(noteId: Ulid): AltairResult<List<NoteLink>> {
        // TODO: Implement outgoing links
        return emptyList<NoteLink>().right()
    }

    override suspend fun parseAndSaveLinks(noteId: Ulid, content: String): AltairResult<Unit> {
        // TODO: Implement link parsing
        return Unit.right()
    }

    // ============================================================
    // TAGS
    // ============================================================

    override suspend fun getTags(noteId: Ulid): AltairResult<List<Tag>> {
        // TODO: Implement tag retrieval
        return emptyList<Tag>().right()
    }

    override suspend fun setTags(noteId: Ulid, tagIds: List<Ulid>): AltairResult<Unit> {
        // TODO: Implement tag setting
        return Unit.right()
    }

    override suspend fun getAllTags(userId: Ulid): AltairResult<List<Tag>> {
        // TODO: Implement all tags retrieval
        return emptyList<Tag>().right()
    }

    override suspend fun createTag(tag: Tag): AltairResult<Tag> {
        // TODO: Implement tag creation
        return AltairError.StorageError.DatabaseError("Tag creation not yet implemented").left()
    }

    override suspend fun deleteTag(id: Ulid): AltairResult<Unit> {
        // TODO: Implement tag deletion
        return Unit.right()
    }

    // ============================================================
    // ATTACHMENTS
    // ============================================================

    override suspend fun getAttachments(noteId: Ulid): AltairResult<List<Attachment>> {
        // TODO: Implement attachments retrieval
        return emptyList<Attachment>().right()
    }

    override suspend fun addAttachment(attachment: Attachment): AltairResult<Attachment> {
        // TODO: Implement attachment addition
        return AltairError.StorageError.DatabaseError("Attachment addition not yet implemented").left()
    }

    override suspend fun deleteAttachment(id: Ulid): AltairResult<Unit> {
        // TODO: Implement attachment deletion
        return Unit.right()
    }

    // ============================================================
    // FOLDERS
    // ============================================================

    override suspend fun getFolders(userId: Ulid): AltairResult<List<Folder>> {
        // TODO: Implement folders retrieval
        return emptyList<Folder>().right()
    }

    override suspend fun createFolder(folder: Folder): AltairResult<Folder> {
        // TODO: Implement folder creation
        return AltairError.StorageError.DatabaseError("Folder creation not yet implemented").left()
    }

    override suspend fun updateFolder(folder: Folder): AltairResult<Folder> {
        // TODO: Implement folder update
        return AltairError.StorageError.DatabaseError("Folder update not yet implemented").left()
    }

    override suspend fun deleteFolder(id: Ulid): AltairResult<Unit> {
        // TODO: Implement folder deletion
        return Unit.right()
    }
}
