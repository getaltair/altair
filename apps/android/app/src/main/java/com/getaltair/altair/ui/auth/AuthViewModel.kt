package com.getaltair.altair.ui.auth

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.getaltair.altair.data.auth.TokenPreferences
import com.getaltair.altair.domain.repository.AuthRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

sealed class AuthUiState {
    object Idle : AuthUiState()

    object Loading : AuthUiState()

    object Success : AuthUiState()

    data class Error(
        val message: String,
    ) : AuthUiState()
}

class AuthViewModel(
    private val authRepository: AuthRepository,
    private val tokenPreferences: TokenPreferences,
) : ViewModel() {
    private val _uiState = MutableStateFlow<AuthUiState>(AuthUiState.Idle)
    val uiState: StateFlow<AuthUiState> = _uiState

    val isAuthenticated: MutableStateFlow<Boolean> = MutableStateFlow(tokenPreferences.accessToken != null)

    fun login(
        email: String,
        password: String,
    ) {
        viewModelScope.launch {
            _uiState.value = AuthUiState.Loading
            try {
                authRepository.login(email, password)
                isAuthenticated.value = true
                _uiState.value = AuthUiState.Success
            } catch (e: Exception) {
                _uiState.value = AuthUiState.Error(e.message ?: "Login failed")
            }
        }
    }

    fun register(
        email: String,
        password: String,
        displayName: String,
    ) {
        viewModelScope.launch {
            _uiState.value = AuthUiState.Loading
            try {
                authRepository.register(email, password, displayName)
                isAuthenticated.value = true
                _uiState.value = AuthUiState.Success
            } catch (e: Exception) {
                _uiState.value = AuthUiState.Error(e.message ?: "Registration failed")
            }
        }
    }
}
