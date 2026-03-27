package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class KnowledgeNoteSnapshot(
    val id: UUID,
    val noteId: UUID,
    val content: String,
    val createdAt: Instant,
    val createdByProcess: String?,
)
