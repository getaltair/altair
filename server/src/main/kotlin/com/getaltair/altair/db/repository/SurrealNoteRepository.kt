@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.NoteError
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.NoteRepository
import com.getaltair.altair.repository.PageRequest
import com.getaltair.altair.repository.PageResult
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlin.time.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory

class SurrealNoteRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : NoteRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<NoteError, Note> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM note WHERE id = note:\$id AND user_id = user:\$userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).mapLeft { NoteError.NotFound(id) }
                    .bind()
            parseNote(result) ?: raise(NoteError.NotFound(id))
        }

    override suspend fun save(entity: Note): Either<NoteError, Note> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .executeBind(
                        """
                        UPDATE note:${'$'}id SET
                            title = ${'$'}title,
                            content = ${'$'}content,
                            folder_id = ${entity.folderId?.let { "folder:${'$'}folderId" } ?: "NONE"},
                            initiative_id = ${entity.initiativeId?.let { "initiative:${'$'}initiativeId" } ?: "NONE"},
                            is_pinned = ${'$'}isPinned,
                            updated_at = time::now()
                        WHERE user_id = user:${'$'}userId
                        """.trimIndent(),
                        buildMap {
                            put("id", entity.id.value)
                            put("title", entity.title)
                            put("content", entity.content)
                            entity.folderId?.let { put("folderId", it.value) }
                            entity.initiativeId?.let { put("initiativeId", it.value) }
                            put("isPinned", entity.isPinned)
                            put("userId", userId.value)
                        },
                    ).mapLeft { NoteError.NotFound(entity.id) }
                    .bind()
            } else {
                db
                    .executeBind(
                        """
                        CREATE note:${'$'}id CONTENT {
                            user_id: user:${'$'}userId,
                            title: ${'$'}title,
                            content: ${'$'}content,
                            folder_id: ${entity.folderId?.let { "folder:${'$'}folderId" } ?: "NONE"},
                            initiative_id: ${entity.initiativeId?.let { "initiative:${'$'}initiativeId" } ?: "NONE"},
                            is_pinned: ${'$'}isPinned
                        }
                        """.trimIndent(),
                        buildMap {
                            put("id", entity.id.value)
                            put("userId", userId.value)
                            put("title", entity.title)
                            put("content", entity.content)
                            entity.folderId?.let { put("folderId", it.value) }
                            entity.initiativeId?.let { put("initiativeId", it.value) }
                            put("isPinned", entity.isPinned)
                        },
                    ).mapLeft { NoteError.NotFound(entity.id) }
                    .bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<NoteError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE note:\$id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:\$userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).mapLeft { NoteError.NotFound(id) }
                .bind()
        }

    override fun findAll(): Flow<List<Note>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM note WHERE user_id = user:\$userId AND deleted_at IS NONE ORDER BY updated_at DESC",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override fun findByFolder(folderId: Ulid?): Flow<List<Note>> =
        flow {
            val result =
                if (folderId != null) {
                    db.queryBind(
                        "SELECT * FROM note WHERE user_id = user:\$userId AND folder_id = folder:\$folderId AND deleted_at IS NONE ORDER BY title",
                        mapOf("userId" to userId.value, "folderId" to folderId.value),
                    )
                } else {
                    db.queryBind(
                        "SELECT * FROM note WHERE user_id = user:\$userId AND folder_id IS NONE AND deleted_at IS NONE ORDER BY title",
                        mapOf("userId" to userId.value),
                    )
                }
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override fun findPinned(): Flow<List<Note>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM note WHERE user_id = user:\$userId AND is_pinned = true AND deleted_at IS NONE",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<Note>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM note WHERE user_id = user:\$userId AND initiative_id = initiative:\$initiativeId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "initiativeId" to initiativeId.value),
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override suspend fun search(query: String): Either<NoteError, List<Note>> =
        either {
            val result =
                db
                    .queryBind(
                        """
                        SELECT * FROM note WHERE user_id = user:${'$'}userId AND deleted_at IS NONE
                        AND (string::lowercase(title) CONTAINS string::lowercase(${'$'}query)
                             OR string::lowercase(content) CONTAINS string::lowercase(${'$'}query))
                        ORDER BY updated_at DESC
                        """.trimIndent(),
                        mapOf("userId" to userId.value, "query" to query),
                    ).mapLeft { NoteError.NotFound(Ulid.generate()) }
                    .bind()
            parseNotes(result)
        }

    override suspend fun searchPaged(
        query: String,
        page: PageRequest,
    ): Either<NoteError, PageResult<Note>> =
        either {
            val notes = search(query).bind()
            val paged = notes.drop(page.offset).take(page.limit)
            PageResult(
                items = paged,
                totalCount = notes.size,
                hasMore = page.offset + page.limit < notes.size,
            )
        }

    override fun findBacklinks(noteId: Ulid): Flow<List<Note>> =
        flow {
            val result =
                db.queryBind(
                    """
                    SELECT note.* FROM note_link
                    INNER JOIN note ON note_link.source_note_id = note.id
                    WHERE note_link.target_note_id = note:${'$'}noteId
                    AND note.user_id = user:${'$'}userId AND note.deleted_at IS NONE
                    """.trimIndent(),
                    mapOf("noteId" to noteId.value, "userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override fun findForwardLinks(noteId: Ulid): Flow<List<Note>> =
        flow {
            val result =
                db.queryBind(
                    """
                    SELECT note.* FROM note_link
                    INNER JOIN note ON note_link.target_note_id = note.id
                    WHERE note_link.source_note_id = note:${'$'}noteId
                    AND note.user_id = user:${'$'}userId AND note.deleted_at IS NONE
                    """.trimIndent(),
                    mapOf("noteId" to noteId.value, "userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override suspend fun togglePinned(id: Ulid): Either<NoteError, Note> =
        either {
            val note = findById(id).bind()
            db
                .executeBind(
                    "UPDATE note:\$id SET is_pinned = \$isPinned, updated_at = time::now() WHERE user_id = user:\$userId;",
                    mapOf("id" to id.value, "isPinned" to !note.isPinned, "userId" to userId.value),
                ).mapLeft { NoteError.NotFound(id) }
                .bind()
            findById(id).bind()
        }

    override suspend fun moveToFolder(
        id: Ulid,
        folderId: Ulid?,
    ): Either<NoteError, Note> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE note:\$id SET folder_id = ${folderId?.let { "folder:\$folderId" } ?: "NONE"}, updated_at = time::now() WHERE user_id = user:\$userId",
                    buildMap {
                        put("id", id.value)
                        folderId?.let { put("folderId", it.value) }
                        put("userId", userId.value)
                    },
                ).mapLeft { NoteError.NotFound(id) }
                .bind()
            findById(id).bind()
        }

    private fun parseNote(result: String): Note? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToNote(it) }
        } catch (e: Exception) {
            logger.warn("Failed to parse note: ${e.message}", e)
            null
        }

    private fun parseNotes(result: String): List<Note> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToNote(it.jsonObject)
                } catch (e: Exception) {
                    logger.warn("Failed to parse note element: ${e.message}", e)
                    null
                }
            }
        } catch (e: Exception) {
            logger.warn("Failed to parse notes array: ${e.message}", e)
            emptyList()
        }

    private fun mapToNote(obj: kotlinx.serialization.json.JsonObject): Note {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return Note(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            title = obj["title"]?.jsonPrimitive?.content ?: "",
            content = obj["content"]?.jsonPrimitive?.content ?: "",
            folderId =
                obj["folder_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            initiativeId =
                obj["initiative_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            isPinned = obj["is_pinned"]?.jsonPrimitive?.content?.toBooleanStrictOrNull() ?: false,
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
            deletedAt = obj["deleted_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
        )
    }

    private fun parseInstant(value: String?): Instant =
        value?.let {
            try {
                Instant.parse(it)
            } catch (e: Exception) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST

    companion object {
        private val logger = LoggerFactory.getLogger(SurrealNoteRepository::class.java)
    }
}
