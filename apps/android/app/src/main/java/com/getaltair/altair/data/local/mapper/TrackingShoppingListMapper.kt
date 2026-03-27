package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.TrackingShoppingListEntity
import com.getaltair.altair.domain.entity.ShoppingListStatus
import com.getaltair.altair.domain.entity.TrackingShoppingList
import java.time.Instant

fun TrackingShoppingListEntity.toDomain(): TrackingShoppingList = TrackingShoppingList(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    status = ShoppingListStatus.fromString(status),
    createdAt = Instant.ofEpochMilli(createdAt),
    updatedAt = Instant.ofEpochMilli(updatedAt),
)

fun TrackingShoppingList.toEntity(): TrackingShoppingListEntity = TrackingShoppingListEntity(
    id = id,
    userId = userId,
    householdId = householdId,
    name = name,
    status = status.value,
    createdAt = createdAt.toEpochMilli(),
    updatedAt = updatedAt.toEpochMilli(),
)
