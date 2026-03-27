package com.getaltair.altair.ui.guidance.initiative

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.repository.InitiativeRepository
import com.getaltair.altair.ui.common.UiState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.launch

class InitiativeListViewModel(
    private val initiativeRepository: InitiativeRepository,
) : ViewModel() {

    private val _uiState = MutableStateFlow<InitiativeListUiState>(UiState.Loading)
    val uiState: StateFlow<InitiativeListUiState> = _uiState.asStateFlow()

    init {
        loadInitiatives()
    }

    private fun loadInitiatives() {
        viewModelScope.launch {
            initiativeRepository.getAll()
                .catch { e ->
                    _uiState.value =
                        UiState.Error(e.message ?: "Unknown error")
                }
                .collect { initiatives ->
                    _uiState.value = UiState.Success(initiatives)
                }
        }
    }
}
