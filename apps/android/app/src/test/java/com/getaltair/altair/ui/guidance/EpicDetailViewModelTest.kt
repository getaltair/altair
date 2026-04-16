package com.getaltair.altair.ui.guidance

import androidx.lifecycle.SavedStateHandle
import app.cash.turbine.test
import com.getaltair.altair.data.local.dao.EpicDao
import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.entity.EpicEntity
import com.getaltair.altair.data.local.entity.QuestEntity
import com.powersync.PowerSyncDatabase
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.test.UnconfinedTestDispatcher
import kotlinx.coroutines.test.advanceUntilIdle
import kotlinx.coroutines.test.resetMain
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.test.setMain
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

/**
 * Unit tests for [EpicDetailViewModel].
 * Covers DAO-to-StateFlow wiring for epic and quest lists.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class EpicDetailViewModelTest {
    private val testDispatcher = UnconfinedTestDispatcher()

    private lateinit var savedStateHandle: SavedStateHandle
    private lateinit var epicDao: EpicDao
    private lateinit var questDao: QuestDao
    private lateinit var db: PowerSyncDatabase

    @BeforeEach
    fun setUp() {
        Dispatchers.setMain(testDispatcher)

        savedStateHandle = mockk()
        epicDao = mockk(relaxed = true)
        questDao = mockk(relaxed = true)
        db = mockk(relaxed = true)

        // Default stubs — overridden per-test as needed
        every { savedStateHandle.get<String>("id") } returns "epic-1"
        every { epicDao.watchById(any()) } returns flowOf(null)
        every { questDao.watchByEpicId(any()) } returns flowOf(emptyList())
    }

    @AfterEach
    fun tearDown() {
        Dispatchers.resetMain()
    }

    private fun makeViewModel() =
        EpicDetailViewModel(
            savedStateHandle = savedStateHandle,
            epicDao = epicDao,
            questDao = questDao,
            db = db,
        )

    /**
     * When epicDao.watchById(id) emits an EpicEntity,
     * the ViewModel's epic StateFlow must expose that entity.
     */
    @Test
    fun `epic_loadsFromDao_whenEpicIdPresent`() =
        runTest {
            val fixture = makeEpic("epic-1")
            every { epicDao.watchById("epic-1") } returns MutableStateFlow(fixture)

            val viewModel = makeViewModel()
            advanceUntilIdle()

            viewModel.epic.test {
                assertEquals(fixture, awaitItem())
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * When questDao.watchByEpicId(id) emits a list of QuestEntity,
     * the ViewModel's quests StateFlow must expose that list.
     */
    @Test
    fun `quests_populateFromQuestDao_watchByEpicId`() =
        runTest {
            val quest1 = makeQuest("quest-1", epicId = "epic-1")
            val quest2 = makeQuest("quest-2", epicId = "epic-1")
            every { questDao.watchByEpicId("epic-1") } returns MutableStateFlow(listOf(quest1, quest2))

            val viewModel = makeViewModel()
            advanceUntilIdle()

            viewModel.quests.test {
                val items = awaitItem()
                assertEquals(2, items.size)
                assertEquals("quest-1", items[0].id)
                assertEquals("quest-2", items[1].id)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * When SavedStateHandle does not contain an "id" key, the ViewModel constructor
     * throws IllegalStateException via checkNotNull. The null-id case is not a valid
     * runtime state — the VM is always navigated to with an explicit id.
     */
    @Test
    fun `epic_isNull_whenSavedStateHandleIdIsNull`() {
        every { savedStateHandle.get<String>("id") } returns null

        assertThrows<IllegalStateException> {
            makeViewModel()
        }
    }

    // ─── Helpers ────────────────────────────────────────────────────────────

    private fun makeEpic(id: String) =
        EpicEntity(
            id = id,
            initiativeId = "initiative-1",
            title = "Test Epic",
            description = null,
            status = "active",
            sortOrder = 0,
            userId = "user-1",
            createdAt = "2026-01-01T00:00:00Z",
            updatedAt = "2026-01-01T00:00:00Z",
            deletedAt = null,
        )

    private fun makeQuest(
        id: String,
        epicId: String,
    ) = QuestEntity(
        id = id,
        title = "Test Quest",
        description = null,
        status = "not_started",
        priority = "medium",
        dueDate = null,
        epicId = epicId,
        initiativeId = null,
        routineId = null,
        userId = "user-1",
        createdAt = "2026-01-01T00:00:00Z",
        updatedAt = "2026-01-01T00:00:00Z",
        deletedAt = null,
    )
}
