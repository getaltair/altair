@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.NoteError
import com.getaltair.altair.domain.model.knowledge.NoteLink
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.NoteLinkRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlin.time.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import kotlin.time.Clock

class SurrealNoteLinkRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : NoteLinkRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<NoteError, NoteLink> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM note_link WHERE id = note_link:\$id AND user_id = user:\$userId",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).mapLeft { NoteError.NotFound(id) }
                    .bind()
            parseNoteLink(result) ?: raise(NoteError.NotFound(id))
        }

    override suspend fun save(entity: NoteLink): Either<NoteError, NoteLink> =
        either {
            db
                .executeBind(
                    """
                    CREATE note_link:${'$'}id CONTENT {
                        user_id: user:${'$'}userId,
                        source_note_id: note:${'$'}sourceNoteId,
                        target_note_id: note:${'$'}targetNoteId,
                        context: ${'$'}context
                    };
                    """.trimIndent(),
                    mapOf(
                        "id" to entity.id.value,
                        "userId" to userId.value,
                        "sourceNoteId" to entity.sourceNoteId.value,
                        "targetNoteId" to entity.targetNoteId.value,
                        "context" to entity.context,
                    ),
                ).mapLeft { NoteError.NotFound(entity.id) }
                .bind()
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<NoteError, Unit> =
        either {
            db
                .executeBind(
                    "DELETE note_link:\$id WHERE user_id = user:\$userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).mapLeft { NoteError.NotFound(id) }
                .bind()
        }

    override fun findAll(): Flow<List<NoteLink>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM note_link WHERE user_id = user:\$userId",
                    mapOf("userId" to userId.value),
                )
            emit(result.fold({ emptyList() }, { parseNoteLinks(it) }))
        }

    override fun findBySource(sourceNoteId: Ulid): Flow<List<NoteLink>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM note_link WHERE user_id = user:\$userId AND source_note_id = note:\$sourceNoteId",
                    mapOf("userId" to userId.value, "sourceNoteId" to sourceNoteId.value),
                )
            emit(result.fold({ emptyList() }, { parseNoteLinks(it) }))
        }

    override fun findByTarget(targetNoteId: Ulid): Flow<List<NoteLink>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM note_link WHERE user_id = user:\$userId AND target_note_id = note:\$targetNoteId",
                    mapOf("userId" to userId.value, "targetNoteId" to targetNoteId.value),
                )
            emit(result.fold({ emptyList() }, { parseNoteLinks(it) }))
        }

    override suspend fun findBySourceAndTarget(
        sourceNoteId: Ulid,
        targetNoteId: Ulid,
    ): Either<NoteError, NoteLink> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM note_link WHERE user_id = user:\$userId AND source_note_id = note:\$sourceNoteId AND target_note_id = note:\$targetNoteId",
                        mapOf("userId" to userId.value, "sourceNoteId" to sourceNoteId.value, "targetNoteId" to targetNoteId.value),
                    ).mapLeft { NoteError.LinkNotFound(sourceNoteId, targetNoteId) }
                    .bind()
            parseNoteLink(result) ?: raise(NoteError.LinkNotFound(sourceNoteId, targetNoteId))
        }

    override suspend fun linkNotes(
        sourceNoteId: Ulid,
        targetNoteId: Ulid,
        context: String?,
    ): Either<NoteError, NoteLink> =
        either {
            val existing = findBySourceAndTarget(sourceNoteId, targetNoteId)
            if (existing.isRight()) {
                val link = existing.getOrNull()!!
                db
                    .executeBind(
                        "UPDATE note_link:\$id SET context = \$context, updated_at = time::now() WHERE user_id = user:\$userId;",
                        mapOf("id" to link.id.value, "context" to context, "userId" to userId.value),
                    ).mapLeft { NoteError.LinkNotFound(sourceNoteId, targetNoteId) }
                    .bind()
                findById(link.id).bind()
            } else {
                val now = Clock.System.now()
                val link =
                    NoteLink(
                        id = Ulid.generate(),
                        userId = userId,
                        sourceNoteId = sourceNoteId,
                        targetNoteId = targetNoteId,
                        context = context,
                        createdAt = now,
                        updatedAt = now,
                    )
                save(link).bind()
            }
        }

    override suspend fun unlinkNotes(
        sourceNoteId: Ulid,
        targetNoteId: Ulid,
    ): Either<NoteError, Unit> =
        either {
            val link = findBySourceAndTarget(sourceNoteId, targetNoteId).bind()
            delete(link.id).bind()
        }

    override suspend fun syncLinksFromContent(noteId: Ulid): Either<NoteError, List<NoteLink>> =
        either {
            // This would parse wiki-links from note content and create/update links
            // For now, return existing links from this note
            val result =
                db
                    .queryBind(
                        "SELECT * FROM note_link WHERE user_id = user:\$userId AND source_note_id = note:\$noteId",
                        mapOf("userId" to userId.value, "noteId" to noteId.value),
                    ).mapLeft { NoteError.NotFound(noteId) }
                    .bind()
            parseNoteLinks(result)
        }

    private fun parseNoteLink(result: String): NoteLink? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToNoteLink(it) }
        } catch (e: Exception) {
            null
        }

    private fun parseNoteLinks(result: String): List<NoteLink> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToNoteLink(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
            emptyList()
        }

    private fun mapToNoteLink(obj: kotlinx.serialization.json.JsonObject): NoteLink {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val sourceId =
            obj["source_note_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val targetId =
            obj["target_note_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return NoteLink(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            sourceNoteId = Ulid(sourceId),
            targetNoteId = Ulid(targetId),
            context = obj["context"]?.jsonPrimitive?.content,
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
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
