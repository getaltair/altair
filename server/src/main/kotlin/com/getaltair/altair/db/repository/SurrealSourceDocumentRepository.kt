package com.getaltair.altair.db.repository

import arrow.core.Either
import arrow.core.raise.either
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.system.SourceDocument
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.ExtractionStatus
import com.getaltair.altair.domain.types.enums.SourceType
import com.getaltair.altair.repository.SourceDocumentRepository
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.flow
import kotlinx.datetime.Instant
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive

class SurrealSourceDocumentRepository(
    private val db: SurrealDbClient,
    private val userId: String,
) : SourceDocumentRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    override suspend fun findById(id: Ulid): Either<DomainError, SourceDocument> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM source_document:${id.value} WHERE user_id = user:$userId AND deleted_at IS NONE",
                    ).bind()
            parseSourceDocument(result) ?: raise(DomainError.NotFoundError("SourceDocument", id.value))
        }

    override suspend fun save(entity: SourceDocument): Either<DomainError, SourceDocument> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                db
                    .execute(
                        """
                        UPDATE source_document:${entity.id.value} SET
                            title = '${entity.title.replace("'", "''")}',
                            source_type = '${entity.sourceType.name.lowercase()}',
                            source_path = '${entity.sourcePath.replace("'", "''")}',
                            mime_type = ${entity.mimeType?.let { "'$it'" } ?: "NONE"},
                            file_size_bytes = ${entity.fileSizeBytes ?: "NONE"},
                            page_count = ${entity.pageCount ?: "NONE"},
                            extraction_status = '${entity.extractionStatus.name.lowercase()}',
                            extracted_text = ${entity.extractedText?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                            initiative_id = ${entity.initiativeId?.let { "initiative:${it.value}" } ?: "NONE"},
                            updated_at = time::now()
                        WHERE user_id = user:$userId;
                        """.trimIndent(),
                    ).bind()
            } else {
                db
                    .execute(
                        """
                        CREATE source_document:${entity.id.value} CONTENT {
                            user_id: user:$userId,
                            title: '${entity.title.replace("'", "''")}',
                            source_type: '${entity.sourceType.name.lowercase()}',
                            source_path: '${entity.sourcePath.replace("'", "''")}',
                            mime_type: ${entity.mimeType?.let { "'$it'" } ?: "NONE"},
                            file_size_bytes: ${entity.fileSizeBytes ?: "NONE"},
                            page_count: ${entity.pageCount ?: "NONE"},
                            extraction_status: '${entity.extractionStatus.name.lowercase()}',
                            extracted_text: NONE,
                            watched_folder_id: ${entity.watchedFolderId?.let { "'${it.value}'" } ?: "NONE"},
                            initiative_id: ${entity.initiativeId?.let { "initiative:${it.value}" } ?: "NONE"}
                        };
                        """.trimIndent(),
                    ).bind()
            }
            findById(entity.id).bind()
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .execute(
                    "UPDATE source_document:${id.value} SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:$userId;",
                ).bind()
        }

    override fun findAll(): Flow<List<SourceDocument>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM source_document WHERE user_id = user:$userId AND deleted_at IS NONE ORDER BY created_at DESC",
                )
            emit(result.fold({ emptyList() }, { parseSourceDocuments(it) }))
        }

    override fun findByExtractionStatus(status: ExtractionStatus): Flow<List<SourceDocument>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM source_document WHERE user_id = user:$userId AND extraction_status = '${status.name.lowercase()}' AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseSourceDocuments(it) }))
        }

    override fun findBySourceType(sourceType: SourceType): Flow<List<SourceDocument>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM source_document WHERE user_id = user:$userId AND source_type = '${sourceType.name.lowercase()}' AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseSourceDocuments(it) }))
        }

    override fun findByWatchedFolder(watchedFolderId: Ulid): Flow<List<SourceDocument>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM source_document WHERE user_id = user:$userId AND watched_folder_id = '${watchedFolderId.value}' AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseSourceDocuments(it) }))
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<SourceDocument>> =
        flow {
            val result =
                db.query<Any>(
                    "SELECT * FROM source_document WHERE user_id = user:$userId AND initiative_id = initiative:${initiativeId.value} AND deleted_at IS NONE",
                )
            emit(result.fold({ emptyList() }, { parseSourceDocuments(it) }))
        }

    override suspend fun search(query: String): Either<DomainError, List<SourceDocument>> =
        either {
            val result =
                db
                    .query<Any>(
                        """
                        SELECT * FROM source_document WHERE user_id = user:$userId AND deleted_at IS NONE
                        AND (string::lowercase(title) CONTAINS string::lowercase('${query.replace("'", "''")}')
                             OR string::lowercase(extracted_text) CONTAINS string::lowercase('${query.replace(
                            "'",
                            "''",
                        )}'))
                        """.trimIndent(),
                    ).bind()
            parseSourceDocuments(result)
        }

    override suspend fun findPendingExtraction(): Either<DomainError, List<SourceDocument>> =
        either {
            val result =
                db
                    .query<Any>(
                        "SELECT * FROM source_document WHERE user_id = user:$userId AND extraction_status IN ['pending', 'failed'] AND deleted_at IS NONE",
                    ).bind()
            parseSourceDocuments(result)
        }

    override suspend fun updateExtractionStatus(
        id: Ulid,
        status: ExtractionStatus,
        extractedText: String?,
    ): Either<DomainError, SourceDocument> =
        either {
            findById(id).bind()
            db
                .execute(
                    """
                    UPDATE source_document:${id.value} SET
                        extraction_status = '${status.name.lowercase()}',
                        extracted_text = ${extractedText?.let { "'${it.replace("'", "''")}'" } ?: "NONE"},
                        updated_at = time::now()
                    WHERE user_id = user:$userId;
                    """.trimIndent(),
                ).bind()
            findById(id).bind()
        }

    private fun parseSourceDocument(result: String): SourceDocument? =
        try {
            json
                .parseToJsonElement(result)
                .jsonArray
                .firstOrNull()
                ?.jsonObject
                ?.let { mapToSourceDocument(it) }
        } catch (e: Exception) {
            null
        }

    private fun parseSourceDocuments(result: String): List<SourceDocument> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToSourceDocument(it.jsonObject)
                } catch (e: Exception) {
                    null
                }
            }
        } catch (e: Exception) {
            emptyList()
        }

    private fun mapToSourceDocument(obj: kotlinx.serialization.json.JsonObject): SourceDocument {
        val id = obj["id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        val userIdStr = obj["user_id"]?.jsonPrimitive?.content?.substringAfter(":") ?: throw IllegalStateException()
        return SourceDocument(
            id = Ulid(id),
            userId = Ulid(userIdStr),
            title = obj["title"]?.jsonPrimitive?.content ?: "",
            sourceType = SourceType.valueOf(obj["source_type"]?.jsonPrimitive?.content?.uppercase() ?: "FILE"),
            sourcePath = obj["source_path"]?.jsonPrimitive?.content ?: "",
            mimeType = obj["mime_type"]?.jsonPrimitive?.content,
            fileSizeBytes = obj["file_size_bytes"]?.jsonPrimitive?.content?.toLongOrNull(),
            pageCount = obj["page_count"]?.jsonPrimitive?.content?.toIntOrNull(),
            extractionStatus =
                ExtractionStatus.valueOf(
                    obj["extraction_status"]?.jsonPrimitive?.content?.uppercase() ?: "PENDING",
                ),
            extractedText = obj["extracted_text"]?.jsonPrimitive?.content,
            watchedFolderId = obj["watched_folder_id"]?.jsonPrimitive?.content?.let { Ulid(it) },
            initiativeId =
                obj["initiative_id"]
                    ?.jsonPrimitive
                    ?.content
                    ?.substringAfter(":")
                    ?.let { Ulid(it) },
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
