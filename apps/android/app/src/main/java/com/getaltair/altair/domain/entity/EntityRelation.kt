package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class EntityRelation(
    val id: UUID,
    val fromEntityType: String,
    val fromEntityId: UUID,
    val toEntityType: String,
    val toEntityId: UUID,
    val relationType: String,
    val sourceType: String,
    val status: String,
    val confidence: Double?,
    val createdAt: Instant,
    val updatedAt: Instant,
)
