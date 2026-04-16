package com.getaltair.altair.ui.guidance

import androidx.compose.ui.test.assertCountEquals
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithText
import androidx.navigation.NavController
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.ui.theme.AltairTheme
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class QuestDetailScreenTest {
    @get:Rule
    val composeTestRule = createComposeRule()

    private fun buildQuest(status: String): QuestEntity =
        QuestEntity(
            id = "quest-1",
            title = "Test Quest",
            description = "A test quest description",
            status = status,
            priority = "medium",
            dueDate = null,
            epicId = null,
            initiativeId = null,
            routineId = null,
            userId = "user-1",
            createdAt = "2026-01-01T00:00:00",
            updatedAt = "2026-01-01T00:00:00",
            deletedAt = null,
        )

    private fun buildViewModel(quest: QuestEntity): QuestDetailViewModel {
        val viewModel = mockk<QuestDetailViewModel>(relaxed = true)
        every { viewModel.quest } returns MutableStateFlow(quest)
        // Delegate to real logic for valid transitions
        every { viewModel.validTransitions(quest.status) } answers {
            val transitions =
                mapOf(
                    "not_started" to listOf("in_progress", "deferred", "cancelled"),
                    "in_progress" to listOf("completed", "deferred", "cancelled"),
                    "deferred" to listOf("not_started", "in_progress", "cancelled"),
                    "completed" to emptyList(),
                    "cancelled" to emptyList(),
                )
            transitions[quest.status] ?: emptyList()
        }
        return viewModel
    }

    /**
     * FA-004: for a not_started quest, the valid transition "In progress" is shown;
     * "Completed" (only valid from in_progress) must not appear.
     */
    @Test
    fun notStarted_showsInProgressNotCompleted() {
        val quest = buildQuest("not_started")
        val viewModel = buildViewModel(quest)
        val navController = mockk<NavController>(relaxed = true)

        composeTestRule.setContent {
            AltairTheme {
                QuestDetailScreen(
                    questId = quest.id,
                    navController = navController,
                    viewModel = viewModel,
                )
            }
        }

        // "In progress" is the capitalised display of the "in_progress" transition
        composeTestRule.onNodeWithText("In progress").assertIsDisplayed()
        // "Completed" is only a valid transition from in_progress, so it must not appear
        composeTestRule.onAllNodesWithText("Completed").assertCountEquals(0)
    }

    /**
     * FA-004: for an in_progress quest, the valid transition "Completed" is shown.
     */
    @Test
    fun inProgress_showsCompleted() {
        val quest = buildQuest("in_progress")
        val viewModel = buildViewModel(quest)
        val navController = mockk<NavController>(relaxed = true)

        composeTestRule.setContent {
            AltairTheme {
                QuestDetailScreen(
                    questId = quest.id,
                    navController = navController,
                    viewModel = viewModel,
                )
            }
        }

        composeTestRule.onNodeWithText("Completed").assertIsDisplayed()
    }
}
