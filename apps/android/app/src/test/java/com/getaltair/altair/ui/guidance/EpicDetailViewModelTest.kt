package com.getaltair.altair.ui.guidance

import android.content.Context
import androidx.arch.core.executor.testing.InstantTaskExecutorRule
import androidx.lifecycle.SavedStateHandle
import androidx.room.Room
import androidx.test.core.app.ApplicationProvider
import app.cash.turbine.test
import com.getaltair.altair.data.local.AltairDatabase
import com.getaltair.altair.data.local.dao.EpicDao
import com.getaltair.altair.data.local.dao.QuestDao
import com.getaltair.altair.data.local.entity.EpicEntity
import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.ui.UiState
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.flow.flow
import kotlinx.coroutines.flow.flowOf
import kotlinx.coroutines.test.UnconfinedTestDispatcher
import kotlinx.coroutines.test.resetMain
import kotlinx.coroutines.test.runTest
import kotlinx.coroutines.test.setMain
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

/**
 * Unit tests for [EpicDetailViewModel], covering UiState transitions per Feature 010.
 *
 * Most tests use an in-memory Room database so queries execute against real SQL.
 * The exception is the DAO-exception test, which uses a mockk DAO to trigger the
 * `.catch { }` error path that cannot be provoked through a real Room DAO.
 */
@OptIn(ExperimentalCoroutinesApi::class)
@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34], application = android.app.Application::class)
class EpicDetailViewModelTest {
    @get:Rule
    val instantTaskExecutorRule = InstantTaskExecutorRule()

    private val testDispatcher = UnconfinedTestDispatcher()

    private lateinit var db: AltairDatabase
    private lateinit var epicDao: EpicDao
    private lateinit var questDao: QuestDao

    @Before
    fun setUp() {
        Dispatchers.setMain(testDispatcher)
        val context: Context = ApplicationProvider.getApplicationContext()
        db =
            Room
                .inMemoryDatabaseBuilder(context, AltairDatabase::class.java)
                .allowMainThreadQueries()
                .build()
        epicDao = db.epicDao()
        questDao = db.questDao()
    }

    @After
    fun tearDown() {
        db.close()
        Dispatchers.resetMain()
    }

    // ─── epic StateFlow ────────────────────────────────────────────────────────

    /**
     * The initial value of `epic` must be [UiState.Loading] before the Room flow
     * has emitted anything.
     *
     * `stateIn` uses [UiState.Loading] as the initial value, so the first item
     * collected from the StateFlow is always Loading regardless of DB state.
     */
    @Test
    fun epic_emitsLoading_initially() =
        runTest {
            val vm = buildViewModel(epicId = "epic-1")

            vm.epic.test {
                assertTrue(
                    "First emission must be UiState.Loading",
                    awaitItem() is UiState.Loading,
                )
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * When an [EpicEntity] with the matching id is present in the database,
     * the `epic` flow must emit [UiState.Success] containing that entity.
     */
    @Test
    fun epic_emitsSuccess_whenEntityPresent() =
        runTest {
            val entity = makeEpic("epic-1")
            epicDao.upsert(entity)

            val vm = buildViewModel(epicId = "epic-1")

            vm.epic.test {
                val first = awaitItem()
                val success =
                    if (first is UiState.Success) {
                        first
                    } else {
                        // Loading → Success
                        awaitItem() as UiState.Success
                    }
                assertEquals("epic-1", success.data?.id)
                assertEquals("Epic epic-1", success.data?.title)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * When the DAO's `watchById` flow throws an exception, the `epic` flow
     * must emit [UiState.Error] with a non-blank message.
     *
     * This path exercises the `.catch { emit(UiState.Error(…)) }` operator in the
     * ViewModel.  A real Room DAO cannot be made to throw after subscribe, so a
     * mockk DAO is used here.
     */
    @Test
    fun epic_emitsError_onDaoException() =
        runTest {
            val failingEpicDao = mockk<EpicDao>()
            every { failingEpicDao.watchById(any()) } returns
                flow { throw RuntimeException("DB failure") }

            val failingQuestDao = mockk<QuestDao>()
            every { failingQuestDao.watchByEpicId(any()) } returns flowOf(emptyList())

            val vm =
                EpicDetailViewModel(
                    savedStateHandle = SavedStateHandle(mapOf("id" to "epic-1")),
                    epicDao = failingEpicDao,
                    questDao = failingQuestDao,
                )

            vm.epic.test {
                val first = awaitItem()
                val error =
                    if (first is UiState.Error) {
                        first
                    } else {
                        awaitItem() as UiState.Error
                    }
                assertTrue(
                    "Error message must not be blank",
                    error.message.isNotBlank(),
                )
                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── quests StateFlow ──────────────────────────────────────────────────────

    /**
     * When quests with a matching `epic_id` are present in the database,
     * the `quests` flow must emit [UiState.Success] containing those quests.
     * Quests belonging to a different epic must not appear.
     */
    @Test
    fun quests_emitsSuccess_withList() =
        runTest {
            epicDao.upsert(makeEpic("epic-1"))
            questDao.upsert(makeQuest("quest-1", epicId = "epic-1"))
            questDao.upsert(makeQuest("quest-2", epicId = "epic-1"))
            // Quest for a different epic — must not appear
            questDao.upsert(makeQuest("quest-3", epicId = "epic-other"))

            val vm = buildViewModel(epicId = "epic-1")

            vm.quests.test {
                val first = awaitItem()
                val success =
                    if (first is UiState.Success) {
                        first
                    } else {
                        awaitItem() as UiState.Success
                    }
                assertEquals(
                    "Only quests belonging to epic-1 must be returned",
                    2,
                    success.data.size,
                )
                assertTrue(success.data.all { it.epicId == "epic-1" })
                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── Helpers ──────────────────────────────────────────────────────────────

    private fun buildViewModel(epicId: String): EpicDetailViewModel =
        EpicDetailViewModel(
            savedStateHandle = SavedStateHandle(mapOf("id" to epicId)),
            epicDao = epicDao,
            questDao = questDao,
        )

    private fun makeEpic(id: String) =
        EpicEntity(
            id = id,
            initiativeId = null,
            title = "Epic $id",
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
        title = "Quest $id",
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
