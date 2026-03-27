package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class Initiative(
    val id: UUID,
    val userId: UUID,
    val householdId: UUID?,
    val name: String,
    val description: String?,
    val status: String,
    val createdAt: Instant,
    val updatedAt: Instant,
)
