package com.getaltair.altair.data.repository.mobile

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.getaltair.altair.data.entity.mobile.Note
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
 * Integration tests for NoteRepositoryImpl.
 *
 * Tests CRUD operations using an in-memory SQLite database.
 * Follows TDD RED-GREEN-REFACTOR pattern.
 */
class NoteRepositoryTest {

    private lateinit var driver: JdbcSqliteDriver
    private lateinit var database: AltairDatabase
    private lateinit var repository: NoteRepositoryImpl

    @BeforeTest
    fun setUp() {
        driver = JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        AltairDatabase.Schema.create(driver)
        database = AltairDatabase(driver)
        repository = NoteRepositoryImpl(database)
    }

    @AfterTest
    fun tearDown() {
        driver.close()
    }

    @Test
    fun `create note generates ID and timestamps`() = runTest {
        val note = Note(
            id = "",
            title = "Test Note",
            content = "Note content here",
            createdAt = "",
            updatedAt = "",
        )

        val created = repository.create(note)

        assertNotNull(created.id)
        assertTrue(created.id.isNotBlank())
        assertEquals("Test Note", created.title)
        assertEquals("Note content here", created.content)
        assertNotNull(created.createdAt)
        assertNotNull(created.updatedAt)
        assertNull(created.deletedAt)
        assertEquals(0, created.syncVersion)
    }

    @Test
    fun `create note with existing ID preserves ID`() = runTest {
        val existingId = "01HWNOTE123456789ABCDEFGH"
        val note = Note(
            id = existingId,
            title = "Note with ID",
            createdAt = "",
            updatedAt = "",
        )

        val created = repository.create(note)

        assertEquals(existingId, created.id)
    }

    @Test
    fun `findById returns note when exists`() = runTest {
        val note = Note(
            id = "",
            title = "Find Me Note",
            content = "Content",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(note)

        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(created.id, found.id)
        assertEquals("Find Me Note", found.title)
    }

    @Test
    fun `findById returns null when not exists`() = runTest {
        val found = repository.findById("nonexistent-id")

        assertNull(found)
    }

    @Test
    fun `update note changes values and increments syncVersion`() = runTest {
        val note = Note(
            id = "",
            title = "Original Title",
            content = "Original Content",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(note)

        val updated = repository.update(
            created.copy(
                title = "Updated Title",
                content = "Updated Content",
            ),
        )

        assertEquals("Updated Title", updated.title)
        assertEquals("Updated Content", updated.content)
        assertEquals(1, updated.syncVersion)

        // Verify persisted
        val found = repository.findById(created.id)
        assertNotNull(found)
        assertEquals("Updated Title", found.title)
        assertEquals(1, found.syncVersion)
    }

    @Test
    fun `delete performs soft delete`() = runTest {
        val note = Note(
            id = "",
            title = "Delete Me Note",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(note)

        val result = repository.delete(created.id)

        assertTrue(result)

        // Note should still exist but be marked as deleted
        val found = repository.findById(created.id)
        assertNotNull(found)
        assertNotNull(found.deletedAt)
        assertTrue(found.isDeleted)
    }

    @Test
    fun `delete returns false for nonexistent note`() = runTest {
        val result = repository.delete("nonexistent-id")

        assertFalse(result)
    }

    @Test
    fun `delete returns false for already deleted note`() = runTest {
        val note = Note(
            id = "",
            title = "Delete Twice Note",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(note)

        repository.delete(created.id)
        val result = repository.delete(created.id)

        assertFalse(result)
    }

    @Test
    fun `findAll excludes soft-deleted notes`() = runTest {
        val note1 = repository.create(
            Note(id = "", title = "Note 1", createdAt = "", updatedAt = ""),
        )
        val note2 = repository.create(
            Note(id = "", title = "Note 2", createdAt = "", updatedAt = ""),
        )
        repository.create(
            Note(id = "", title = "Note 3", createdAt = "", updatedAt = ""),
        )

        // Delete note2
        repository.delete(note2.id)

        val all = repository.findAll()

        assertEquals(2, all.size)
        assertTrue(all.any { it.title == "Note 1" })
        assertTrue(all.any { it.title == "Note 3" })
        assertFalse(all.any { it.title == "Note 2" })
    }

    @Test
    fun `findAll returns empty list when no notes`() = runTest {
        val all = repository.findAll()

        assertTrue(all.isEmpty())
    }

    @Test
    fun `note with folderId is stored correctly`() = runTest {
        val folderId = "01HWFOLDER23456789ABCDEFGH"
        val note = Note(
            id = "",
            title = "Note in Folder",
            folderId = folderId,
            createdAt = "",
            updatedAt = "",
        )

        val created = repository.create(note)
        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(folderId, found.folderId)
    }

    @Test
    fun `findByFolderId returns notes in folder`() = runTest {
        val folderId = "01HWFOLDER23456789ABCDEFGH"

        repository.create(
            Note(id = "", title = "Note in Folder 1", folderId = folderId, createdAt = "", updatedAt = ""),
        )
        repository.create(
            Note(id = "", title = "Note in Folder 2", folderId = folderId, createdAt = "", updatedAt = ""),
        )
        repository.create(
            Note(id = "", title = "Note without Folder", folderId = null, createdAt = "", updatedAt = ""),
        )

        val folderNotes = repository.findByFolderId(folderId)

        assertEquals(2, folderNotes.size)
        assertTrue(folderNotes.all { it.folderId == folderId })
    }

    @Test
    fun `findByFolderId excludes soft-deleted notes`() = runTest {
        val folderId = "01HWFOLDER23456789ABCDEFGH"

        val note1 = repository.create(
            Note(id = "", title = "Note 1", folderId = folderId, createdAt = "", updatedAt = ""),
        )
        repository.create(
            Note(id = "", title = "Note 2", folderId = folderId, createdAt = "", updatedAt = ""),
        )

        repository.delete(note1.id)

        val folderNotes = repository.findByFolderId(folderId)

        assertEquals(1, folderNotes.size)
        assertEquals("Note 2", folderNotes.first().title)
    }

    @Test
    fun `note hasContent returns correct value`() = runTest {
        val noteWithContent = Note(
            id = "1",
            title = "With Content",
            content = "Some content",
            createdAt = "",
            updatedAt = "",
        )
        val noteWithoutContent = Note(
            id = "2",
            title = "Without Content",
            content = null,
            createdAt = "",
            updatedAt = "",
        )
        val noteWithBlankContent = Note(
            id = "3",
            title = "Blank Content",
            content = "   ",
            createdAt = "",
            updatedAt = "",
        )

        assertTrue(noteWithContent.hasContent)
        assertFalse(noteWithoutContent.hasContent)
        assertFalse(noteWithBlankContent.hasContent)
    }
}
