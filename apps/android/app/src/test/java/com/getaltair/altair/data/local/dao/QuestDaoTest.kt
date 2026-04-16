package com.getaltair.altair.data.local.dao

import android.content.Context
import androidx.arch.core.executor.testing.InstantTaskExecutorRule
import androidx.room.Room
import androidx.test.core.app.ApplicationProvider
import app.cash.turbine.test
import com.getaltair.altair.data.local.AltairDatabase
import com.getaltair.altair.data.local.entity.QuestEntity
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34], application = android.app.Application::class)
class QuestDaoTest {
    @get:Rule
    val instantTaskExecutorRule = InstantTaskExecutorRule()

    private lateinit var db: AltairDatabase
    private lateinit var questDao: QuestDao

    @Before
    fun setup() {
        val context: Context = ApplicationProvider.getApplicationContext()
        db =
            Room
                .inMemoryDatabaseBuilder(context, AltairDatabase::class.java)
                .allowMainThreadQueries()
                .build()
        questDao = db.questDao()
    }

    @After
    fun teardown() {
        db.close()
    }

    /**
     * Verifies that watchAll emits the initial empty list and then re-emits after an upsert,
     * with the new item present in the second emission.
     */
    @Test
    fun questDao_flowEmitsOnUpsert() =
        runTest {
            questDao.watchAll("user-1").test {
                val initial = awaitItem()
                assertEquals(0, initial.size)

                questDao.upsert(makeQuest("q1", "user-1"))

                val afterInsert = awaitItem()
                assertEquals(1, afterInsert.size)
                assertEquals("q1", afterInsert[0].id)

                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * Verifies that a second upsert with the same id (replace) triggers another emission
     * reflecting the updated title.
     */
    @Test
    fun questDao_flowEmitsOnUpdate() =
        runTest {
            questDao.upsert(makeQuest("q1", "user-1"))

            questDao.watchAll("user-1").test {
                val initial = awaitItem()
                assertEquals("Quest q1", initial[0].title)

                questDao.upsert(makeQuest("q1", "user-1").copy(title = "Updated Title"))

                val afterUpdate = awaitItem()
                assertEquals("Updated Title", afterUpdate[0].title)

                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * Verifies that soft-deleted quests (deleted_at non-null) are excluded from watchAll.
     */
    @Test
    fun questDao_watchAll_excludesSoftDeleted() =
        runTest {
            questDao.upsert(makeQuest("q1", "user-1"))
            questDao.upsert(
                makeQuest("q2", "user-1").copy(deletedAt = "2026-01-02T00:00:00Z"),
            )

            questDao.watchAll("user-1").test {
                val items = awaitItem()
                assertEquals(1, items.size)
                assertEquals("q1", items[0].id)
                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── Helpers ──────────────────────────────────────────────────────────────

    private fun makeQuest(
        id: String,
        userId: String,
    ) = QuestEntity(
        id = id,
        title = "Quest $id",
        description = null,
        status = "not_started",
        priority = "medium",
        dueDate = null,
        epicId = null,
        initiativeId = null,
        routineId = null,
        userId = userId,
        createdAt = "2026-01-01T00:00:00Z",
        updatedAt = "2026-01-01T00:00:00Z",
        deletedAt = null,
    )
}
