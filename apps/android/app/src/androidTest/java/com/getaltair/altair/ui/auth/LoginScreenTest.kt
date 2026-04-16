package com.getaltair.altair.ui.auth

import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.compose.ui.test.performTextInput
import androidx.navigation.NavController
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.getaltair.altair.ui.theme.AltairTheme
import io.mockk.every
import io.mockk.mockk
import io.mockk.verify
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class LoginScreenTest {
    @get:Rule
    val composeTestRule = createComposeRule()

    @Test
    fun login_callsViewModelWithCredentials() {
        val viewModel = mockk<AuthViewModel>(relaxed = true)
        every { viewModel.uiState } returns MutableStateFlow(AuthUiState.Idle)

        val navController = mockk<NavController>(relaxed = true)

        composeTestRule.setContent {
            AltairTheme {
                LoginScreen(
                    navController = navController,
                    viewModel = viewModel,
                )
            }
        }

        composeTestRule.onNodeWithText("Email").performClick()
        composeTestRule.onNodeWithText("Email").performTextInput("email@example.com")

        composeTestRule.onNodeWithText("Password").performClick()
        composeTestRule.onNodeWithText("Password").performTextInput("password123")

        composeTestRule.onNodeWithText("Sign In").performClick()

        verify { viewModel.login("email@example.com", "password123") }
    }
}
