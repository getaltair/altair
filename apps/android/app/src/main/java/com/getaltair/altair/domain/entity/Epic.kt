package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class Epic(
    val id: UUID,
    val initiativeId: UUID?,
    val userId: UUID,
    val name: String,
    val description: String?,
    val status: String,
    val priority: String,
    val createdAt: Instant,
    val updatedAt: Instant,
)
