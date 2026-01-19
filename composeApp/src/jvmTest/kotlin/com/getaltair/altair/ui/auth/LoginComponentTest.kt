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
 * Tests for LoginComponent.
 *
 * Verifies:
 * - Form validation (email, password)
 * - Login flow success and error handling
 * - Navigation callbacks
 * - State management
 */
@Suppress("TooManyFunctions")
class LoginComponentTest {
    private lateinit var tokenStorage: FakeSecureTokenStorage
    private lateinit var authService: FakePublicAuthService
    private lateinit var authManager: AuthManager
    private lateinit var scope: CoroutineScope

    private var loginSuccessCalled = false
    private var navigateToRegisterCalled = false

    @BeforeTest
    fun setup() {
        tokenStorage = FakeSecureTokenStorage()
        authService = FakePublicAuthService()
        scope = CoroutineScope(Dispatchers.Default)
        authManager = AuthManager(tokenStorage, authService, scope)
        loginSuccessCalled = false
        navigateToRegisterCalled = false
    }

    private fun createTestComponentContext(): DefaultComponentContext {
        val lifecycle = LifecycleRegistry()
        lifecycle.resume()
        return DefaultComponentContext(lifecycle = lifecycle)
    }

    private fun createComponent(): LoginComponent =
        LoginComponent(
            componentContext = createTestComponentContext(),
            authManager = authManager,
            onLoginSuccess = { loginSuccessCalled = true },
            onNavigateToRegister = { navigateToRegisterCalled = true },
        )

    // ===== Form Validation Tests =====

    @Test
    fun `email validation shows error for blank email`() {
        val component = createComponent()

        component.onEmailChanged("")
        component.onPasswordChanged("password123")
        component.onLoginClicked()

        assertEquals("Email is required", component.state.value.emailError)
        assertNull(component.state.value.passwordError)
    }

    @Test
    fun `email validation shows error for invalid email format`() {
        val component = createComponent()

        component.onEmailChanged("invalid-email")
        component.onPasswordChanged("password123")
        component.onLoginClicked()

        assertEquals("Enter a valid email address", component.state.value.emailError)
    }

    @Test
    fun `password validation shows error for blank password`() {
        val component = createComponent()

        component.onEmailChanged("test@example.com")
        component.onPasswordChanged("")
        component.onLoginClicked()

        assertEquals("Password is required", component.state.value.passwordError)
        assertNull(component.state.value.emailError)
    }

    @Test
    fun `valid input clears previous errors`() {
        val component = createComponent()

        // First trigger validation errors
        component.onLoginClicked()
        assertNotNull(component.state.value.emailError)
        assertNotNull(component.state.value.passwordError)

        // Now enter valid values - errors should clear
        component.onEmailChanged("test@example.com")
        assertNull(component.state.value.emailError)

        component.onPasswordChanged("password123")
        assertNull(component.state.value.passwordError)
    }

    // ===== Login Flow Tests =====

    @Test
    fun `successful login calls onLoginSuccess`() {
        runBlocking {
            val component = createComponent()
            authService.loginResponse = createAuthResponse()

            component.onEmailChanged("test@example.com")
            component.onPasswordChanged("password123")
            component.onLoginClicked()

            // Wait for async operation to complete
            waitForCondition { !component.state.value.isLoading }

            assertTrue(loginSuccessCalled)
            assertFalse(component.state.value.isLoading)
            assertNull(component.state.value.error)
        }
    }

    @Test
    fun `failed login shows error message`() {
        runBlocking {
            val component = createComponent()
            authService.loginError = IllegalArgumentException("Invalid credentials")

            component.onEmailChanged("test@example.com")
            component.onPasswordChanged("wrong-password")
            component.onLoginClicked()

            // Wait for async operation to complete
            waitForCondition { !component.state.value.isLoading }

            assertFalse(loginSuccessCalled)
            assertFalse(component.state.value.isLoading)
            assertNotNull(component.state.value.error)
        }
    }

    @Test
    fun `login shows loading state while authenticating`() {
        val component = createComponent()
        authService.loginResponse = createAuthResponse()

        component.onEmailChanged("test@example.com")
        component.onPasswordChanged("password123")
        component.onLoginClicked()

        // Check loading state before coroutine completes
        assertTrue(component.state.value.isLoading)
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

    // ===== State Management Tests =====

    @Test
    fun `onEmailChanged updates email in state`() {
        val component = createComponent()

        component.onEmailChanged("user@example.com")

        assertEquals("user@example.com", component.state.value.email)
    }

    @Test
    fun `onPasswordChanged updates password in state`() {
        val component = createComponent()

        component.onPasswordChanged("secret123")

        assertEquals("secret123", component.state.value.password)
    }

    @Test
    fun `initial state has empty fields and no errors`() {
        val component = createComponent()
        val state = component.state.value

        assertEquals("", state.email)
        assertEquals("", state.password)
        assertNull(state.emailError)
        assertNull(state.passwordError)
        assertNull(state.error)
        assertFalse(state.isLoading)
    }

    // ===== Navigation Tests =====

    @Test
    fun `onRegisterClicked calls navigation callback`() {
        val component = createComponent()

        component.onRegisterClicked()

        assertTrue(navigateToRegisterCalled)
    }

    // ===== Helper Methods =====

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
