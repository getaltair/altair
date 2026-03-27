package com.getaltair.altair.ui.settings

data class SettingsUiState(
    val email: String = "",
    val displayName: String = "",
    val isSyncing: Boolean = false,
    val isDarkMode: Boolean = false,
)
