package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class Epic(
    val id: UUID,
    val initiativeId: UUID?,
    val userId: UUID,
    val name: String,
    val description: String?,
    val status: EpicStatus,
    val priority: Priority,
    val createdAt: Instant,
    val updatedAt: Instant,
)
