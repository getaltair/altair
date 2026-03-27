package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class AttachmentMetadata(
    val id: UUID,
    val entityType: String,
    val entityId: UUID,
    val filePath: String,
    val mimeType: String,
    val fileSizeBytes: Long,
    val createdAt: Instant,
)
