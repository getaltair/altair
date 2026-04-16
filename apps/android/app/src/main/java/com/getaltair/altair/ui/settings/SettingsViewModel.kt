package com.getaltair.altair.ui.settings

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.auth.TokenPreferences
import com.getaltair.altair.domain.repository.AuthRepository
import kotlinx.coroutines.launch

class SettingsViewModel(
    private val authRepository: AuthRepository,
    private val tokenPreferences: TokenPreferences,
) : ViewModel() {
    fun logout() {
        viewModelScope.launch {
            authRepository.logout()
        }
    }
}
