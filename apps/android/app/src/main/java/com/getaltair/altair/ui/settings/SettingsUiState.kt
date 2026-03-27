package com.getaltair.altair.ui.settings

sealed class SettingsUiState {
    data object Loading : SettingsUiState()
    data class Success(
        val email: String,
        val displayName: String,
        val isSyncing: Boolean,
        val isDarkMode: Boolean,
    ) : SettingsUiState()
    data class Error(val message: String) : SettingsUiState()
}
