package com.getaltair.altair.ui.auth

import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.service.auth.AuthManager
import com.getaltair.altair.service.auth.FakePublicAuthService
import com.getaltair.altair.service.auth.FakeSecureTokenStorage
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.runBlocking
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Tests for RegisterComponent.
 *
 * Verifies:
 * - Form validation (email, display name, password, confirm password, invite code)
 * - Registration flow success and error handling
 * - Navigation callbacks
 * - State management
 */
@Suppress("TooManyFunctions")
class RegisterComponentTest {
    private lateinit var tokenStorage: FakeSecureTokenStorage
    private lateinit var authService: FakePublicAuthService
    private lateinit var authManager: AuthManager
    private lateinit var scope: CoroutineScope

    private var registerSuccessCalled = false
    private var navigateToLoginCalled = false

    @BeforeTest
    fun setup() {
        tokenStorage = FakeSecureTokenStorage()
        authService = FakePublicAuthService()
        scope = CoroutineScope(Dispatchers.Default)
        authManager = AuthManager(tokenStorage, authService, scope)
        registerSuccessCalled = false
        navigateToLoginCalled = false
    }

    private suspend fun waitForCondition(
        timeoutMs: Long = 1000,
        condition: () -> Boolean,
    ) {
        val start = System.currentTimeMillis()
        while (!condition() && System.currentTimeMillis() - start < timeoutMs) {
            delay(10)
        }
    }

    private fun createTestComponentContext(): DefaultComponentContext {
        val lifecycle = LifecycleRegistry()
        lifecycle.resume()
        return DefaultComponentContext(lifecycle = lifecycle)
    }

    private fun createComponent(): RegisterComponent =
        RegisterComponent(
            componentContext = createTestComponentContext(),
            authManager = authManager,
            onRegisterSuccess = { registerSuccessCalled = true },
            onNavigateToLogin = { navigateToLoginCalled = true },
        )

    // ===== Form Validation Tests =====

    @Test
    fun `email validation shows error for blank email`() {
        val component = createComponent()

        fillValidForm(component, email = "")
        component.onRegisterClicked()

        assertEquals("Email is required", component.state.value.emailError)
    }

    @Test
    fun `email validation shows error for invalid email format`() {
        val component = createComponent()

        fillValidForm(component, email = "invalid-email")
        component.onRegisterClicked()

        assertEquals("Enter a valid email address", component.state.value.emailError)
    }

    @Test
    fun `display name validation shows error for blank name`() {
        val component = createComponent()

        fillValidForm(component, displayName = "")
        component.onRegisterClicked()

        assertEquals("Display name is required", component.state.value.displayNameError)
    }

    @Test
    fun `display name validation shows error for short name`() {
        val component = createComponent()

        fillValidForm(component, displayName = "A")
        component.onRegisterClicked()

        assertEquals("Display name must be at least 2 characters", component.state.value.displayNameError)
    }

    @Test
    fun `password validation shows error for blank password`() {
        val component = createComponent()

        fillValidForm(component, password = "", confirmPassword = "")
        component.onRegisterClicked()

        assertEquals("Password is required", component.state.value.passwordError)
    }

    @Test
    fun `password validation shows error for short password`() {
        val component = createComponent()

        fillValidForm(component, password = "short", confirmPassword = "short")
        component.onRegisterClicked()

        assertEquals("Password must be at least 8 characters", component.state.value.passwordError)
    }

    @Test
    fun `confirm password validation shows error for blank confirmation`() {
        val component = createComponent()

        fillValidForm(component, confirmPassword = "")
        component.onRegisterClicked()

        assertEquals("Please confirm your password", component.state.value.confirmPasswordError)
    }

    @Test
    fun `confirm password validation shows error for mismatch`() {
        val component = createComponent()

        fillValidForm(component, password = "password123", confirmPassword = "different123")
        component.onRegisterClicked()

        assertEquals("Passwords do not match", component.state.value.confirmPasswordError)
    }

    @Test
    fun `valid input clears previous errors`() {
        val component = createComponent()

        // First trigger validation errors
        component.onRegisterClicked()
        assertNotNull(component.state.value.emailError)

        // Now enter valid value - error should clear
        component.onEmailChanged("test@example.com")
        assertNull(component.state.value.emailError)
    }

    // ===== Registration Flow Tests =====

    @Test
    fun `successful registration calls onRegisterSuccess`() {
        runBlocking {
            val component = createComponent()
            authService.registerResponse = createAuthResponse()

            fillValidForm(component)
            component.onRegisterClicked()

            waitForCondition { !component.state.value.isLoading }

            assertTrue(registerSuccessCalled)
            assertFalse(component.state.value.isLoading)
            assertNull(component.state.value.error)
        }
    }

