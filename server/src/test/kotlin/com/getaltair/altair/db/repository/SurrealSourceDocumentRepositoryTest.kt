package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbTestContainer
import com.getaltair.altair.domain.model.system.SourceDocument
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.ExtractionStatus
import com.getaltair.altair.domain.types.enums.SourceType
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
class SurrealSourceDocumentRepositoryTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var repository: SurrealSourceDocumentRepository
    private val testUserId = Ulid("01TSTACCTSRCD0000000000S00")

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
        repository = SurrealSourceDocumentRepository(dbClient, testUserId)
        // Clean up source documents before each test
        runBlocking {
            dbClient.execute("DELETE source_document;")
        }
    }

    @Test
    fun `save creates new source document`(): Unit =
        runBlocking {
            val doc = createTestSourceDocument()

            val result = repository.save(doc)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals(doc.id, saved.id)
                assertEquals(doc.title, saved.title)
                assertEquals(doc.mimeType, saved.mimeType)
            }
            Unit
        }

    @Test
    fun `findById returns saved source document`(): Unit =
        runBlocking {
            val doc = createTestSourceDocument()
            repository.save(doc)

            val result = repository.findById(doc.id)

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(doc.id, found.id)
                assertEquals(doc.title, found.title)
            }
            Unit
        }

    @Test
    fun `save updates existing source document`(): Unit =
        runBlocking {
            val doc = createTestSourceDocument(title = "Original Title")
            repository.save(doc)

            val updated = doc.copy(title = "Updated Title", mimeType = "application/pdf")
            val result = repository.save(updated)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals("Updated Title", saved.title)
                assertEquals("application/pdf", saved.mimeType)
            }
            Unit
        }

    @Test
    fun `findAll returns all source documents`(): Unit =
        runBlocking {
            val doc1 = createTestSourceDocument(title = "Document 1")
            val doc2 = createTestSourceDocument(title = "Document 2")
            repository.save(doc1)
            repository.save(doc2)

            val documents = repository.findAll().first()

            assertEquals(2, documents.size)
        }

    // --- SQL Injection Prevention Tests (CREATE path) ---

    @Test
    fun `save handles mimeType with single quotes without SQL error on create`(): Unit =
        runBlocking {
            val doc = createTestSourceDocument(mimeType = "application/x-test'type")

            val result = repository.save(doc)

            assertTrue(result.isRight(), "Should save source document with single quote in mimeType")
            result.onRight { saved ->
                assertEquals("application/x-test'type", saved.mimeType)
            }
            Unit
        }

    @Test
    fun `save handles mimeType with SQL injection attempt on create`(): Unit =
        runBlocking {
            val maliciousMimeType = "text/plain'; DROP TABLE source_document; --"
            val doc = createTestSourceDocument(mimeType = maliciousMimeType)

            val result = repository.save(doc)

            assertTrue(result.isRight(), "Should save source document with SQL injection attempt in mimeType")
            result.onRight { saved ->
                assertEquals(maliciousMimeType, saved.mimeType)
            }

            // Verify source_document table still exists and has data
            val allDocs = repository.findAll().first()
            assertEquals(1, allDocs.size, "Source document table should still exist with one document")
        }

    @Test
    fun `save handles mimeType with multiple single quotes on create`(): Unit =
        runBlocking {
            val doc = createTestSourceDocument(mimeType = "text/x-custom'with''multiple'''quotes")

            val result = repository.save(doc)

            assertTrue(result.isRight(), "Should save source document with multiple single quotes in mimeType")
            result.onRight { saved ->
                assertEquals("text/x-custom'with''multiple'''quotes", saved.mimeType)
            }
            Unit
        }

    // --- SQL Injection Prevention Tests (UPDATE path) ---

    @Test
    fun `save handles mimeType with single quotes without SQL error on update`(): Unit =
        runBlocking {
            val doc = createTestSourceDocument(mimeType = "text/plain")
            repository.save(doc)

            val updated = doc.copy(mimeType = "application/x-updated'type")
            val result = repository.save(updated)

            assertTrue(result.isRight(), "Should update source document with single quote in mimeType")
            result.onRight { saved ->
                assertEquals("application/x-updated'type", saved.mimeType)
            }
            Unit
        }

    @Test
    fun `save handles mimeType with SQL injection attempt on update`(): Unit =
        runBlocking {
            val doc = createTestSourceDocument(mimeType = "text/plain")
            repository.save(doc)

            val maliciousMimeType = "application/pdf'; DROP TABLE source_document; --"
            val updated = doc.copy(mimeType = maliciousMimeType)
            val result = repository.save(updated)

            assertTrue(result.isRight(), "Should update source document with SQL injection attempt in mimeType")
            result.onRight { saved ->
                assertEquals(maliciousMimeType, saved.mimeType)
            }

            // Verify source_document table still exists and has data
            val allDocs = repository.findAll().first()
            assertEquals(1, allDocs.size, "Source document table should still exist with one document")
        }

    @Test
    fun `save handles mimeType with multiple single quotes on update`(): Unit =
        runBlocking {
            val doc = createTestSourceDocument(mimeType = "text/plain")
            repository.save(doc)

            val updated = doc.copy(mimeType = "text/x-custom'with''multiple'''quotes")
            val result = repository.save(updated)

            assertTrue(result.isRight(), "Should update source document with multiple single quotes in mimeType")
            result.onRight { saved ->
                assertEquals("text/x-custom'with''multiple'''quotes", saved.mimeType)
            }
            Unit
        }

    @Test
    fun `save handles null mimeType`(): Unit =
        runBlocking {
            val doc = createTestSourceDocument(mimeType = null)

            val result = repository.save(doc)

            assertTrue(result.isRight(), "Should save source document with null mimeType")
            result.onRight { saved ->
                assertEquals(null, saved.mimeType)
            }
            Unit
        }

    @Test
    fun `save handles mimeType with complex boundary parameters`(): Unit =
        runBlocking {
            val complexMimeType = "multipart/form-data; boundary='----=_Part_123_456.789'; charset=utf-8"
            val doc = createTestSourceDocument(mimeType = complexMimeType)

            val result = repository.save(doc)

            assertTrue(result.isRight(), "Should save source document with complex mimeType")
            result.onRight { saved ->
                assertEquals(complexMimeType, saved.mimeType)
            }
            Unit
        }

    private fun createTestSourceDocument(
        title: String = "Test Document",
        mimeType: String? = "text/plain",
    ): SourceDocument {
        val now = Clock.System.now()
        return SourceDocument(
            id = Ulid.generate(),
            userId = testUserId,
            title = title,
            sourceType = SourceType.FILE,
            sourcePath = "/documents/${Ulid.generate().value}/test.txt",
            mimeType = mimeType,
            fileSizeBytes = 2048,
            pageCount = 1,
            extractionStatus = ExtractionStatus.PENDING,
            extractedText = null,
            watchedFolderId = null,
            initiativeId = null,
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
