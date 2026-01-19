package com.getaltair.altair.ui.auth

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.widthIn
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
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
 * Registration screen composable.
 *
 * Displays registration form with email, display name, password fields,
 * optional invite code, and navigation to login.
 */
@Composable
fun RegisterScreen(
    component: RegisterComponent,
    modifier: Modifier = Modifier,
) {
    val state by component.state.collectAsState()
    val scrollState = rememberScrollState()

    AltairSurface(modifier = modifier.fillMaxSize()) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .verticalScroll(scrollState)
                .padding(AltairTheme.Spacing.lg),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
        ) {
            Column(
                modifier = Modifier.widthIn(max = 400.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                RegisterHeader()
                RegisterErrorMessage(state.error)
                RegisterFormFields(state, component)
                RegisterActions(
                    isLoading = state.isLoading,
                    onRegisterClick = component::onRegisterClicked,
                    onLoginClick = component::onLoginClicked,
                )
            }
        }
    }
}

@Composable
private fun RegisterHeader() {
    Column(horizontalAlignment = Alignment.CenterHorizontally) {
        AltairText(
            text = "Create Account",
            style = AltairTheme.Typography.headlineLarge,
            color = AltairTheme.Colors.textPrimary,
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.sm))
        AltairText(
            text = "Join Altair to get started",
            style = AltairTheme.Typography.bodyLarge,
            color = AltairTheme.Colors.textSecondary,
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.xl))
    }
}

@Composable
private fun RegisterErrorMessage(error: String?) {
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
private fun RegisterFormFields(
    state: RegisterState,
    component: RegisterComponent,
) {
    Column(modifier = Modifier.fillMaxWidth()) {
        AltairTextField(
            value = state.email,
            onValueChange = component::onEmailChanged,
            label = "Email",
            placeholder = "Enter your email",
            isError = state.emailError != null,
            errorMessage = state.emailError,
            enabled = !state.isLoading,
            modifier = Modifier.fillMaxWidth(),
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairTextField(
            value = state.displayName,
            onValueChange = component::onDisplayNameChanged,
            label = "Display Name",
            placeholder = "Enter your display name",
            isError = state.displayNameError != null,
            errorMessage = state.displayNameError,
            enabled = !state.isLoading,
            modifier = Modifier.fillMaxWidth(),
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairTextField(
            value = state.password,
            onValueChange = component::onPasswordChanged,
            label = "Password",
            placeholder = "Create a password",
            isError = state.passwordError != null,
            errorMessage = state.passwordError,
            visualTransformation = PasswordVisualTransformation(),
            enabled = !state.isLoading,
            modifier = Modifier.fillMaxWidth(),
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairTextField(
            value = state.confirmPassword,
            onValueChange = component::onConfirmPasswordChanged,
            label = "Confirm Password",
            placeholder = "Confirm your password",
            isError = state.confirmPasswordError != null,
            errorMessage = state.confirmPasswordError,
            visualTransformation = PasswordVisualTransformation(),
            enabled = !state.isLoading,
            modifier = Modifier.fillMaxWidth(),
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairTextField(
            value = state.inviteCode,
            onValueChange = component::onInviteCodeChanged,
            label = "Invite Code (optional)",
            placeholder = "Enter invite code if you have one",
            isError = state.inviteCodeError != null,
            errorMessage = state.inviteCodeError ?: "Required after first user is created",
            enabled = !state.isLoading,
            modifier = Modifier.fillMaxWidth(),
        )
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.lg))
    }
}

@Composable
private fun RegisterActions(
    isLoading: Boolean,
    onRegisterClick: () -> Unit,
    onLoginClick: () -> Unit,
) {
    Column(modifier = Modifier.fillMaxWidth()) {
        AltairButton(
            onClick = onRegisterClick,
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
                    text = "Create Account",
                    style = AltairTheme.Typography.labelLarge,
                )
            }
        }
        Spacer(modifier = Modifier.height(AltairTheme.Spacing.md))
        AltairTextButton(
            onClick = onLoginClick,
            enabled = !isLoading,
        ) {
            AltairText(
                text = "Already have an account? Sign In",
                style = AltairTheme.Typography.labelLarge,
                color = AltairTheme.Colors.textSecondary,
            )
        }
    }
}
