package com.getaltair.altair.data.local.dao

import android.content.Context
import androidx.arch.core.executor.testing.InstantTaskExecutorRule
import androidx.room.Room
import androidx.test.core.app.ApplicationProvider
import app.cash.turbine.test
import com.getaltair.altair.data.local.AltairDatabase
import com.getaltair.altair.data.local.entity.EntityRelationEntity
import kotlinx.coroutines.test.runTest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import org.robolectric.annotation.Config

@RunWith(RobolectricTestRunner::class)
@Config(sdk = [34], application = android.app.Application::class)
class EntityRelationDaoTest {
    @get:Rule
    val instantTaskExecutorRule = InstantTaskExecutorRule()

    private lateinit var db: AltairDatabase
    private lateinit var entityRelationDao: EntityRelationDao

    @Before
    fun setup() {
        val context: Context = ApplicationProvider.getApplicationContext()
        db =
            Room
                .inMemoryDatabaseBuilder(context, AltairDatabase::class.java)
                .allowMainThreadQueries()
                .build()
        entityRelationDao = db.entityRelationDao()
    }

    @After
    fun teardown() {
        db.close()
    }

    /**
     * watchBacklinksForNote returns only relations where to_entity_id matches the target
     * AND relation_type is 'note_link'. Relations pointing to a different target id are excluded.
     */
    @Test
    fun entityRelationDao_watchBacklinksForNote_returnsOnlyMatchingTargetId() =
        runTest {
            val matchingLink = makeRelation("r1", toEntityId = "note-target", relationType = "note_link")
            val wrongTargetLink = makeRelation("r2", toEntityId = "note-other", relationType = "note_link")
            val wrongTypeLink = makeRelation("r3", toEntityId = "note-target", relationType = "tag_link")

            entityRelationDao.upsert(matchingLink)
            entityRelationDao.upsert(wrongTargetLink)
            entityRelationDao.upsert(wrongTypeLink)

            entityRelationDao.watchBacklinksForNote("note-target").test {
                val items = awaitItem()
                assertEquals(1, items.size)
                assertEquals("r1", items[0].id)
                assertEquals("note-target", items[0].toEntityId)
                assertEquals("note_link", items[0].relationType)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * Soft-deleted backlinks (deleted_at non-null) are excluded from watchBacklinksForNote results.
     */
    @Test
    fun entityRelationDao_watchBacklinksForNote_excludesSoftDeleted() =
        runTest {
            val activeLink = makeRelation("r1", toEntityId = "note-target", relationType = "note_link")
            val deletedLink =
                makeRelation("r2", toEntityId = "note-target", relationType = "note_link")
                    .copy(deletedAt = "2026-01-02T00:00:00Z")

            entityRelationDao.upsert(activeLink)
            entityRelationDao.upsert(deletedLink)

            entityRelationDao.watchBacklinksForNote("note-target").test {
                val items = awaitItem()
                assertEquals(1, items.size)
                assertEquals("r1", items[0].id)
                cancelAndIgnoreRemainingEvents()
            }
        }

    /**
     * When multiple note_link relations point to the same target from different sources,
     * all are returned.
     */
    @Test
    fun entityRelationDao_watchBacklinksForNote_returnsAllMatchingBacklinks() =
        runTest {
            val link1 = makeRelation("r1", toEntityId = "note-target", relationType = "note_link", fromEntityId = "note-src-a")
            val link2 = makeRelation("r2", toEntityId = "note-target", relationType = "note_link", fromEntityId = "note-src-b")
            val link3 = makeRelation("r3", toEntityId = "note-other", relationType = "note_link", fromEntityId = "note-src-a")

            entityRelationDao.upsert(link1)
            entityRelationDao.upsert(link2)
            entityRelationDao.upsert(link3)

            entityRelationDao.watchBacklinksForNote("note-target").test {
                val items = awaitItem()
                assertEquals(2, items.size)
                assertTrue(items.all { it.toEntityId == "note-target" })
                cancelAndIgnoreRemainingEvents()
            }
        }

    // ─── Helpers ──────────────────────────────────────────────────────────────

    private fun makeRelation(
        id: String,
        toEntityId: String,
        relationType: String,
        fromEntityId: String = "note-src",
    ) = EntityRelationEntity(
        id = id,
        fromEntityType = "note",
        fromEntityId = fromEntityId,
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
}
