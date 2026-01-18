package com.getaltair.altair.ui.auth

import arrow.core.Either
import com.arkivanov.decompose.ComponentContext
import com.arkivanov.essenty.lifecycle.coroutines.coroutineScope
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.service.auth.AuthManager
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

/**
 * Component for the registration screen using Decompose.
 *
 * Manages registration form state, validation, and account creation flow.
 */
@Suppress("TooManyFunctions") // UI component with form field handlers and validation
class RegisterComponent(
    componentContext: ComponentContext,
    private val authManager: AuthManager,
    private val onRegisterSuccess: () -> Unit,
    private val onNavigateToLogin: () -> Unit,
) : ComponentContext by componentContext {
    // Scope is automatically cancelled when the component is destroyed
    private val scope = coroutineScope(Dispatchers.Main.immediate + SupervisorJob())

    private val _state = MutableStateFlow(RegisterState())
    val state: StateFlow<RegisterState> = _state.asStateFlow()

    fun onEmailChanged(email: String) {
        _state.update { it.copy(email = email, emailError = null, error = null) }
    }

    fun onDisplayNameChanged(displayName: String) {
        _state.update { it.copy(displayName = displayName, displayNameError = null, error = null) }
    }

    fun onPasswordChanged(password: String) {
        _state.update { it.copy(password = password, passwordError = null, error = null) }
    }

    fun onConfirmPasswordChanged(confirmPassword: String) {
        _state.update { it.copy(confirmPassword = confirmPassword, confirmPasswordError = null, error = null) }
    }

    fun onInviteCodeChanged(inviteCode: String) {
        _state.update { it.copy(inviteCode = inviteCode, inviteCodeError = null, error = null) }
    }

    fun onRegisterClicked() {
        val currentState = _state.value

        // Validate inputs
        val emailError = validateEmail(currentState.email)
        val displayNameError = validateDisplayName(currentState.displayName)
        val passwordError = validatePassword(currentState.password)
        val confirmPasswordError = validateConfirmPassword(currentState.password, currentState.confirmPassword)

        val hasValidationErrors =
            listOf(emailError, displayNameError, passwordError, confirmPasswordError).any { it != null }

        if (hasValidationErrors) {
            _state.update {
                it.copy(
                    emailError = emailError,
                    displayNameError = displayNameError,
                    passwordError = passwordError,
                    confirmPasswordError = confirmPasswordError,
                )
            }
            return
        }

        // Perform registration
        _state.update { it.copy(isLoading = true, error = null) }

        scope.launch {
            val result =
                authManager.register(
                    email = currentState.email,
                    password = currentState.password,
                    displayName = currentState.displayName,
                    inviteCode = currentState.inviteCode.takeIf { it.isNotBlank() },
                )

            when (result) {
                is Either.Left -> {
                    val error = result.value
                    _state.update {
                        it.copy(
                            isLoading = false,
                            error = error.toUserMessage(),
                            inviteCodeError =
                                if (error is AuthError.InvalidInviteCode || error is AuthError.InvalidInvite) {
                                    "Invalid or expired invite code"
                                } else {
                                    null
                                },
                        )
                    }
                }
                is Either.Right -> {
                    _state.update { it.copy(isLoading = false) }
                    onRegisterSuccess()
                }
            }
        }
    }

    fun onLoginClicked() {
        onNavigateToLogin()
    }

    private fun validateEmail(email: String): String? =
        when {
            email.isBlank() -> "Email is required"
            !email.contains("@") -> "Enter a valid email address"
            else -> null
        }

    private fun validateDisplayName(displayName: String): String? =
        when {
            displayName.isBlank() -> "Display name is required"
            displayName.length < 2 -> "Display name must be at least 2 characters"
            else -> null
        }

    private fun validatePassword(password: String): String? =
        when {
            password.isBlank() -> "Password is required"
            password.length < MIN_PASSWORD_LENGTH -> "Password must be at least $MIN_PASSWORD_LENGTH characters"
            else -> null
        }

    private fun validateConfirmPassword(
        password: String,
        confirmPassword: String,
    ): String? =
        when {
            confirmPassword.isBlank() -> "Please confirm your password"
            password != confirmPassword -> "Passwords do not match"
            else -> null
        }

    companion object {
        private const val MIN_PASSWORD_LENGTH = 8
    }
}

/**
 * State for the registration screen.
 */
data class RegisterState(
    val email: String = "",
    val displayName: String = "",
    val password: String = "",
    val confirmPassword: String = "",
    val inviteCode: String = "",
    val emailError: String? = null,
    val displayNameError: String? = null,
    val passwordError: String? = null,
    val confirmPasswordError: String? = null,
    val inviteCodeError: String? = null,
    val isLoading: Boolean = false,
    val error: String? = null,
)
