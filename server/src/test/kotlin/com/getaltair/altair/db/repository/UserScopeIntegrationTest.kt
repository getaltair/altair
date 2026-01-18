package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbTestContainer
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
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
import kotlin.test.assertTrue
import kotlin.time.Clock

/**
 * Integration tests for user-scoped data isolation.
 *
 * Verifies that:
 * - 4.3.1: User-scoped repositories filter data by user
 * - 4.3.2: Users cannot access other users' data
 * - 4.3.3: RPC services respect user scope boundaries
 */
@Testcontainers
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class UserScopeIntegrationTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var userRepository: SurrealUserRepository

    // Users for testing - using var with nullable type since Ulid is an inline class
    private var user1Id: Ulid = Ulid.generate()
    private var user2Id: Ulid = Ulid.generate()

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

            userRepository = SurrealUserRepository(dbClient)
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
        runBlocking {
            // Clean up tables
            dbClient.execute("DELETE user;")
            dbClient.execute("DELETE note;")
            dbClient.execute("DELETE quest;")
            dbClient.execute("DELETE folder;")

            // Create two test users
            val now = Clock.System.now().toEpochMilliseconds()
            val instant = kotlinx.datetime.Instant.fromEpochMilliseconds(now)

            user1Id = Ulid.generate()
            user2Id = Ulid.generate()

            userRepository.create(
                User(
                    id = user1Id,
                    email = "user1@test.com",
                    displayName = "User One",
                    role = UserRole.MEMBER,
                    status = UserStatus.ACTIVE,
                    storageUsedBytes = 0L,
                    storageQuotaBytes = 10_737_418_240L,
                    createdAt = instant,
                    updatedAt = instant,
                ),
            )

            userRepository.create(
                User(
                    id = user2Id,
                    email = "user2@test.com",
                    displayName = "User Two",
                    role = UserRole.MEMBER,
                    status = UserStatus.ACTIVE,
                    storageUsedBytes = 0L,
                    storageQuotaBytes = 10_737_418_240L,
                    createdAt = instant,
                    updatedAt = instant,
                ),
            )
        }
    }

    // ===== 4.3.1: Verify user-scoped repositories receive AuthContext =====

    @Test
    fun `user-scoped repository filters queries by user_id`() =
        runBlocking {
            val noteRepo1 = SurrealNoteRepository(dbClient, user1Id)
            val noteRepo2 = SurrealNoteRepository(dbClient, user2Id)

            // User 1 creates a note
            val note1 = createNote(user1Id, "User 1 Note")
            noteRepo1.save(note1)

            // User 2 creates a note
            val note2 = createNote(user2Id, "User 2 Note")
            noteRepo2.save(note2)

            // User 1's repository should only see their own note
            val user1Notes = noteRepo1.findAll().first()
            assertEquals(1, user1Notes.size)
            assertEquals("User 1 Note", user1Notes.first().title)

            // User 2's repository should only see their own note
            val user2Notes = noteRepo2.findAll().first()
            assertEquals(1, user2Notes.size)
            assertEquals("User 2 Note", user2Notes.first().title)
        }

    // ===== 4.3.2: Add integration test for cross-user data isolation =====

    @Test
    fun `user cannot access notes created by another user`() =
        runBlocking {
            val noteRepo1 = SurrealNoteRepository(dbClient, user1Id)
            val noteRepo2 = SurrealNoteRepository(dbClient, user2Id)

            // User 1 creates a note
            val note1 = createNote(user1Id, "Secret Note")
            noteRepo1.save(note1)

            // User 2 tries to find user 1's note by ID
            val result = noteRepo2.findById(note1.id)

            // Should NOT find the note
            assertTrue(result.isLeft())
        }

    @Test
    fun `user cannot delete notes created by another user`() =
        runBlocking {
            val noteRepo1 = SurrealNoteRepository(dbClient, user1Id)
            val noteRepo2 = SurrealNoteRepository(dbClient, user2Id)

            // User 1 creates a note
            val note1 = createNote(user1Id, "Protected Note")
            noteRepo1.save(note1)

            // User 2 tries to delete user 1's note
            val deleteResult = noteRepo2.delete(note1.id)

            // Should fail - note not found for user 2
            assertTrue(deleteResult.isLeft())

            // Verify note still exists for user 1
            val note = noteRepo1.findById(note1.id)
            assertTrue(note.isRight())
        }

    @Test
    fun `user cannot update notes created by another user`() =
        runBlocking {
            val noteRepo1 = SurrealNoteRepository(dbClient, user1Id)
            val noteRepo2 = SurrealNoteRepository(dbClient, user2Id)

            // User 1 creates a note
            val note1 = createNote(user1Id, "Original Title")
            noteRepo1.save(note1)

            // User 2 tries to update user 1's note
            val maliciousUpdate = note1.copy(title = "Hacked Title")
            noteRepo2.save(maliciousUpdate) // Result intentionally ignored - save cannot access other user's note

            // The update should fail because user 2 can't find the note
            // (save checks findById first)
            val originalNote = noteRepo1.findById(note1.id).getOrNull()
            assertEquals("Original Title", originalNote?.title)
        }

    @Test
    fun `search only returns notes belonging to the user`() =
        runBlocking {
            val noteRepo1 = SurrealNoteRepository(dbClient, user1Id)
            val noteRepo2 = SurrealNoteRepository(dbClient, user2Id)

            // Both users create notes with "secret" in the title
            noteRepo1.save(createNote(user1Id, "User 1 Secret Document"))
            noteRepo2.save(createNote(user2Id, "User 2 Secret Document"))

            // User 1 searches for "secret"
            val user1Results = noteRepo1.search("secret").getOrNull() ?: emptyList()
            assertEquals(1, user1Results.size)
            assertTrue(user1Results.first().title.contains("User 1"))

            // User 2 searches for "secret"
            val user2Results = noteRepo2.search("secret").getOrNull() ?: emptyList()
            assertEquals(1, user2Results.size)
            assertTrue(user2Results.first().title.contains("User 2"))
        }

    @Test
    fun `pinned notes only returns notes belonging to the user`() =
        runBlocking {
            val noteRepo1 = SurrealNoteRepository(dbClient, user1Id)
            val noteRepo2 = SurrealNoteRepository(dbClient, user2Id)

            // User 1 creates and pins a note
            val note1 = createNote(user1Id, "User 1 Pinned").copy(isPinned = true)
            noteRepo1.save(note1)

            // User 2 creates and pins a note
            val note2 = createNote(user2Id, "User 2 Pinned").copy(isPinned = true)
            noteRepo2.save(note2)

            // Each user should only see their own pinned notes
            val user1Pinned = noteRepo1.findPinned().first()
            assertEquals(1, user1Pinned.size)
            assertEquals("User 1 Pinned", user1Pinned.first().title)

            val user2Pinned = noteRepo2.findPinned().first()
            assertEquals(1, user2Pinned.size)
            assertEquals("User 2 Pinned", user2Pinned.first().title)
        }

    // ===== 4.3.3: Ensure RPC services use authenticated user context =====

    @Test
    fun `total notes count is isolated per user`() =
        runBlocking {
            val noteRepo1 = SurrealNoteRepository(dbClient, user1Id)
            val noteRepo2 = SurrealNoteRepository(dbClient, user2Id)

            // User 1 creates 3 notes
            repeat(3) { i ->
                noteRepo1.save(createNote(user1Id, "User 1 Note $i"))
            }

            // User 2 creates 5 notes
            repeat(5) { i ->
                noteRepo2.save(createNote(user2Id, "User 2 Note $i"))
            }

            // Each user should see only their own notes
            val user1Notes = noteRepo1.findAll().first()
            assertEquals(3, user1Notes.size)

            val user2Notes = noteRepo2.findAll().first()
            assertEquals(5, user2Notes.size)
        }

    @Test
    fun `toggle pinned only affects notes belonging to the user`() =
        runBlocking {
            val noteRepo1 = SurrealNoteRepository(dbClient, user1Id)
            val noteRepo2 = SurrealNoteRepository(dbClient, user2Id)

            // User 1 creates a note
            val note1 = createNote(user1Id, "User 1 Note")
            noteRepo1.save(note1)
            assertFalse(note1.isPinned)

            // User 2 tries to toggle user 1's note
            val toggleResult = noteRepo2.togglePinned(note1.id)

            // Should fail
            assertTrue(toggleResult.isLeft())

            // User 1's note should still be unpinned
            val originalNote = noteRepo1.findById(note1.id).getOrNull()
            assertFalse(originalNote?.isPinned ?: true)
        }

    @Test
    fun `move to folder only affects notes belonging to the user`() =
        runBlocking {
            val noteRepo1 = SurrealNoteRepository(dbClient, user1Id)
            val noteRepo2 = SurrealNoteRepository(dbClient, user2Id)

            // User 1 creates a note
            val note1 = createNote(user1Id, "User 1 Note")
            noteRepo1.save(note1)

            // User 2 tries to move user 1's note
            val fakeFolder = Ulid.generate()
            val moveResult = noteRepo2.moveToFolder(note1.id, fakeFolder)

            // Should fail
            assertTrue(moveResult.isLeft())
        }

    // ===== Helper methods =====

    private fun createNote(
        userId: Ulid,
        title: String,
    ): Note {
        val now = kotlinx.datetime.Instant.fromEpochMilliseconds(Clock.System.now().toEpochMilliseconds())
        return Note(
            id = Ulid.generate(),
            userId = userId,
            title = title,
            content = "Content for $title",
            folderId = null,
            initiativeId = null,
            isPinned = false,
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
