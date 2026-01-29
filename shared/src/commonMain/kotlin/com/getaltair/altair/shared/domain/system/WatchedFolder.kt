package com.getaltair.altair.shared.domain.system

import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.altair.shared.domain.common.WatchedFolderStatus
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * Represents a filesystem folder monitored for automatic document ingestion.
 *
 * WatchedFolders enable automatic knowledge capture workflows:
 * 1. User designates a folder (e.g., ~/Documents/Research)
 * 2. Altair periodically scans for new or modified files
 * 3. Matching files are automatically registered as SourceDocuments
 * 4. Extraction jobs are queued for new documents
 * 5. Changes to existing documents trigger re-extraction
 *
 * Use cases:
 * - Research folder: Auto-ingest PDFs, papers, and notes for a project
 * - Reading inbox: Monitor ~/Downloads for new ebooks or articles
 * - Code documentation: Watch a docs/ folder for Markdown files
 * - Meeting notes: Auto-capture notes from a synced folder
 *
 * The folder watch system uses:
 * - Include patterns (globs) to match desired files
 * - Exclude patterns to filter out unwanted files
 * - Periodic scanning at configurable intervals (minimum 1 minute)
 * - Optional Initiative linking for automatic document categorization
 *
 * @property id Unique identifier for this watched folder
 * @property userId Owner of this watched folder configuration
 * @property path Absolute filesystem path to the watched directory
 * @property includePatterns Glob patterns for files to include
 * @property excludePatterns Glob patterns for files to exclude
 * @property initiativeId Optional Initiative to auto-link ingested documents
 * @property scanInterval Scan frequency in minutes (minimum 1)
 * @property status Current operational status (ACTIVE, PAUSED, ERROR)
 * @property lastScannedAt Timestamp of most recent scan (null if never scanned)
 * @property createdAt Timestamp when this watch was configured
 * @property updatedAt Timestamp of last modification to this configuration
 *
 * @throws IllegalArgumentException if path is blank or scanInterval is less than 1
 */
@Serializable
data class WatchedFolder(
    val id: Ulid,
    val userId: Ulid,
    val path: String,
    val includePatterns: List<String>,  // globs
    val excludePatterns: List<String>,
    val initiativeId: Ulid?,
    val scanInterval: Int,              // minutes
    val status: WatchedFolderStatus,
    val lastScannedAt: Instant?,
    val createdAt: Instant,
    val updatedAt: Instant
) {
    init {
        require(path.isNotBlank()) { "Path is required and cannot be blank" }
        require(scanInterval >= 1) { "Scan interval must be at least 1 minute, got $scanInterval" }
    }

    /**
     * Indicates whether this folder is currently being actively monitored.
     */
    val isActive: Boolean get() = status == WatchedFolderStatus.ACTIVE
}
