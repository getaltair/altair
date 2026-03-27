package com.getaltair.altair.ui.guidance.routine

import com.getaltair.altair.domain.entity.Routine

sealed class RoutineListUiState {
    data object Loading : RoutineListUiState()
    data class Success(val routines: List<Routine>) : RoutineListUiState()
    data class Error(val message: String) : RoutineListUiState()
}
