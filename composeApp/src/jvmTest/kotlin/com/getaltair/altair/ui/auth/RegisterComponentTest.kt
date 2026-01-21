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
 * Tests for RegisterComponent.
 *
 * Verifies:
 * - Form validation (email, display name, password, confirm password, invite code)
 * - Registration flow success and error handling
 * - Navigation callbacks
 * - State management
 */
class RegisterComponentTest :
    BehaviorSpec({
        lateinit var tokenStorage: FakeSecureTokenStorage
        lateinit var authService: FakePublicAuthService
        lateinit var authManager: AuthManager
        lateinit var scope: CoroutineScope

        var registerSuccessCalled = false
        var navigateToLoginCalled = false

        beforeEach {
            tokenStorage = FakeSecureTokenStorage()
            authService = FakePublicAuthService()
            scope = CoroutineScope(Dispatchers.Default)
            authManager = AuthManager(tokenStorage, authService, scope)
            registerSuccessCalled = false
            navigateToLoginCalled = false
        }

        fun createTestComponentContext(): DefaultComponentContext {
            val lifecycle = LifecycleRegistry()
            lifecycle.resume()
            return DefaultComponentContext(lifecycle = lifecycle)
        }

        fun createComponent(): RegisterComponent =
            RegisterComponent(
                componentContext = createTestComponentContext(),
                authManager = authManager,
                onRegisterSuccess = { registerSuccessCalled = true },
                onNavigateToLogin = { navigateToLoginCalled = true },
            )

        @Suppress("LongParameterList")
        fun fillValidForm(
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

                    fillValidForm(component, email = "")
                    component.onRegisterClicked()

                    component.state.value.emailError shouldBe "Email is required"
                }
            }

            `when`("email has invalid format") {
                then("shows invalid email error") {
                    val component = createComponent()

                    fillValidForm(component, email = "invalid-email")
                    component.onRegisterClicked()

                    component.state.value.emailError shouldBe "Enter a valid email address"
                }
            }

            `when`("display name is blank") {
                then("shows display name required error") {
                    val component = createComponent()

                    fillValidForm(component, displayName = "")
                    component.onRegisterClicked()

                    component.state.value.displayNameError shouldBe "Display name is required"
                }
            }

            `when`("display name is too short") {
                then("shows minimum length error") {
                    val component = createComponent()

                    fillValidForm(component, displayName = "A")
                    component.onRegisterClicked()

                    component.state.value.displayNameError shouldBe "Display name must be at least 2 characters"
                }
            }

            `when`("password is blank") {
                then("shows password required error") {
                    val component = createComponent()

                    fillValidForm(component, password = "", confirmPassword = "")
                    component.onRegisterClicked()

                    component.state.value.passwordError shouldBe "Password is required"
                }
            }

            `when`("password is too short") {
                then("shows minimum length error") {
                    val component = createComponent()

                    fillValidForm(component, password = "short", confirmPassword = "short")
                    component.onRegisterClicked()

                    component.state.value.passwordError shouldBe "Password must be at least 8 characters"
                }
            }

            `when`("confirm password is blank") {
                then("shows confirmation required error") {
                    val component = createComponent()

                    fillValidForm(component, confirmPassword = "")
                    component.onRegisterClicked()

                    component.state.value.confirmPasswordError shouldBe "Please confirm your password"
                }
            }

            `when`("passwords do not match") {
                then("shows mismatch error") {
                    val component = createComponent()

                    fillValidForm(component, password = "password123", confirmPassword = "different123")
                    component.onRegisterClicked()

                    component.state.value.confirmPasswordError shouldBe "Passwords do not match"
                }
            }

            `when`("entering valid input after errors") {
                then("clears previous errors") {
                    val component = createComponent()

                    // First trigger validation errors
                    component.onRegisterClicked()
                    component.state.value.emailError
                        .shouldNotBeNull()

                    // Now enter valid value - error should clear
                    component.onEmailChanged("test@example.com")
                    component.state.value.emailError
                        .shouldBeNull()
                }
            }
        }

        given("registration flow") {
            `when`("registration succeeds") {
                then("calls onRegisterSuccess callback") {
                    val component = createComponent()
                    authService.registerResponse = createAuthResponse()

                    fillValidForm(component)
                    component.onRegisterClicked()

                    eventually(1.seconds) {
                        component.state.value.isLoading
                            .shouldBeFalse()
                    }

                    registerSuccessCalled.shouldBeTrue()
                    component.state.value.error
                        .shouldBeNull()
                }
            }

            `when`("registration with invite code") {
                then("passes code to auth manager") {
                    val component = createComponent()
                    authService.registerResponse = createAuthResponse()

                    fillValidForm(component, inviteCode = "INVITE123")
                    component.onRegisterClicked()

                    eventually(1.seconds) {
                        component.state.value.isLoading
                            .shouldBeFalse()
                    }

                    authService.lastRegisterRequest?.inviteCode shouldBe "INVITE123"
                }
            }

            `when`("registration without invite code") {
                then("sends null for invite code") {
                    val component = createComponent()
                    authService.registerResponse = createAuthResponse()

                    fillValidForm(component, inviteCode = "")
                    component.onRegisterClicked()

                    eventually(1.seconds) {
                        component.state.value.isLoading
                            .shouldBeFalse()
                    }

                    authService.lastRegisterRequest?.inviteCode.shouldBeNull()
                }
            }

            `when`("registration fails") {
                then("shows error message") {
                    val component = createComponent()
                    authService.registerError = IllegalArgumentException("Email already registered")

                    fillValidForm(component)
                    component.onRegisterClicked()

                    eventually(1.seconds) {
                        component.state.value.isLoading
                            .shouldBeFalse()
                    }

                    registerSuccessCalled.shouldBeFalse()
                    component.state.value.error
                        .shouldNotBeNull()
                }
            }

            `when`("invalid invite code") {
                then("shows specific invite code error") {
                    val component = createComponent()
                    authService.registerError = IllegalArgumentException("Invalid invite code")

                    fillValidForm(component, inviteCode = "INVALID")
                    component.onRegisterClicked()

                    eventually(1.seconds) {
                        component.state.value.isLoading
                            .shouldBeFalse()
                    }

                    component.state.value.inviteCodeError shouldBe "Invalid or expired invite code"
                }
            }

            `when`("registration is in progress") {
                then("shows loading state") {
                    val component = createComponent()
                    authService.registerResponse = createAuthResponse()

                    fillValidForm(component)
                    component.onRegisterClicked()

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

            `when`("display name is changed") {
                then("updates display name in state") {
                    val component = createComponent()

                    component.onDisplayNameChanged("John Doe")

                    component.state.value.displayName shouldBe "John Doe"
                }
            }

            `when`("password is changed") {
                then("updates password in state") {
                    val component = createComponent()

                    component.onPasswordChanged("secret123")

                    component.state.value.password shouldBe "secret123"
                }
            }

            `when`("confirm password is changed") {
                then("updates confirm password in state") {
                    val component = createComponent()

                    component.onConfirmPasswordChanged("secret123")

                    component.state.value.confirmPassword shouldBe "secret123"
                }
            }

            `when`("invite code is changed") {
                then("updates invite code in state") {
                    val component = createComponent()

                    component.onInviteCodeChanged("INVITE123")

                    component.state.value.inviteCode shouldBe "INVITE123"
                }
            }

            `when`("component is initialized") {
                then("has empty fields and no errors") {
                    val component = createComponent()
                    val state = component.state.value

                    state.email shouldBe ""
                    state.displayName shouldBe ""
                    state.password shouldBe ""
                    state.confirmPassword shouldBe ""
                    state.inviteCode shouldBe ""
                    state.emailError.shouldBeNull()
                    state.displayNameError.shouldBeNull()
                    state.passwordError.shouldBeNull()
                    state.confirmPasswordError.shouldBeNull()
                    state.inviteCodeError.shouldBeNull()
                    state.error.shouldBeNull()
                    state.isLoading.shouldBeFalse()
                }
            }
        }

        given("navigation") {
            `when`("login is clicked") {
                then("calls navigation callback") {
                    val component = createComponent()

                    component.onLoginClicked()

                    navigateToLoginCalled.shouldBeTrue()
                }
            }
        }
    })
