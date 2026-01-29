package com.getaltair.altair.shared.core.state

import arrow.optics.optics
import com.getaltair.altair.shared.domain.system.User
import com.getaltair.altair.shared.domain.guidance.Quest
import com.getaltair.altair.shared.domain.guidance.EnergyBudget

/**
 * Root application state with Arrow Optics support.
 * Updated for Phase 2 to use actual domain types.
 */
@optics
data class AppState(
    val isLoading: Boolean = false,
    val currentUser: User? = null,
    val guidance: GuidanceState = GuidanceState()
) {
    companion object
}

/**
 * Guidance module state with active quest and energy tracking.
 */
@optics
data class GuidanceState(
    val activeQuest: Quest? = null,
    val wipCount: Int = 0,
    val todayBudget: EnergyBudget? = null
) {
    companion object
}
