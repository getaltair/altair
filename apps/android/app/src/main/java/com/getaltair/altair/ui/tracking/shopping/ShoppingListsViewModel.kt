package com.getaltair.altair.ui.tracking.shopping

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.entity.ShoppingListStatus
import com.getaltair.altair.domain.entity.TrackingShoppingList
import com.getaltair.altair.domain.repository.TrackingShoppingListRepository
import java.time.Instant
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.launch

class ShoppingListsViewModel(
    private val shoppingListRepository: TrackingShoppingListRepository,
) : ViewModel() {

    private val _uiState = MutableStateFlow(ShoppingListsUiState(isLoading = true))
    val uiState: StateFlow<ShoppingListsUiState> = _uiState.asStateFlow()

    init {
        loadLists()
    }

    private fun loadLists() {
        viewModelScope.launch {
            shoppingListRepository.getAll()
                .catch { _uiState.value = _uiState.value.copy(isLoading = false) }
                .collect { lists ->
                    _uiState.value = _uiState.value.copy(
                        lists = lists,
                        isLoading = false,
                    )
                }
        }
    }

    fun createList(name: String) {
        viewModelScope.launch {
            val now = Instant.now()
            val list = TrackingShoppingList(
                id = UUID.randomUUID(),
                userId = UUID.randomUUID(), // TODO: get from auth context
                householdId = UUID.randomUUID(), // TODO: get from auth context
                name = name,
                status = ShoppingListStatus.ACTIVE,
                createdAt = now,
                updatedAt = now,
            )
            shoppingListRepository.create(list)
        }
    }
}
