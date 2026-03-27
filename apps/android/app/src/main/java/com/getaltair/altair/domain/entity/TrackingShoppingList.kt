package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class TrackingShoppingList(
    val id: UUID,
    val userId: UUID,
    val householdId: UUID,
    val name: String,
    val status: ShoppingListStatus,
    val createdAt: Instant,
    val updatedAt: Instant,
)
