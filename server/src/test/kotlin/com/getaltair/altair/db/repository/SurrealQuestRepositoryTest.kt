package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbContainerExtension
import com.getaltair.altair.domain.QuestError
import com.getaltair.altair.domain.model.guidance.Quest
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.QuestStatus
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf
import kotlinx.coroutines.flow.first
import kotlin.time.Clock

/**
 * Tests for SurrealQuestRepository using Testcontainers.
 *
 * Verifies:
 * - CRUD operations (save, findById, delete)
 * - Status transition state machine (valid and invalid transitions)
 * - WIP limit enforcement (prevents too many active quests)
 * - Query operations (findByStatus, countActive)
 */
class SurrealQuestRepositoryTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient
        lateinit var repository: SurrealQuestRepository
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
            repository = SurrealQuestRepository(dbClient, testUserId)
            // Clean up quests before each test
            dbClient.execute("DELETE quest;")
        }

        given("CRUD operations") {
            `when`("saving a new quest") {
                then("creates the quest successfully") {
                    val quest = createTestQuest(testUserId)

                    val result = repository.save(quest)

                    result.shouldBeRight()
                    val saved = result.getOrNull()
                    saved?.id shouldBe quest.id
                    saved?.title shouldBe quest.title
                    saved?.status shouldBe quest.status
                }
            }

            `when`("finding a saved quest by ID") {
                then("returns the quest") {
                    val quest = createTestQuest(testUserId)
                    repository.save(quest)

                    val result = repository.findById(quest.id)

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.id shouldBe quest.id
                    found?.title shouldBe quest.title
                }
            }

            `when`("finding a non-existent quest") {
                then("returns NotFound error") {
                    val result = repository.findById(Ulid.generate())

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<QuestError.NotFound>()
                }
            }

            `when`("deleting a quest") {
                then("soft deletes the quest") {
                    val quest = createTestQuest(testUserId)
                    repository.save(quest)

                    val deleteResult = repository.delete(quest.id)
                    deleteResult.shouldBeRight()

                    val findResult = repository.findById(quest.id)
                    findResult.shouldBeLeft()
                }
            }
        }

        given("status transitions") {
            `when`("transitioning from BACKLOG to ACTIVE") {
                then("succeeds") {
                    val quest = createTestQuest(testUserId, status = QuestStatus.BACKLOG)
                    repository.save(quest)

                    val result = repository.transitionStatus(quest.id, QuestStatus.ACTIVE)

                    result.shouldBeRight()
                    val updated = result.getOrNull()
                    updated?.status shouldBe QuestStatus.ACTIVE
                }
            }

            `when`("transitioning from ACTIVE to COMPLETED") {
                then("succeeds") {
                    val quest = createTestQuest(testUserId, status = QuestStatus.BACKLOG)
                    repository.save(quest)
                    repository.transitionStatus(quest.id, QuestStatus.ACTIVE)

                    val result = repository.transitionStatus(quest.id, QuestStatus.COMPLETED)

                    result.shouldBeRight()
                    val updated = result.getOrNull()
                    updated?.status shouldBe QuestStatus.COMPLETED
                }
            }

            `when`("transitioning from ACTIVE to ABANDONED") {
                then("succeeds") {
                    val quest = createTestQuest(testUserId, status = QuestStatus.BACKLOG)
                    repository.save(quest)
                    repository.transitionStatus(quest.id, QuestStatus.ACTIVE)

                    val result = repository.transitionStatus(quest.id, QuestStatus.ABANDONED)

                    result.shouldBeRight()
                    val updated = result.getOrNull()
                    updated?.status shouldBe QuestStatus.ABANDONED
                }
            }

            `when`("transitioning from BACKLOG to COMPLETED") {
                then("fails with InvalidStatusTransition") {
                    val quest = createTestQuest(testUserId, status = QuestStatus.BACKLOG)
                    repository.save(quest)

                    val result = repository.transitionStatus(quest.id, QuestStatus.COMPLETED)

                    result.shouldBeLeft()
                    val error = result.leftOrNull()
                    error.shouldBeInstanceOf<QuestError.InvalidStatusTransition>()
                    (error as QuestError.InvalidStatusTransition).currentStatus shouldBe QuestStatus.BACKLOG
                    error.targetStatus shouldBe QuestStatus.COMPLETED
                }
            }

            `when`("transitioning from BACKLOG to ABANDONED") {
                then("fails with InvalidStatusTransition") {
                    val quest = createTestQuest(testUserId, status = QuestStatus.BACKLOG)
                    repository.save(quest)

                    val result = repository.transitionStatus(quest.id, QuestStatus.ABANDONED)

                    result.shouldBeLeft()
                    val error = result.leftOrNull()
                    error.shouldBeInstanceOf<QuestError.InvalidStatusTransition>()
                    (error as QuestError.InvalidStatusTransition).currentStatus shouldBe QuestStatus.BACKLOG
                    error.targetStatus shouldBe QuestStatus.ABANDONED
                }
            }
        }

        given("WIP limit enforcement") {
            `when`("activating a quest when at WIP limit") {
                then("rejects the activation") {
                    // Create and activate WIP_LIMIT quests
                    repeat(DEFAULT_WIP_LIMIT) { i ->
                        val quest = createTestQuest(testUserId, title = "Active Quest $i", status = QuestStatus.BACKLOG)
                        repository.save(quest)
                        repository.transitionStatus(quest.id, QuestStatus.ACTIVE)
                    }

                    // Verify we have WIP_LIMIT active quests
                    val activeCount = repository.countActive().getOrNull()
                    activeCount shouldBe DEFAULT_WIP_LIMIT

                    // Try to activate one more quest - should fail
                    val oneMoreQuest =
                        createTestQuest(testUserId, title = "One More Quest", status = QuestStatus.BACKLOG)
                    repository.save(oneMoreQuest)

                    val result = repository.transitionStatus(oneMoreQuest.id, QuestStatus.ACTIVE)

                    result.shouldBeLeft()
                    val error = result.leftOrNull()
                    error.shouldBeInstanceOf<QuestError.WipLimitExceeded>()
                    (error as QuestError.WipLimitExceeded).currentWip shouldBe DEFAULT_WIP_LIMIT
                    error.maxWip shouldBe DEFAULT_WIP_LIMIT
                }
            }

            `when`("completing an active quest") {
                then("allows activating another quest") {
                    // Fill up WIP limit
                    val activeQuests =
                        (1..DEFAULT_WIP_LIMIT).map { i ->
                            val quest =
                                createTestQuest(testUserId, title = "Active Quest $i", status = QuestStatus.BACKLOG)
                            repository.save(quest)
                            repository.transitionStatus(quest.id, QuestStatus.ACTIVE)
                            quest
                        }

                    // Complete one quest
                    repository.transitionStatus(activeQuests.first().id, QuestStatus.COMPLETED)

                    // Now we should be able to activate another
                    val newQuest = createTestQuest(testUserId, title = "New Quest", status = QuestStatus.BACKLOG)
                    repository.save(newQuest)

                    val result = repository.transitionStatus(newQuest.id, QuestStatus.ACTIVE)

                    result.shouldBeRight()
                    val updated = result.getOrNull()
                    updated?.status shouldBe QuestStatus.ACTIVE
                }
            }
        }

        given("query operations") {
            `when`("counting active quests") {
                then("returns correct count") {
                    // Create mix of statuses
                    val backlogQuest =
                        createTestQuest(testUserId, title = "Backlog Quest", status = QuestStatus.BACKLOG)
                    repository.save(backlogQuest)

                    val activeQuest1 =
                        createTestQuest(testUserId, title = "Active Quest 1", status = QuestStatus.BACKLOG)
                    repository.save(activeQuest1)
                    repository.transitionStatus(activeQuest1.id, QuestStatus.ACTIVE)

                    val activeQuest2 =
                        createTestQuest(testUserId, title = "Active Quest 2", status = QuestStatus.BACKLOG)
                    repository.save(activeQuest2)
                    repository.transitionStatus(activeQuest2.id, QuestStatus.ACTIVE)

                    val result = repository.countActive()

                    result.shouldBeRight()
                    val count = result.getOrNull() ?: 0
                    count shouldBe 2
                }
            }

            `when`("finding quests by status") {
                then("returns only matching quests") {
                    val backlogQuest = createTestQuest(testUserId, title = "Backlog", status = QuestStatus.BACKLOG)
                    val activeQuest = createTestQuest(testUserId, title = "Active", status = QuestStatus.BACKLOG)
                    repository.save(backlogQuest)
                    repository.save(activeQuest)
                    repository.transitionStatus(activeQuest.id, QuestStatus.ACTIVE)

                    val activeQuests = repository.findByStatus(QuestStatus.ACTIVE).first()

                    activeQuests shouldHaveSize 1
                    activeQuests.first().title shouldBe "Active"
                }
            }
        }
    }) {
    companion object {
        private const val DEFAULT_WIP_LIMIT = 5

        private fun createTestQuest(
            userId: Ulid,
            title: String = "Test Quest",
            status: QuestStatus = QuestStatus.BACKLOG,
        ): Quest {
            val now = Clock.System.now()
            return Quest(
                id = Ulid.generate(),
                userId = userId,
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
    }
}
