package com.getaltair.altair.ui.guidance.initiative

import com.getaltair.altair.domain.entity.Epic
import com.getaltair.altair.domain.entity.Initiative

sealed class InitiativeDetailUiState {
    data object Loading : InitiativeDetailUiState()
    data class Success(
        val initiative: Initiative,
        val epics: List<Epic>,
    ) : InitiativeDetailUiState()

    data class Error(val message: String) : InitiativeDetailUiState()
}
