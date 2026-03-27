package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.InitiativeEntity
import com.getaltair.altair.domain.entity.Initiative
import com.getaltair.altair.domain.entity.InitiativeStatus
import java.time.Instant

fun InitiativeEntity.toDomain(): Initiative = Initiative(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    status = InitiativeStatus.fromString(status),
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun Initiative.toEntity(): InitiativeEntity = InitiativeEntity(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    status = status.value,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)
