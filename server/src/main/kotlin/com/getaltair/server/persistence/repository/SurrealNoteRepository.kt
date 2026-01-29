package com.getaltair.server.persistence.repository

import arrow.core.flatMap
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.error.AltairError
import com.getaltair.altair.shared.domain.error.AltairResult
import com.getaltair.altair.shared.domain.knowledge.Attachment
import com.getaltair.altair.shared.domain.knowledge.Folder
import com.getaltair.altair.shared.domain.knowledge.Note
import com.getaltair.altair.shared.domain.knowledge.NoteLink
import com.getaltair.altair.shared.domain.knowledge.Tag
import com.getaltair.altair.shared.repository.NoteRepository
import com.getaltair.server.auth.AuthContext
import com.getaltair.server.persistence.SurrealDbClient

/**
 * SurrealDB implementation of [NoteRepository] for the Knowledge module.
 *
 * Provides comprehensive note management including:
 * - CRUD operations with soft-delete support
 * - Full-text and semantic (vector) search
 * - Bidirectional wiki-link management
 * - Tag and folder organization
 * - File attachments
 *
 * All queries are user-scoped via [AuthContext] for multi-tenant isolation.
 */
class SurrealNoteRepository(
    db: SurrealDbClient,
    auth: AuthContext
) : BaseSurrealRepository<Note>(db, auth, "note", Note::class), NoteRepository {

    // ============================================================
    // CORE CRUD
    // ============================================================

    override suspend fun getById(id: Ulid): AltairResult<Note> {
        return findById(id)
    }

    override suspend fun getAllForUser(userId: Ulid): AltairResult<List<Note>> {
        return findAllForUser()
    }

    override suspend fun create(note: Note): AltairResult<Note> {
        val sql = """
            CREATE note:${note.id} CONTENT {
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

        val now = now()
        return db.queryOne(
            sql,
            params(
                "title" to note.title,
                "content" to note.content,
                "folderId" to note.folderId?.toString(),
                "initiativeId" to note.initiativeId?.toString(),
                "embedding" to note.embedding,
                "now" to now.toString()
            ),
            Note::class
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create note").left()
        }
    }

    override suspend fun update(note: Note): AltairResult<Note> {
        val sql = """
            UPDATE note:${note.id}
            SET title = ${'$'}title,
                content = ${'$'}content,
                folder_id = ${'$'}folderId,
                initiative_id = ${'$'}initiativeId,
                updated_at = ${'$'}now
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return db.queryOne(
            sql,
            params(
                "title" to note.title,
                "content" to note.content,
                "folderId" to note.folderId?.toString(),
                "initiativeId" to note.initiativeId?.toString(),
                "now" to now().toString()
            ),
            Note::class
        ).flatMap { result ->
            result?.right() ?: notFoundError(note.id).left()
        }
    }

    override suspend fun softDelete(id: Ulid): AltairResult<Unit> {
        return softDeleteEntity(id)
    }

    override suspend fun restore(id: Ulid): AltairResult<Unit> {
        return restoreEntity(id)
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
            AND note.user_id = ${'$'}userId
            AND note.deleted_at IS NONE
            FETCH note
        """.trimIndent()

        return db.query(
            sql,
            params("tagId" to "tag:$tagId"),
            Note::class
        )
    }

    // ============================================================
    // SEARCH
    // ============================================================

    override suspend fun search(userId: Ulid, query: String): AltairResult<List<Note>> {
        val sql = """
            SELECT * FROM note
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
            AND (title @@ ${'$'}query OR content @@ ${'$'}query)
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(
            sql,
            params("query" to query),
            Note::class
        )
    }

    override suspend fun semanticSearch(
        userId: Ulid,
        embedding: List<Float>,
        limit: Int
    ): AltairResult<List<Note>> {
        val sql = """
            SELECT *, vector::similarity::cosine(embedding, ${'$'}vec) AS score
            FROM note
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE AND embedding IS NOT NONE
            ORDER BY score DESC
            LIMIT ${'$'}limit
        """.trimIndent()

        return db.query(
            sql,
            params("vec" to embedding, "limit" to limit),
            Note::class
        )
    }

    // ============================================================
    // EMBEDDING
    // ============================================================

    override suspend fun updateEmbedding(id: Ulid, embedding: List<Float>): AltairResult<Unit> {
        val sql = """
            UPDATE note:$id
            SET embedding = ${'$'}embedding, updated_at = ${'$'}now
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE
        """.trimIndent()

        return db.query(
            sql,
            params("embedding" to embedding, "now" to now().toString()),
            Note::class
        ).flatMap { results ->
            if (results.isNotEmpty()) {
                Unit.right()
            } else {
                notFoundError(id).left()
            }
        }
    }

    // ============================================================
    // LINKS (bidirectional [[wikilinks]])
    // ============================================================

    override suspend fun getBacklinks(noteId: Ulid): AltairResult<List<NoteLink>> {
        val sql = """
            SELECT * FROM note_link
            WHERE target_id = ${'$'}targetId
        """.trimIndent()

        return db.query(
            sql,
            params("targetId" to "note:$noteId"),
            NoteLink::class
        )
    }

    override suspend fun getOutgoingLinks(noteId: Ulid): AltairResult<List<NoteLink>> {
        val sql = """
            SELECT * FROM note_link
            WHERE source_id = ${'$'}sourceId
        """.trimIndent()

        return db.query(
            sql,
            params("sourceId" to "note:$noteId"),
            NoteLink::class
        )
    }

    override suspend fun parseAndSaveLinks(noteId: Ulid, content: String): AltairResult<Unit> {
        // First, delete existing outgoing links for this note
        val deleteSql = """
            DELETE FROM note_link WHERE source_id = ${'$'}sourceId
        """.trimIndent()

        return db.query(deleteSql, params("sourceId" to "note:$noteId"), NoteLink::class)
            .flatMap {
                // Extract [[wikilinks]] from content
                val wikiLinkPattern = Regex("""\[\[([^\]]+)\]\]""")
                val matches = wikiLinkPattern.findAll(content).toList()
                val now = now()

                // For each match, find target note by title and create NoteLink
                var result: AltairResult<Unit> = Unit.right()

                for (match in matches) {
                    if (result.isLeft()) break

                    val linkText = match.groupValues[1]
                    val context = extractContext(content, match.range)

                    result = result.flatMap {
                        findNoteByTitle(linkText).flatMap { targetNote ->
                            if (targetNote != null) {
                                val linkId = Ulid.generate()
                                val createSql = """
                                    CREATE note_link:$linkId CONTENT {
                                        source_id: ${'$'}sourceId,
                                        target_id: ${'$'}targetId,
                                        context: ${'$'}context,
                                        created_at: ${'$'}now
                                    }
                                """.trimIndent()

                                db.queryOne(
                                    createSql,
                                    params(
                                        "sourceId" to "note:$noteId",
                                        "targetId" to "note:${targetNote.id}",
                                        "context" to context,
                                        "now" to now.toString()
                                    ),
                                    NoteLink::class
                                ).map { Unit }
                            } else {
                                // Target note not found - skip silently
                                Unit.right()
                            }
                        }
                    }
                }

                result
            }
    }

    /**
     * Finds a note by its title for resolving wikilinks.
     */
    private suspend fun findNoteByTitle(title: String): AltairResult<Note?> {
        val sql = """
            SELECT * FROM note
            WHERE user_id = ${'$'}userId AND deleted_at IS NONE AND title = ${'$'}title
            LIMIT 1
        """.trimIndent()

        return db.queryOne(sql, params("title" to title), Note::class)
    }

    /**
     * Extracts surrounding context for a link from the content.
     * Returns approximately 50 characters before and after the link.
     */
    private fun extractContext(content: String, linkRange: IntRange): String {
        val contextRadius = 50
        val start = maxOf(0, linkRange.first - contextRadius)
        val end = minOf(content.length, linkRange.last + contextRadius)

        val prefix = if (start > 0) "..." else ""
        val suffix = if (end < content.length) "..." else ""

        return "$prefix${content.substring(start, end)}$suffix"
    }

    // ============================================================
    // TAGS
    // ============================================================

    override suspend fun getTags(noteId: Ulid): AltairResult<List<Tag>> {
        val sql = """
            SELECT tag.* FROM note_tag
            WHERE note_id = ${'$'}noteId
            FETCH tag
        """.trimIndent()

        return db.query(
            sql,
            params("noteId" to "note:$noteId"),
            Tag::class
        )
    }

    override suspend fun setTags(noteId: Ulid, tagIds: List<Ulid>): AltairResult<Unit> {
        // Delete existing tag associations
        val deleteSql = """
            DELETE FROM note_tag WHERE note_id = ${'$'}noteId
        """.trimIndent()

        return db.query(deleteSql, params("noteId" to "note:$noteId"), Tag::class)
            .flatMap {
                // Create new associations
                var result: AltairResult<Unit> = Unit.right()

                for (tagId in tagIds) {
                    if (result.isLeft()) break

                    val junctionId = Ulid.generate()
                    val createSql = """
                        CREATE note_tag:$junctionId CONTENT {
                            note_id: ${'$'}noteId,
                            tag_id: ${'$'}tagId
                        }
                    """.trimIndent()

                    result = result.flatMap {
                        db.queryOne(
                            createSql,
                            params("noteId" to "note:$noteId", "tagId" to "tag:$tagId"),
                            Tag::class
                        ).map { Unit }
                    }
                }

                result
            }
    }

    override suspend fun getAllTags(userId: Ulid): AltairResult<List<Tag>> {
        val sql = """
            SELECT * FROM tag
            WHERE user_id = ${'$'}userId
            ORDER BY name ASC
        """.trimIndent()

        return db.query(sql, baseParams(), Tag::class)
    }

    override suspend fun createTag(tag: Tag): AltairResult<Tag> {
        val sql = """
            CREATE tag:${tag.id} CONTENT {
                user_id: ${'$'}userId,
                name: ${'$'}name,
                color: ${'$'}color
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            params("name" to tag.name, "color" to tag.color),
            Tag::class
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create tag").left()
        }
    }

    override suspend fun deleteTag(id: Ulid): AltairResult<Unit> {
        // First, remove tag from all notes
        val removeAssociationsSql = """
            DELETE FROM note_tag WHERE tag_id = ${'$'}tagId
        """.trimIndent()

        return db.query(removeAssociationsSql, params("tagId" to "tag:$id"), Tag::class)
            .flatMap {
                // Then delete the tag
                val deleteSql = """
                    DELETE tag:$id WHERE user_id = ${'$'}userId
                """.trimIndent()

                db.query(deleteSql, baseParams(), Tag::class).map { Unit }
            }
    }

    // ============================================================
    // ATTACHMENTS
    // ============================================================

    override suspend fun getAttachments(noteId: Ulid): AltairResult<List<Attachment>> {
        val sql = """
            SELECT * FROM attachment
            WHERE note_id = ${'$'}noteId AND user_id = ${'$'}userId
            ORDER BY created_at DESC
        """.trimIndent()

        return db.query(
            sql,
            params("noteId" to "note:$noteId"),
            Attachment::class
        )
    }

    override suspend fun addAttachment(attachment: Attachment): AltairResult<Attachment> {
        val sql = """
            CREATE attachment:${attachment.id} CONTENT {
                user_id: ${'$'}userId,
                note_id: ${'$'}noteId,
                inbox_id: ${'$'}inboxId,
                filename: ${'$'}filename,
                mime_type: ${'$'}mimeType,
                size_bytes: ${'$'}sizeBytes,
                storage_key: ${'$'}storageKey,
                hash: ${'$'}hash,
                created_at: ${'$'}now
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            params(
                "noteId" to attachment.noteId?.let { "note:$it" },
                "inboxId" to attachment.inboxId?.let { "inbox:$it" },
                "filename" to attachment.filename,
                "mimeType" to attachment.mimeType,
                "sizeBytes" to attachment.sizeBytes,
                "storageKey" to attachment.storageKey,
                "hash" to attachment.hash,
                "now" to now().toString()
            ),
            Attachment::class
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create attachment").left()
        }
    }

    override suspend fun deleteAttachment(id: Ulid): AltairResult<Unit> {
        val sql = """
            DELETE attachment:$id WHERE user_id = ${'$'}userId
        """.trimIndent()

        return db.query(sql, baseParams(), Attachment::class).map { Unit }
    }

    // ============================================================
    // FOLDERS
    // ============================================================

    override suspend fun getFolders(userId: Ulid): AltairResult<List<Folder>> {
        val sql = """
            SELECT * FROM folder
            WHERE user_id = ${'$'}userId
            ORDER BY parent_id, order ASC
        """.trimIndent()

        return db.query(sql, baseParams(), Folder::class)
    }

    override suspend fun createFolder(folder: Folder): AltairResult<Folder> {
        val sql = """
            CREATE folder:${folder.id} CONTENT {
                user_id: ${'$'}userId,
                name: ${'$'}name,
                parent_id: ${'$'}parentId,
                order: ${'$'}order,
                created_at: ${'$'}now
            }
        """.trimIndent()

        return db.queryOne(
            sql,
            params(
                "name" to folder.name,
                "parentId" to folder.parentId?.let { "folder:$it" },
                "order" to folder.order,
                "now" to now().toString()
            ),
            Folder::class
        ).flatMap { result ->
            result?.right() ?: AltairError.StorageError.DatabaseError("Failed to create folder").left()
        }
    }

    override suspend fun updateFolder(folder: Folder): AltairResult<Folder> {
        val sql = """
            UPDATE folder:${folder.id}
            SET name = ${'$'}name,
                parent_id = ${'$'}parentId,
                order = ${'$'}order
            WHERE user_id = ${'$'}userId
        """.trimIndent()

        return db.queryOne(
            sql,
            params(
                "name" to folder.name,
                "parentId" to folder.parentId?.let { "folder:$it" },
                "order" to folder.order
            ),
            Folder::class
        ).flatMap { result ->
            result?.right() ?: AltairError.NotFoundError.FolderNotFound(folder.id.toString()).left()
        }
    }

    override suspend fun deleteFolder(id: Ulid): AltairResult<Unit> {
        // First, get the folder to find its parent
        val getFolderSql = """
            SELECT * FROM folder:$id WHERE user_id = ${'$'}userId
        """.trimIndent()

        return db.queryOne(getFolderSql, baseParams(), Folder::class)
            .flatMap { folder ->
                if (folder == null) {
                    return@flatMap AltairError.NotFoundError.FolderNotFound(id.toString()).left()
                }

                // Move all notes in this folder to the parent folder
                val moveNotesSql = """
                    UPDATE note
                    SET folder_id = ${'$'}parentId, updated_at = ${'$'}now
                    WHERE folder_id = ${'$'}folderId AND user_id = ${'$'}userId AND deleted_at IS NONE
                """.trimIndent()

                db.query(
                    moveNotesSql,
                    params(
                        "parentId" to folder.parentId?.let { "folder:$it" },
                        "folderId" to "folder:$id",
                        "now" to now().toString()
                    ),
                    Note::class
                ).flatMap {
                    // Move child folders to the parent folder
                    val moveChildFoldersSql = """
                        UPDATE folder
                        SET parent_id = ${'$'}parentId
                        WHERE parent_id = ${'$'}folderId AND user_id = ${'$'}userId
                    """.trimIndent()

                    db.query(
                        moveChildFoldersSql,
                        params(
                            "parentId" to folder.parentId?.let { "folder:$it" },
                            "folderId" to "folder:$id"
                        ),
                        Folder::class
                    )
                }.flatMap {
                    // Finally, delete the folder
                    val deleteSql = """
                        DELETE folder:$id WHERE user_id = ${'$'}userId
                    """.trimIndent()

                    db.query(deleteSql, baseParams(), Folder::class).map { Unit }
                }
            }
    }

    // ============================================================
    // ERROR HANDLING
    // ============================================================

    override fun notFoundError(id: Ulid): AltairError {
        return AltairError.NotFoundError.NoteNotFound(id.toString())
    }
}
