package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbContainerExtension
import com.getaltair.altair.domain.NoteError
import com.getaltair.altair.domain.model.knowledge.Note
import com.getaltair.altair.domain.types.Ulid
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.inspectors.forAll
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf
import kotlinx.coroutines.flow.first
import kotlin.time.Clock

/**
 * Tests for SurrealNoteRepository using Testcontainers.
 *
 * Verifies:
 * - CRUD operations (save, findById, delete, findAll)
 * - Pinning operations (togglePinned, findPinned)
 * - Folder operations (findByFolder, moveToFolder)
 * - Search functionality (by title and content, case insensitive)
 * - User isolation (notes scoped to user, cross-user access prevention)
 */
class SurrealNoteRepositoryTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient
        lateinit var repository: SurrealNoteRepository
        val testUserId = Ulid("01TESTACCT0000000000000001")

        beforeSpec {
            val config = SurrealDbContainerExtension.createNetworkConfig()
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

        afterSpec {
            dbClient.close()
        }

        beforeEach {
            repository = SurrealNoteRepository(dbClient, testUserId)
            // Clean up notes and folders before each test
            dbClient.execute("DELETE note;")
            dbClient.execute("DELETE folder;")
        }

        given("CRUD operations") {
            `when`("saving a new note") {
                then("creates the note successfully") {
                    val note = createTestNote(testUserId)

                    val result = repository.save(note)

                    result.shouldBeRight()
                    val saved = result.getOrNull()
                    saved?.id shouldBe note.id
                    saved?.title shouldBe note.title
                    saved?.content shouldBe note.content
                }
            }

            `when`("updating an existing note") {
                then("persists the changes") {
                    val note = createTestNote(testUserId, title = "Original Title")
                    repository.save(note)

                    val updated = note.copy(title = "Updated Title", content = "Updated content")
                    val result = repository.save(updated)

                    result.shouldBeRight()
                    val saved = result.getOrNull()
                    saved?.title shouldBe "Updated Title"
                    saved?.content shouldBe "Updated content"
                }
            }

            `when`("finding a note by ID") {
                then("returns the note when it exists") {
                    val note = createTestNote(testUserId)
                    repository.save(note)

                    val result = repository.findById(note.id)

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.id shouldBe note.id
                    found?.title shouldBe note.title
                }

                then("returns error when note doesn't exist") {
                    val result = repository.findById(Ulid.generate())

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<NoteError.NotFound>()
                }
            }

            `when`("deleting a note") {
                then("soft deletes the note") {
                    val note = createTestNote(testUserId)
                    repository.save(note)

                    val deleteResult = repository.delete(note.id)
                    deleteResult.shouldBeRight()

                    val findResult = repository.findById(note.id)
                    findResult.shouldBeLeft()
                }
            }

            `when`("finding all notes") {
                then("returns all non-deleted notes") {
                    val note1 = createTestNote(testUserId, title = "Note 1")
                    val note2 = createTestNote(testUserId, title = "Note 2")
                    val note3 = createTestNote(testUserId, title = "Note 3")
                    repository.save(note1)
                    repository.save(note2)
                    repository.save(note3)
                    repository.delete(note3.id)

                    val allNotes = repository.findAll().first()

                    allNotes shouldHaveSize 2
                }
            }
        }

        given("pinning operations") {
            `when`("toggling pin on unpinned note") {
                then("pins the note") {
                    val note = createTestNote(testUserId, isPinned = false)
                    repository.save(note)

                    val result = repository.togglePinned(note.id)

                    result.shouldBeRight()
                    val toggled: Note? = result.getOrNull()
                    toggled!!.isPinned.shouldBeTrue()
                }
            }

            `when`("toggling pin on pinned note") {
                then("unpins the note") {
                    val note = createTestNote(testUserId, isPinned = true)
                    repository.save(note)

                    val result = repository.togglePinned(note.id)

                    result.shouldBeRight()
                    val toggled: Note? = result.getOrNull()
                    toggled!!.isPinned.shouldBeFalse()
                }
            }

            `when`("finding pinned notes") {
                then("returns only pinned notes") {
                    val pinnedNote = createTestNote(testUserId, title = "Pinned Note", isPinned = true)
                    val unpinnedNote = createTestNote(testUserId, title = "Unpinned Note", isPinned = false)
                    repository.save(pinnedNote)
                    repository.save(unpinnedNote)

                    val pinnedNotes = repository.findPinned().first()

                    pinnedNotes shouldHaveSize 1
                    pinnedNotes.first().title shouldBe "Pinned Note"
                    pinnedNotes.first().isPinned.shouldBeTrue()
                }
            }
        }

        given("folder operations") {
            `when`("finding notes by folder") {
                then("returns notes in specific folder") {
                    // Create a folder
                    val folderId = Ulid.generate()
                    dbClient.execute(
                        "CREATE folder:${folderId.value} CONTENT { " +
                            "user_id: user:${testUserId.value}, name: 'Test Folder' };",
                    )

                    val noteInFolder = createTestNote(testUserId, title = "In Folder", folderId = folderId)
                    val noteInRoot = createTestNote(testUserId, title = "In Root", folderId = null)
                    repository.save(noteInFolder)
                    repository.save(noteInRoot)

                    val folderNotes = repository.findByFolder(folderId).first()

                    folderNotes shouldHaveSize 1
                    folderNotes.first().title shouldBe "In Folder"
                }

                then("returns root notes when folder is null") {
                    // Create a folder
                    val folderId = Ulid.generate()
                    dbClient.execute(
                        "CREATE folder:${folderId.value} CONTENT { " +
                            "user_id: user:${testUserId.value}, name: 'Test Folder' };",
                    )

                    val noteInFolder = createTestNote(testUserId, title = "In Folder", folderId = folderId)
                    val noteInRoot = createTestNote(testUserId, title = "In Root", folderId = null)
                    repository.save(noteInFolder)
                    repository.save(noteInRoot)

                    val rootNotes = repository.findByFolder(null).first()

                    rootNotes shouldHaveSize 1
                    rootNotes.first().title shouldBe "In Root"
                }
            }

            `when`("moving note to different folder") {
                then("updates the folder reference") {
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

                    val note = createTestNote(testUserId, folderId = folder1Id)
                    repository.save(note)

                    val result = repository.moveToFolder(note.id, folder2Id)

                    result.shouldBeRight()
                    val moved = result.getOrNull()
                    moved?.folderId shouldBe folder2Id
                }
            }

            `when`("moving note to root") {
                then("clears the folder reference") {
                    val folderId = Ulid.generate()
                    dbClient.execute(
                        "CREATE folder:${folderId.value} CONTENT { " +
                            "user_id: user:${testUserId.value}, name: 'Test Folder' };",
                    )

                    val note = createTestNote(testUserId, folderId = folderId)
                    repository.save(note)

                    val result = repository.moveToFolder(note.id, null)

                    result.shouldBeRight()
                    val moved = result.getOrNull()
                    moved?.folderId.shouldBeNull()
                }
            }
        }

        given("search functionality") {
            `when`("searching by title") {
                then("finds matching notes") {
                    val note1 = createTestNote(testUserId, title = "Meeting Notes", content = "Some content")
                    val note2 = createTestNote(testUserId, title = "Shopping List", content = "Buy groceries")
                    repository.save(note1)
                    repository.save(note2)

                    val result = repository.search("Meeting")

                    result.shouldBeRight()
                    val found = result.getOrNull() ?: emptyList()
                    found shouldHaveSize 1
                    found.first().title shouldBe "Meeting Notes"
                }
            }

            `when`("searching by content") {
                then("finds matching notes") {
                    val note1 = createTestNote(testUserId, title = "Note 1", content = "Contains important information")
                    val note2 = createTestNote(testUserId, title = "Note 2", content = "Something else")
                    repository.save(note1)
                    repository.save(note2)

                    val result = repository.search("important")

                    result.shouldBeRight()
                    val found = result.getOrNull() ?: emptyList()
                    found shouldHaveSize 1
                    found.first().title shouldBe "Note 1"
                }
            }

            `when`("searching with different case") {
                then("is case insensitive") {
                    val note = createTestNote(testUserId, title = "UPPERCASE Title", content = "lowercase content")
                    repository.save(note)

                    val result = repository.search("uppercase")

                    result.shouldBeRight()
                    val found = result.getOrNull() ?: emptyList()
                    found shouldHaveSize 1
                }
            }
        }

        given("user isolation") {
            `when`("accessing another user's note by ID") {
                then("returns NotFound error") {
                    // Create note as testUserId
                    val note = createTestNote(testUserId)
                    repository.save(note)

                    // Create second user and their repository
                    val (_, otherUserRepository) = createOtherUser(dbClient)

                    // Other user should not be able to access the note
                    val result = otherUserRepository.findById(note.id)

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<NoteError.NotFound>()
                }
            }

            `when`("finding all notes") {
                then("only returns current user's notes") {
                    // Create notes for testUserId
                    repository.save(createTestNote(testUserId, title = "User1 Note 1"))
                    repository.save(createTestNote(testUserId, title = "User1 Note 2"))

                    // Create second user and their notes
                    val (otherUserId, otherUserRepository) = createOtherUser(dbClient)
                    otherUserRepository.save(createTestNote(otherUserId, title = "User2 Note"))

                    // testUserId should only see their own notes
                    val testUserNotes = repository.findAll().first()
                    testUserNotes shouldHaveSize 2
                    testUserNotes.forAll { it.userId shouldBe testUserId }

                    // otherUserId should only see their own notes
                    val otherUserNotes = otherUserRepository.findAll().first()
                    otherUserNotes shouldHaveSize 1
                    otherUserNotes.first().title shouldBe "User2 Note"
                }
            }

            `when`("attempting to delete another user's note") {
                then("fails silently") {
                    // Create note as testUserId
                    val note = createTestNote(testUserId)
                    repository.save(note)

                    // Create second user
                    val (_, otherUserRepository) = createOtherUser(dbClient)

                    // Other user tries to delete - should fail
                    val deleteResult = otherUserRepository.delete(note.id)
                    deleteResult.shouldBeLeft()

                    // Original user should still be able to access the note
                    val findResult = repository.findById(note.id)
                    findResult.shouldBeRight()
                }
            }

            `when`("searching for notes") {
                then("only returns current user's notes") {
                    // Create a note for testUserId with searchable content
                    repository.save(
                        createTestNote(testUserId, title = "Searchable Meeting Notes", content = "Important meeting"),
                    )

                    // Create second user with similar note
                    val (otherUserId, otherUserRepository) = createOtherUser(dbClient)
                    otherUserRepository.save(
                        createTestNote(otherUserId, title = "Searchable Private Notes", content = "Private meeting"),
                    )

                    // testUserId searches for "Searchable" - should only find their own note
                    val searchResult = repository.search("Searchable")
                    searchResult.shouldBeRight()
                    val testUserNotes = searchResult.getOrNull() ?: emptyList()
                    testUserNotes shouldHaveSize 1
                    testUserNotes.first().title shouldBe "Searchable Meeting Notes"

                    // otherUserId searches for "Searchable" - should only find their own note
                    val otherSearchResult = otherUserRepository.search("Searchable")
                    otherSearchResult.shouldBeRight()
                    val otherUserNotes = otherSearchResult.getOrNull() ?: emptyList()
                    otherUserNotes shouldHaveSize 1
                    otherUserNotes.first().title shouldBe "Searchable Private Notes"
                }
            }
        }
    }) {
    companion object {
        private const val OTHER_USER_ID = "01TESTACCT1000000000000001"
        private const val OTHER_USER_EMAIL = "other@test.com"

        /**
         * Creates a second test user for user isolation tests.
         *
         * @return Pair of (userId, repository for that user)
         */
        private suspend fun createOtherUser(dbClient: SurrealDbClient): Pair<Ulid, SurrealNoteRepository> {
            val otherUserId = Ulid(OTHER_USER_ID)
            dbClient.execute(
                "CREATE user:${otherUserId.value} CONTENT { " +
                    "email: '$OTHER_USER_EMAIL', display_name: 'Other User', " +
                    "role: 'member', status: 'active' };",
            )
            return otherUserId to SurrealNoteRepository(dbClient, otherUserId)
        }

        private fun createTestNote(
            userId: Ulid,
            title: String = "Test Note",
            content: String = "Test content",
            folderId: Ulid? = null,
            isPinned: Boolean = false,
        ): Note {
            val now = Clock.System.now()
            return Note(
                id = Ulid.generate(),
                userId = userId,
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
    }
}
