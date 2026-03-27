package com.getaltair.altair.ui.guidance.routine

import androidx.lifecycle.SavedStateHandle
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.domain.repository.RoutineRepository
import com.getaltair.altair.navigation.Screen
import com.getaltair.altair.ui.common.UiState
import java.util.UUID
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.catch
import kotlinx.coroutines.launch

class RoutineDetailViewModel(
    savedStateHandle: SavedStateHandle,
    private val routineRepository: RoutineRepository,
) : ViewModel() {

    private val routineId: UUID = UUID.fromString(
        requireNotNull(savedStateHandle.get<String>(Screen.RoutineDetail.ARG_ID)) {
            "Missing routine ID navigation argument"
        },
    )

    private val _uiState = MutableStateFlow<RoutineDetailUiState>(UiState.Loading)
    val uiState: StateFlow<RoutineDetailUiState> = _uiState.asStateFlow()

    init {
        loadRoutine()
    }

    private fun loadRoutine() {
        viewModelScope.launch {
            routineRepository.getById(routineId)
                .catch { e ->
                    _uiState.value =
                        UiState.Error(e.message ?: "Unknown error")
                }
                .collect { routine ->
                    _uiState.value = if (routine != null) {
                        UiState.Success(routine)
                    } else {
                        UiState.Error("Routine not found")
                    }
                }
        }
    }
}
