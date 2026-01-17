package com.getaltair.altair.db

import kotlinx.coroutines.runBlocking
import java.util.UUID
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.time.Clock

/**
 * Tests for SQLDelight database queries.
 *
 * Uses in-memory database for testing.
 */
class SqlDelightDatabaseTest {
    private val database = createDatabase()

    @Test
    fun `user insert and find works`() =
        runBlocking {
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

            assertNotNull(user)
            assertEquals(email, user.email)
            assertEquals("Test User", user.display_name)
        }

    @Test
    fun `initiative insert and find works`() =
        runBlocking {
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

            assertNotNull(initiative)
            assertEquals("Test Initiative", initiative.name)
            assertEquals("active", initiative.status)
        }

    @Test
    fun `initiative soft delete hides from queries`() =
        runBlocking {
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

            assertNull(initiative)
        }

    @Test
    fun `note insert and search works`() =
        runBlocking {
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

            assertEquals(1, searchResults.size)
            assertEquals("Meeting Notes", searchResults.first().title)
        }

    @Test
    fun `tag and note relationship works`() =
        runBlocking {
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

            assertEquals(1, noteTags.size)
            assertEquals("work", noteTags.first().name)
        }

    private suspend fun createTestUser(): String {
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
}
