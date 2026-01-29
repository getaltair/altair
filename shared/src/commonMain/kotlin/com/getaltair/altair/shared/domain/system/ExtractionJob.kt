package com.getaltair.altair.shared.domain.system

import com.getaltair.altair.shared.domain.common.JobStatus
import com.getaltair.altair.shared.domain.common.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Represents a background job for extracting knowledge from a SourceDocument.
 *
 * Extraction jobs handle the asynchronous processing pipeline for source documents:
 * 1. Job is queued when a new SourceDocument is registered or becomes STALE
 * 2. Worker picks up the job and transitions status to PROCESSING
 * 3. Content is extracted, hashed, and optionally embedded
 * 4. SourceDocument is updated with extracted content
 * 5. Job completes with COMPLETED or FAILED status
 *
 * The extraction pipeline typically includes:
 * - Content extraction (text from PDF, HTML from web page, etc.)
 * - SHA-256 hashing for change detection
 * - Text cleaning and normalization
 * - Optional embedding generation for semantic search
 * - Metadata extraction (author, date, tags, etc.)
 *
 * Jobs track their lifecycle for monitoring and debugging:
 * - createdAt: When the job was queued
 * - startedAt: When a worker began processing
 * - completedAt: When processing finished (success or failure)
 *
 * Failed jobs record error messages for troubleshooting and can be:
 * - Retried automatically with exponential backoff
 * - Manually retried by the user
 * - Marked as permanently failed after max retries
 *
 * @property id Unique identifier for this extraction job
 * @property sourceDocumentId Reference to the SourceDocument being processed
 * @property status Current job status (QUEUED, PROCESSING, COMPLETED, FAILED)
 * @property errorMessage Error details if extraction failed (null otherwise)
 * @property createdAt Timestamp when this job was queued
 * @property startedAt Timestamp when processing began (null if not yet started)
 * @property completedAt Timestamp when processing finished (null if still running or queued)
 */
@Serializable
data class ExtractionJob(
    val id: Ulid,
    val sourceDocumentId: Ulid,
    val status: JobStatus,
    val errorMessage: String?,
    val createdAt: Instant,
    val startedAt: Instant?,
    val completedAt: Instant?
) {
    /**
     * Indicates whether the extraction successfully completed.
     */
    val isComplete: Boolean get() = status == JobStatus.COMPLETED

    /**
     * Indicates whether the extraction failed with errors.
     */
    val isFailed: Boolean get() = status == JobStatus.FAILED

    /**
     * Indicates whether the extraction is currently being processed.
     */
    val isRunning: Boolean get() = status == JobStatus.PROCESSING
}
