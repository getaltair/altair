package com.getaltair.altair.ui.tracking.item

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.local.dao.EntityRelationDao
import com.getaltair.altair.data.local.dao.TrackingItemEventDao
import com.getaltair.altair.data.local.entity.TrackingItemEventEntity
import com.getaltair.altair.domain.entity.ItemEventType
import com.getaltair.altair.domain.entity.TrackingItemEvent
import com.getaltair.altair.domain.repository.TrackingItemRepository
import com.getaltair.altair.navigation.Screen
import java.time.Instant
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.launch

class ItemDetailViewModel(
    savedStateHandle: SavedStateHandle,
    private val trackingItemRepository: TrackingItemRepository,
    private val trackingItemEventDao: TrackingItemEventDao,
    private val entityRelationDao: EntityRelationDao,
) : ViewModel() {

    private val itemId: UUID = UUID.fromString(
        requireNotNull(savedStateHandle.get<String>(Screen.ItemDetail.ARG_ITEM_ID)) {
            "Missing item ID navigation argument"
        },
    )

    private val _uiState = MutableStateFlow(ItemDetailUiState())
    val uiState: StateFlow<ItemDetailUiState> = _uiState.asStateFlow()

    init {
        loadItem()
        loadEvents()
        loadRelations()
    }

    private fun loadItem() {
        viewModelScope.launch {
            trackingItemRepository.getById(itemId)
                .catch { /* keep loading state */ }
                .collect { item ->
                    _uiState.value = _uiState.value.copy(
                        item = item,
                        isLoading = false,
                    )
                }
        }
    }

    private fun loadEvents() {
        viewModelScope.launch {
            trackingItemEventDao.getByItemId(itemId)
                .catch { /* keep current state */ }
                .collect { entities ->
                    _uiState.value = _uiState.value.copy(
                        events = entities.map { entity ->
                            TrackingItemEvent(
                                id = entity.id,
                                itemId = entity.itemId,
                                userId = entity.userId,
                                eventType = ItemEventType.fromString(entity.eventType),
                                quantityChange = entity.quantityChange,
                                notes = entity.notes,
                                createdAt = Instant.ofEpochMilli(entity.createdAt),
                            )
                        },
                    )
                }
        }
    }

    private fun loadRelations() {
        viewModelScope.launch {
            entityRelationDao.getByEntity("tracking_item", itemId)
                .catch { /* keep current state */ }
                .collect { relations ->
                    _uiState.value = _uiState.value.copy(relations = relations)
                }
        }
    }

    fun adjustQuantity(delta: Int) {
        val currentItem = _uiState.value.item ?: return
        val newQuantity = (currentItem.quantity + delta).coerceAtLeast(0)

        viewModelScope.launch {
            val event = TrackingItemEventEntity(
                id = UUID.randomUUID(),
                itemId = itemId,
                userId = currentItem.userId,
                eventType = ItemEventType.ADJUSTED.value,
                quantityChange = delta,
                notes = null,
                createdAt = System.currentTimeMillis(),
            )
            trackingItemEventDao.insert(event)

            val updatedItem = currentItem.copy(
                quantity = newQuantity,
                updatedAt = Instant.now(),
            )
            trackingItemRepository.update(updatedItem)
        }
    }

    fun onDeleteRequest() {
        _uiState.value = _uiState.value.copy(showDeleteConfirm = true)
    }

    fun onDeleteConfirm() {
        val item = _uiState.value.item ?: return
        viewModelScope.launch {
            trackingItemRepository.delete(item)
        }
        _uiState.value = _uiState.value.copy(showDeleteConfirm = false)
    }

    fun onDeleteDismiss() {
        _uiState.value = _uiState.value.copy(showDeleteConfirm = false)
    }
}
