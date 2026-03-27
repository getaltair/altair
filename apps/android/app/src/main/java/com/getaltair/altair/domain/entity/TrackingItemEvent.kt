package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class TrackingItemEvent(
    val id: UUID,
    val itemId: UUID,
    val userId: UUID,
    val eventType: ItemEventType,
    val quantityChange: Int,
    val notes: String?,
    val createdAt: Instant,
)
