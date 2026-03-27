package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.domain.entity.Quest
import java.time.Instant
import java.time.LocalDate

fun QuestEntity.toDomain(): Quest = Quest(
    id = id,
    epicId = epicId,
    initiativeId = initiativeId,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    status = status,
    priority = priority,
    dueDate = dueDate?.let { LocalDate.parse(it) },
    estimatedMinutes = estimatedMinutes,
    completedAt = completedAt?.let { Instant.ofEpochMilli(it) },
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun Quest.toEntity(): QuestEntity = QuestEntity(
    id = id,
    epicId = epicId,
    initiativeId = initiativeId,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    status = status,
    priority = priority,
    dueDate = dueDate?.toString(),
    estimatedMinutes = estimatedMinutes,
    completedAt = completedAt?.toEpochMilli(),
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)
