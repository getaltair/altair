package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.TagEntity
import com.getaltair.altair.domain.entity.Tag
import java.time.Instant

fun TagEntity.toDomain(): Tag = Tag(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    color = color,
    createdAt = Instant.ofEpochMilli(createdAt),
)

fun Tag.toEntity(): TagEntity = TagEntity(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    color = color,
    createdAt = createdAt.toEpochMilli(),
)
