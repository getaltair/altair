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
import androidx.compose.foundation.text.KeyboardActions
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.foundation.verticalScroll
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.material3.TextButton
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.input.ImeAction
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp

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

    Surface(
        modifier = modifier.fillMaxSize(),
        color = MaterialTheme.colorScheme.background,
    ) {
        Column(
            modifier =
                Modifier
                    .fillMaxSize()
                    .verticalScroll(scrollState)
                    .padding(24.dp),
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
        Text(
            text = "Create Account",
            style = MaterialTheme.typography.headlineMedium,
            color = MaterialTheme.colorScheme.onBackground,
        )
        Spacer(modifier = Modifier.height(8.dp))
        Text(
            text = "Join Altair to get started",
            style = MaterialTheme.typography.bodyLarge,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Spacer(modifier = Modifier.height(32.dp))
    }
}

@Composable
private fun RegisterErrorMessage(error: String?) {
    if (error != null) {
        Text(
            text = error,
            style = MaterialTheme.typography.bodyMedium,
            color = MaterialTheme.colorScheme.error,
            modifier = Modifier.padding(bottom = 16.dp),
        )
    }
}

@Composable
private fun RegisterFormFields(
    state: RegisterState,
    component: RegisterComponent,
) {
    Column(modifier = Modifier.fillMaxWidth()) {
        EmailField(state, component::onEmailChanged)
        Spacer(modifier = Modifier.height(16.dp))
        DisplayNameField(state, component::onDisplayNameChanged)
        Spacer(modifier = Modifier.height(16.dp))
        PasswordFields(state, component::onPasswordChanged, component::onConfirmPasswordChanged)
        Spacer(modifier = Modifier.height(16.dp))
        InviteCodeField(state, component::onInviteCodeChanged, component::onRegisterClicked)
        Spacer(modifier = Modifier.height(24.dp))
    }
}

@Composable
private fun EmailField(
    state: RegisterState,
    onEmailChange: (String) -> Unit,
) {
    OutlinedTextField(
        value = state.email,
        onValueChange = onEmailChange,
        label = { Text("Email") },
        singleLine = true,
        isError = state.emailError != null,
        supportingText = state.emailError?.let { { Text(it) } },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Email, imeAction = ImeAction.Next),
        enabled = !state.isLoading,
        modifier = Modifier.fillMaxWidth(),
    )
}

@Composable
private fun DisplayNameField(
    state: RegisterState,
    onDisplayNameChange: (String) -> Unit,
) {
    OutlinedTextField(
        value = state.displayName,
        onValueChange = onDisplayNameChange,
        label = { Text("Display Name") },
        singleLine = true,
        isError = state.displayNameError != null,
        supportingText = state.displayNameError?.let { { Text(it) } },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Text, imeAction = ImeAction.Next),
        enabled = !state.isLoading,
        modifier = Modifier.fillMaxWidth(),
    )
}

@Composable
private fun PasswordFields(
    state: RegisterState,
    onPasswordChange: (String) -> Unit,
    onConfirmPasswordChange: (String) -> Unit,
) {
    Column(modifier = Modifier.fillMaxWidth()) {
        OutlinedTextField(
            value = state.password,
            onValueChange = onPasswordChange,
            label = { Text("Password") },
            singleLine = true,
            isError = state.passwordError != null,
            supportingText = state.passwordError?.let { { Text(it) } },
            visualTransformation = PasswordVisualTransformation(),
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Password, imeAction = ImeAction.Next),
            enabled = !state.isLoading,
            modifier = Modifier.fillMaxWidth(),
        )
        Spacer(modifier = Modifier.height(16.dp))
        OutlinedTextField(
            value = state.confirmPassword,
            onValueChange = onConfirmPasswordChange,
            label = { Text("Confirm Password") },
            singleLine = true,
            isError = state.confirmPasswordError != null,
            supportingText = state.confirmPasswordError?.let { { Text(it) } },
            visualTransformation = PasswordVisualTransformation(),
            keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Password, imeAction = ImeAction.Next),
            enabled = !state.isLoading,
            modifier = Modifier.fillMaxWidth(),
        )
    }
}

@Composable
private fun InviteCodeField(
    state: RegisterState,
    onInviteCodeChange: (String) -> Unit,
    onRegisterClick: () -> Unit,
) {
    OutlinedTextField(
        value = state.inviteCode,
        onValueChange = onInviteCodeChange,
        label = { Text("Invite Code (optional)") },
        singleLine = true,
        isError = state.inviteCodeError != null,
        supportingText =
            state.inviteCodeError?.let { { Text(it) } }
                ?: { Text("Required after first user is created") },
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Text, imeAction = ImeAction.Done),
        keyboardActions = KeyboardActions(onDone = { onRegisterClick() }),
        enabled = !state.isLoading,
        modifier = Modifier.fillMaxWidth(),
    )
}

@Composable
private fun RegisterActions(
    isLoading: Boolean,
    onRegisterClick: () -> Unit,
    onLoginClick: () -> Unit,
) {
    Column(modifier = Modifier.fillMaxWidth()) {
        Button(
            onClick = onRegisterClick,
            enabled = !isLoading,
            modifier = Modifier.fillMaxWidth(),
        ) {
            if (isLoading) {
                CircularProgressIndicator(modifier = Modifier.height(20.dp), strokeWidth = 2.dp)
            } else {
                Text("Create Account")
            }
        }
        Spacer(modifier = Modifier.height(16.dp))
        TextButton(onClick = onLoginClick, enabled = !isLoading) {
            Text("Already have an account? Sign In")
        }
    }
}
