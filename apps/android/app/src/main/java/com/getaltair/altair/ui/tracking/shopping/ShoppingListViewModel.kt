package com.getaltair.altair.ui.tracking.shopping

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.entity.ShoppingListStatus
import com.getaltair.altair.domain.repository.TrackingShoppingListRepository
import com.getaltair.altair.navigation.Screen
import java.time.Instant
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.launch

class ShoppingListViewModel(
    savedStateHandle: SavedStateHandle,
    private val shoppingListRepository: TrackingShoppingListRepository,
) : ViewModel() {

    private val listId: UUID = UUID.fromString(
        requireNotNull(savedStateHandle.get<String>(Screen.ShoppingListDetail.ARG_LIST_ID)) {
            "Missing shopping list ID navigation argument"
        },
    )

    private val _uiState = MutableStateFlow(ShoppingListUiState())
    val uiState: StateFlow<ShoppingListUiState> = _uiState.asStateFlow()

    init {
        loadList()
        loadItems()
    }

    private fun loadList() {
        viewModelScope.launch {
            shoppingListRepository.getById(listId)
                .catch { /* keep loading state */ }
                .collect { list ->
                    _uiState.value = _uiState.value.copy(
                        list = list,
                        isLoading = false,
                    )
                }
        }
    }

    private fun loadItems() {
        viewModelScope.launch {
            shoppingListRepository.getItemsByListId(listId)
                .catch { /* keep current state */ }
                .collect { items ->
                    _uiState.value = _uiState.value.copy(
                        items = items.sortedBy { it.isChecked },
                    )
                }
        }
    }

    fun toggleItem(itemId: UUID) {
        viewModelScope.launch {
            shoppingListRepository.toggleItem(itemId)
        }
    }

    fun markComplete() {
        val currentList = _uiState.value.list ?: return
        viewModelScope.launch {
            val updatedList = currentList.copy(
                status = ShoppingListStatus.COMPLETED,
                updatedAt = Instant.now(),
            )
            shoppingListRepository.update(updatedList)
        }
    }
}
