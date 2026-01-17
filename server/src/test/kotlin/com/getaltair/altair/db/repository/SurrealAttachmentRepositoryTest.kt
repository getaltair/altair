package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbTestContainer
import com.getaltair.altair.domain.model.knowledge.Attachment
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
import kotlin.test.assertTrue
import kotlin.time.Clock

@Testcontainers
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class SurrealAttachmentRepositoryTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var repository: SurrealAttachmentRepository
    private val testUserId = Ulid("01TSTACCTATMT0000000000A00")
    private val testNoteId = Ulid("01TSTN0TEATMT0000000000N00")

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

            // Create test note for attachment association
            dbClient.execute(
                "CREATE note:${testNoteId.value} CONTENT { " +
                    "user_id: user:${testUserId.value}, title: 'Test Note', content: 'Test content', is_pinned: false };",
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
        repository = SurrealAttachmentRepository(dbClient, testUserId)
        // Clean up attachments before each test
        runBlocking {
            dbClient.execute("DELETE attachment;")
        }
    }

    @Test
    fun `save creates new attachment`(): Unit =
        runBlocking {
            val attachment = createTestAttachment()

            val result = repository.save(attachment)

            result.onLeft { error ->
                println("Save failed with error: $error")
            }
            assertTrue(result.isRight(), "Save failed: ${result.leftOrNull()}")
            result.onRight { saved ->
                assertEquals(attachment.id, saved.id)
                assertEquals(attachment.filename, saved.filename)
                assertEquals(attachment.mimeType, saved.mimeType)
            }
            Unit
        }

    @Test
    fun `findById returns saved attachment`(): Unit =
        runBlocking {
            val attachment = createTestAttachment()
            repository.save(attachment)

            val result = repository.findById(attachment.id)

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(attachment.id, found.id)
                assertEquals(attachment.filename, found.filename)
            }
            Unit
        }

    @Test
    fun `findAll returns all attachments`(): Unit =
        runBlocking {
            val attachment1 = createTestAttachment(filename = "file1.txt")
            val attachment2 = createTestAttachment(filename = "file2.txt")
            repository.save(attachment1)
            repository.save(attachment2)

            val attachments = repository.findAll().first()

            assertEquals(2, attachments.size)
        }

    @Test
    fun `findByNote returns attachments for specific note`(): Unit =
        runBlocking {
            val attachment = createTestAttachment()
            repository.save(attachment)

            val noteAttachments = repository.findByNote(testNoteId).first()

            assertEquals(1, noteAttachments.size)
            assertEquals(attachment.id, noteAttachments.first().id)
        }

    // --- SQL Injection Prevention Tests ---

    @Test
    fun `save handles mimeType with single quotes without SQL error`(): Unit =
        runBlocking {
            val attachment = createTestAttachment(mimeType = "application/x-test'type")

            val result = repository.save(attachment)

            assertTrue(result.isRight(), "Should save attachment with single quote in mimeType")
            result.onRight { saved ->
                assertEquals("application/x-test'type", saved.mimeType)
            }
            Unit
        }

    @Test
    fun `save handles mimeType with SQL injection attempt`(): Unit =
        runBlocking {
            val maliciousMimeType = "text/plain'; DROP TABLE attachment; --"
            val attachment = createTestAttachment(mimeType = maliciousMimeType)

            val result = repository.save(attachment)

            assertTrue(result.isRight(), "Should save attachment with SQL injection attempt in mimeType")
            result.onRight { saved ->
                assertEquals(maliciousMimeType, saved.mimeType)
            }

            // Verify attachment table still exists and has data
            val allAttachments = repository.findAll().first()
            assertEquals(1, allAttachments.size, "Attachment table should still exist with one attachment")
        }

    @Test
    fun `save handles mimeType with multiple single quotes`(): Unit =
        runBlocking {
            val attachment = createTestAttachment(mimeType = "text/x-custom'with''multiple'''quotes")

            val result = repository.save(attachment)

            assertTrue(result.isRight(), "Should save attachment with multiple single quotes in mimeType")
            result.onRight { saved ->
                assertEquals("text/x-custom'with''multiple'''quotes", saved.mimeType)
            }
            Unit
        }

    @Test
    fun `save handles mimeType with unicode and special characters`(): Unit =
        runBlocking {
            val attachment = createTestAttachment(mimeType = "application/x-custom'type;charset=utf-8;boundary='----=_Part")

            val result = repository.save(attachment)

            assertTrue(result.isRight(), "Should save attachment with complex mimeType")
            result.onRight { saved ->
                assertEquals("application/x-custom'type;charset=utf-8;boundary='----=_Part", saved.mimeType)
            }
            Unit
        }

    @Test
    fun `findByMimeType handles prefix with single quotes`(): Unit =
        runBlocking {
            val attachment = createTestAttachment(mimeType = "application/x-test'type")
            repository.save(attachment)

            val results = repository.findByMimeType("application/x-test'").first()

            assertEquals(1, results.size)
            assertEquals(attachment.id, results.first().id)
        }

    private fun createTestAttachment(
        filename: String = "test-file.txt",
        mimeType: String = "text/plain",
    ): Attachment {
        val now = Clock.System.now()
        return Attachment(
            id = Ulid.generate(),
            userId = testUserId,
            noteId = testNoteId,
            inboxItemId = null,
            filename = filename,
            mimeType = mimeType,
            sizeBytes = 1024,
            storagePath = "/storage/${Ulid.generate().value}/$filename",
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
