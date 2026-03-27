package com.getaltair.altair.ui.guidance.initiative

import com.getaltair.altair.domain.entity.Epic
import com.getaltair.altair.domain.entity.Initiative
import com.getaltair.altair.ui.common.UiState

data class InitiativeDetailData(
    val initiative: Initiative,
    val epics: List<Epic>,
)

typealias InitiativeDetailUiState = UiState<InitiativeDetailData>
