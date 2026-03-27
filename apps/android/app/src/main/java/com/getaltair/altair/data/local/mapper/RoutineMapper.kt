package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.RoutineEntity
import com.getaltair.altair.domain.entity.Routine
import java.time.Instant

fun RoutineEntity.toDomain(): Routine = Routine(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    frequency = frequency,
    status = status,
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun Routine.toEntity(): RoutineEntity = RoutineEntity(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    frequency = frequency,
    status = status,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)
