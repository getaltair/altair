package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.EpicEntity
import com.getaltair.altair.domain.entity.Epic
import java.time.Instant

fun EpicEntity.toDomain(): Epic = Epic(
    id = id,
    initiativeId = initiativeId,
    userId = userId,
    name = name,
    description = description,
    status = status,
    priority = priority,
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun Epic.toEntity(): EpicEntity = EpicEntity(
    id = id,
    initiativeId = initiativeId,
    userId = userId,
    name = name,
    description = description,
    status = status,
    priority = priority,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)
