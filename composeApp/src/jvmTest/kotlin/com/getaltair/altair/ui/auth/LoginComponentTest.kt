package com.getaltair.altair.ui.auth

import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.service.auth.AuthManager
import com.getaltair.altair.service.auth.FakePublicAuthService
import com.getaltair.altair.service.auth.FakeSecureTokenStorage
import io.kotest.assertions.timing.eventually
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.nulls.shouldNotBeNull
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlin.time.Duration.Companion.seconds

/**
 * Tests for LoginComponent.
 *
 * Verifies:
 * - Form validation (email, password)
 * - Login flow success and error handling
 * - Navigation callbacks
 * - State management
 */
class LoginComponentTest :
    BehaviorSpec({
        lateinit var tokenStorage: FakeSecureTokenStorage
        lateinit var authService: FakePublicAuthService
        lateinit var authManager: AuthManager
        lateinit var scope: CoroutineScope

        var loginSuccessCalled = false
        var navigateToRegisterCalled = false

        beforeEach {
            tokenStorage = FakeSecureTokenStorage()
            authService = FakePublicAuthService()
            scope = CoroutineScope(Dispatchers.Default)
            authManager = AuthManager(tokenStorage, authService, scope)
            loginSuccessCalled = false
            navigateToRegisterCalled = false
        }

        fun createTestComponentContext(): DefaultComponentContext {
            val lifecycle = LifecycleRegistry()
            lifecycle.resume()
            return DefaultComponentContext(lifecycle = lifecycle)
        }

        fun createComponent(): LoginComponent =
            LoginComponent(
                componentContext = createTestComponentContext(),
                authManager = authManager,
                onLoginSuccess = { loginSuccessCalled = true },
                onNavigateToRegister = { navigateToRegisterCalled = true },
            )

        fun createAuthResponse() =
            AuthResponse(
                accessToken = "access-token",
                refreshToken = "refresh-token",
                expiresIn = 900,
                userId = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV"),
                displayName = "Test User",
                role = UserRole.MEMBER,
            )

        given("form validation") {
            `when`("email is blank") {
                then("shows email required error") {
                    val component = createComponent()

                    component.onEmailChanged("")
                    component.onPasswordChanged("password123")
                    component.onLoginClicked()

                    component.state.value.emailError shouldBe "Email is required"
                    component.state.value.passwordError
                        .shouldBeNull()
                }
            }

            `when`("email has invalid format") {
                then("shows invalid email error") {
                    val component = createComponent()

                    component.onEmailChanged("invalid-email")
                    component.onPasswordChanged("password123")
                    component.onLoginClicked()

                    component.state.value.emailError shouldBe "Enter a valid email address"
                }
            }

            `when`("password is blank") {
                then("shows password required error") {
                    val component = createComponent()

                    component.onEmailChanged("test@example.com")
                    component.onPasswordChanged("")
                    component.onLoginClicked()

                    component.state.value.passwordError shouldBe "Password is required"
                    component.state.value.emailError
                        .shouldBeNull()
                }
            }

            `when`("entering valid input after errors") {
                then("clears previous errors") {
                    val component = createComponent()

                    // First trigger validation errors
                    component.onLoginClicked()
                    component.state.value.emailError
                        .shouldNotBeNull()
                    component.state.value.passwordError
                        .shouldNotBeNull()

                    // Now enter valid values - errors should clear
                    component.onEmailChanged("test@example.com")
                    component.state.value.emailError
                        .shouldBeNull()

                    component.onPasswordChanged("password123")
                    component.state.value.passwordError
                        .shouldBeNull()
                }
            }
        }

        given("login flow") {
            `when`("login succeeds") {
                then("calls onLoginSuccess callback") {
                    val component = createComponent()
                    authService.loginResponse = createAuthResponse()

                    component.onEmailChanged("test@example.com")
                    component.onPasswordChanged("password123")
                    component.onLoginClicked()

                    // Wait for async operation to complete
                    eventually(1.seconds) {
                        component.state.value.isLoading
                            .shouldBeFalse()
                    }

                    loginSuccessCalled.shouldBeTrue()
                    component.state.value.error
                        .shouldBeNull()
                }
            }

            `when`("login fails") {
                then("shows error message") {
                    val component = createComponent()
                    authService.loginError = IllegalArgumentException("Invalid credentials")

                    component.onEmailChanged("test@example.com")
                    component.onPasswordChanged("wrong-password")
                    component.onLoginClicked()

                    // Wait for async operation to complete
                    eventually(1.seconds) {
                        component.state.value.isLoading
                            .shouldBeFalse()
                    }

                    loginSuccessCalled.shouldBeFalse()
                    component.state.value.error
                        .shouldNotBeNull()
                }
            }

            `when`("login is in progress") {
                then("shows loading state") {
                    val component = createComponent()
                    authService.loginResponse = createAuthResponse()

                    component.onEmailChanged("test@example.com")
                    component.onPasswordChanged("password123")
                    component.onLoginClicked()

                    // Check loading state before coroutine completes
                    component.state.value.isLoading
                        .shouldBeTrue()
                }
            }
        }

        given("state management") {
            `when`("email is changed") {
                then("updates email in state") {
                    val component = createComponent()

                    component.onEmailChanged("user@example.com")

                    component.state.value.email shouldBe "user@example.com"
                }
            }

            `when`("password is changed") {
                then("updates password in state") {
                    val component = createComponent()

                    component.onPasswordChanged("secret123")

                    component.state.value.password shouldBe "secret123"
                }
            }

            `when`("component is initialized") {
                then("has empty fields and no errors") {
                    val component = createComponent()
                    val state = component.state.value

                    state.email shouldBe ""
                    state.password shouldBe ""
                    state.emailError.shouldBeNull()
                    state.passwordError.shouldBeNull()
                    state.error.shouldBeNull()
                    state.isLoading.shouldBeFalse()
                }
            }
        }

        given("navigation") {
            `when`("register is clicked") {
                then("calls navigation callback") {
                    val component = createComponent()

                    component.onRegisterClicked()

                    navigateToRegisterCalled.shouldBeTrue()
                }
            }
        }
    })
