package com.getaltair.altair.ui.knowledge

import androidx.lifecycle.SavedStateHandle
import app.cash.turbine.test
import com.getaltair.altair.data.auth.TokenPreferences
import com.getaltair.altair.data.local.dao.EntityRelationDao
import com.getaltair.altair.data.local.dao.NoteDao
import com.getaltair.altair.data.local.dao.NoteSnapshotDao
import com.getaltair.altair.data.local.entity.EntityRelationEntity
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

/**
 * Unit tests for [NoteDetailViewModel], covering FA-010:
 * the backlinks flow is populated from EntityRelationDao.watchBacklinksForNote().
 */
@OptIn(ExperimentalCoroutinesApi::class)
class NoteDetailViewModelTest {
    private val testDispatcher = UnconfinedTestDispatcher()

    private lateinit var savedStateHandle: SavedStateHandle
    private lateinit var noteDao: NoteDao
    private lateinit var entityRelationDao: EntityRelationDao
    private lateinit var noteSnapshotDao: NoteSnapshotDao
    private lateinit var db: PowerSyncDatabase
    private lateinit var tokenPreferences: TokenPreferences

    @BeforeEach
    fun setUp() {
        Dispatchers.setMain(testDispatcher)

        savedStateHandle = mockk()
        noteDao = mockk(relaxed = true)
        entityRelationDao = mockk(relaxed = true)
        noteSnapshotDao = mockk(relaxed = true)
        db = mockk(relaxed = true)
        tokenPreferences = mockk(relaxed = true)

        every { savedStateHandle.get<String>("id") } returns "note-1"
        every { noteDao.watchById(any()) } returns flowOf(null)
        every { entityRelationDao.watchBacklinksForNote(any()) } returns flowOf(emptyList())
        every { noteSnapshotDao.watchAll(any()) } returns flowOf(emptyList())
        every { tokenPreferences.accessToken } returns null
    }

    @AfterEach
    fun tearDown() {
        Dispatchers.resetMain()
    }

    private fun makeViewModel() =
        NoteDetailViewModel(
            savedStateHandle = savedStateHandle,
            noteDao = noteDao,
            entityRelationDao = entityRelationDao,
            noteSnapshotDao = noteSnapshotDao,
            db = db,
            tokenPreferences = tokenPreferences,
        )

    /**
     * FA-010: When EntityRelationDao.watchBacklinksForNote("note-1") emits 2 items,
     * the ViewModel's backlinks StateFlow must expose exactly those 2 items.
     */
    @Test
    fun `backlinks_populatedFromDao - emits 2 fixture backlinks`() =
        runTest {
            val backlink1 = makeBacklink("rel-1", fromNoteId = "note-2", toNoteId = "note-1")
            val backlink2 = makeBacklink("rel-2", fromNoteId = "note-3", toNoteId = "note-1")

            val backlinksFlow = MutableStateFlow(listOf(backlink1, backlink2))
            every { entityRelationDao.watchBacklinksForNote("note-1") } returns backlinksFlow

            val viewModel = makeViewModel()
            advanceUntilIdle()

            viewModel.backlinks.test {
                val items = awaitItem()
                assertEquals(2, items.size, "Expected 2 backlinks from dao")
                assertEquals("rel-1", items[0].id)
                assertEquals("rel-2", items[1].id)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * FA-010: When the DAO emits an empty list, backlinks is empty.
     */
    @Test
    fun `backlinks_empty - emits empty list when no backlinks exist`() =
        runTest {
            every { entityRelationDao.watchBacklinksForNote("note-1") } returns flowOf(emptyList())

            val viewModel = makeViewModel()
            advanceUntilIdle()

            viewModel.backlinks.test {
                val items = awaitItem()
                assertEquals(0, items.size)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * FA-010: Backlinks update reactively when the DAO flow emits a new list.
     */
    @Test
    fun `backlinks_reactiveUpdate - reflects dao emission changes`() =
        runTest {
            val backlinksFlow = MutableStateFlow<List<EntityRelationEntity>>(emptyList())
            every { entityRelationDao.watchBacklinksForNote("note-1") } returns backlinksFlow

            val viewModel = makeViewModel()
            advanceUntilIdle()

            viewModel.backlinks.test {
                // Initial empty state
                assertEquals(0, awaitItem().size)

                // DAO emits a new backlink
                backlinksFlow.value = listOf(makeBacklink("rel-1", "note-2", "note-1"))
                val updated = awaitItem()
                assertEquals(1, updated.size)
                assertEquals("rel-1", updated[0].id)

                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── Helpers ──────────────────────────────────────────────────────────────

    private fun makeBacklink(
        id: String,
        fromNoteId: String,
        toNoteId: String,
    ) = EntityRelationEntity(
        id = id,
        fromEntityType = "note",
        fromEntityId = fromNoteId,
        toEntityType = "note",
        toEntityId = toNoteId,
        relationType = "note_link",
        sourceType = "manual",
        status = "active",
        confidence = null,
        evidence = null,
        userId = "user-1",
        createdAt = "2026-01-01T00:00:00Z",
        updatedAt = "2026-01-01T00:00:00Z",
        deletedAt = null,
    )
}
