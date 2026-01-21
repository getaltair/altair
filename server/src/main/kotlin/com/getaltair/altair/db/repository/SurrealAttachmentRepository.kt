@file:Suppress("detekt:MaxLineLength")

package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.knowledge.Attachment
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.repository.AttachmentRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory
import kotlin.time.Instant

class SurrealAttachmentRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : AttachmentRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    companion object {
        private val logger = LoggerFactory.getLogger(SurrealAttachmentRepository::class.java)
    }

    override suspend fun findById(id: Ulid): Either<DomainError, Attachment> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM attachment WHERE id = attachment:\$id AND user_id = user:\$userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()
            parseAttachment(result) ?: raise(DomainError.NotFoundError("Attachment", id.value))
        }

    override suspend fun save(entity: Attachment): Either<DomainError, Attachment> =
        either {
            db
                .executeBind(
                    """
                    CREATE attachment:${'$'}id CONTENT {
                        user_id: user:${'$'}userId,
                        note_id: ${'$'}noteId,
                        inbox_item_id: ${'$'}inboxItemId,
                        filename: ${'$'}filename,
                        mime_type: ${'$'}mimeType,
                        size_bytes: ${'$'}sizeBytes,
                        storage_path: ${'$'}storagePath
                    };
                    """.trimIndent(),
                    mapOf(
                        "id" to entity.id.value,
                        "userId" to userId.value,
                        "noteId" to entity.noteId?.let { "note:${it.value}" },
                        "inboxItemId" to entity.inboxItemId?.let { "inbox_item:${it.value}" },
                        "filename" to entity.filename,
                        "mimeType" to entity.mimeType,
                        "sizeBytes" to entity.sizeBytes,
                        "storagePath" to entity.storagePath,
                    ),
                ).bind()
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE attachment:\$id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:\$userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).bind()
        }

    override fun findAll(): Flow<List<Attachment>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM attachment WHERE user_id = user:\$userId AND deleted_at IS NONE ORDER BY created_at DESC",
                    mapOf("userId" to userId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error: ERROR_MSG")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error: ERROR_MSG")
                            is DomainError.NotFoundError -> logger.warn("Database not found error: ${error.resource} ${error.id}")
                            is DomainError.ValidationError -> logger.warn("Database validation error: ${error.field} - ERROR_MSG")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error: ERROR_MSG")
                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseAttachments(it) },
                ),
            )
        }

    override fun findByNote(noteId: Ulid): Flow<List<Attachment>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM attachment WHERE user_id = user:\$userId AND note_id = note:\$noteId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "noteId" to noteId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error: ERROR_MSG")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error: ERROR_MSG")
                            is DomainError.NotFoundError -> logger.warn("Database not found error: ${error.resource} ${error.id}")
                            is DomainError.ValidationError -> logger.warn("Database validation error: ${error.field} - ERROR_MSG")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error: ERROR_MSG")
                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseAttachments(it) },
                ),
            )
        }

    override fun findByInboxItem(inboxItemId: Ulid): Flow<List<Attachment>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM attachment WHERE user_id = user:\$userId AND inbox_item_id = inbox_item:\$inboxItemId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "inboxItemId" to inboxItemId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error: ERROR_MSG")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error: ERROR_MSG")
                            is DomainError.NotFoundError -> logger.warn("Database not found error: ${error.resource} ${error.id}")
                            is DomainError.ValidationError -> logger.warn("Database validation error: ${error.field} - ERROR_MSG")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error: ERROR_MSG")
                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseAttachments(it) },
                ),
            )
        }

    override suspend fun associateWithNote(
        id: Ulid,
        noteId: Ulid,
    ): Either<DomainError, Attachment> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE attachment:\$id SET note_id = note:\$noteId, updated_at = time::now() WHERE user_id = user:\$userId;",
                    mapOf("id" to id.value, "noteId" to noteId.value, "userId" to userId.value),
                ).bind()
            findById(id).bind()
        }

    override suspend fun getTotalStorageUsed(): Either<DomainError, Long> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT math::sum(size_bytes) AS total FROM attachment WHERE user_id = user:\$userId AND deleted_at IS NONE GROUP ALL",
                        mapOf("userId" to userId.value),
                    ).bind()
            try {
                json
                    .parseToJsonElement(
                        result,
                    ).jsonArray
                    .firstOrNull()
                    ?.jsonObject
                    ?.get("total")
                    ?.jsonPrimitive
                    ?.content
                    ?.toLongOrNull()
                    ?: 0L
            } catch (e: SerializationException) {
                logger.warn("Failed to parse total storage: ${e.message}", e)
                0L
            } catch (e: IllegalStateException) {
                logger.warn("Failed to parse total storage: ${e.message}", e)
                0L
            } catch (e: IllegalArgumentException) {
                logger.warn("Failed to parse total storage: ${e.message}", e)
                0L
            }
        }

    override fun findByMimeType(mimeTypePrefix: String): Flow<List<Attachment>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM attachment WHERE user_id = user:\$userId AND string::startsWith(mime_type, \$mimeTypePrefix) AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "mimeTypePrefix" to mimeTypePrefix),
                )
            emit(
                result.fold(
                    ifLeft = { error ->
                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database network error: ERROR_MSG")
                            is DomainError.UnexpectedError -> logger.warn("Database unexpected error: ERROR_MSG")
                            is DomainError.NotFoundError -> logger.warn("Database not found error: ${error.resource} ${error.id}")
                            is DomainError.ValidationError -> logger.warn("Database validation error: ${error.field} - ERROR_MSG")
                            is DomainError.UnauthorizedError -> logger.warn("Database unauthorized error: ERROR_MSG")
                            else -> logger.warn("Database error: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseAttachments(it) },
                ),
            )
        }

    override suspend fun findOrphaned(): Either<DomainError, List<Attachment>> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM attachment WHERE user_id = user:\$userId AND note_id IS NONE AND inbox_item_id IS NONE AND deleted_at IS NONE",
                        mapOf("userId" to userId.value),
                    ).bind()
            parseAttachments(result)
        }

    private fun parseAttachment(result: String): Attachment? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToAttachment(it) }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse attachment: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse attachment: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse attachment: ${e.message}", e)
            null
        }

    private fun parseAttachments(result: String): List<Attachment> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToAttachment(it.jsonObject)
                } catch (e: SerializationException) {
                    logger.warn("Failed to parse attachment element: ${e.message}", e)
                    null
                } catch (e: IllegalStateException) {
                    logger.warn("Failed to parse attachment element: ${e.message}", e)
                    null
                } catch (e: IllegalArgumentException) {
                    logger.warn("Failed to parse attachment element: ${e.message}", e)
                    null
                }
            }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse attachments array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse attachments array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse attachments array: ${e.message}", e)
            emptyList()
        }

    private fun mapToAttachment(obj: kotlinx.serialization.json.JsonObject): Attachment {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return Attachment(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            noteId =
                obj["note_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            inboxItemId =
                obj["inbox_item_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
            filename = obj["filename"]?.jsonPrimitive?.content ?: "",
            mimeType = obj["mime_type"]?.jsonPrimitive?.content ?: "",
            sizeBytes = obj["size_bytes"]?.jsonPrimitive?.content?.toLongOrNull() ?: 0L,
            storagePath = obj["storage_path"]?.jsonPrimitive?.content ?: "",
            createdAt = parseInstant(obj["created_at"]?.jsonPrimitive?.content),
            updatedAt = parseInstant(obj["updated_at"]?.jsonPrimitive?.content),
            deletedAt = obj["deleted_at"]?.jsonPrimitive?.content?.let { parseInstant(it) },
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
}
