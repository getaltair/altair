package com.getaltair.altair.ui.tracking.shopping

import com.getaltair.altair.domain.entity.TrackingShoppingList

data class ShoppingListsUiState(
    val lists: List<TrackingShoppingList> = emptyList(),
    val isLoading: Boolean = false,
)
