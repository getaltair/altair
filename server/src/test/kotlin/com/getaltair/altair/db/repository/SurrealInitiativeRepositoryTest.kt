package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbTestContainer
import com.getaltair.altair.domain.model.system.Initiative
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.InitiativeStatus
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
class SurrealInitiativeRepositoryTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var repository: SurrealInitiativeRepository
    private val testUserId = Ulid("01TESTACCT00000000000000")

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
                    "email: 'test@test.com', hashed_password: 'hash', " +
                    "display_name: 'Test User', role: 'member' };",
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
        repository = SurrealInitiativeRepository(dbClient, testUserId)
        // Clean up initiatives before each test
        runBlocking {
            dbClient.execute("DELETE initiative;")
        }
    }

    @Test
    fun `save creates new initiative`() =
        runBlocking {
            val initiative = createTestInitiative()

            val result = repository.save(initiative)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals(initiative.id, saved.id)
                assertEquals(initiative.name, saved.name)
                assertEquals(initiative.status, saved.status)
            }
        }

    @Test
    fun `findById returns saved initiative`() =
        runBlocking {
            val initiative = createTestInitiative()
            repository.save(initiative)

            val result = repository.findById(initiative.id)

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(initiative.id, found.id)
                assertEquals(initiative.name, found.name)
            }
        }

    @Test
    fun `findById returns error for non-existent initiative`() =
        runBlocking {
            val result = repository.findById(Ulid.generate())

            assertTrue(result.isLeft())
        }

    @Test
    fun `findAll returns all initiatives`() =
        runBlocking {
            val initiative1 = createTestInitiative(name = "Initiative 1")
            val initiative2 = createTestInitiative(name = "Initiative 2")
            repository.save(initiative1)
            repository.save(initiative2)

            val result = repository.findAll().first()

            assertEquals(2, result.size)
        }

    @Test
    fun `findByStatus returns only matching initiatives`() =
        runBlocking {
            val activeInitiative = createTestInitiative(name = "Active", status = InitiativeStatus.ACTIVE)
            val pausedInitiative = createTestInitiative(name = "Paused", status = InitiativeStatus.PAUSED)
            repository.save(activeInitiative)
            repository.save(pausedInitiative)

            val result = repository.findByStatus(InitiativeStatus.ACTIVE).first()

            assertEquals(1, result.size)
            assertEquals("Active", result.first().name)
        }

    @Test
    fun `delete soft deletes initiative`() =
        runBlocking {
            val initiative = createTestInitiative()
            repository.save(initiative)

            val deleteResult = repository.delete(initiative.id)
            assertTrue(deleteResult.isRight())

            val findResult = repository.findById(initiative.id)
            assertTrue(findResult.isLeft())
        }

    @Test
    fun `update modifies existing initiative`() =
        runBlocking {
            val initiative = createTestInitiative(name = "Original")
            repository.save(initiative)

            val updated = initiative.copy(name = "Updated")
            val result = repository.save(updated)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals("Updated", saved.name)
            }
        }

    @Test
    fun `searchByName finds initiatives by partial name match`() =
        runBlocking {
            repository.save(createTestInitiative(name = "My Health Goals"))
            repository.save(createTestInitiative(name = "Career Development"))
            repository.save(createTestInitiative(name = "Learning Goals"))

            val result = repository.searchByName("goals")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(2, found.size)
            }
        }

    private fun createTestInitiative(
        name: String = "Test Initiative",
        status: InitiativeStatus = InitiativeStatus.ACTIVE,
    ): Initiative {
        val now = Clock.System.now()
        return Initiative(
            id = Ulid.generate(),
            userId = testUserId,
            name = name,
            description = "Test description",
            status = status,
            color = "#FF5733",
            icon = "target",
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
