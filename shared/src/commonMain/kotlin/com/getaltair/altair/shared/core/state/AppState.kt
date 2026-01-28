package com.getaltair.altair.shared.core.state

import arrow.optics.optics

/**
 * Root application state with Arrow Optics for nested updates.
 * This demonstrates optics usage - actual state will expand in later phases.
 */
@optics
data class AppState(
    val isLoading: Boolean = false,
    val currentUser: UserState? = null,
    val guidance: GuidanceState = GuidanceState()
) {
    companion object
}

@optics
data class UserState(
    val id: String,
    val displayName: String
) {
    companion object
}

@optics
data class GuidanceState(
    val activeQuestId: String? = null,
    val wipCount: Int = 0
) {
    companion object
}
