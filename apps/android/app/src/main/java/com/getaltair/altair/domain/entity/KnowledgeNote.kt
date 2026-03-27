package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class KnowledgeNote(
    val id: UUID,
    val userId: UUID,
    val householdId: UUID?,
    val initiativeId: UUID?,
    val title: String,
    val content: String?,
    val contentType: ContentType,
    val isPinned: Boolean,
    val createdAt: Instant,
    val updatedAt: Instant,
)
