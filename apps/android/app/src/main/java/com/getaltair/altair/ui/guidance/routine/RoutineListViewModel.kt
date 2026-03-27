package com.getaltair.altair.ui.guidance.routine

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.repository.RoutineRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.launch

class RoutineListViewModel(
    private val routineRepository: RoutineRepository,
) : ViewModel() {

    private val _uiState = MutableStateFlow<RoutineListUiState>(RoutineListUiState.Loading)
    val uiState: StateFlow<RoutineListUiState> = _uiState.asStateFlow()

    init {
        loadRoutines()
    }

    private fun loadRoutines() {
        viewModelScope.launch {
            routineRepository.getAll()
                .catch { e ->
                    _uiState.value =
                        RoutineListUiState.Error(e.message ?: "Unknown error")
                }
                .collect { routines ->
                    _uiState.value = RoutineListUiState.Success(routines)
                }
        }
    }
}
