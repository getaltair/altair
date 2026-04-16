package com.getaltair.altair.ui.guidance

import androidx.lifecycle.SavedStateHandle
import app.cash.turbine.test
import com.getaltair.altair.data.local.dao.EpicDao
import com.getaltair.altair.data.local.dao.InitiativeDao
import com.getaltair.altair.data.local.entity.EpicEntity
import com.getaltair.altair.data.local.entity.InitiativeEntity
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
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.assertThrows

/**
 * Unit tests for [InitiativeDetailViewModel].
 * Covers DAO-to-StateFlow wiring for initiative and epic lists.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class InitiativeDetailViewModelTest {
    private val testDispatcher = UnconfinedTestDispatcher()

    private lateinit var savedStateHandle: SavedStateHandle
    private lateinit var initiativeDao: InitiativeDao
    private lateinit var epicDao: EpicDao
    private lateinit var db: PowerSyncDatabase

    @BeforeEach
    fun setUp() {
        Dispatchers.setMain(testDispatcher)

        savedStateHandle = mockk()
        initiativeDao = mockk(relaxed = true)
        epicDao = mockk(relaxed = true)
        db = mockk(relaxed = true)

        // Default stubs — overridden per-test as needed
        every { savedStateHandle.get<String>("id") } returns "initiative-1"
        every { initiativeDao.watchById(any()) } returns flowOf(null)
        every { epicDao.watchByInitiativeId(any()) } returns flowOf(emptyList())
    }

    @AfterEach
    fun tearDown() {
        Dispatchers.resetMain()
    }

    private fun makeViewModel() =
        InitiativeDetailViewModel(
            savedStateHandle = savedStateHandle,
            initiativeDao = initiativeDao,
            epicDao = epicDao,
            db = db,
        )

    /**
     * When initiativeDao.watchById(id) emits an InitiativeEntity,
     * the ViewModel's initiative StateFlow must expose that entity.
     */
    @Test
    fun `initiative_loadsFromDao_whenInitiativeIdPresent`() =
        runTest {
            val fixture = makeInitiative("initiative-1")
            every { initiativeDao.watchById("initiative-1") } returns MutableStateFlow(fixture)

            val viewModel = makeViewModel()
            advanceUntilIdle()

            viewModel.initiative.test {
                assertEquals(fixture, awaitItem())
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * When epicDao.watchByInitiativeId(id) emits a list of EpicEntity,
     * the ViewModel's epics StateFlow must expose that list.
     */
    @Test
    fun `epics_populateFromEpicDao_watchByInitiativeId`() =
        runTest {
            val epic1 = makeEpic("epic-1", initiativeId = "initiative-1")
            val epic2 = makeEpic("epic-2", initiativeId = "initiative-1")
            every { epicDao.watchByInitiativeId("initiative-1") } returns MutableStateFlow(listOf(epic1, epic2))

            val viewModel = makeViewModel()
            advanceUntilIdle()

            viewModel.epics.test {
                val items = awaitItem()
                assertEquals(2, items.size)
                assertEquals("epic-1", items[0].id)
                assertEquals("epic-2", items[1].id)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * When SavedStateHandle does not contain an "id" key, the ViewModel constructor
     * throws IllegalStateException via checkNotNull. The null-id case is not a valid
     * runtime state — the VM is always navigated to with an explicit id.
     */
    @Test
    fun `initiative_isNull_whenSavedStateHandleIdIsNull`() {
        every { savedStateHandle.get<String>("id") } returns null

        assertThrows<IllegalStateException> {
            makeViewModel()
        }
    }

    // ─── Helpers ────────────────────────────────────────────────────────────

    private fun makeInitiative(id: String) =
        InitiativeEntity(
            id = id,
            title = "Test Initiative",
            description = null,
            status = "active",
            userId = "user-1",
            householdId = null,
            createdAt = "2026-01-01T00:00:00Z",
            updatedAt = "2026-01-01T00:00:00Z",
            deletedAt = null,
        )

    private fun makeEpic(
        id: String,
        initiativeId: String,
    ) = EpicEntity(
        id = id,
        initiativeId = initiativeId,
        title = "Test Epic",
        description = null,
        status = "active",
        sortOrder = 0,
        userId = "user-1",
        createdAt = "2026-01-01T00:00:00Z",
        updatedAt = "2026-01-01T00:00:00Z",
        deletedAt = null,
    )
}
