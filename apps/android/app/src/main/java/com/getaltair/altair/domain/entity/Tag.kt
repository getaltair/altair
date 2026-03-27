package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class Tag(
    val id: UUID,
    val userId: UUID,
    val householdId: UUID?,
    val name: String,
    val color: String?,
    val createdAt: Instant,
)