    @Test
    fun `registration with invite code passes code to auth manager`() {
        runBlocking {
            val component = createComponent()
            authService.registerResponse = createAuthResponse()

            fillValidForm(component, inviteCode = "INVITE123")
            component.onRegisterClicked()

            waitForCondition { !component.state.value.isLoading }

            assertEquals("INVITE123", authService.lastRegisterRequest?.inviteCode)
        }
    }

    @Test
    fun `registration without invite code sends null`() {
        runBlocking {
            val component = createComponent()
            authService.registerResponse = createAuthResponse()

            fillValidForm(component, inviteCode = "")
            component.onRegisterClicked()

            waitForCondition { !component.state.value.isLoading }

            assertNull(authService.lastRegisterRequest?.inviteCode)
        }
    }

    @Test
    fun `failed registration shows error message`() {
        runBlocking {
            val component = createComponent()
            authService.registerError = IllegalArgumentException("Email already registered")

            fillValidForm(component)
            component.onRegisterClicked()

            waitForCondition { !component.state.value.isLoading }

            assertFalse(registerSuccessCalled)
            assertFalse(component.state.value.isLoading)
            assertNotNull(component.state.value.error)
        }
    }

    @Test
    fun `invalid invite code shows specific error`() {
        runBlocking {
            val component = createComponent()
            authService.registerError = IllegalArgumentException("Invalid invite code")

            fillValidForm(component, inviteCode = "INVALID")
            component.onRegisterClicked()

            waitForCondition { !component.state.value.isLoading }

            assertEquals("Invalid or expired invite code", component.state.value.inviteCodeError)
        }
    }

    @Test
    fun `registration shows loading state while registering`() {
        val component = createComponent()
        authService.registerResponse = createAuthResponse()

        fillValidForm(component)
        component.onRegisterClicked()

        // Check loading state before coroutine completes
        assertTrue(component.state.value.isLoading)
    }

    // ===== State Management Tests =====

    @Test
    fun `onEmailChanged updates email in state`() {
        val component = createComponent()

        component.onEmailChanged("user@example.com")

        assertEquals("user@example.com", component.state.value.email)
    }

    @Test
    fun `onDisplayNameChanged updates display name in state`() {
        val component = createComponent()

        component.onDisplayNameChanged("John Doe")

        assertEquals("John Doe", component.state.value.displayName)
    }

    @Test
    fun `onPasswordChanged updates password in state`() {
        val component = createComponent()

        component.onPasswordChanged("secret123")

        assertEquals("secret123", component.state.value.password)
    }

    @Test
    fun `onConfirmPasswordChanged updates confirm password in state`() {
        val component = createComponent()

        component.onConfirmPasswordChanged("secret123")

        assertEquals("secret123", component.state.value.confirmPassword)
    }

    @Test
    fun `onInviteCodeChanged updates invite code in state`() {
        val component = createComponent()

        component.onInviteCodeChanged("INVITE123")

        assertEquals("INVITE123", component.state.value.inviteCode)
    }

    @Test
    fun `initial state has empty fields and no errors`() {
        val component = createComponent()
        val state = component.state.value

        assertEquals("", state.email)
        assertEquals("", state.displayName)
        assertEquals("", state.password)
        assertEquals("", state.confirmPassword)
        assertEquals("", state.inviteCode)
        assertNull(state.emailError)
        assertNull(state.displayNameError)
        assertNull(state.passwordError)
        assertNull(state.confirmPasswordError)
        assertNull(state.inviteCodeError)
        assertNull(state.error)
        assertFalse(state.isLoading)
    }

    // ===== Navigation Tests =====

    @Test
    fun `onLoginClicked calls navigation callback`() {
        val component = createComponent()

        component.onLoginClicked()

        assertTrue(navigateToLoginCalled)
    }

    // ===== Helper Methods =====

    @Suppress("LongParameterList")
    private fun fillValidForm(
        component: RegisterComponent,
        email: String = "test@example.com",
        displayName: String = "Test User",
        password: String = "password123",
        confirmPassword: String = "password123",
        inviteCode: String = "",
    ) {
        component.onEmailChanged(email)
        component.onDisplayNameChanged(displayName)
        component.onPasswordChanged(password)
        component.onConfirmPasswordChanged(confirmPassword)
        component.onInviteCodeChanged(inviteCode)
    }

    private fun createAuthResponse() =
        AuthResponse(
            accessToken = "access-token",
            refreshToken = "refresh-token",
            expiresIn = 900,
            userId = "user-123",
            displayName = "Test User",
            role = "member",
        )
}
