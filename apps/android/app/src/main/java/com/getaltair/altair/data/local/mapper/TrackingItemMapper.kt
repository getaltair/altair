package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.TrackingItemEntity
import com.getaltair.altair.domain.entity.TrackingItem
import com.getaltair.altair.domain.entity.TrackingItemStatus
import java.time.Instant

fun TrackingItemEntity.toDomain(): TrackingItem = TrackingItem(
    id = id,
    userId = userId,
    householdId = householdId,
    categoryId = categoryId,
    locationId = locationId,
    name = name,
    description = description,
    quantity = quantity,
    unit = unit,
    minQuantity = minQuantity,
    barcode = barcode,
    status = TrackingItemStatus.fromString(status),
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun TrackingItem.toEntity(): TrackingItemEntity = TrackingItemEntity(
    id = id,
    userId = userId,
    householdId = householdId,
    categoryId = categoryId,
    locationId = locationId,
    name = name,
    description = description,
    quantity = quantity,
    unit = unit,
    minQuantity = minQuantity,
    barcode = barcode,
    status = status.value,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)
