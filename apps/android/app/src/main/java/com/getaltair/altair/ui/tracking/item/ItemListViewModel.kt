package com.getaltair.altair.ui.tracking.item

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.entity.TrackingItemStatus
import com.getaltair.altair.domain.repository.TrackingItemRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.launch

class ItemListViewModel(
    private val trackingItemRepository: TrackingItemRepository,
) : ViewModel() {

    private val _uiState = MutableStateFlow(ItemListUiState(isLoading = true))
    val uiState: StateFlow<ItemListUiState> = _uiState.asStateFlow()

    init {
        loadItems()
    }

    private fun loadItems() {
        viewModelScope.launch {
            trackingItemRepository.getAll()
                .catch { _uiState.value = _uiState.value.copy(isLoading = false) }
                .collect { items ->
                    _uiState.value = _uiState.value.copy(
                        items = applyFilter(items, _uiState.value.activeFilter),
                        isLoading = false,
                    )
                }
        }
    }

    fun onSearchQueryChange(query: String) {
        _uiState.value = _uiState.value.copy(searchQuery = query, isLoading = true)
        viewModelScope.launch {
            val flow = if (query.isBlank()) {
                trackingItemRepository.getAll()
            } else {
                trackingItemRepository.search(query)
            }
            flow.catch { _uiState.value = _uiState.value.copy(isLoading = false) }
                .collect { items ->
                    _uiState.value = _uiState.value.copy(
                        items = applyFilter(items, _uiState.value.activeFilter),
                        isLoading = false,
                    )
                }
        }
    }

    fun onFilterChange(filter: ItemFilter) {
        _uiState.value = _uiState.value.copy(activeFilter = filter)
        val query = _uiState.value.searchQuery
        viewModelScope.launch {
            val flow = if (query.isBlank()) {
                trackingItemRepository.getAll()
            } else {
                trackingItemRepository.search(query)
            }
            flow.catch { /* keep current state */ }
                .collect { items ->
                    _uiState.value = _uiState.value.copy(
                        items = applyFilter(items, filter),
                    )
                }
        }
    }

    private fun applyFilter(
        items: List<com.getaltair.altair.domain.entity.TrackingItem>,
        filter: ItemFilter,
    ) = when (filter) {
        ItemFilter.ALL -> items
        ItemFilter.ACTIVE -> items.filter { it.status == TrackingItemStatus.ACTIVE }
        ItemFilter.ARCHIVED -> items.filter { it.status == TrackingItemStatus.ARCHIVED }
        ItemFilter.LOW_STOCK -> items.filter {
            it.minQuantity != null && it.quantity <= it.minQuantity
        }
    }
}
