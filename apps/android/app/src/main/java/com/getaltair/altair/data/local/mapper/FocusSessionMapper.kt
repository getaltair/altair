package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.FocusSessionEntity
import com.getaltair.altair.domain.entity.FocusSession
import java.time.Instant

fun FocusSessionEntity.toDomain(): FocusSession = FocusSession(
    id = id,
    questId = questId,
    userId = userId,
    startedAt = Instant.ofEpochMilli(startedAt),
    endedAt = endedAt?.let { Instant.ofEpochMilli(it) },
    durationMinutes = durationMinutes,
    notes = notes,
    createdAt = Instant.ofEpochMilli(createdAt),
)

fun FocusSession.toEntity(): FocusSessionEntity = FocusSessionEntity(
    id = id,
    questId = questId,
    userId = userId,
    startedAt = startedAt.toEpochMilli(),
    endedAt = endedAt?.toEpochMilli(),
    durationMinutes = durationMinutes,
    notes = notes,
    createdAt = createdAt.toEpochMilli(),
)
