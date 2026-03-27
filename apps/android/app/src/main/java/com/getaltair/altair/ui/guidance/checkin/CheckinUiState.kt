package com.getaltair.altair.ui.guidance.checkin

sealed class CheckinUiState {
    data object Loading : CheckinUiState()
    data class Ready(
        val energyLevel: Int?,
        val mood: String,
        val notes: String,
        val isSaving: Boolean = false,
        val isSaved: Boolean = false,
    ) : CheckinUiState()

    data class Error(val message: String) : CheckinUiState()
}
