package com.getaltair.altair.domain.entity

import java.time.Instant
import java.time.LocalDate
import java.util.UUID

data class DailyCheckin(
    val id: UUID,
    val userId: UUID,
    val date: LocalDate,
    val energyLevel: Int?,
    val mood: String?,
    val notes: String?,
    val createdAt: Instant,
)
