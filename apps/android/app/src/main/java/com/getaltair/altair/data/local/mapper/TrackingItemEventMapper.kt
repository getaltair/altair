package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.TrackingItemEventEntity
import com.getaltair.altair.domain.entity.ItemEventType
import com.getaltair.altair.domain.entity.TrackingItemEvent
import java.time.Instant

fun TrackingItemEventEntity.toDomain(): TrackingItemEvent = TrackingItemEvent(
    id = id,
    itemId = itemId,
    userId = userId,
    eventType = ItemEventType.fromString(eventType),
    quantityChange = quantityChange,
    notes = notes,
    createdAt = Instant.ofEpochMilli(createdAt),
)

fun TrackingItemEvent.toEntity(): TrackingItemEventEntity = TrackingItemEventEntity(
    id = id,
    itemId = itemId,
    userId = userId,
    eventType = eventType.value,
    quantityChange = quantityChange,
    notes = notes,
    createdAt = createdAt.toEpochMilli(),
)
