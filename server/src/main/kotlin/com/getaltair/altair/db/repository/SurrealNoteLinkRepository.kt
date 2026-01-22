@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.NoteError
import com.getaltair.altair.domain.model.knowledge.NoteLink
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.NoteLinkRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory
import kotlin.time.Clock
import kotlin.time.Instant

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
                    ).mapLeft { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error in findById for ${id.value}: $error (converting to NotFound)")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error in findById for ${id.value}: $error (converting to NotFound)")
                            is DomainError.NotFoundError -> logger.warn("Database not found error in findById for ${id.value}: ${error.resource} ${error.id} (converting to NotFound)")
                            is DomainError.ValidationError -> logger.warn("Database validation error in findById for ${id.value}: ${error.field} - $error (converting to NotFound)")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error in findById for ${id.value}: $error (converting to NotFound)")
                            else -> logger.warn("Database error in findById for ${id.value}: $error (converting to NotFound)")
                        }
                        NoteError.NotFound(id)
                    }.bind()
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
                ).mapLeft { error ->
                    when (error) {
                        is DomainError.NetworkError -> logger.warn("Database network error saving note link ${entity.id.value}: $error (converting to NotFound)")
                        is DomainError.UnexpectedError -> logger.warn("Database unexpected error saving note link ${entity.id.value}: $error (converting to NotFound)")
                        is DomainError.NotFoundError -> logger.warn("Database not found error saving note link ${entity.id.value}: ${error.resource} ${error.id} (converting to NotFound)")
                        is DomainError.ValidationError -> logger.warn("Database validation error saving note link ${entity.id.value}: ${error.field} - $error (converting to NotFound)")
                        is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error saving note link ${entity.id.value}: $error (converting to NotFound)")
                        else -> logger.warn("Database error: $error")
                    }
                    NoteError.NotFound(entity.id)
                }.bind()
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<NoteError, Unit> =
        either {
            db
                .executeBind(
                    "DELETE note_link:\$id WHERE user_id = user:\$userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).mapLeft { error ->
                    when (error) {
                        is DomainError.NetworkError -> logger.warn("Database network error in delete for ${id.value}: $error (converting to NotFound)")
                        is DomainError.UnexpectedError -> logger.warn("Database unexpected error in delete for ${id.value}: $error (converting to NotFound)")
                        is DomainError.NotFoundError -> logger.warn("Database not found error in delete for ${id.value}: ${error.resource} ${error.id} (converting to NotFound)")
                        is DomainError.ValidationError -> logger.warn("Database validation error in delete for ${id.value}: ${error.field} - $error (converting to NotFound)")
                        is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error in delete for ${id.value}: $error (converting to NotFound)")
                        else -> logger.warn("Database error: $error")
                    }
                    NoteError.NotFound(id)
                }.bind()
        }

    override fun findAll(): Flow<List<NoteLink>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM note_link WHERE user_id = user:\$userId",
                    mapOf("userId" to userId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error: ${error.message}")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error: ${error.message}")
                            is DomainError.NotFoundError -> logger.warn("Database not found error: ${error.resource} ${error.id}")
                            is DomainError.ValidationError -> logger.warn("Database validation error: ${error.field} - ${error.message}")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error: ${error.message}")
                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseNoteLinks(it) },
                ),
            )
        }

    override fun findBySource(sourceNoteId: Ulid): Flow<List<NoteLink>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM note_link WHERE user_id = user:\$userId AND source_note_id = note:\$sourceNoteId",
                    mapOf("userId" to userId.value, "sourceNoteId" to sourceNoteId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error: ${error.message}")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error: ${error.message}")
                            is DomainError.NotFoundError -> logger.warn("Database not found error: ${error.resource} ${error.id}")
                            is DomainError.ValidationError -> logger.warn("Database validation error: ${error.field} - ${error.message}")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error: ${error.message}")
                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseNoteLinks(it) },
                ),
            )
        }

    override fun findByTarget(targetNoteId: Ulid): Flow<List<NoteLink>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM note_link WHERE user_id = user:\$userId AND target_note_id = note:\$targetNoteId",
                    mapOf("userId" to userId.value, "targetNoteId" to targetNoteId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error: ${error.message}")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error: ${error.message}")
                            is DomainError.NotFoundError -> logger.warn("Database not found error: ${error.resource} ${error.id}")
                            is DomainError.ValidationError -> logger.warn("Database validation error: ${error.field} - ${error.message}")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error: ${error.message}")
                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseNoteLinks(it) },
                ),
            )
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
                    ).mapLeft { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error in findBySourceAndTarget: $error (converting to LinkNotFound)")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error in findBySourceAndTarget: $error (converting to LinkNotFound)")
                            is DomainError.NotFoundError -> logger.warn("Database not found error in findBySourceAndTarget: ${error.resource} ${error.id} (converting to LinkNotFound)")
                            is DomainError.ValidationError -> logger.warn("Database validation error in findBySourceAndTarget: ${error.field} - $error (converting to LinkNotFound)")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error in findBySourceAndTarget: $error (converting to LinkNotFound)")
                            else -> logger.warn("Database error: $error")
                        }
                        NoteError.LinkNotFound(sourceNoteId, targetNoteId)
                    }.bind()
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
                    ).mapLeft { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error updating note link context: $error (converting to LinkNotFound)")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error updating note link context: $error (converting to LinkNotFound)")
                            is DomainError.NotFoundError -> logger.warn("Database not found error updating note link context: ${error.resource} ${error.id} (converting to LinkNotFound)")
                            is DomainError.ValidationError -> logger.warn("Database validation error updating note link context: ${error.field} - $error (converting to LinkNotFound)")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error updating note link context: $error (converting to LinkNotFound)")
                            else -> logger.warn("Database error: $error")
                        }
                        NoteError.LinkNotFound(sourceNoteId, targetNoteId)
                    }.bind()
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
                    ).mapLeft { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error in syncLinksFromContent for ${noteId.value}: $error (converting to NotFound)")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error in syncLinksFromContent for ${noteId.value}: $error (converting to NotFound)")
                            is DomainError.NotFoundError -> logger.warn("Database not found error in syncLinksFromContent for ${noteId.value}: ${error.resource} ${error.id} (converting to NotFound)")
                            is DomainError.ValidationError -> logger.warn("Database validation error in syncLinksFromContent for ${noteId.value}: ${error.field} - $error (converting to NotFound)")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error in syncLinksFromContent for ${noteId.value}: $error (converting to NotFound)")
                            else -> logger.warn("Database error: $error")
                        }
                        NoteError.NotFound(noteId)
                    }.bind()
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
        } catch (e: SerializationException) {
            logger.warn("Failed to parse note link: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse note link: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse note link: ${e.message}", e)
            null
        }

    private fun parseNoteLinks(result: String): List<NoteLink> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToNoteLink(it.jsonObject)
                } catch (e: SerializationException) {
                    logger.warn("Failed to parse note link element: ${e.message}", e)
                    null
                } catch (e: IllegalStateException) {
                    logger.warn("Failed to parse note link element: ${e.message}", e)
                    null
                } catch (e: IllegalArgumentException) {
                    logger.warn("Failed to parse note link element: ${e.message}", e)
                    null
                }
            }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse note links array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse note links array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse note links array: ${e.message}", e)
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
            } catch (e: SerializationException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            } catch (e: IllegalStateException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            } catch (e: IllegalArgumentException) {
                logger.warn("Failed to parse instant '$value': ${e.message}")
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST

    companion object {
        private val logger = LoggerFactory.getLogger(SurrealNoteLinkRepository::class.java)
    }
}
