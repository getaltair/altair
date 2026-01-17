package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.common.SoftDeletable
import com.getaltair.altair.domain.common.Timestamped
import com.getaltair.altair.domain.types.Ulid
import kotlinx.datetime.Instant
import kotlinx.serialization.Serializable

/**
 * A folder being monitored for automatic document import.
 *
 * WatchedFolders enable automatic ingestion of documents from specified
 * directories. New files are detected and created as SourceDocuments.
 * This is a desktop-only feature.
 */
@Serializable
data class WatchedFolder(
    val id: Ulid,
    val userId: Ulid,
    val path: String,
    val name: String,
    val isActive: Boolean,
    val includeSubfolders: Boolean,
    val filePatterns: List<String>,
    val lastScannedAt: Instant?,
    override val createdAt: Instant,
    override val updatedAt: Instant,
    override val deletedAt: Instant? = null,
) : Timestamped, SoftDeletable {
    init {
        require(path.isNotBlank()) { "WatchedFolder path must not be blank" }
        require(name.isNotBlank()) { "WatchedFolder name must not be blank" }
        require(name.length <= 100) { "WatchedFolder name must be at most 100 characters" }
    }
}
