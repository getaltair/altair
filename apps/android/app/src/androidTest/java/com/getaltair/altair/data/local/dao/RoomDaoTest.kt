package com.getaltair.altair.data.local.dao

import android.content.Context
import androidx.arch.core.executor.testing.InstantTaskExecutorRule
import androidx.room.Room
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import app.cash.turbine.test
import com.getaltair.altair.data.local.AltairDatabase
import com.getaltair.altair.data.local.entity.EntityRelationEntity
import com.getaltair.altair.data.local.entity.QuestEntity
import com.getaltair.altair.data.local.entity.TrackingItemEntity
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class RoomDaoTest {
    @get:Rule
    val instantTaskRule = InstantTaskExecutorRule()

    private lateinit var db: AltairDatabase

    @Before
    fun setUp() {
        val context: Context = ApplicationProvider.getApplicationContext()
        db =
            Room
                .inMemoryDatabaseBuilder(context, AltairDatabase::class.java)
                .allowMainThreadQueries()
                .build()
    }

    @After
    fun tearDown() {
        db.close()
    }

    // ─── QuestDao ─────────────────────────────────────────────────────────────

    @Test
    fun questDao_watchAll_returnsOnlyMatchingUserId() =
        runTest {
            val user1Quest1 = makeQuest("q1", "user-1")
            val user1Quest2 = makeQuest("q2", "user-1")
            val user2Quest = makeQuest("q3", "user-2")

            db.questDao().upsert(user1Quest1)
            db.questDao().upsert(user1Quest2)
            db.questDao().upsert(user2Quest)

            db.questDao().watchAll("user-1").test {
                val items = awaitItem()
                assertEquals(2, items.size)
                assert(items.all { it.userId == "user-1" }) {
                    "Expected only user-1 quests, got: ${items.map { it.userId }}"
                }
                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── EntityRelationDao ────────────────────────────────────────────────────

    @Test
    fun entityRelationDao_watchBacklinksForNote_returnsOnlyNoteLinks() =
        runTest {
            val noteLink = makeRelation("r1", "note-1", "note_link")
            val tagLink = makeRelation("r2", "note-1", "tag_link")

            db.entityRelationDao().upsert(noteLink)
            db.entityRelationDao().upsert(tagLink)

            db.entityRelationDao().watchBacklinksForNote("note-1").test {
                val items = awaitItem()
                assertEquals(1, items.size)
                assertEquals("note_link", items[0].relationType)
                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── TrackingItemDao ──────────────────────────────────────────────────────

    @Test
    fun trackingItemDao_upsertAndWatchById_returnsUpdatedQuantity() =
        runTest {
            val item = makeTrackingItem("item-1", quantity = 5.0)
            db.trackingItemDao().upsert(item)

            val updated = item.copy(quantity = 3.0)
            db.trackingItemDao().upsert(updated)

            db.trackingItemDao().watchById("item-1").test {
                val first = awaitItem()
                assertEquals(3.0, first?.quantity)
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

    private fun makeRelation(
        id: String,
        toEntityId: String,
        relationType: String,
    ) = EntityRelationEntity(
        id = id,
        fromEntityType = "note",
        fromEntityId = "note-src",
        toEntityType = "note",
        toEntityId = toEntityId,
        relationType = relationType,
        sourceType = "manual",
        status = "accepted",
        confidence = null,
        evidence = null,
        userId = "user-1",
        createdAt = "2026-01-01T00:00:00Z",
        updatedAt = "2026-01-01T00:00:00Z",
        deletedAt = null,
    )

    private fun makeTrackingItem(
        id: String,
        quantity: Double,
    ) = TrackingItemEntity(
        id = id,
        name = "Item $id",
        description = null,
        quantity = quantity,
        barcode = null,
        locationId = null,
        categoryId = null,
        userId = "user-1",
        householdId = "hh-1",
        initiativeId = null,
        expiresAt = null,
        createdAt = "2026-01-01T00:00:00Z",
        updatedAt = "2026-01-01T00:00:00Z",
        deletedAt = null,
    )
}
