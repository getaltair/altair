package com.getaltair.altair.data.local.mapper

import com.getaltair.altair.data.local.entity.TrackingShoppingListItemEntity
import com.getaltair.altair.domain.entity.TrackingShoppingListItem
import java.time.Instant

fun TrackingShoppingListItemEntity.toDomain(): TrackingShoppingListItem = TrackingShoppingListItem(
    id = id,
    shoppingListId = shoppingListId,
    itemId = itemId,
    name = name,
    quantity = quantity,
    unit = unit,
    isChecked = isChecked == 1,
    createdAt = Instant.ofEpochMilli(createdAt),
)

fun TrackingShoppingListItem.toEntity(): TrackingShoppingListItemEntity = TrackingShoppingListItemEntity(
    id = id,
    shoppingListId = shoppingListId,
    itemId = itemId,
    name = name,
    quantity = quantity,
    unit = unit,
    isChecked = if (isChecked) 1 else 0,
    createdAt = createdAt.toEpochMilli(),
)
