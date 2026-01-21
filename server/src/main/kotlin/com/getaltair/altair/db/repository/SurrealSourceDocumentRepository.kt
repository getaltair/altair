@file:Suppress("detekt:MaxLineLength")

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
import kotlinx.serialization.SerializationException
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonArray
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import org.slf4j.LoggerFactory
import kotlin.time.Instant

class SurrealSourceDocumentRepository(
    private val db: SurrealDbClient,
    private val userId: Ulid,
) : SourceDocumentRepository {
    private val json =
        Json {
            ignoreUnknownKeys = true
            isLenient = true
        }

    companion object {
        private val logger = LoggerFactory.getLogger(SurrealSourceDocumentRepository::class.java)
    }

    override suspend fun findById(id: Ulid): Either<DomainError, SourceDocument> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM source_document WHERE id = source_document:\$id AND user_id = user:\$userId AND deleted_at IS NONE",
                        mapOf("id" to id.value, "userId" to userId.value),
                    ).bind()
            parseSourceDocument(result) ?: raise(DomainError.NotFoundError("SourceDocument", id.value))
        }

    override suspend fun save(entity: SourceDocument): Either<DomainError, SourceDocument> =
        either {
            val existing = findById(entity.id)
            if (existing.isRight()) {
                updateSourceDocument(entity).bind()
            } else {
                insertSourceDocument(entity).bind()
            }
            findById(entity.id).bind()
        }

    private suspend fun updateSourceDocument(entity: SourceDocument): Either<DomainError, Unit> =
        db.executeBind(
            """
            UPDATE source_document:${'$'}id SET
                title = ${'$'}title,
                source_type = ${'$'}sourceType,
                source_path = ${'$'}sourcePath,
                mime_type = ${'$'}mimeType,
                file_size_bytes = ${'$'}fileSizeBytes,
                page_count = ${'$'}pageCount,
                extraction_status = ${'$'}extractionStatus,
                extracted_text = ${'$'}extractedText,
                initiative_id = ${'$'}initiativeId,
                updated_at = time::now()
            WHERE user_id = user:${'$'}userId;
            """.trimIndent(),
            buildSourceDocumentParams(entity, isUpdate = true),
        )

    private suspend fun insertSourceDocument(entity: SourceDocument): Either<DomainError, Unit> =
        db.executeBind(
            """
            CREATE source_document:${'$'}id CONTENT {
                user_id: user:${'$'}userId,
                title: ${'$'}title,
                source_type: ${'$'}sourceType,
                source_path: ${'$'}sourcePath,
                mime_type: ${'$'}mimeType,
                file_size_bytes: ${'$'}fileSizeBytes,
                page_count: ${'$'}pageCount,
                extraction_status: ${'$'}extractionStatus,
                extracted_text: NONE,
                watched_folder_id: ${'$'}watchedFolderId,
                initiative_id: ${'$'}initiativeId
            };
            """.trimIndent(),
            buildSourceDocumentParams(entity, isUpdate = false),
        )

    private fun buildSourceDocumentParams(
        entity: SourceDocument,
        isUpdate: Boolean,
    ): Map<String, Any?> =
        buildMap {
            put("id", entity.id.value)
            put("userId", userId.value)
            put("title", entity.title)
            put("sourceType", entity.sourceType.name.lowercase())
            put("sourcePath", entity.sourcePath)
            put("mimeType", entity.mimeType)
            put("fileSizeBytes", entity.fileSizeBytes)
            put("pageCount", entity.pageCount)
            put("extractionStatus", entity.extractionStatus.name.lowercase())
            put("initiativeId", entity.initiativeId?.let { "initiative:${it.value}" })
            if (isUpdate) {
                put("extractedText", entity.extractedText)
            } else {
                put("watchedFolderId", entity.watchedFolderId?.value)
            }
        }

    override suspend fun delete(id: Ulid): Either<DomainError, Unit> =
        either {
            findById(id).bind()
            db
                .executeBind(
                    "UPDATE source_document:\$id SET deleted_at = time::now(), updated_at = time::now() WHERE user_id = user:\$userId;",
                    mapOf("id" to id.value, "userId" to userId.value),
                ).bind()
        }

    override fun findAll(): Flow<List<SourceDocument>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM source_document WHERE user_id = user:\$userId AND deleted_at IS NONE ORDER BY created_at DESC",
                    mapOf("userId" to userId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findAll: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findAll: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findAll: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findAll: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findAll: ERROR_MSG")

                            else -> logger.warn("Database error in findAll: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseSourceDocuments(it) },
                ),
            )
        }

    override fun findByExtractionStatus(status: ExtractionStatus): Flow<List<SourceDocument>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM source_document WHERE user_id = user:\$userId AND extraction_status = \$status AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "status" to status.name.lowercase()),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findByExtractionStatus: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findByExtractionStatus: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findByExtractionStatus: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findByExtractionStatus: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findByExtractionStatus: ERROR_MSG")

                            else -> logger.warn("Database error in findByExtractionStatus: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseSourceDocuments(it) },
                ),
            )
        }

    override fun findBySourceType(sourceType: SourceType): Flow<List<SourceDocument>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM source_document WHERE user_id = user:\$userId AND source_type = \$sourceType AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "sourceType" to sourceType.name.lowercase()),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findBySourceType: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findBySourceType: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findBySourceType: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findBySourceType: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findBySourceType: ERROR_MSG")

                            else -> logger.warn("Database error in findBySourceType: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseSourceDocuments(it) },
                ),
            )
        }

    override fun findByWatchedFolder(watchedFolderId: Ulid): Flow<List<SourceDocument>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM source_document WHERE user_id = user:\$userId AND watched_folder_id = \$watchedFolderId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "watchedFolderId" to watchedFolderId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findByWatchedFolder: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findByWatchedFolder: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findByWatchedFolder: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findByWatchedFolder: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findByWatchedFolder: ERROR_MSG")

                            else -> logger.warn("Database error in findByWatchedFolder: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseSourceDocuments(it) },
                ),
            )
        }

    override fun findByInitiative(initiativeId: Ulid): Flow<List<SourceDocument>> =
        flow {
            val result =
                db.queryBind(
                    "SELECT * FROM source_document WHERE user_id = user:\$userId AND initiative_id = initiative:\$initiativeId AND deleted_at IS NONE",
                    mapOf("userId" to userId.value, "initiativeId" to initiativeId.value),
                )
            emit(
                result.fold(
                    ifLeft = { error ->

                        when (error) {
                            is DomainError.NetworkError -> logger.warn("Database error in findByInitiative: ERROR_MSG")

                            is DomainError.UnexpectedError -> logger.warn("Database error in findByInitiative: ERROR_MSG")

                            is DomainError.NotFoundError -> logger.warn("Database error in findByInitiative: ${error.resource} ${error.id}")

                            is DomainError.ValidationError -> logger.warn("Database error in findByInitiative: ${error.field} - ERROR_MSG")

                            is DomainError.UnauthorizedError -> logger.warn("Database error in findByInitiative: ERROR_MSG")

                            else -> logger.warn("Database error in findByInitiative: $error")
                        }
                        emptyList()
                    },
                    ifRight = { parseSourceDocuments(it) },
                ),
            )
        }

    override suspend fun search(query: String): Either<DomainError, List<SourceDocument>> =
        either {
            val result =
                db
                    .queryBind(
                        """
                        SELECT * FROM source_document WHERE user_id = user:${'$'}userId AND deleted_at IS NONE
                        AND (string::lowercase(title) CONTAINS string::lowercase(${'$'}query)
                             OR string::lowercase(extracted_text) CONTAINS string::lowercase(${'$'}query))
                        """.trimIndent(),
                        mapOf("userId" to userId.value, "query" to query),
                    ).bind()
            parseSourceDocuments(result)
        }

    override suspend fun findPendingExtraction(): Either<DomainError, List<SourceDocument>> =
        either {
            val result =
                db
                    .queryBind(
                        "SELECT * FROM source_document WHERE user_id = user:\$userId AND extraction_status IN ['pending', 'failed'] AND deleted_at IS NONE",
                        mapOf("userId" to userId.value),
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
                .executeBind(
                    """
                    UPDATE source_document:${'$'}id SET
                        extraction_status = ${'$'}status,
                        extracted_text = ${'$'}extractedText,
                        updated_at = time::now()
                    WHERE user_id = user:${'$'}userId;
                    """.trimIndent(),
                    mapOf(
                        "id" to id.value,
                        "status" to status.name.lowercase(),
                        "extractedText" to extractedText,
                        "userId" to userId.value,
                    ),
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
        } catch (e: SerializationException) {
            logger.warn("Failed to parse source document: ${e.message}", e)
            null
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse source document: ${e.message}", e)
            null
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse source document: ${e.message}", e)
            null
        }

    private fun parseSourceDocuments(result: String): List<SourceDocument> =
        try {
            json.parseToJsonElement(result).jsonArray.mapNotNull {
                try {
                    mapToSourceDocument(it.jsonObject)
                } catch (e: SerializationException) {
                    logger.warn("Failed to parse source document element: ${e.message}", e)
                    null
                } catch (e: IllegalStateException) {
                    logger.warn("Failed to parse source document element: ${e.message}", e)
                    null
                } catch (e: IllegalArgumentException) {
                    logger.warn("Failed to parse source document element: ${e.message}", e)
                    null
                }
            }
        } catch (e: SerializationException) {
            logger.warn("Failed to parse source documents array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalStateException) {
            logger.warn("Failed to parse source documents array: ${e.message}", e)
            emptyList()
        } catch (e: IllegalArgumentException) {
            logger.warn("Failed to parse source documents array: ${e.message}", e)
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
