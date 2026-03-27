package com.getaltair.altair.ui.settings

import androidx.lifecycle.ViewModel
import com.getaltair.altair.data.sync.SyncManager
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update

class SettingsViewModel(
    private val syncManager: SyncManager,
) : ViewModel() {

    private val _uiState = MutableStateFlow(
        SettingsUiState(
            email = "user@example.com",
            displayName = "Altair User",
            isSyncing = syncManager.isSyncing(),
        ),
    )
    val uiState: StateFlow<SettingsUiState> = _uiState.asStateFlow()

    fun toggleDarkMode() {
        _uiState.update { it.copy(isDarkMode = !it.isDarkMode) }
    }

    fun refreshSyncStatus() {
        _uiState.update { it.copy(isSyncing = syncManager.isSyncing()) }
    }
}
