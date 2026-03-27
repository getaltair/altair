package com.getaltair.altair.ui.guidance.initiative

import com.getaltair.altair.domain.entity.Initiative

sealed class InitiativeListUiState {
    data object Loading : InitiativeListUiState()
    data class Success(val initiatives: List<Initiative>) : InitiativeListUiState()
    data class Error(val message: String) : InitiativeListUiState()
}
