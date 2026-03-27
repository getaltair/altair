package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class TrackingItem(
    val id: UUID,
    val userId: UUID,
    val householdId: UUID,
    val categoryId: UUID?,
    val locationId: UUID?,
    val name: String,
    val description: String?,
    val quantity: Int,
    val unit: String?,
    val minQuantity: Int?,
    val barcode: String?,
    val status: TrackingItemStatus,
    val createdAt: Instant,
    val updatedAt: Instant,
)
