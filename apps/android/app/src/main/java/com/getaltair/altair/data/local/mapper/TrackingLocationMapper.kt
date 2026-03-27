package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.TrackingLocationEntity
import com.getaltair.altair.domain.entity.TrackingLocation
import java.time.Instant

fun TrackingLocationEntity.toDomain(): TrackingLocation = TrackingLocation(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    parentLocationId = parentLocationId,
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun TrackingLocation.toEntity(): TrackingLocationEntity = TrackingLocationEntity(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    description = description,
    parentLocationId = parentLocationId,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)
