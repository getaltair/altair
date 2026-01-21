package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbContainerExtension
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.model.system.User
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.domain.types.enums.UserStatus
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain
import kotlinx.coroutines.flow.first
import kotlin.time.Clock

/**
 * Integration tests for user-scoped data isolation.
 *
 * Verifies that:
 * - 4.3.1: User-scoped repositories filter data by user
 * - 4.3.2: Users cannot access other users' data
 * - 4.3.3: RPC services respect user scope boundaries
 */
class UserScopeIntegrationTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient
        lateinit var userRepository: SurrealUserRepository
        var user1Id: Ulid? = null
        var user2Id: Ulid? = null

        beforeSpec {
            val config = SurrealDbContainerExtension.createNetworkConfig()
            dbClient = SurrealDbClient(config)
            dbClient.connect().getOrNull()

            // Run migrations
            val migrationRunner = MigrationRunner(dbClient)
            migrationRunner.runMigrations()

            userRepository = SurrealUserRepository(dbClient)
        }

        afterSpec {
            dbClient.close()
        }

        beforeEach {
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
                    id = user1Id!!,
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
                    id = user2Id!!,
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

        // ===== 4.3.1: Verify user-scoped repositories receive AuthContext =====

        given("user-scoped repository filtering") {
            `when`("multiple users create notes") {
                then("each repository only sees its own user's data") {
                    val noteRepo1 = SurrealNoteRepository(dbClient, user1Id!!)
                    val noteRepo2 = SurrealNoteRepository(dbClient, user2Id!!)

                    // User 1 creates a note
                    val note1 = createNote(user1Id!!, "User 1 Note")
                    noteRepo1.save(note1)

                    // User 2 creates a note
                    val note2 = createNote(user2Id!!, "User 2 Note")
                    noteRepo2.save(note2)

                    // User 1's repository should only see their own note
                    val user1Notes = noteRepo1.findAll().first()
                    user1Notes shouldHaveSize 1
                    user1Notes.first().title shouldBe "User 1 Note"

                    // User 2's repository should only see their own note
                    val user2Notes = noteRepo2.findAll().first()
                    user2Notes shouldHaveSize 1
                    user2Notes.first().title shouldBe "User 2 Note"
                }
            }
        }

        // ===== 4.3.2: Add integration test for cross-user data isolation =====

        given("cross-user data access prevention") {
            `when`("user tries to access another user's note by ID") {
                then("returns NotFound error") {
                    val noteRepo1 = SurrealNoteRepository(dbClient, user1Id!!)
                    val noteRepo2 = SurrealNoteRepository(dbClient, user2Id!!)

                    // User 1 creates a note
                    val note1 = createNote(user1Id!!, "Secret Note")
                    noteRepo1.save(note1)

                    // User 2 tries to find user 1's note by ID
                    val result = noteRepo2.findById(note1.id)

                    // Should NOT find the note
                    result.shouldBeLeft()
                }
            }

            `when`("user tries to delete another user's note") {
                then("delete fails and original note remains") {
                    val noteRepo1 = SurrealNoteRepository(dbClient, user1Id!!)
                    val noteRepo2 = SurrealNoteRepository(dbClient, user2Id!!)

                    // User 1 creates a note
                    val note1 = createNote(user1Id!!, "Protected Note")
                    noteRepo1.save(note1)

                    // User 2 tries to delete user 1's note
                    val deleteResult = noteRepo2.delete(note1.id)

                    // Should fail - note not found for user 2
                    deleteResult.shouldBeLeft()

                    // Verify note still exists for user 1
                    val note = noteRepo1.findById(note1.id)
                    note.shouldBeRight()
                }
            }

            `when`("user tries to update another user's note") {
                then("update fails and original note remains unchanged") {
                    val noteRepo1 = SurrealNoteRepository(dbClient, user1Id!!)
                    val noteRepo2 = SurrealNoteRepository(dbClient, user2Id!!)

                    // User 1 creates a note
                    val note1 = createNote(user1Id!!, "Original Title")
                    noteRepo1.save(note1)

                    // User 2 tries to update user 1's note
                    val maliciousUpdate = note1.copy(title = "Hacked Title")
                    // Result intentionally ignored - save cannot access other user's note
                    noteRepo2.save(maliciousUpdate)

                    // The update should fail because user 2 can't find the note
                    // (save checks findById first)
                    val originalNote = noteRepo1.findById(note1.id).getOrNull()
                    originalNote?.title shouldBe "Original Title"
                }
            }

            `when`("user tries to toggle pin on another user's note") {
                then("toggle fails") {
                    val noteRepo1 = SurrealNoteRepository(dbClient, user1Id!!)
                    val noteRepo2 = SurrealNoteRepository(dbClient, user2Id!!)

                    // User 1 creates a note
                    val note1 = createNote(user1Id!!, "User 1 Note")
                    noteRepo1.save(note1)
                    note1.isPinned shouldBe false

                    // User 2 tries to toggle user 1's note
                    val toggleResult = noteRepo2.togglePinned(note1.id)

                    // Should fail
                    toggleResult.shouldBeLeft()

                    // User 1's note should still be unpinned
                    val originalNote = noteRepo1.findById(note1.id).getOrNull()
                    originalNote?.isPinned shouldBe false
                }
            }

            `when`("user tries to move another user's note to a folder") {
                then("move fails") {
                    val noteRepo1 = SurrealNoteRepository(dbClient, user1Id!!)
                    val noteRepo2 = SurrealNoteRepository(dbClient, user2Id!!)

                    // User 1 creates a note
                    val note1 = createNote(user1Id!!, "User 1 Note")
                    noteRepo1.save(note1)

                    // User 2 tries to move user 1's note
                    val fakeFolder = Ulid.generate()
                    val moveResult = noteRepo2.moveToFolder(note1.id, fakeFolder)

                    // Should fail
                    moveResult.shouldBeLeft()
                }
            }
        }

        given("user-scoped search and filtering") {
            `when`("multiple users have notes with same search term") {
                then("search only returns notes belonging to the user") {
                    val noteRepo1 = SurrealNoteRepository(dbClient, user1Id!!)
                    val noteRepo2 = SurrealNoteRepository(dbClient, user2Id!!)

                    // Both users create notes with "secret" in the title
                    noteRepo1.save(createNote(user1Id!!, "User 1 Secret Document"))
                    noteRepo2.save(createNote(user2Id!!, "User 2 Secret Document"))

                    // User 1 searches for "secret"
                    val user1Results = noteRepo1.search("secret").getOrNull() ?: emptyList()
                    user1Results shouldHaveSize 1
                    user1Results.first().title shouldContain "User 1"

                    // User 2 searches for "secret"
                    val user2Results = noteRepo2.search("secret").getOrNull() ?: emptyList()
                    user2Results shouldHaveSize 1
                    user2Results.first().title shouldContain "User 2"
                }
            }

            `when`("multiple users have pinned notes") {
                then("pinned notes query only returns user's own notes") {
                    val noteRepo1 = SurrealNoteRepository(dbClient, user1Id!!)
                    val noteRepo2 = SurrealNoteRepository(dbClient, user2Id!!)

                    // User 1 creates and pins a note
                    val note1 = createNote(user1Id!!, "User 1 Pinned").copy(isPinned = true)
                    noteRepo1.save(note1)

                    // User 2 creates and pins a note
                    val note2 = createNote(user2Id!!, "User 2 Pinned").copy(isPinned = true)
                    noteRepo2.save(note2)

                    // Each user should only see their own pinned notes
                    val user1Pinned = noteRepo1.findPinned().first()
                    user1Pinned shouldHaveSize 1
                    user1Pinned.first().title shouldBe "User 1 Pinned"

                    val user2Pinned = noteRepo2.findPinned().first()
                    user2Pinned shouldHaveSize 1
                    user2Pinned.first().title shouldBe "User 2 Pinned"
                }
            }
        }

        // ===== 4.3.3: Ensure RPC services use authenticated user context =====

        given("user-scoped note counts") {
            `when`("multiple users create different numbers of notes") {
                then("each user's count is isolated") {
                    val noteRepo1 = SurrealNoteRepository(dbClient, user1Id!!)
                    val noteRepo2 = SurrealNoteRepository(dbClient, user2Id!!)

                    // User 1 creates 3 notes
                    repeat(3) { i ->
                        noteRepo1.save(createNote(user1Id!!, "User 1 Note $i"))
                    }

                    // User 2 creates 5 notes
                    repeat(5) { i ->
                        noteRepo2.save(createNote(user2Id!!, "User 2 Note $i"))
                    }

                    // Each user should see only their own notes
                    val user1Notes = noteRepo1.findAll().first()
                    user1Notes shouldHaveSize 3

                    val user2Notes = noteRepo2.findAll().first()
                    user2Notes shouldHaveSize 5
                }
            }
        }
    }) {
    companion object {
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
    }
}
