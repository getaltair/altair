package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class FocusSession(
    val id: UUID,
    val questId: UUID,
    val userId: UUID,
    val startedAt: Instant,
    val endedAt: Instant?,
    val durationMinutes: Int?,
    val notes: String?,
    val createdAt: Instant,
)
