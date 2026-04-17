package com.getaltair.altair.ui.guidance

import android.content.Context
import androidx.arch.core.executor.testing.InstantTaskExecutorRule
import androidx.lifecycle.SavedStateHandle
import androidx.room.Room
import androidx.test.core.app.ApplicationProvider
import app.cash.turbine.test
import com.getaltair.altair.data.local.AltairDatabase
import com.getaltair.altair.data.local.dao.EpicDao
import com.getaltair.altair.data.local.dao.InitiativeDao
import com.getaltair.altair.data.local.entity.EpicEntity
import com.getaltair.altair.data.local.entity.InitiativeEntity
import com.getaltair.altair.ui.UiState
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.awaitCancellation
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
 * Unit tests for [InitiativeDetailViewModel], covering UiState transitions per Feature 010.
 *
 * Most tests use an in-memory Room database so queries execute against real SQL.
 * The exception is the DAO-exception test, which uses a mockk DAO to trigger the
 * `.catch { }` error path that cannot be provoked through a real Room DAO.
 */
@OptIn(ExperimentalCoroutinesApi::class)
@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34], application = android.app.Application::class)
class InitiativeDetailViewModelTest {
    @get:Rule
    val instantTaskExecutorRule = InstantTaskExecutorRule()

    private val testDispatcher = UnconfinedTestDispatcher()

    private lateinit var db: AltairDatabase
    private lateinit var initiativeDao: InitiativeDao
    private lateinit var epicDao: EpicDao

    @Before
    fun setUp() {
        Dispatchers.setMain(testDispatcher)
        val context: Context = ApplicationProvider.getApplicationContext()
        db =
            Room
                .inMemoryDatabaseBuilder(context, AltairDatabase::class.java)
                .allowMainThreadQueries()
                .build()
        initiativeDao = db.initiativeDao()
        epicDao = db.epicDao()
    }

    @After
    fun tearDown() {
        db.close()
        Dispatchers.resetMain()
    }

    // ─── initiative StateFlow ──────────────────────────────────────────────────

    /**
     * The initial value of `initiative` must be [UiState.Loading] before the Room
     * flow has emitted anything.
     *
     * `stateIn` uses [UiState.Loading] as the initial value, so the first item
     * collected from the StateFlow is always Loading regardless of DB state.
     */
    @Test
    fun initiative_emitsLoading_initially() =
        runTest {
            val neverInitiativeDao = mockk<InitiativeDao>()
            every { neverInitiativeDao.watchById(any()) } returns flow { awaitCancellation() }
            val neverEpicDao = mockk<EpicDao>()
            every { neverEpicDao.watchByInitiativeId(any()) } returns flow { awaitCancellation() }
            val vm =
                InitiativeDetailViewModel(
                    savedStateHandle = SavedStateHandle(mapOf("id" to "init-1")),
                    initiativeDao = neverInitiativeDao,
                    epicDao = neverEpicDao,
                )

            vm.initiative.test {
                assertTrue(
                    "First emission must be UiState.Loading",
                    awaitItem() is UiState.Loading,
                )
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * When an [InitiativeEntity] with the matching id is present in the database,
     * the `initiative` flow must emit [UiState.Success] containing that entity.
     */
    @Test
    fun initiative_emitsSuccess_whenEntityPresent() =
        runTest {
            val entity = makeInitiative("init-1")
            initiativeDao.upsert(entity)

            val vm = buildViewModel(initiativeId = "init-1")

            vm.initiative.test {
                // Discard Loading (stateIn initial value)
                val first = awaitItem()
                val success =
                    if (first is UiState.Success) {
                        first
                    } else {
                        // Loading → Success
                        awaitItem() as UiState.Success
                    }
                assertEquals("init-1", success.data?.id)
                assertEquals("Initiative init-1", success.data?.title)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * When the DAO's `watchById` flow throws an exception, the `initiative` flow
     * must emit [UiState.Error] with a non-blank message.
     *
     * This path exercises the `.catch { emit(UiState.Error(…)) }` operator in the
     * ViewModel.  A real Room DAO cannot be made to throw after subscribe, so a
     * mockk DAO is used here.
     */
    @Test
    fun initiative_emitsError_onDaoException() =
        runTest {
            val failingInitiativeDao = mockk<InitiativeDao>()
            every { failingInitiativeDao.watchById(any()) } returns
                flow { throw RuntimeException("DB failure") }

            val failingEpicDao = mockk<EpicDao>()
            every { failingEpicDao.watchByInitiativeId(any()) } returns flowOf(emptyList())

            val vm =
                InitiativeDetailViewModel(
                    savedStateHandle = SavedStateHandle(mapOf("id" to "init-1")),
                    initiativeDao = failingInitiativeDao,
                    epicDao = failingEpicDao,
                )

            vm.initiative.test {
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

    /**
     * When the requested id is not present in the database, [InitiativeDao.watchById]
     * emits `null`.  The ViewModel maps null to [UiState.Error] ("Not found").
     */
    @Test
    fun initiative_emitsError_whenIdNotFound() =
        runTest {
            // No entity inserted — DB is empty
            val vm = buildViewModel(initiativeId = "nonexistent-id")

            vm.initiative.test {
                val first = awaitItem()
                val error =
                    if (first is UiState.Error) {
                        first
                    } else {
                        awaitItem() as UiState.Error
                    }
                assertTrue(
                    "Error message must not be blank when entity does not exist",
                    error.message.isNotBlank(),
                )
                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── epics StateFlow ───────────────────────────────────────────────────────

    /**
     * When epics with a matching `initiative_id` are present in the database,
     * the `epics` flow must emit [UiState.Success] containing those epics.
     */
    @Test
    fun epics_emitsSuccess_withList() =
        runTest {
            val initiative = makeInitiative("init-1")
            initiativeDao.upsert(initiative)
            epicDao.upsert(makeEpic("epic-1", initiativeId = "init-1"))
            epicDao.upsert(makeEpic("epic-2", initiativeId = "init-1"))
            // Epic for a different initiative — must not appear
            epicDao.upsert(makeEpic("epic-3", initiativeId = "init-other"))

            val vm = buildViewModel(initiativeId = "init-1")

            vm.epics.test {
                val first = awaitItem()
                val success =
                    if (first is UiState.Success) {
                        first
                    } else {
                        awaitItem() as UiState.Success
                    }
                assertEquals(
                    "Only epics belonging to init-1 must be returned",
                    2,
                    success.data.size,
                )
                assertTrue(success.data.all { it.initiativeId == "init-1" })
                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── Helpers ──────────────────────────────────────────────────────────────

    private fun buildViewModel(initiativeId: String): InitiativeDetailViewModel =
        InitiativeDetailViewModel(
            savedStateHandle = SavedStateHandle(mapOf("id" to initiativeId)),
            initiativeDao = initiativeDao,
            epicDao = epicDao,
        )

    private fun makeInitiative(id: String) =
        InitiativeEntity(
            id = id,
            title = "Initiative $id",
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
        title = "Epic $id",
        description = null,
        status = "active",
        sortOrder = 0,
        userId = "user-1",
        createdAt = "2026-01-01T00:00:00Z",
        updatedAt = "2026-01-01T00:00:00Z",
        deletedAt = null,
    )
}
