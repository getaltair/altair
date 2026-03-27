package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.TrackingCategoryEntity
import com.getaltair.altair.domain.entity.TrackingCategory
import java.time.Instant

fun TrackingCategoryEntity.toDomain(): TrackingCategory = TrackingCategory(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    parentCategoryId = parentCategoryId,
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun TrackingCategory.toEntity(): TrackingCategoryEntity = TrackingCategoryEntity(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    parentCategoryId = parentCategoryId,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)
