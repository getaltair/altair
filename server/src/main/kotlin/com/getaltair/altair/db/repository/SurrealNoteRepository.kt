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
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

class SurrealNoteRepository(
    private val db: SurrealDbClient,
    private val userId: String,
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
                    .query<Any>(
                        "SELECT * FROM note:${id.value} WHERE user_id = user:$userId AND deleted_at IS NONE",
                    ).mapLeft { NoteError.NotFound(id) }
                    .bind()
            parseNote(result) ?: raise(NoteError.NotFound(id))
        }

    override suspend fun save(entity: Note): Either<NoteError, Note> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .execute(
                        """
                        UPDATE note:${entity.id.value} SET
                            title = '${entity.title.replace("'", "''")}',
                            content = '${entity.content.replace("'", "''")}',
                            folder_id = ${entity.folderId?.let { "folder:${it.value}" } ?: "NONE"},
                            initiative_id = ${entity.initiativeId?.let { "initiative:${it.value}" } ?: "NONE"},
                            is_pinned = ${entity.isPinned},
                            updated_at = time::now()
                        WHERE user_id = user:$userId;
                        """.trimIndent(),
                    ).mapLeft { NoteError.NotFound(entity.id) }
                    .bind()
            } else {
                db
                    .execute(
                        """
                        CREATE note:${entity.id.value} CONTENT {
                            user_id: user:$userId,
                            title: '${entity.title.replace("'", "''")}',
                            content: '${entity.content.replace("'", "''")}',
                            folder_id: ${entity.folderId?.let { "folder:${it.value}" } ?: "NONE"},
                            initiative_id: ${entity.initiativeId?.let { "initiative:${it.value}" } ?: "NONE"},
                            is_pinned: ${entity.isPinned}
                        };
                        """.trimIndent(),
                    ).mapLeft { NoteError.NotFound(entity.id) }
                    .bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<NoteError, Unit> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE note:${id.value} SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:$userId;",
                ).mapLeft { NoteError.NotFound(id) }
                .bind()
        }

    override fun findAll(): Flow<List<Note>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM note WHERE user_id = user:$userId AND deleted_at IS NONE ORDER BY updated_at DESC",
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override fun findByFolder(folderId: Ulid?): Flow<List<Note>> =
        flow {
            val filter = folderId?.let { "folder_id = folder:${it.value}" } ?: "folder_id IS NONE"
            val result =
                db.query<Any>(
                    "SELECT * FROM note WHERE user_id = user:$userId AND $filter AND deleted_at IS NONE ORDER BY title",
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override fun findPinned(): Flow<List<Note>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM note WHERE user_id = user:$userId AND is_pinned = true AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<Note>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM note WHERE user_id = user:$userId AND initiative_id = initiative:${initiativeId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override suspend fun search(query: String): Either<NoteError, List<Note>> =
        either {
            val result =
                db
                    .query<Any>(
                        """
                        SELECT * FROM note WHERE user_id = user:$userId AND deleted_at IS NONE
                        AND (string::lowercase(title) CONTAINS string::lowercase('${query.replace("'", "''")}')
                             OR string::lowercase(content) CONTAINS string::lowercase('${query.replace("'", "''")}'))
                        ORDER BY updated_at DESC
                        """.trimIndent(),
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
                db.query<Any>(
                    """
                    SELECT note.* FROM note_link
                    INNER JOIN note ON note_link.source_note_id = note.id
                    WHERE note_link.target_note_id = note:${noteId.value}
                    AND note.user_id = user:$userId AND note.deleted_at IS NONE
                    """.trimIndent(),
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override fun findForwardLinks(noteId: Ulid): Flow<List<Note>> =
        flow {
            val result =
                db.query<Any>(
                    """
                    SELECT note.* FROM note_link
                    INNER JOIN note ON note_link.target_note_id = note.id
                    WHERE note_link.source_note_id = note:${noteId.value}
                    AND note.user_id = user:$userId AND note.deleted_at IS NONE
                    """.trimIndent(),
                )
            emit(result.fold({ emptyList() }, { parseNotes(it) }))
        }

    override suspend fun togglePinned(id: Ulid): Either<NoteError, Note> =
        either {
            val note = findById(id).bind()
            db
                .execute(
                    "UPDATE note:${id.value} SET is_pinned = ${!note.isPinned}, updated_at = time::now() WHERE user_id = user:$userId;",
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
            val folderRef = folderId?.let { "folder:${it.value}" } ?: "NONE"
            db
                .execute(
                    "UPDATE note:${id.value} SET folder_id = $folderRef, updated_at = time::now() WHERE user_id = user:$userId;",
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
            null
        }

    private fun parseNotes(result: String): List<Note> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToNote(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
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
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
