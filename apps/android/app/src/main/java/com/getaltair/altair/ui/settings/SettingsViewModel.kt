package com.getaltair.altair.ui.settings

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.preferences.ThemePreferences
import com.getaltair.altair.data.sync.SyncManager
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch

class SettingsViewModel(
    private val syncManager: SyncManager,
    private val themePreferences: ThemePreferences,
) : ViewModel() {

    private val _uiState = MutableStateFlow<SettingsUiState>(SettingsUiState.Loading)
    val uiState: StateFlow<SettingsUiState> = _uiState.asStateFlow()

    init {
        viewModelScope.launch {
            themePreferences.isDarkMode.collect { isDark ->
                _uiState.value = SettingsUiState.Success(
                    // TODO: Replace with real user data from auth when available
                    email = "user@example.com",
                    displayName = "Altair User",
                    isSyncing = syncManager.isSyncing(),
                    isDarkMode = isDark,
                )
            }
        }
    }

    fun toggleDarkMode() {
        val current = _uiState.value
        if (current is SettingsUiState.Success) {
            viewModelScope.launch {
                themePreferences.setDarkMode(!current.isDarkMode)
            }
        }
    }

    fun refreshSyncStatus() {
        val current = _uiState.value
        if (current is SettingsUiState.Success) {
            _uiState.value = current.copy(isSyncing = syncManager.isSyncing())
        }
    }
}
