package com.getaltair.altair.ui.today

import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onNodeWithText
import androidx.compose.ui.test.performTouchInput
import androidx.compose.ui.test.swipeRight
import androidx.navigation.NavController
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.data.local.entity.RoutineEntity
import com.getaltair.altair.data.local.entity.UserEntity
import com.getaltair.altair.ui.theme.AltairTheme
import com.powersync.sync.SyncStatusData
import io.mockk.every
import io.mockk.mockk
import io.mockk.verify
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class TodayScreenTest {
    @get:Rule
    val composeTestRule = createComposeRule()

    private fun buildQuest(
        id: String,
        title: String,
        status: String,
    ): QuestEntity =
        QuestEntity(
            id = id,
            title = title,
            description = null,
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

    private fun buildViewModel(quests: List<QuestEntity>): TodayViewModel {
        val viewModel = mockk<TodayViewModel>(relaxed = true)
        every { viewModel.currentUser } returns
            MutableStateFlow(
                UserEntity(
                    id = "user-1",
                    passwordHash = null,
                    email = "test@example.com",
                    displayName = "Test User",
                    isAdmin = 0,
                    status = "active",
                    createdAt = "2026-01-01T00:00:00",
                    updatedAt = "2026-01-01T00:00:00",
                    deletedAt = null,
                ),
            )
        every { viewModel.todayQuests } returns MutableStateFlow(quests)
        every { viewModel.dueRoutines } returns MutableStateFlow(emptyList<RoutineEntity>())
        every { viewModel.isTodayCheckinDone } returns MutableStateFlow(true)
        every { viewModel.checkinEnergy } returns MutableStateFlow(3)
        every { viewModel.checkinMood } returns MutableStateFlow(3)
        every { viewModel.syncStatus } returns
            MutableStateFlow(
                mockk<SyncStatusData>(relaxed = true) {
                    every { connected } returns true
                    every { uploading } returns false
                },
            )
        return viewModel
    }

    @Test
    fun swipeRight_inProgress_triggersComplete() {
        val inProgressQuest = buildQuest("q1", "In Progress Quest", "in_progress")
        val notStartedQuest = buildQuest("q2", "Not Started Quest", "not_started")
        val viewModel = buildViewModel(listOf(inProgressQuest, notStartedQuest))

        val navController = mockk<NavController>(relaxed = true)

        composeTestRule.setContent {
            AltairTheme {
                TodayScreen(
                    navController = navController,
                    viewModel = viewModel,
                )
            }
        }

        composeTestRule
            .onNodeWithText("In Progress Quest")
            .performTouchInput { swipeRight() }

        verify { viewModel.completeQuest("q1") }
    }

    @Test
    fun allGuidance_buttonExists() {
        val viewModel = buildViewModel(emptyList())
        val navController = mockk<NavController>(relaxed = true)

        composeTestRule.setContent {
            AltairTheme {
                TodayScreen(
                    navController = navController,
                    viewModel = viewModel,
                )
            }
        }

        composeTestRule.onNodeWithText("All Guidance").assertIsDisplayed()
    }
}
