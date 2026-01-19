package com.getaltair.altair.db.repository

import com.getaltair.altair.db.MigrationRunner
import com.getaltair.altair.db.SurrealDbClient
import com.getaltair.altair.db.SurrealDbTestContainer
import com.getaltair.altair.domain.ItemError
import com.getaltair.altair.domain.model.tracking.Container
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.AfterAll
import org.junit.jupiter.api.BeforeAll
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestInstance
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlin.test.assertNull
import kotlin.test.assertTrue
import kotlin.time.Clock
import org.testcontainers.junit.jupiter.Container as TestContainer

@Testcontainers
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class SurrealContainerRepositoryTest {
    private lateinit var dbClient: SurrealDbClient
    private lateinit var repository: SurrealContainerRepository
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
        repository = SurrealContainerRepository(dbClient, testUserId)
        // Clean up containers and locations before each test
        runBlocking {
            dbClient.execute("DELETE container;")
            dbClient.execute("DELETE location;")
        }
    }

    @Test
    fun `save creates new container`(): Unit =
        runBlocking {
            val container = createTestContainer()

            val result = repository.save(container)

            assertTrue(result.isRight())
            result.onRight { saved ->
                assertEquals(container.id, saved.id)
                assertEquals(container.name, saved.name)
            }
        }

    @Test
    fun `findById returns saved container`(): Unit =
        runBlocking {
            val container = createTestContainer()
            repository.save(container)

            val result = repository.findById(container.id)

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(container.id, found.id)
                assertEquals(container.name, found.name)
            }
        }

    @Test
    fun `findById returns error for non-existent container`(): Unit =
        runBlocking {
            val result = repository.findById(Ulid.generate())

            assertTrue(result.isLeft())
            result.onLeft { error ->
                assertIs<ItemError.NotFound>(error)
            }
        }

    @Test
    fun `nestInContainer creates parent-child relationship`(): Unit =
        runBlocking {
            val parentContainer = createTestContainer(name = "Parent Box")
            val childContainer = createTestContainer(name = "Child Box")
            repository.save(parentContainer)
            repository.save(childContainer)

            val result = repository.nestInContainer(childContainer.id, parentContainer.id)

            assertTrue(result.isRight())
            result.onRight { nested ->
                assertEquals(parentContainer.id, nested.parentContainerId)
            }
        }

    @Test
    fun `findByParentContainer returns nested containers`(): Unit =
        runBlocking {
            val parentContainer = createTestContainer(name = "Parent Box")
            val childContainer1 = createTestContainer(name = "Child 1")
            val childContainer2 = createTestContainer(name = "Child 2")
            val otherContainer = createTestContainer(name = "Other Box")

            repository.save(parentContainer)
            repository.save(childContainer1)
            repository.save(childContainer2)
            repository.save(otherContainer)

            repository.nestInContainer(childContainer1.id, parentContainer.id)
            repository.nestInContainer(childContainer2.id, parentContainer.id)

            val children = repository.findByParentContainer(parentContainer.id).first()

            assertEquals(2, children.size)
            assertTrue(children.all { it.parentContainerId == parentContainer.id })
        }

    @Test
    fun `unnest removes parent relationship`(): Unit =
        runBlocking {
            val parentContainer = createTestContainer(name = "Parent Box")
            val childContainer = createTestContainer(name = "Child Box")
            repository.save(parentContainer)
            repository.save(childContainer)
            repository.nestInContainer(childContainer.id, parentContainer.id)

            val result = repository.unnest(childContainer.id)

            assertTrue(result.isRight())
            result.onRight { unnested ->
                assertNull(unnested.parentContainerId)
            }
        }

    @Test
    fun `findRoots returns only containers without parent`(): Unit =
        runBlocking {
            val rootContainer1 = createTestContainer(name = "Root 1")
            val rootContainer2 = createTestContainer(name = "Root 2")
            val nestedContainer = createTestContainer(name = "Nested")

            repository.save(rootContainer1)
            repository.save(rootContainer2)
            repository.save(nestedContainer)
            repository.nestInContainer(nestedContainer.id, rootContainer1.id)

            val roots = repository.findRoots().first()

            assertEquals(2, roots.size)
            assertTrue(roots.all { it.parentContainerId == null })
        }

    @Test
    fun `moveToLocation updates container location`(): Unit =
        runBlocking {
            // Create a location
            val locationId = Ulid.generate()
            dbClient.execute(
                "CREATE location:${locationId.value} CONTENT { " +
                    "user_id: user:${testUserId.value}, name: 'Test Location' };",
            )

            val container = createTestContainer()
            repository.save(container)

            val result = repository.moveToLocation(container.id, locationId)

            assertTrue(result.isRight())
            result.onRight { moved ->
                assertEquals(locationId, moved.locationId)
            }
        }

    @Test
    fun `moveToLocation removes container from parent`(): Unit =
        runBlocking {
            val parentContainer = createTestContainer(name = "Parent")
            val childContainer = createTestContainer(name = "Child")
            repository.save(parentContainer)
            repository.save(childContainer)
            repository.nestInContainer(childContainer.id, parentContainer.id)

            val locationId = Ulid.generate()
            dbClient.execute(
                "CREATE location:${locationId.value} CONTENT { " +
                    "user_id: user:${testUserId.value}, name: 'New Location' };",
            )

            val result = repository.moveToLocation(childContainer.id, locationId)

            assertTrue(result.isRight())
            result.onRight { moved ->
                assertEquals(locationId, moved.locationId)
                assertNull(moved.parentContainerId)
            }
        }

    @Test
    fun `findByLocation returns containers at location`(): Unit =
        runBlocking {
            val locationId = Ulid.generate()
            dbClient.execute(
                "CREATE location:${locationId.value} CONTENT { " +
                    "user_id: user:${testUserId.value}, name: 'Test Location' };",
            )

            val containerAtLocation = createTestContainer(name = "At Location")
            val containerElsewhere = createTestContainer(name = "Elsewhere")

            repository.save(containerAtLocation)
            repository.save(containerElsewhere)
            repository.moveToLocation(containerAtLocation.id, locationId)

            val found = repository.findByLocation(locationId).first()

            assertEquals(1, found.size)
            assertEquals("At Location", found.first().name)
        }

    @Test
    fun `searchByNameOrLabel finds by name`(): Unit =
        runBlocking {
            val container1 = createTestContainer(name = "Kitchen Box", label = "K-001")
            val container2 = createTestContainer(name = "Bedroom Box", label = "B-001")
            repository.save(container1)
            repository.save(container2)

            val result = repository.searchByNameOrLabel("Kitchen")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(1, found.size)
                assertEquals("Kitchen Box", found.first().name)
            }
        }

    @Test
    fun `searchByNameOrLabel finds by label`(): Unit =
        runBlocking {
            val container1 = createTestContainer(name = "Box 1", label = "STORAGE-A")
            val container2 = createTestContainer(name = "Box 2", label = "STORAGE-B")
            repository.save(container1)
            repository.save(container2)

            val result = repository.searchByNameOrLabel("STORAGE-A")

            assertTrue(result.isRight())
            result.onRight { found ->
                assertEquals(1, found.size)
                assertEquals("Box 1", found.first().name)
            }
        }

    @Test
    fun `getPath returns container hierarchy`(): Unit =
        runBlocking {
            val grandparent = createTestContainer(name = "Grandparent")
            val parent = createTestContainer(name = "Parent")
            val child = createTestContainer(name = "Child")

            repository.save(grandparent)
            repository.save(parent)
            repository.save(child)

            repository.nestInContainer(parent.id, grandparent.id)
            repository.nestInContainer(child.id, parent.id)

            val result = repository.getPath(child.id)

            assertTrue(result.isRight())
            result.onRight { path ->
                assertEquals(3, path.size)
                assertEquals("Grandparent", path[0].name)
                assertEquals("Parent", path[1].name)
                assertEquals("Child", path[2].name)
            }
        }

    @Test
    fun `delete soft deletes container`(): Unit =
        runBlocking {
            val container = createTestContainer()
            repository.save(container)

            val deleteResult = repository.delete(container.id)
            assertTrue(deleteResult.isRight())

            val findResult = repository.findById(container.id)
            assertTrue(findResult.isLeft())
        }

    @Test
    fun `findAll returns only non-deleted containers`(): Unit =
        runBlocking {
            val container1 = createTestContainer(name = "Container 1")
            val container2 = createTestContainer(name = "Container 2")
            val container3 = createTestContainer(name = "Container 3")

            repository.save(container1)
            repository.save(container2)
            repository.save(container3)
            repository.delete(container3.id)

            val all = repository.findAll().first()

            assertEquals(2, all.size)
        }

    private fun createTestContainer(
        name: String = "Test Container",
        description: String? = "Test description",
        label: String? = null,
    ): Container {
        val now = Clock.System.now()
        return Container(
            id = Ulid.generate(),
            userId = testUserId,
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

    companion object {
        @TestContainer
        val container = SurrealDbTestContainer()
    }
}
