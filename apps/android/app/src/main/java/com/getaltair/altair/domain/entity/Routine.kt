package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class Routine(
    val id: UUID,
    val userId: UUID,
    val householdId: UUID?,
    val name: String,
    val description: String?,
    val frequency: Frequency,
    val status: RoutineStatus,
    val createdAt: Instant,
    val updatedAt: Instant,
)
