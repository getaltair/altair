package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbTestContainer
import com.getaltair.altair.domain.QuestError
import com.getaltair.altair.domain.model.guidance.Quest
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.QuestStatus
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
import kotlin.test.assertIs
import kotlin.test.assertTrue
import kotlin.time.Clock

@Testcontainers
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class SurrealQuestRepositoryTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var repository: SurrealQuestRepository
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
        repository = SurrealQuestRepository(dbClient, testUserId)
        // Clean up quests before each test
        runBlocking {
            dbClient.execute("DELETE quest;")
        }
    }

    @Test
    fun `save creates new quest`() =
        runBlocking {
            val quest = createTestQuest()

            val result = repository.save(quest)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals(quest.id, saved.id)
                assertEquals(quest.title, saved.title)
                assertEquals(quest.status, saved.status)
            }
        }

    @Test
    fun `findById returns saved quest`() =
        runBlocking {
            val quest = createTestQuest()
            repository.save(quest)

            val result = repository.findById(quest.id)

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(quest.id, found.id)
                assertEquals(quest.title, found.title)
            }
        }

    @Test
    fun `findById returns error for non-existent quest`() =
        runBlocking {
            val result = repository.findById(Ulid.generate())

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<QuestError.NotFound>(error)
            }
        }

    @Test
    fun `transitionStatus from BACKLOG to ACTIVE succeeds`() =
        runBlocking {
            val quest = createTestQuest(status = QuestStatus.BACKLOG)
            repository.save(quest)

            val result = repository.transitionStatus(quest.id, QuestStatus.ACTIVE)

            assertTrue(result.isRight())
            result.onRight { updated ->
                assertEquals(QuestStatus.ACTIVE, updated.status)
            }
        }

    @Test
    fun `transitionStatus from BACKLOG to COMPLETED fails`() =
        runBlocking {
            val quest = createTestQuest(status = QuestStatus.BACKLOG)
            repository.save(quest)

            val result = repository.transitionStatus(quest.id, QuestStatus.COMPLETED)

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<QuestError.InvalidStatusTransition>(error)
                assertEquals(QuestStatus.BACKLOG, error.currentStatus)
                assertEquals(QuestStatus.COMPLETED, error.targetStatus)
            }
        }

    @Test
    fun `transitionStatus from ACTIVE to COMPLETED succeeds`() =
        runBlocking {
            val quest = createTestQuest(status = QuestStatus.BACKLOG)
            repository.save(quest)
            repository.transitionStatus(quest.id, QuestStatus.ACTIVE)

            val result = repository.transitionStatus(quest.id, QuestStatus.COMPLETED)

            assertTrue(result.isRight())
            result.onRight { updated ->
                assertEquals(QuestStatus.COMPLETED, updated.status)
            }
        }

    @Test
    fun `transitionStatus from ACTIVE to ABANDONED succeeds`() =
        runBlocking {
            val quest = createTestQuest(status = QuestStatus.BACKLOG)
            repository.save(quest)
            repository.transitionStatus(quest.id, QuestStatus.ACTIVE)

            val result = repository.transitionStatus(quest.id, QuestStatus.ABANDONED)

            assertTrue(result.isRight())
            result.onRight { updated ->
                assertEquals(QuestStatus.ABANDONED, updated.status)
            }
        }

    @Test
    fun `transitionStatus enforces WIP limit when activating quest`() =
        runBlocking {
            // Create and activate WIP_LIMIT quests
            repeat(DEFAULT_WIP_LIMIT) { i ->
                val quest = createTestQuest(title = "Active Quest $i", status = QuestStatus.BACKLOG)
                repository.save(quest)
                repository.transitionStatus(quest.id, QuestStatus.ACTIVE)
            }

            // Verify we have WIP_LIMIT active quests
            val activeCount = repository.countActive().getOrNull()
            assertEquals(DEFAULT_WIP_LIMIT, activeCount)

            // Try to activate one more quest - should fail
            val oneMoreQuest = createTestQuest(title = "One More Quest", status = QuestStatus.BACKLOG)
            repository.save(oneMoreQuest)

            val result = repository.transitionStatus(oneMoreQuest.id, QuestStatus.ACTIVE)

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<QuestError.WipLimitExceeded>(error)
                assertEquals(DEFAULT_WIP_LIMIT, error.currentWip)
                assertEquals(DEFAULT_WIP_LIMIT, error.maxWip)
            }
        }

    @Test
    fun `countActive returns correct count`() =
        runBlocking {
            // Create mix of statuses
            val backlogQuest = createTestQuest(title = "Backlog Quest", status = QuestStatus.BACKLOG)
            repository.save(backlogQuest)

            val activeQuest1 = createTestQuest(title = "Active Quest 1", status = QuestStatus.BACKLOG)
            repository.save(activeQuest1)
            repository.transitionStatus(activeQuest1.id, QuestStatus.ACTIVE)

            val activeQuest2 = createTestQuest(title = "Active Quest 2", status = QuestStatus.BACKLOG)
            repository.save(activeQuest2)
            repository.transitionStatus(activeQuest2.id, QuestStatus.ACTIVE)

            val result = repository.countActive()

            assertTrue(result.isRight())
            result.onRight { count ->
                assertEquals(2, count)
            }
        }

    @Test
    fun `findByStatus returns only matching quests`() =
        runBlocking {
            val backlogQuest = createTestQuest(title = "Backlog", status = QuestStatus.BACKLOG)
            val activeQuest = createTestQuest(title = "Active", status = QuestStatus.BACKLOG)
            repository.save(backlogQuest)
            repository.save(activeQuest)
            repository.transitionStatus(activeQuest.id, QuestStatus.ACTIVE)

            val activeQuests = repository.findByStatus(QuestStatus.ACTIVE).first()

            assertEquals(1, activeQuests.size)
            assertEquals("Active", activeQuests.first().title)
        }

    @Test
    fun `delete soft deletes quest`() =
        runBlocking {
            val quest = createTestQuest()
            repository.save(quest)

            val deleteResult = repository.delete(quest.id)
            assertTrue(deleteResult.isRight())

            val findResult = repository.findById(quest.id)
            assertTrue(findResult.isLeft())
        }

    @Test
    fun `WIP limit allows activation after completing an active quest`() =
        runBlocking {
            // Fill up WIP limit
            val activeQuests =
                (1..DEFAULT_WIP_LIMIT).map { i ->
                    val quest = createTestQuest(title = "Active Quest $i", status = QuestStatus.BACKLOG)
                    repository.save(quest)
                    repository.transitionStatus(quest.id, QuestStatus.ACTIVE)
                    quest
                }

            // Complete one quest
            repository.transitionStatus(activeQuests.first().id, QuestStatus.COMPLETED)

            // Now we should be able to activate another
            val newQuest = createTestQuest(title = "New Quest", status = QuestStatus.BACKLOG)
            repository.save(newQuest)

            val result = repository.transitionStatus(newQuest.id, QuestStatus.ACTIVE)

            assertTrue(result.isRight())
            result.onRight { updated ->
                assertEquals(QuestStatus.ACTIVE, updated.status)
            }
        }

    private fun createTestQuest(
        title: String = "Test Quest",
        status: QuestStatus = QuestStatus.BACKLOG,
    ): Quest {
        val now = Clock.System.now()
        return Quest(
            id = Ulid.generate(),
            userId = testUserId,
            title = title,
            description = "Test description",
            energyCost = 2,
            status = status,
            epicId = null,
            routineId = null,
            initiativeId = null,
            dueDate = null,
            scheduledDate = null,
            createdAt = now,
            updatedAt = now,
            startedAt = null,
            completedAt = null,
            deletedAt = null,
        )
    }

    companion object {
        @Container
        val container = SurrealDbTestContainer()

        private const val DEFAULT_WIP_LIMIT = 5
    }
}
