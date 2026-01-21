package com.getaltair.altair.db

import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.nulls.shouldNotBeNull
import io.kotest.matchers.shouldBe
import java.util.UUID
import kotlin.time.Clock

/**
 * Tests for SQLDelight database queries.
 *
 * Uses in-memory database for testing.
 */
class SqlDelightDatabaseTest :
    BehaviorSpec({
        val database = createDatabase()

        suspend fun createTestUser(): String {
            val userId = "user_${UUID.randomUUID()}"
            val now = Clock.System.now().toString()

            database.userQueries.insert(
                id = userId,
                email = "test_$userId@example.com",
                hashed_password = "hash",
                display_name = "Test User",
                role = "member",
                preferences = null,
                created_at = now,
                updated_at = now,
            )

            return userId
        }

        given("user queries") {
            `when`("inserting and finding user") {
                then("user is stored and retrieved correctly") {
                    val userId = "user123"
                    val email = "test@example.com"
                    val now = Clock.System.now().toString()

                    database.userQueries.insert(
                        id = userId,
                        email = email,
                        hashed_password = "hashedpassword",
                        display_name = "Test User",
                        role = "member",
                        preferences = null,
                        created_at = now,
                        updated_at = now,
                    )

                    val user = database.userQueries.findById(userId).executeAsOneOrNull()

                    user.shouldNotBeNull()
                    user.email shouldBe email
                    user.display_name shouldBe "Test User"
                }
            }
        }

        given("initiative queries") {
            `when`("inserting and finding initiative") {
                then("initiative is stored and retrieved correctly") {
                    val userId = createTestUser()
                    val initiativeId = "initiative123"
                    val now = Clock.System.now().toString()

                    database.initiativeQueries.insert(
                        id = initiativeId,
                        user_id = userId,
                        name = "Test Initiative",
                        description = "A test initiative",
                        status = "active",
                        target_date = null,
                        color = "#FF5733",
                        icon = "target",
                        sort_order = 0,
                        created_at = now,
                        updated_at = now,
                    )

                    val initiative =
                        database.initiativeQueries
                            .findById(
                                id = initiativeId,
                                user_id = userId,
                            ).executeAsOneOrNull()

                    initiative.shouldNotBeNull()
                    initiative.name shouldBe "Test Initiative"
                    initiative.status shouldBe "active"
                }
            }

            `when`("soft deleting initiative") {
                then("initiative is hidden from queries") {
                    val userId = createTestUser()
                    val initiativeId = "initiative456"
                    val now = Clock.System.now().toString()

                    database.initiativeQueries.insert(
                        id = initiativeId,
                        user_id = userId,
                        name = "To Delete",
                        description = null,
                        status = "active",
                        target_date = null,
                        color = null,
                        icon = null,
                        sort_order = 0,
                        created_at = now,
                        updated_at = now,
                    )

                    // Soft delete
                    database.initiativeQueries.softDelete(
                        deleted_at = now,
                        updated_at = now,
                        id = initiativeId,
                        user_id = userId,
                    )

                    val initiative =
                        database.initiativeQueries
                            .findById(
                                id = initiativeId,
                                user_id = userId,
                            ).executeAsOneOrNull()

                    initiative.shouldBeNull()
                }
            }
        }

        given("note queries") {
            `when`("inserting and searching notes") {
                then("notes are found by title search") {
                    val userId = createTestUser()
                    val noteId = "note123"
                    val now = Clock.System.now().toString()

                    database.noteQueries.insert(
                        id = noteId,
                        user_id = userId,
                        folder_id = null,
                        title = "Meeting Notes",
                        content = "Important points discussed in the meeting",
                        content_format = "markdown",
                        initiative_id = null,
                        is_pinned = 0,
                        word_count = 6,
                        created_at = now,
                        updated_at = now,
                    )

                    // searchByTitle takes (user_id, value_)
                    val searchResults =
                        database.noteQueries
                            .searchByTitle(
                                user_id = userId,
                                value_ = "meeting",
                            ).executeAsList()

                    searchResults shouldHaveSize 1
                    searchResults.first().title shouldBe "Meeting Notes"
                }
            }
        }

        given("tag and note relationships") {
            `when`("associating tag with note") {
                then("tags are retrieved for note") {
                    val userId = createTestUser()
                    val tagId = "tag123"
                    val noteId = "note456"
                    val now = Clock.System.now().toString()

                    // Create tag
                    database.tagQueries.insert(
                        id = tagId,
                        user_id = userId,
                        name = "work",
                        color = "#0000FF",
                        created_at = now,
                        updated_at = now,
                    )

                    // Create note
                    database.noteQueries.insert(
                        id = noteId,
                        user_id = userId,
                        folder_id = null,
                        title = "Work Note",
                        content = "Content",
                        content_format = "markdown",
                        initiative_id = null,
                        is_pinned = 0,
                        word_count = 1,
                        created_at = now,
                        updated_at = now,
                    )

                    // Associate tag with note
                    database.tagQueries.addTagToNote(
                        note_id = noteId,
                        tag_id = tagId,
                        user_id = userId,
                        created_at = now,
                    )

                    val noteTags =
                        database.tagQueries
                            .findByNote(
                                note_id = noteId,
                                user_id = userId,
                            ).executeAsList()

                    noteTags shouldHaveSize 1
                    noteTags.first().name shouldBe "work"
                }
            }
        }
    })
