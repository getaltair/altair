package com.getaltair.altair.ui.guidance

import android.content.Context
import androidx.arch.core.executor.testing.InstantTaskExecutorRule
import androidx.compose.ui.test.assertCountEquals
import androidx.compose.ui.test.assertIsDisplayed
import androidx.compose.ui.test.junit4.createComposeRule
import androidx.compose.ui.test.onAllNodesWithText
import androidx.compose.ui.test.onNodeWithText
import androidx.lifecycle.SavedStateHandle
import androidx.navigation.NavController
import androidx.room.Room
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import com.getaltair.altair.data.local.AltairDatabase
import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.ui.theme.AltairTheme
import com.powersync.PowerSyncDatabase
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.flow.MutableStateFlow
import org.junit.After
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class QuestDetailScreenTest {
    @get:Rule
    val composeTestRule = createComposeRule()

    @get:Rule
    val instantTaskRule = InstantTaskExecutorRule()

    private var inMemoryDb: AltairDatabase? = null

    @After
    fun tearDown() {
        inMemoryDb?.close()
        inMemoryDb = null
    }

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

    /**
     * S025 / FA-004: for a not_started quest, the "In progress" transition button is shown
     * (rendered via real ViewModel backed by in-memory Room DAO).
     */
    @Test
    fun questDetail_notStarted_showsStartAction() {
        val context: Context = ApplicationProvider.getApplicationContext()
        val db =
            Room
                .inMemoryDatabaseBuilder(context, AltairDatabase::class.java)
                .allowMainThreadQueries()
                .build()
                .also { inMemoryDb = it }

        val quest = buildQuest("not_started")
        // Insert synchronously via allowMainThreadQueries
        kotlinx.coroutines.runBlocking { db.questDao().upsert(quest) }

        val savedStateHandle = SavedStateHandle(mapOf("id" to quest.id))
        val powerSyncDb = mockk<PowerSyncDatabase>(relaxed = true)
        val viewModel = QuestDetailViewModel(savedStateHandle, db.questDao(), powerSyncDb)

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

        // "In progress" is the capitalised display of the "in_progress" transition for not_started
        composeTestRule.onNodeWithText("In progress").assertIsDisplayed()
    }

    /**
     * S025 / FA-004: for a not_started quest, the "Completed" transition button must NOT be shown
     * (rendered via real ViewModel backed by in-memory Room DAO).
     */
    @Test
    fun questDetail_notStarted_doesNotShowCompleteAction() {
        val context: Context = ApplicationProvider.getApplicationContext()
        val db =
            Room
                .inMemoryDatabaseBuilder(context, AltairDatabase::class.java)
                .allowMainThreadQueries()
                .build()
                .also { inMemoryDb = it }

        val quest = buildQuest("not_started")
        kotlinx.coroutines.runBlocking { db.questDao().upsert(quest) }

        val savedStateHandle = SavedStateHandle(mapOf("id" to quest.id))
        val powerSyncDb = mockk<PowerSyncDatabase>(relaxed = true)
        val viewModel = QuestDetailViewModel(savedStateHandle, db.questDao(), powerSyncDb)

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

        // "Completed" is only a valid transition from in_progress, not from not_started
        composeTestRule.onAllNodesWithText("Completed").assertCountEquals(0)
    }
}
