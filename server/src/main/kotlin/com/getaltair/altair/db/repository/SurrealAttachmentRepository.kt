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
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

class SurrealAttachmentRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : AttachmentRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<DomainError, Attachment> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM attachment:${id.value} WHERE user_id = user:${userId.value} AND deleted_at IS NONE",
                    ).bind()
            parseAttachment(result) ?: raise(DomainError.NotFoundError("Attachment", id.value))
        }

    override suspend fun save(entity: Attachment): Either<DomainError, Attachment> =
        either {
            db
                .execute(
                    """
                    CREATE attachment:${entity.id.value} CONTENT {
                        user_id: user:${userId.value},
                        note_id: ${entity.noteId?.let { "note:${it.value}" } ?: "NONE"},
                        inbox_item_id: ${entity.inboxItemId?.let { "inbox_item:${it.value}" } ?: "NONE"},
                        filename: '${entity.filename.replace("'", "''")}',
                        mime_type: '${entity.mimeType}',
                        size_bytes: ${entity.sizeBytes},
                        storage_path: '${entity.storagePath.replace("'", "''")}'
                    };
                    """.trimIndent(),
                ).bind()
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE attachment:${id.value} SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).bind()
        }

    override fun findAll(): Flow<List<Attachment>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM attachment WHERE user_id = user:${userId.value} AND deleted_at IS NONE ORDER BY created_at DESC",
                )
            emit(result.fold({ emptyList() }, { parseAttachments(it) }))
        }

    override fun findByNote(noteId: Ulid): Flow<List<Attachment>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM attachment WHERE user_id = user:${userId.value} AND note_id = note:${noteId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseAttachments(it) }))
        }

    override fun findByInboxItem(inboxItemId: Ulid): Flow<List<Attachment>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM attachment WHERE user_id = user:${userId.value} AND inbox_item_id = inbox_item:${inboxItemId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseAttachments(it) }))
        }

    override suspend fun associateWithNote(
        id: Ulid,
        noteId: Ulid,
    ): Either<DomainError, Attachment> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE attachment:${id.value} SET note_id = note:${noteId.value}, updated_at = time::now() WHERE user_id = user:${userId.value};",
                ).bind()
            findById(id).bind()
        }

    override suspend fun getTotalStorageUsed(): Either<DomainError, Long> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT math::sum(size_bytes) AS total FROM attachment WHERE user_id = user:${userId.value} AND deleted_at IS NONE GROUP ALL",
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
            } catch (e: Exception) {
                0L
            }
        }

    override fun findByMimeType(mimeTypePrefix: String): Flow<List<Attachment>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM attachment WHERE user_id = user:${userId.value} AND string::startsWith(mime_type, '$mimeTypePrefix') AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseAttachments(it) }))
        }

    override suspend fun findOrphaned(): Either<DomainError, List<Attachment>> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM attachment WHERE user_id = user:${userId.value} AND note_id IS NONE AND inbox_item_id IS NONE AND deleted_at IS NONE",
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
        } catch (e: Exception) {
            null
        }

    private fun parseAttachments(result: String): List<Attachment> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToAttachment(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
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
            } catch (e: Exception) {
                Instant.DISTANT_PAST
            }
        } ?: Instant.DISTANT_PAST
}
