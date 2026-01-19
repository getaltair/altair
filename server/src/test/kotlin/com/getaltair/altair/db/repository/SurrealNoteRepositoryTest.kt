package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbTestContainer
import com.getaltair.altair.domain.NoteError
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.AfterAll
import org.junit.jupiter.api.BeforeAll
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestInstance
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertIs
import kotlin.test.assertTrue
import kotlin.time.Clock

@Testcontainers
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class SurrealNoteRepositoryTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var repository: SurrealNoteRepository
    private val testUserId = Ulid("01TESTACCT0000000000000001")

    @BeforeAll
    fun setupContainer() {
        container.start()
        runBlocking {
            val config = container.createNetworkConfig()
            dbClient = SurrealDbClient(config)
            dbClient.connect().getOrNull()

            // Run migrations
            val migrationRunner = MigrationRunner(dbClient)
            migrationRunner.runMigrations()

            // Create test user
            dbClient.execute(
                "CREATE user:${testUserId.value} CONTENT { " +
                    "email: 'test@test.com', display_name: 'Test User', role: 'member', status: 'active' };",
            )
        }
    }

    @AfterAll
    fun tearDown() {
        runBlocking {
            dbClient.close()
        }
        container.stop()
    }

    @BeforeEach
    fun setup() {
        repository = SurrealNoteRepository(dbClient, testUserId)
        // Clean up notes and folders before each test
        runBlocking {
            dbClient.execute("DELETE note;")
            dbClient.execute("DELETE folder;")
        }
    }

    @Test
    fun `save creates new note`(): Unit =
        runBlocking {
            val note = createTestNote()

            val result = repository.save(note)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals(note.id, saved.id)
                assertEquals(note.title, saved.title)
                assertEquals(note.content, saved.content)
            }
        }

    @Test
    fun `findById returns saved note`(): Unit =
        runBlocking {
            val note = createTestNote()
            repository.save(note)

            val result = repository.findById(note.id)

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(note.id, found.id)
                assertEquals(note.title, found.title)
            }
        }

    @Test
    fun `findById returns error for non-existent note`(): Unit =
        runBlocking {
            val result = repository.findById(Ulid.generate())

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<NoteError.NotFound>(error)
            }
        }

    @Test
    fun `save updates existing note`(): Unit =
        runBlocking {
            val note = createTestNote(title = "Original Title")
            repository.save(note)

            val updated = note.copy(title = "Updated Title", content = "Updated content")
            val result = repository.save(updated)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals("Updated Title", saved.title)
                assertEquals("Updated content", saved.content)
            }
        }

    @Test
    fun `togglePinned pins unpinned note`(): Unit =
        runBlocking {
            val note = createTestNote(isPinned = false)
            repository.save(note)

            val result = repository.togglePinned(note.id)

            assertTrue(result.isRight())
            result.onRight { toggled ->
                assertTrue(toggled.isPinned)
            }
        }

    @Test
    fun `togglePinned unpins pinned note`(): Unit =
        runBlocking {
            val note = createTestNote(isPinned = true)
            repository.save(note)

            val result = repository.togglePinned(note.id)

            assertTrue(result.isRight())
            result.onRight { toggled ->
                assertFalse(toggled.isPinned)
            }
        }

    @Test
    fun `findPinned returns only pinned notes`(): Unit =
        runBlocking {
            val pinnedNote = createTestNote(title = "Pinned Note", isPinned = true)
            val unpinnedNote = createTestNote(title = "Unpinned Note", isPinned = false)
            repository.save(pinnedNote)
            repository.save(unpinnedNote)

            val pinnedNotes = repository.findPinned().first()

            assertEquals(1, pinnedNotes.size)
            assertEquals("Pinned Note", pinnedNotes.first().title)
            assertTrue(pinnedNotes.first().isPinned)
        }

    @Test
    fun `findByFolder returns notes in specific folder`(): Unit =
        runBlocking {
            // Create a folder
            val folderId = Ulid.generate()
            dbClient.execute(
                "CREATE folder:${folderId.value} CONTENT { " +
                    "user_id: user:${testUserId.value}, name: 'Test Folder' };",
            )

            val noteInFolder = createTestNote(title = "In Folder", folderId = folderId)
            val noteInRoot = createTestNote(title = "In Root", folderId = null)
            repository.save(noteInFolder)
            repository.save(noteInRoot)

            val folderNotes = repository.findByFolder(folderId).first()

            assertEquals(1, folderNotes.size)
            assertEquals("In Folder", folderNotes.first().title)
        }

    @Test
    fun `findByFolder with null returns root notes`(): Unit =
        runBlocking {
            // Create a folder
            val folderId = Ulid.generate()
            dbClient.execute(
                "CREATE folder:${folderId.value} CONTENT { " +
                    "user_id: user:${testUserId.value}, name: 'Test Folder' };",
            )

            val noteInFolder = createTestNote(title = "In Folder", folderId = folderId)
            val noteInRoot = createTestNote(title = "In Root", folderId = null)
            repository.save(noteInFolder)
            repository.save(noteInRoot)

            val rootNotes = repository.findByFolder(null).first()

            assertEquals(1, rootNotes.size)
            assertEquals("In Root", rootNotes.first().title)
        }

    @Test
    fun `moveToFolder moves note to different folder`(): Unit =
        runBlocking {
            // Create two folders
            val folder1Id = Ulid.generate()
            val folder2Id = Ulid.generate()
            dbClient.execute(
                "CREATE folder:${folder1Id.value} CONTENT { " +
                    "user_id: user:${testUserId.value}, name: 'Folder 1' };",
            )
            dbClient.execute(
                "CREATE folder:${folder2Id.value} CONTENT { " +
                    "user_id: user:${testUserId.value}, name: 'Folder 2' };",
            )

            val note = createTestNote(folderId = folder1Id)
            repository.save(note)

            val result = repository.moveToFolder(note.id, folder2Id)

            assertTrue(result.isRight())
            result.onRight { moved ->
                assertEquals(folder2Id, moved.folderId)
            }
        }

    @Test
    fun `moveToFolder moves note to root`(): Unit =
        runBlocking {
            val folderId = Ulid.generate()
            dbClient.execute(
                "CREATE folder:${folderId.value} CONTENT { " +
                    "user_id: user:${testUserId.value}, name: 'Test Folder' };",
            )

            val note = createTestNote(folderId = folderId)
            repository.save(note)

            val result = repository.moveToFolder(note.id, null)

            assertTrue(result.isRight())
            result.onRight { moved ->
                assertEquals(null, moved.folderId)
            }
        }

    @Test
    fun `search finds notes by title`(): Unit =
        runBlocking {
            val note1 = createTestNote(title = "Meeting Notes", content = "Some content")
            val note2 = createTestNote(title = "Shopping List", content = "Buy groceries")
            repository.save(note1)
            repository.save(note2)

            val result = repository.search("Meeting")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(1, found.size)
                assertEquals("Meeting Notes", found.first().title)
            }
        }

    @Test
    fun `search finds notes by content`(): Unit =
        runBlocking {
            val note1 = createTestNote(title = "Note 1", content = "Contains important information")
            val note2 = createTestNote(title = "Note 2", content = "Something else")
            repository.save(note1)
            repository.save(note2)

            val result = repository.search("important")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(1, found.size)
                assertEquals("Note 1", found.first().title)
            }
        }

    @Test
    fun `search is case insensitive`(): Unit =
        runBlocking {
            val note = createTestNote(title = "UPPERCASE Title", content = "lowercase content")
            repository.save(note)

            val result = repository.search("uppercase")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(1, found.size)
            }
        }

    @Test
    fun `delete soft deletes note`(): Unit =
        runBlocking {
            val note = createTestNote()
            repository.save(note)

            val deleteResult = repository.delete(note.id)
            assertTrue(deleteResult.isRight())

            val findResult = repository.findById(note.id)
            assertTrue(findResult.isLeft())
        }

    @Test
    fun `findAll returns all non-deleted notes`(): Unit =
        runBlocking {
            val note1 = createTestNote(title = "Note 1")
            val note2 = createTestNote(title = "Note 2")
            val note3 = createTestNote(title = "Note 3")
            repository.save(note1)
            repository.save(note2)
            repository.save(note3)
            repository.delete(note3.id)

            val allNotes = repository.findAll().first()

            assertEquals(2, allNotes.size)
        }

    // --- User Isolation Tests ---

    @Test
    fun `findById returns error when accessing another user's note`(): Unit =
        runBlocking {
            // Create note as testUserId
            val note = createTestNote()
            repository.save(note)

            // Create second user and their repository
            val otherUserId = Ulid("01TESTACCT1000000000000001")
            dbClient.execute(
                "CREATE user:${otherUserId.value} CONTENT { " +
                    "email: 'other@test.com', display_name: 'Other User', " +
                    "role: 'member', status: 'active' };",
            )
            val otherUserRepository = SurrealNoteRepository(dbClient, otherUserId)

            // Other user should not be able to access the note
            val result = otherUserRepository.findById(note.id)

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<NoteError.NotFound>(error)
            }
        }

    @Test
    fun `findAll only returns current user's notes`(): Unit =
        runBlocking {
            // Create notes for testUserId
            repository.save(createTestNote(title = "User1 Note 1"))
            repository.save(createTestNote(title = "User1 Note 2"))

            // Create second user and their notes
            val otherUserId = Ulid("01TESTACCT1000000000000001")
            dbClient.execute(
                "CREATE user:${otherUserId.value} CONTENT { " +
                    "email: 'other@test.com', display_name: 'Other User', " +
                    "role: 'member', status: 'active' };",
            )
            val otherUserRepository = SurrealNoteRepository(dbClient, otherUserId)
            val now = Clock.System.now()
            val otherUserNote =
                Note(
                    id = Ulid.generate(),
                    userId = otherUserId,
                    title = "User2 Note",
                    content = "Other user content",
                    folderId = null,
                    initiativeId = null,
                    isPinned = false,
                    createdAt = now,
                    updatedAt = now,
                    deletedAt = null,
                )
            otherUserRepository.save(otherUserNote)

            // testUserId should only see their own notes
            val testUserNotes = repository.findAll().first()
            assertEquals(2, testUserNotes.size)
            assertTrue(testUserNotes.all { it.userId == testUserId })

            // otherUserId should only see their own notes
            val otherUserNotes = otherUserRepository.findAll().first()
            assertEquals(1, otherUserNotes.size)
            assertEquals("User2 Note", otherUserNotes.first().title)
        }

    @Test
    fun `delete fails silently for another user's note`(): Unit =
        runBlocking {
            // Create note as testUserId
            val note = createTestNote()
            repository.save(note)

            // Create second user
            val otherUserId = Ulid("01TESTACCT1000000000000001")
            dbClient.execute(
                "CREATE user:${otherUserId.value} CONTENT { " +
                    "email: 'other@test.com', display_name: 'Other User', " +
                    "role: 'member', status: 'active' };",
            )
            val otherUserRepository = SurrealNoteRepository(dbClient, otherUserId)

            // Other user tries to delete - should fail
            val deleteResult = otherUserRepository.delete(note.id)
            assertTrue(deleteResult.isLeft())

            // Original user should still be able to access the note
            val findResult = repository.findById(note.id)
            assertTrue(findResult.isRight())
        }

    @Test
    fun `search only returns current user's notes`(): Unit =
        runBlocking {
            // Create a note for testUserId with searchable content
            repository.save(createTestNote(title = "Searchable Meeting Notes", content = "Important meeting"))

            // Create second user with similar note
            val otherUserId = Ulid("01TESTACCT1000000000000001")
            dbClient.execute(
                "CREATE user:${otherUserId.value} CONTENT { " +
                    "email: 'other@test.com', display_name: 'Other User', " +
                    "role: 'member', status: 'active' };",
            )
            val otherUserRepository = SurrealNoteRepository(dbClient, otherUserId)
            val now = Clock.System.now()
            val otherUserNote =
                Note(
                    id = Ulid.generate(),
                    userId = otherUserId,
                    title = "Searchable Private Notes",
                    content = "Private meeting content",
                    folderId = null,
                    initiativeId = null,
                    isPinned = false,
                    createdAt = now,
                    updatedAt = now,
                    deletedAt = null,
                )
            otherUserRepository.save(otherUserNote)

            // testUserId searches for "Searchable" - should only find their own note
            val searchResult = repository.search("Searchable")
            assertTrue(searchResult.isRight())
            searchResult.onRight { notes ->
                assertEquals(1, notes.size)
                assertEquals("Searchable Meeting Notes", notes.first().title)
            }

            // otherUserId searches for "Searchable" - should only find their own note
            val otherSearchResult = otherUserRepository.search("Searchable")
            assertTrue(otherSearchResult.isRight())
            otherSearchResult.onRight { notes ->
                assertEquals(1, notes.size)
                assertEquals("Searchable Private Notes", notes.first().title)
            }
        }

    private fun createTestNote(
        title: String = "Test Note",
        content: String = "Test content",
        folderId: Ulid? = null,
        isPinned: Boolean = false,
    ): Note {
        val now = Clock.System.now()
        return Note(
            id = Ulid.generate(),
            userId = testUserId,
            title = title,
            content = content,
            folderId = folderId,
            initiativeId = null,
            isPinned = isPinned,
            createdAt = now,
            updatedAt = now,
            deletedAt = null,
        )
    }

    companion object {
        @Container
        val container = SurrealDbTestContainer()
    }
}
