package com.getaltair.altair.ui.tracking.item

import com.getaltair.altair.data.local.entity.EntityRelationEntity
import com.getaltair.altair.domain.entity.TrackingItem
import com.getaltair.altair.domain.entity.TrackingItemEvent

data class ItemDetailUiState(
    val item: TrackingItem? = null,
    val events: List<TrackingItemEvent> = emptyList(),
    val relations: List<EntityRelationEntity> = emptyList(),
    val isLoading: Boolean = true,
    val showDeleteConfirm: Boolean = false,
)
