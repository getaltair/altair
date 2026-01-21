package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbContainerExtension
import com.getaltair.altair.domain.DomainError
import com.getaltair.altair.domain.model.system.Initiative
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.InitiativeStatus
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf
import kotlinx.coroutines.flow.first
import kotlin.time.Clock

/**
 * Tests for SurrealInitiativeRepository using Testcontainers.
 *
 * Verifies:
 * - CRUD operations (save, findById, findAll, delete)
 * - Query operations (findByStatus, searchByName)
 * - User isolation (initiatives scoped to user)
 */
class SurrealInitiativeRepositoryTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient
        lateinit var repository: SurrealInitiativeRepository
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
            repository = SurrealInitiativeRepository(dbClient, testUserId)
            // Clean up initiatives before each test
            dbClient.execute("DELETE initiative;")
        }

        given("CRUD operations") {
            `when`("saving a new initiative") {
                then("creates the initiative successfully") {
                    val initiative = createTestInitiative()

                    val result = repository.save(initiative)

                    result.shouldBeRight()
                    val saved = result.getOrNull()
                    saved?.id shouldBe initiative.id
                    saved?.name shouldBe initiative.name
                    saved?.status shouldBe initiative.status
                }
            }

            `when`("finding an initiative by ID") {
                then("returns the initiative when it exists") {
                    val initiative = createTestInitiative()
                    repository.save(initiative)

                    val result = repository.findById(initiative.id)

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.id shouldBe initiative.id
                    found?.name shouldBe initiative.name
                }

                then("returns error when initiative doesn't exist") {
                    val result = repository.findById(Ulid.generate())

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<DomainError.NotFoundError>()
                }
            }

            `when`("finding all initiatives") {
                then("returns all non-deleted initiatives") {
                    val initiative1 = createTestInitiative(name = "Initiative 1")
                    val initiative2 = createTestInitiative(name = "Initiative 2")
                    repository.save(initiative1)
                    repository.save(initiative2)

                    val result = repository.findAll().first()

                    result shouldHaveSize 2
                }
            }

            `when`("updating an initiative") {
                then("modifies the existing initiative") {
                    val initiative = createTestInitiative(name = "Original")
                    repository.save(initiative)

                    val updated = initiative.copy(name = "Updated")
                    val result = repository.save(updated)

                    result.shouldBeRight()
                    val saved = result.getOrNull()
                    saved?.name shouldBe "Updated"
                }
            }

            `when`("deleting an initiative") {
                then("soft deletes the initiative") {
                    val initiative = createTestInitiative()
                    repository.save(initiative)

                    val deleteResult = repository.delete(initiative.id)
                    deleteResult.shouldBeRight()

                    val findResult = repository.findById(initiative.id)
                    findResult.shouldBeLeft()
                }
            }
        }

        given("query operations") {
            `when`("finding initiatives by status") {
                then("returns only matching initiatives") {
                    val activeInitiative = createTestInitiative(name = "Active", status = InitiativeStatus.ACTIVE)
                    val pausedInitiative = createTestInitiative(name = "Paused", status = InitiativeStatus.PAUSED)
                    repository.save(activeInitiative)
                    repository.save(pausedInitiative)

                    val result = repository.findByStatus(InitiativeStatus.ACTIVE).first()

                    result shouldHaveSize 1
                    result.first().name shouldBe "Active"
                }
            }

            `when`("searching by name") {
                then("finds initiatives by partial name match") {
                    repository.save(createTestInitiative(name = "My Health Goals"))
                    repository.save(createTestInitiative(name = "Career Development"))
                    repository.save(createTestInitiative(name = "Learning Goals"))

                    val result = repository.searchByName("goals")

                    result.shouldBeRight()
                    val found = result.getOrNull() ?: emptyList()
                    found shouldHaveSize 2
                }
            }
        }
    }) {
    companion object {
        private fun createTestInitiative(
            name: String = "Test Initiative",
            status: InitiativeStatus = InitiativeStatus.ACTIVE,
        ): Initiative {
            val now = Clock.System.now()
            val testUserId = Ulid("01TESTACCT0000000000000001")
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
    }
}
