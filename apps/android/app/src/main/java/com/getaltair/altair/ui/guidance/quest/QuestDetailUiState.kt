package com.getaltair.altair.ui.guidance.quest

import com.getaltair.altair.domain.entity.Quest

sealed class QuestDetailUiState {
    data object Loading : QuestDetailUiState()
    data class Success(val quest: Quest) : QuestDetailUiState()
    data class Error(val message: String) : QuestDetailUiState()
}
