package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbContainerExtension
import com.getaltair.altair.domain.ItemError
import com.getaltair.altair.domain.model.tracking.Container
import com.getaltair.altair.domain.types.Ulid
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.inspectors.forAll
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf
import kotlinx.coroutines.flow.first
import kotlin.time.Clock

/**
 * Tests for SurrealContainerRepository using Testcontainers.
 *
 * Verifies:
 * - CRUD operations (save, findById, delete, findAll)
 * - Container hierarchy operations (nesting, unnesting, finding children)
 * - Location management (moving containers to locations)
 * - Search functionality (by name or label)
 * - Path resolution through container hierarchies
 */
class SurrealContainerRepositoryTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient
        lateinit var repository: SurrealContainerRepository
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
            repository = SurrealContainerRepository(dbClient, testUserId)
            // Clean up containers and locations before each test
            dbClient.execute("DELETE container;")
            dbClient.execute("DELETE location;")
        }

        given("CRUD operations") {
            `when`("saving a new container") {
                then("creates the container successfully") {
                    val container = createTestContainer(testUserId)

                    val result = repository.save(container)

                    result.shouldBeRight()
                    val saved = result.getOrNull()
                    saved?.id shouldBe container.id
                    saved?.name shouldBe container.name
                }
            }

            `when`("finding a saved container by ID") {
                then("returns the container") {
                    val container = createTestContainer(testUserId)
                    repository.save(container)

                    val result = repository.findById(container.id)

                    result.shouldBeRight()
                    val found = result.getOrNull()
                    found?.id shouldBe container.id
                    found?.name shouldBe container.name
                }
            }

            `when`("finding a non-existent container") {
                then("returns NotFound error") {
                    val result = repository.findById(Ulid.generate())

                    result.shouldBeLeft()
                    result.leftOrNull().shouldBeInstanceOf<ItemError.NotFound>()
                }
            }

            `when`("deleting a container") {
                then("soft deletes the container") {
                    val container = createTestContainer(testUserId)
                    repository.save(container)

                    val deleteResult = repository.delete(container.id)
                    deleteResult.shouldBeRight()

                    val findResult = repository.findById(container.id)
                    findResult.shouldBeLeft()
                }
            }

            `when`("finding all containers") {
                then("returns only non-deleted containers") {
                    val container1 = createTestContainer(testUserId, name = "Container 1")
                    val container2 = createTestContainer(testUserId, name = "Container 2")
                    val container3 = createTestContainer(testUserId, name = "Container 3")

                    repository.save(container1)
                    repository.save(container2)
                    repository.save(container3)
                    repository.delete(container3.id)

                    val all = repository.findAll().first()

                    all shouldHaveSize 2
                }
            }
        }

        given("container hierarchy operations") {
            `when`("nesting a container in another") {
                then("creates parent-child relationship") {
                    val parentContainer = createTestContainer(testUserId, name = "Parent Box")
                    val childContainer = createTestContainer(testUserId, name = "Child Box")
                    repository.save(parentContainer)
                    repository.save(childContainer)

                    val result = repository.nestInContainer(childContainer.id, parentContainer.id)

                    result.shouldBeRight()
                    val nested = result.getOrNull()
                    nested?.parentContainerId shouldBe parentContainer.id
                }
            }

            `when`("finding containers by parent") {
                then("returns all nested containers") {
                    val parentContainer = createTestContainer(testUserId, name = "Parent Box")
                    val childContainer1 = createTestContainer(testUserId, name = "Child 1")
                    val childContainer2 = createTestContainer(testUserId, name = "Child 2")
                    val otherContainer = createTestContainer(testUserId, name = "Other Box")

                    repository.save(parentContainer)
                    repository.save(childContainer1)
                    repository.save(childContainer2)
                    repository.save(otherContainer)

                    repository.nestInContainer(childContainer1.id, parentContainer.id)
                    repository.nestInContainer(childContainer2.id, parentContainer.id)

                    val children = repository.findByParentContainer(parentContainer.id).first()

                    children shouldHaveSize 2
                    children.forAll { it.parentContainerId shouldBe parentContainer.id }
                }
            }

            `when`("unnesting a container") {
                then("removes parent relationship") {
                    val parentContainer = createTestContainer(testUserId, name = "Parent Box")
                    val childContainer = createTestContainer(testUserId, name = "Child Box")
                    repository.save(parentContainer)
                    repository.save(childContainer)
                    repository.nestInContainer(childContainer.id, parentContainer.id)

                    val result = repository.unnest(childContainer.id)

                    result.shouldBeRight()
                    val unnested = result.getOrNull()
                    unnested?.parentContainerId.shouldBeNull()
                }
            }

            `when`("finding root containers") {
                then("returns only containers without parent") {
                    val rootContainer1 = createTestContainer(testUserId, name = "Root 1")
                    val rootContainer2 = createTestContainer(testUserId, name = "Root 2")
                    val nestedContainer = createTestContainer(testUserId, name = "Nested")

                    repository.save(rootContainer1)
                    repository.save(rootContainer2)
                    repository.save(nestedContainer)
                    repository.nestInContainer(nestedContainer.id, rootContainer1.id)

                    val roots = repository.findRoots().first()

                    roots shouldHaveSize 2
                    roots.forAll { it.parentContainerId.shouldBeNull() }
                }
            }

            `when`("getting container path") {
                then("returns complete hierarchy from root to container") {
                    val grandparent = createTestContainer(testUserId, name = "Grandparent")
                    val parent = createTestContainer(testUserId, name = "Parent")
                    val child = createTestContainer(testUserId, name = "Child")

                    repository.save(grandparent)
                    repository.save(parent)
                    repository.save(child)

                    repository.nestInContainer(parent.id, grandparent.id)
                    repository.nestInContainer(child.id, parent.id)

                    val result = repository.getPath(child.id)

                    result.shouldBeRight()
                    val path = result.getOrNull() ?: emptyList()
                    path shouldHaveSize 3
                    path[0].name shouldBe "Grandparent"
                    path[1].name shouldBe "Parent"
                    path[2].name shouldBe "Child"
                }
            }
        }

        given("location operations") {
            `when`("moving container to a location") {
                then("updates container location") {
                    val locationId = Ulid.generate()
                    dbClient.execute(
                        "CREATE location:${locationId.value} CONTENT { " +
                            "user_id: user:${testUserId.value}, name: 'Test Location' };",
                    )

                    val container = createTestContainer(testUserId)
                    repository.save(container)

                    val result = repository.moveToLocation(container.id, locationId)

                    result.shouldBeRight()
                    val moved = result.getOrNull()
                    moved?.locationId shouldBe locationId
                }
            }

            `when`("moving nested container to location") {
                then("removes container from parent") {
                    val parentContainer = createTestContainer(testUserId, name = "Parent")
                    val childContainer = createTestContainer(testUserId, name = "Child")
                    repository.save(parentContainer)
                    repository.save(childContainer)
                    repository.nestInContainer(childContainer.id, parentContainer.id)

                    val locationId = Ulid.generate()
                    dbClient.execute(
                        "CREATE location:${locationId.value} CONTENT { " +
                            "user_id: user:${testUserId.value}, name: 'New Location' };",
                    )

                    val result = repository.moveToLocation(childContainer.id, locationId)

                    result.shouldBeRight()
                    val moved = result.getOrNull()
                    moved?.locationId shouldBe locationId
                    moved?.parentContainerId.shouldBeNull()
                }
            }

            `when`("finding containers by location") {
                then("returns only containers at that location") {
                    val locationId = Ulid.generate()
                    dbClient.execute(
                        "CREATE location:${locationId.value} CONTENT { " +
                            "user_id: user:${testUserId.value}, name: 'Test Location' };",
                    )

                    val containerAtLocation = createTestContainer(testUserId, name = "At Location")
                    val containerElsewhere = createTestContainer(testUserId, name = "Elsewhere")

                    repository.save(containerAtLocation)
                    repository.save(containerElsewhere)
                    repository.moveToLocation(containerAtLocation.id, locationId)

                    val found = repository.findByLocation(locationId).first()

                    found shouldHaveSize 1
                    found.first().name shouldBe "At Location"
                }
            }
        }

        given("search operations") {
            `when`("searching by name") {
                then("finds containers matching name") {
                    val container1 = createTestContainer(testUserId, name = "Kitchen Box", label = "K-001")
                    val container2 = createTestContainer(testUserId, name = "Bedroom Box", label = "B-001")
                    repository.save(container1)
                    repository.save(container2)

                    val result = repository.searchByNameOrLabel("Kitchen")

                    result.shouldBeRight()
                    val found = result.getOrNull() ?: emptyList()
                    found shouldHaveSize 1
                    found.first().name shouldBe "Kitchen Box"
                }
            }

            `when`("searching by label") {
                then("finds containers matching label") {
                    val container1 = createTestContainer(testUserId, name = "Box 1", label = "STORAGE-A")
                    val container2 = createTestContainer(testUserId, name = "Box 2", label = "STORAGE-B")
                    repository.save(container1)
                    repository.save(container2)

                    val result = repository.searchByNameOrLabel("STORAGE-A")

                    result.shouldBeRight()
                    val found = result.getOrNull() ?: emptyList()
                    found shouldHaveSize 1
                    found.first().name shouldBe "Box 1"
                }
            }
        }
    }) {
    companion object {
        private fun createTestContainer(
            userId: Ulid,
            name: String = "Test Container",
            description: String? = "Test description",
            label: String? = null,
        ): Container {
            val now = Clock.System.now()
            return Container(
                id = Ulid.generate(),
                userId = userId,
                name = name,
                description = description,
                locationId = null,
                parentContainerId = null,
                label = label,
                createdAt = now,
                updatedAt = now,
                deletedAt = null,
            )
        }
    }
}
