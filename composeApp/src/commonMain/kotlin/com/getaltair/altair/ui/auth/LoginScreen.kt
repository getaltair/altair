package com.getaltair.altair.ui.auth

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.widthIn
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import com.getaltair.altair.ui.components.AltairButton
import com.getaltair.altair.ui.components.AltairCircularProgressIndicator
import com.getaltair.altair.ui.components.AltairSurface
import com.getaltair.altair.ui.components.AltairText
import com.getaltair.altair.ui.components.AltairTextButton
import com.getaltair.altair.ui.components.AltairTextField
import com.getaltair.altair.ui.theme.AltairTheme

/**
 * Login screen composable.
 *
 * Displays email and password fields with validation feedback,
 * login button, and navigation to registration.
 */
@Composable
fun LoginScreen(
    component: LoginComponent,
    modifier: Modifier = Modifier,
) {
    val state by component.state.collectAsState()

    AltairSurface(modifier = modifier.fillMaxSize()) {
        Column(
            modifier =
                Modifier
                    .fillMaxSize()
                    .padding(AltairTheme.Spacing.lg),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
        ) {
            Column(
                modifier = Modifier.widthIn(max = 400.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                LoginHeader()
                LoginErrorMessage(state.error)
                LoginForm(
                    state = state,
                    onEmailChange = component::onEmailChanged,
                    onPasswordChange = component::onPasswordChanged,
                )
                LoginActions(
                    isLoading = state.isLoading,
                    onLoginClick = component::onLoginClicked,
                    onRegisterClick = component::onRegisterClicked,
                )
            }
        }
    }
}

@Composable
private fun LoginHeader() {
    Column(horizontalAlignment = Alignment.CenterHorizontally) {
        AltairText(
            text = "Welcome to Altair",
            style = AltairTheme.Typography.headlineLarge,
            color = AltairTheme.Colors.textPrimary,
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.sm))
        AltairText(
            text = "Sign in to continue",
            style = AltairTheme.Typography.bodyLarge,
            color = AltairTheme.Colors.textSecondary,
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.xl))
    }
}

@Composable
private fun LoginErrorMessage(error: String?) {
    if (error != null) {
        AltairText(
            text = error,
            style = AltairTheme.Typography.bodyMedium,
            color = AltairTheme.Colors.error,
            modifier = Modifier.padding(bottom = AltairTheme.Spacing.md),
        )
    }
}

@Composable
private fun LoginForm(
    state: LoginState,
    onEmailChange: (String) -> Unit,
    onPasswordChange: (String) -> Unit,
) {
    Column(modifier = Modifier.fillMaxWidth()) {
        AltairTextField(
            value = state.email,
            onValueChange = onEmailChange,
            label = "Email",
            placeholder = "Enter your email",
            isError = state.emailError != null,
            errorMessage = state.emailError,
            enabled = !state.isLoading,
            modifier = Modifier.fillMaxWidth(),
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairTextField(
            value = state.password,
            onValueChange = onPasswordChange,
            label = "Password",
            placeholder = "Enter your password",
            isError = state.passwordError != null,
            errorMessage = state.passwordError,
            visualTransformation = PasswordVisualTransformation(),
            enabled = !state.isLoading,
            modifier = Modifier.fillMaxWidth(),
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.lg))
    }
}

@Composable
private fun LoginActions(
    isLoading: Boolean,
    onLoginClick: () -> Unit,
    onRegisterClick: () -> Unit,
) {
    Column(modifier = Modifier.fillMaxWidth()) {
        AltairButton(
            onClick = onLoginClick,
            enabled = !isLoading,
            modifier = Modifier.fillMaxWidth(),
        ) {
            if (isLoading) {
                AltairCircularProgressIndicator(
                    size = 20.dp,
                    strokeWidth = 2.dp,
                    color = AltairTheme.Colors.textPrimary,
                )
            } else {
                AltairText(
                    text = "Sign In",
                    style = AltairTheme.Typography.labelLarge,
                )
            }
        }
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairTextButton(
            onClick = onRegisterClick,
            enabled = !isLoading,
        ) {
            AltairText(
                text = "Don't have an account? Register",
                style = AltairTheme.Typography.labelLarge,
                color = AltairTheme.Colors.textSecondary,
            )
        }
    }
}
