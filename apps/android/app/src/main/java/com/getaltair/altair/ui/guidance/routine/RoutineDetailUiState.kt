package com.getaltair.altair.ui.guidance.routine

import com.getaltair.altair.domain.entity.Routine

sealed class RoutineDetailUiState {
    data object Loading : RoutineDetailUiState()
    data class Success(val routine: Routine) : RoutineDetailUiState()
    data class Error(val message: String) : RoutineDetailUiState()
}
