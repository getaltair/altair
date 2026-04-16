package com.getaltair.altair.ui.guidance

import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performClick
import androidx.navigation.NavController
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.getaltair.altair.ui.theme.AltairTheme
import com.powersync.PowerSyncDatabase
import io.mockk.coVerify
import io.mockk.every
import io.mockk.mockk
import io.mockk.verify
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.flowOf
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class FocusSessionScreenTest {
    @get:Rule
    val composeTestRule = createComposeRule()

    /**
     * S022 / Test 1: isFinished_transitionsTrue_viaOnFinish
     *
     * Verifies that when the ViewModel's `isFinished` flow emits `true`
     * (the observable consequence of `CountDownTimer.onFinish()` completing),
     * the screen calls `navController.popBackStack()`.
     *
     * Note: CountDownTimer is a concrete Android class; it cannot be made to
     * fire onFinish() synchronously in a test. This test verifies the same
     * public contract by flipping `isFinished` directly through a mocked VM.
     */
    @Test
    fun isFinished_transitionsTrue_viaOnFinish() {
        val isFinished = MutableStateFlow(false)
        val viewModel = mockk<FocusSessionViewModel>(relaxed = true)
        every { viewModel.remainingMs } returns MutableStateFlow(25 * 60 * 1_000L)
        every { viewModel.isRunning } returns MutableStateFlow(true)
        every { viewModel.isFinished } returns isFinished
        every { viewModel.error } returns MutableStateFlow(null)

        val navController = mockk<NavController>(relaxed = true)

        composeTestRule.setContent {
            AltairTheme {
                FocusSessionScreen(
                    questId = "quest-1",
                    navController = navController,
                    viewModel = viewModel,
                )
            }
        }

        // Simulate timer completion: flip isFinished to true, as CountDownTimer.onFinish() would do
        isFinished.value = true
        composeTestRule.waitForIdle()

        verify { navController.popBackStack() }
    }

    /**
     * S022 / Test 2: recordSession_calledOnFinish
     *
     * Verifies that when a session ends (via the "End Session" button), the ViewModel
     * persists the completed session by calling `db.execute` with an INSERT into
     * `focus_sessions`. This uses the same `recordSession` private path that the
     * timer's `onFinish()` invokes, exercised here via `end()` which is the only
     * publicly reachable path without waiting 25 real minutes.
     *
     * `db.watch` must be stubbed to emit a non-null userId; otherwise `start()`
     * short-circuits with an error and the session is never recorded.
     */
    @Test
    fun recordSession_calledOnFinish() {
        val db = mockk<PowerSyncDatabase>(relaxed = true)
        every {
            db.watch<String?>(
                sql = any(),
                parameters = any(),
                mapper = any(),
            )
        } returns flowOf(listOf("user-1"))

        val viewModel = FocusSessionViewModel(db)

        val navController = mockk<NavController>(relaxed = true)

        composeTestRule.setContent {
            AltairTheme {
                FocusSessionScreen(
                    questId = "quest-1",
                    navController = navController,
                    viewModel = viewModel,
                )
            }
        }

        composeTestRule.waitForIdle()

        // Tap the "End Session" button — this calls viewModel.end() which invokes recordSession
        composeTestRule.onNodeWithText("End Session").performClick()

        composeTestRule.waitForIdle()

        coVerify {
            db.execute(
                match { sql -> sql.contains("INSERT") && sql.contains("focus_sessions") },
                any(),
            )
        }
    }
}
