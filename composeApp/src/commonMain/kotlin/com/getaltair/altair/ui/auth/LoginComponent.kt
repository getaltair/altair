package com.getaltair.altair.ui.auth

import arrow.core.Either
import com.arkivanov.decompose.ComponentContext
import com.arkivanov.essenty.lifecycle.coroutines.coroutineScope
import com.getaltair.altair.service.auth.AuthManager
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

/**
 * Component for the login screen using Decompose.
 *
 * Manages login form state, validation, and authentication flow.
 */
class LoginComponent(
    componentContext: ComponentContext,
    private val authManager: AuthManager,
    private val onLoginSuccess: () -> Unit,
    private val onNavigateToRegister: () -> Unit,
) : ComponentContext by componentContext {
    // Scope is automatically cancelled when the component is destroyed
    private val scope = coroutineScope(Dispatchers.Main.immediate + SupervisorJob())

    private val _state = MutableStateFlow(LoginState())
    val state: StateFlow<LoginState> = _state.asStateFlow()

    fun onEmailChanged(email: String) {
        _state.update { it.copy(email = email, emailError = null, error = null) }
    }

    fun onPasswordChanged(password: String) {
        _state.update { it.copy(password = password, passwordError = null, error = null) }
    }

    fun onLoginClicked() {
        val currentState = _state.value

        // Validate inputs
        val emailError = validateEmail(currentState.email)
        val passwordError = validatePassword(currentState.password)

        if (emailError != null || passwordError != null) {
            _state.update {
                it.copy(
                    emailError = emailError,
                    passwordError = passwordError,
                )
            }
            return
        }

        // Perform login
        _state.update { it.copy(isLoading = true, error = null) }

        scope.launch {
            when (val result = authManager.login(currentState.email, currentState.password)) {
                is Either.Left -> {
                    _state.update {
                        it.copy(
                            isLoading = false,
                            error = result.value.toUserMessage(),
                        )
                    }
                }
                is Either.Right -> {
                    _state.update { it.copy(isLoading = false) }
                    onLoginSuccess()
                }
            }
        }
    }

    fun onRegisterClicked() {
        onNavigateToRegister()
    }

    private fun validateEmail(email: String): String? =
        when {
            email.isBlank() -> "Email is required"
            !email.contains("@") -> "Enter a valid email address"
            else -> null
        }

    private fun validatePassword(password: String): String? = if (password.isBlank()) "Password is required" else null
}

/**
 * State for the login screen.
 */
data class LoginState(
    val email: String = "",
    val password: String = "",
    val emailError: String? = null,
    val passwordError: String? = null,
    val isLoading: Boolean = false,
    val error: String? = null,
)
