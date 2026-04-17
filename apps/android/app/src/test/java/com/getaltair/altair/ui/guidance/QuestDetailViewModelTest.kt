package com.getaltair.altair.ui.guidance

import androidx.lifecycle.SavedStateHandle
import com.getaltair.altair.data.local.dao.QuestDao
import com.powersync.PowerSyncDatabase
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.resetMain
import kotlinx.coroutines.test.setMain
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertFalse
import org.junit.jupiter.api.Assertions.assertTrue
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test

/**
 * Unit tests for [QuestDetailViewModel], covering FA-004:
 * only valid status transitions per 06-state-machines.md are exposed.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class QuestDetailViewModelTest {
    private val testDispatcher = StandardTestDispatcher()

    private lateinit var savedStateHandle: SavedStateHandle
    private lateinit var questDao: QuestDao
    private lateinit var db: PowerSyncDatabase
    private lateinit var viewModel: QuestDetailViewModel

    @BeforeEach
    fun setUp() {
        Dispatchers.setMain(testDispatcher)

        savedStateHandle = SavedStateHandle(mapOf("id" to "quest-1"))
        questDao = mockk(relaxed = true)
        db = mockk(relaxed = true)
        every { questDao.watchById(any()) } returns flowOf(null)

        viewModel = QuestDetailViewModel(savedStateHandle = savedStateHandle, questDao = questDao, db = db)
    }

    @AfterEach
    fun tearDown() {
        Dispatchers.resetMain()
    }

    /**
     * FA-004: A quest with status "not_started" must expose "Start" (in_progress)
     * as a valid transition and must NOT expose "Complete" (completed).
     */
    @Test
    fun `validTransitions_notStarted - exposes in_progress but not completed`() {
        val transitions = viewModel.validTransitions("not_started")

        assertTrue(
            transitions.contains("in_progress"),
            "not_started quest must allow transition to in_progress (Start action)",
        )
        assertFalse(
            transitions.contains("completed"),
            "not_started quest must NOT allow direct transition to completed",
        )
    }

    /**
     * FA-004: A quest with status "in_progress" must expose "completed"
     * as a valid transition (Complete action is available).
     */
    @Test
    fun `validTransitions_inProgress - exposes completed action`() {
        val transitions = viewModel.validTransitions("in_progress")

        assertTrue(
            transitions.contains("completed"),
            "in_progress quest must allow transition to completed (Complete action)",
        )
    }

    /**
     * FA-004: A quest with status "completed" has no valid transitions
     * (terminal state — no actions available).
     */
    @Test
    fun `validTransitions_completed - returns empty (terminal state)`() {
        val transitions = viewModel.validTransitions("completed")

        assertTrue(
            transitions.isEmpty(),
            "completed quest is terminal and must have no available transitions",
        )
    }

    /**
     * FA-004: A quest with status "cancelled" has no valid transitions
     * (terminal state — no actions available).
     */
    @Test
    fun `validTransitions_cancelled - returns empty (terminal state)`() {
        val transitions = viewModel.validTransitions("cancelled")

        assertTrue(
            transitions.isEmpty(),
            "cancelled quest is terminal and must have no available transitions",
        )
    }

    /**
     * FA-004: A quest with status "deferred" can return to not_started or in_progress,
     * but not completed directly.
     */
    @Test
    fun `validTransitions_deferred - allows resumption but not direct completion`() {
        val transitions = viewModel.validTransitions("deferred")

        assertTrue(transitions.contains("not_started"), "deferred quest can go back to not_started")
        assertTrue(transitions.contains("in_progress"), "deferred quest can resume to in_progress")
        assertFalse(transitions.contains("completed"), "deferred quest must not skip directly to completed")
    }
}
