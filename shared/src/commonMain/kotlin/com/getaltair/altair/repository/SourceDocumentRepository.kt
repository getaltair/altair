package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.system.SourceDocument
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.ExtractionStatus
import com.getaltair.altair.domain.types.enums.SourceType
import kotlinx.coroutines.flow.Flow

/**
 * Repository for SourceDocument entities.
 *
 * SourceDocuments are imported external documents for reference and annotation.
 * They can be files (PDF, EPUB), web pages, or auto-imported from watched folders.
 */
interface SourceDocumentRepository : Repository<SourceDocument, DomainError> {
    /**
     * Finds source documents by their extraction status.
     *
     * @param status The extraction status to filter by
     * @return A Flow emitting documents with the specified status
     */
    fun findByExtractionStatus(status: ExtractionStatus): Flow<List<SourceDocument>>

    /**
     * Finds source documents by their source type.
     *
     * @param sourceType The source type to filter by
     * @return A Flow emitting documents of the specified type
     */
    fun findBySourceType(sourceType: SourceType): Flow<List<SourceDocument>>

    /**
     * Finds source documents imported from a specific watched folder.
     *
     * @param watchedFolderId The ULID of the watched folder
     * @return A Flow emitting documents from the watched folder
     */
    fun findByWatchedFolder(watchedFolderId: Ulid): Flow<List<SourceDocument>>

    /**
     * Finds source documents associated with a specific initiative.
     *
     * @param initiativeId The ULID of the initiative
     * @return A Flow emitting documents for the initiative
     */
    fun findByInitiative(initiativeId: Ulid): Flow<List<SourceDocument>>

    /**
     * Searches source documents by title and extracted text.
     *
     * @param query The search query
     * @return Either an error on failure, or matching documents
     */
    suspend fun search(query: String): Either<DomainError, List<SourceDocument>>

    /**
     * Finds documents that need extraction (PENDING or FAILED status).
     *
     * @return Either an error on failure, or documents needing extraction
     */
    suspend fun findPendingExtraction(): Either<DomainError, List<SourceDocument>>

    /**
     * Updates the extraction status of a document.
     *
     * @param id The ULID of the document
     * @param status The new extraction status
     * @param extractedText Optional extracted text (for COMPLETED status)
     * @return Either an error on failure, or the updated document
     */
    suspend fun updateExtractionStatus(
        id: Ulid,
        status: ExtractionStatus,
        extractedText: String? = null,
    ): Either<DomainError, SourceDocument>
}
