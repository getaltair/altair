package com.getaltair.altair.domain.entity

import java.time.Instant
import java.util.UUID

data class TrackingShoppingListItem(
    val id: UUID,
    val shoppingListId: UUID,
    val itemId: UUID?,
    val name: String,
    val quantity: Int,
    val unit: String?,
    val isChecked: Boolean,
    val createdAt: Instant,
)
