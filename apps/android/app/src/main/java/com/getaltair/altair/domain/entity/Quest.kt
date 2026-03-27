package com.getaltair.altair.domain.entity

import java.time.Instant
import java.time.LocalDate
import java.util.UUID

data class Quest(
    val id: UUID,
    val epicId: UUID?,
    val initiativeId: UUID?,
    val userId: UUID,
    val householdId: UUID?,
    val name: String,
    val description: String?,
    val status: QuestStatus,
    val priority: Priority,
    val dueDate: LocalDate?,
    val estimatedMinutes: Int?,
    val completedAt: Instant?,
    val createdAt: Instant,
    val updatedAt: Instant,
)
