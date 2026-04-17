package com.getaltair.altair

import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createEmptyComposeRule
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithText
import androidx.test.core.app.ActivityScenario
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.getaltair.altair.data.auth.TokenPreferences
import org.junit.After
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.koin.java.KoinJavaComponent.get

/**
 * Instrumented integration test for [MainActivity] authentication-gating behavior.
 *
 * Verifies that clearing tokens while the app is running causes the navigation stack
 * to transition to the Login screen and removes all protected screens from the back stack.
 */
@RunWith(AndroidJUnit4::class)
class MainActivityTest {
    @get:Rule
    val composeTestRule = createEmptyComposeRule()

    private val tokenPreferences: TokenPreferences get() = get(TokenPreferences::class.java)

    @Before
    fun seedAuthenticatedState() {
        // Place a fake access token so MainActivity boots into the authenticated state.
        // The token value doesn't matter for navigation — only its presence is checked.
        tokenPreferences.accessToken = "test-access-token"
        tokenPreferences.refreshToken = "test-refresh-token"
    }

    @After
    fun clearAuthState() {
        // Ensure no residual token leaks into subsequent test runs.
        tokenPreferences.clearTokens()
    }

    @Test
    fun unauthenticated_navigatesToLoginScreen() {
        ActivityScenario.launch(MainActivity::class.java).use {
            // Give the authenticated scaffold time to appear before triggering the logout.
            composeTestRule.waitUntil(timeoutMillis = 5_000) {
                composeTestRule
                    .onAllNodesWithText("Today")
                    .fetchSemanticsNodes()
                    .isNotEmpty()
            }

            // Clearing tokens emits false on isLoggedInFlow, which MainActivity's
            // LaunchedEffect(isAuthenticated) picks up and navigates to Login, clearing
            // the back stack via popUpTo(0) { inclusive = true }.
            tokenPreferences.clearTokens()

            // Wait for the Login screen's Email field to become visible.
            composeTestRule.waitUntil(timeoutMillis = 5_000) {
                composeTestRule
                    .onAllNodesWithText("Email")
                    .fetchSemanticsNodes()
                    .isNotEmpty()
            }

            // Assert Login screen elements are present.
            composeTestRule.onNodeWithText("Email").assertIsDisplayed()
            composeTestRule.onNodeWithText("Password").assertIsDisplayed()
            // "Sign In" appears as both the screen title and the button — assert at least one.
            composeTestRule.onAllNodesWithText("Sign In")[0].assertIsDisplayed()
        }
    }
}
