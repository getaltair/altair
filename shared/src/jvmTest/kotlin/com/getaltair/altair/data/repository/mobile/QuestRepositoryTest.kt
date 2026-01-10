package com.getaltair.altair.data.repository.mobile

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.getaltair.altair.data.entity.mobile.Quest
import com.getaltair.altair.database.AltairDatabase
import kotlinx.coroutines.test.runTest
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Integration tests for QuestRepositoryImpl.
 *
 * Tests CRUD operations using an in-memory SQLite database.
 * Follows TDD RED-GREEN-REFACTOR pattern.
 */
class QuestRepositoryTest {

    private lateinit var driver: JdbcSqliteDriver
    private lateinit var database: AltairDatabase
    private lateinit var repository: QuestRepositoryImpl

    @BeforeTest
    fun setUp() {
        driver = JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        AltairDatabase.Schema.create(driver)
        database = AltairDatabase(driver)
        repository = QuestRepositoryImpl(database)
    }

    @AfterTest
    fun tearDown() {
        driver.close()
    }

    @Test
    fun `create quest generates ID and timestamps`() = runTest {
        val quest = Quest(
            id = "",
            title = "Test Quest",
            description = "A test quest description",
            status = Quest.STATUS_PENDING,
            createdAt = "",
            updatedAt = "",
        )

        val created = repository.create(quest)

        assertNotNull(created.id)
        assertTrue(created.id.isNotBlank())
        assertEquals("Test Quest", created.title)
        assertEquals("A test quest description", created.description)
        assertEquals(Quest.STATUS_PENDING, created.status)
        assertNotNull(created.createdAt)
        assertNotNull(created.updatedAt)
        assertNull(created.deletedAt)
        assertEquals(0, created.syncVersion)
    }

    @Test
    fun `create quest with existing ID preserves ID`() = runTest {
        val existingId = "01HWXYZ123456789ABCDEFGH"
        val quest = Quest(
            id = existingId,
            title = "Quest with ID",
            createdAt = "",
            updatedAt = "",
        )

        val created = repository.create(quest)

        assertEquals(existingId, created.id)
    }

    @Test
    fun `findById returns quest when exists`() = runTest {
        val quest = Quest(
            id = "",
            title = "Find Me Quest",
            description = "Description",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(quest)

        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(created.id, found.id)
        assertEquals("Find Me Quest", found.title)
    }

    @Test
    fun `findById returns null when not exists`() = runTest {
        val found = repository.findById("nonexistent-id")

        assertNull(found)
    }

    @Test
    fun `update quest changes values and increments syncVersion`() = runTest {
        val quest = Quest(
            id = "",
            title = "Original Title",
            description = "Original Description",
            status = Quest.STATUS_PENDING,
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(quest)

        val updated = repository.update(
            created.copy(
                title = "Updated Title",
                description = "Updated Description",
                status = Quest.STATUS_IN_PROGRESS,
            ),
        )

        assertEquals("Updated Title", updated.title)
        assertEquals("Updated Description", updated.description)
        assertEquals(Quest.STATUS_IN_PROGRESS, updated.status)
        assertEquals(1, updated.syncVersion)

        // Verify persisted
        val found = repository.findById(created.id)
        assertNotNull(found)
        assertEquals("Updated Title", found.title)
        assertEquals(1, found.syncVersion)
    }

    @Test
    fun `delete performs soft delete`() = runTest {
        val quest = Quest(
            id = "",
            title = "Delete Me Quest",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(quest)

        val result = repository.delete(created.id)

        assertTrue(result)

        // Quest should still exist but be marked as deleted
        val found = repository.findById(created.id)
        assertNotNull(found)
        assertNotNull(found.deletedAt)
        assertTrue(found.isDeleted)
    }

    @Test
    fun `delete returns false for nonexistent quest`() = runTest {
        val result = repository.delete("nonexistent-id")

        assertFalse(result)
    }

    @Test
    fun `delete returns false for already deleted quest`() = runTest {
        val quest = Quest(
            id = "",
            title = "Delete Twice Quest",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(quest)

        repository.delete(created.id)
        val result = repository.delete(created.id)

        assertFalse(result)
    }

    @Test
    fun `findAll excludes soft-deleted quests`() = runTest {
        val quest1 = repository.create(
            Quest(id = "", title = "Quest 1", createdAt = "", updatedAt = ""),
        )
        val quest2 = repository.create(
            Quest(id = "", title = "Quest 2", createdAt = "", updatedAt = ""),
        )
        repository.create(
            Quest(id = "", title = "Quest 3", createdAt = "", updatedAt = ""),
        )

        // Delete quest2
        repository.delete(quest2.id)

        val all = repository.findAll()

        assertEquals(2, all.size)
        assertTrue(all.any { it.title == "Quest 1" })
        assertTrue(all.any { it.title == "Quest 3" })
        assertFalse(all.any { it.title == "Quest 2" })
    }

    @Test
    fun `findAll returns empty list when no quests`() = runTest {
        val all = repository.findAll()

        assertTrue(all.isEmpty())
    }

    @Test
    fun `quest with epicId is stored correctly`() = runTest {
        val epicId = "01HWXYZ123EPIC789ABCDEFGH"
        val quest = Quest(
            id = "",
            title = "Quest with Epic",
            epicId = epicId,
            createdAt = "",
            updatedAt = "",
        )

        val created = repository.create(quest)
        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(epicId, found.epicId)
    }
}
