package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.JobStatus
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A background job for extracting content from a SourceDocument.
 *
 * ExtractionJobs track the progress of AI-powered content extraction,
 * including text extraction, embedding generation, and indexing.
 * They are created when a SourceDocument needs processing.
 */
@Serializable
data class ExtractionJob(
    val id: Ulid,
    val userId: Ulid,
    val sourceDocumentId: Ulid,
    val status: JobStatus,
    val progress: Int,
    val errorMessage: String?,
    val startedAt: Instant?,
    val completedAt: Instant?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
) : Timestamped {
    init {
        require(progress in 0..100) { "Progress must be between 0 and 100" }

        // State consistency based on JobStatus
        when (status) {
            JobStatus.QUEUED -> {
                require(startedAt == null) { "Queued jobs must not have startedAt" }
                require(completedAt == null) { "Queued jobs must not have completedAt" }
            }
            JobStatus.PROCESSING -> {
                require(startedAt != null) { "Processing jobs must have startedAt" }
                require(completedAt == null) { "Processing jobs must not have completedAt" }
            }
            JobStatus.COMPLETED -> {
                require(startedAt != null) { "Completed jobs must have startedAt" }
                require(completedAt != null) { "Completed jobs must have completedAt" }
                require(progress == 100) { "Completed jobs must have progress of 100" }
                require(errorMessage == null) { "Completed jobs must not have errorMessage" }
            }
            JobStatus.FAILED -> {
                require(startedAt != null) { "Failed jobs must have startedAt" }
                require(completedAt != null) { "Failed jobs must have completedAt" }
                require(errorMessage != null) { "Failed jobs must have errorMessage" }
            }
        }

        // Timestamp ordering
        if (startedAt != null && completedAt != null) {
            require(startedAt <= completedAt) { "startedAt must be before or equal to completedAt" }
        }
    }
}
