package com.getaltair.altair.ui.auth

import app.cash.turbine.test
import com.getaltair.altair.data.auth.TokenPreferences
import com.getaltair.altair.domain.repository.AuthRepository
import io.mockk.coEvery
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.test.UnconfinedTestDispatcher
import kotlinx.coroutines.test.advanceUntilIdle
import kotlinx.coroutines.test.resetMain
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.test.setMain
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertInstanceOf
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test

/**
 * Unit tests for [AuthViewModel], covering authentication state transitions and error handling.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class AuthViewModelTest {
    private val testDispatcher = UnconfinedTestDispatcher()

    private lateinit var authRepository: AuthRepository
    private lateinit var tokenPreferences: TokenPreferences
    private lateinit var isLoggedInFlow: MutableStateFlow<Boolean>
    private lateinit var viewModel: AuthViewModel

    @BeforeEach
    fun setUp() {
        Dispatchers.setMain(testDispatcher)

        authRepository = mockk(relaxed = true)
        tokenPreferences = mockk(relaxed = true)

        isLoggedInFlow = MutableStateFlow(false)
        every { tokenPreferences.isLoggedInFlow } returns isLoggedInFlow

        viewModel =
            AuthViewModel(
                authRepository = authRepository,
                tokenPreferences = tokenPreferences,
            )
    }

    @AfterEach
    fun tearDown() {
        Dispatchers.resetMain()
    }

    /**
     * Successful login sets isAuthenticated to true.
     * The token preferences flow mirrors what the repository sets after login.
     */
    @Test
    fun `login_success_setsIsAuthenticatedTrue - isAuthenticated emits true after successful login`() =
        runTest {
            coEvery { authRepository.login(any(), any()) } coAnswers {
                // Simulate token being saved, which would flip isLoggedInFlow in production
                isLoggedInFlow.value = true
            }

            viewModel.isAuthenticated.test {
                // Initial value is false
                awaitItem()

                viewModel.login("user@example.com", "password123")
                advanceUntilIdle()

                val authenticated = awaitItem()
                assertTrue(authenticated, "isAuthenticated must emit true after successful login")

                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * Failed login emits an error state.
     * The repository throws, which must surface as AuthUiState.Error.
     */
    @Test
    fun `login_failure_emitsErrorState - uiState emits Error when login throws`() =
        runTest {
            coEvery { authRepository.login(any(), any()) } throws RuntimeException("Invalid credentials")

            viewModel.uiState.test {
                // Skip initial Idle
                awaitItem()

                viewModel.login("user@example.com", "wrongpassword")
                advanceUntilIdle()

                // Loading state
                awaitItem()

                val state = awaitItem()
                assertInstanceOf(
                    AuthUiState.Error::class.java,
                    state,
                    "uiState must emit Error when login throws",
                )

                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * Failed registration emits an error state.
     * The repository throws, which must surface as AuthUiState.Error.
     */
    @Test
    fun `register_failure_emitsErrorState - uiState emits Error when register throws`() =
        runTest {
            coEvery { authRepository.register(any(), any(), any()) } throws RuntimeException("Email already exists")

            viewModel.uiState.test {
                // Skip initial Idle
                awaitItem()

                viewModel.register("existing@example.com", "password123", "Test User")
                advanceUntilIdle()

                // Loading state
                awaitItem()

                val state = awaitItem()
                assertInstanceOf(
                    AuthUiState.Error::class.java,
                    state,
                    "uiState must emit Error when register throws",
                )

                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * isAuthenticated mirrors the tokenPreferences.isLoggedInFlow sequence.
     * Emitting false then true must be reflected in the ViewModel's isAuthenticated flow.
     */
    @Test
    fun `isAuthenticated_derivesFromTokenPreferencesFlow - mirrors isLoggedInFlow sequence`() =
        runTest {
            viewModel.isAuthenticated.test {
                // Initial value: false
                val first = awaitItem()
                assertTrue(!first, "isAuthenticated must start as false")

                // Simulate token saved externally (e.g. by login)
                isLoggedInFlow.value = true

                val second = awaitItem()
                assertTrue(second, "isAuthenticated must emit true when isLoggedInFlow flips to true")

                cancelAndIgnoreRemainingEvents()
            }
        }
}
