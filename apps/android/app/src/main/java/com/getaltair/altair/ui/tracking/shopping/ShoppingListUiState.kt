package com.getaltair.altair.ui.tracking.shopping

import com.getaltair.altair.domain.entity.TrackingShoppingList
import com.getaltair.altair.domain.entity.TrackingShoppingListItem

data class ShoppingListUiState(
    val list: TrackingShoppingList? = null,
    val items: List<TrackingShoppingListItem> = emptyList(),
    val isLoading: Boolean = true,
)
