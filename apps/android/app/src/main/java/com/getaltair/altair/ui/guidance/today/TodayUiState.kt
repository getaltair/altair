package com.getaltair.altair.ui.guidance.today

import com.getaltair.altair.domain.entity.DailyCheckin
import com.getaltair.altair.domain.entity.Quest
import com.getaltair.altair.domain.entity.Routine

sealed class TodayUiState {
    data object Loading : TodayUiState()
    data class Success(
        val quests: List<Quest>,
        val routines: List<Routine>,
        val checkin: DailyCheckin?,
    ) : TodayUiState()

    data class Error(val message: String) : TodayUiState()
}
