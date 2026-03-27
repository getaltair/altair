package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.RoutineEntity
import com.getaltair.altair.domain.entity.Frequency
import com.getaltair.altair.domain.entity.Routine
import com.getaltair.altair.domain.entity.RoutineStatus
import java.time.Instant

fun RoutineEntity.toDomain(): Routine = Routine(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    frequency = Frequency.fromString(frequency),
    status = RoutineStatus.fromString(status),
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun Routine.toEntity(): RoutineEntity = RoutineEntity(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    frequency = frequency.value,
    status = status.value,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)
