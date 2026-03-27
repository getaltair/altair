package com.getaltair.altair.ui.tracking.item

import com.getaltair.altair.domain.entity.TrackingItem

enum class ItemFilter { ALL, ACTIVE, ARCHIVED, LOW_STOCK }

data class ItemListUiState(
    val items: List<TrackingItem> = emptyList(),
    val searchQuery: String = "",
    val activeFilter: ItemFilter = ItemFilter.ALL,
    val isLoading: Boolean = false,
)
