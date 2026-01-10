package com.getaltair.altair.data.repository.mobile

import app.cash.sqldelight.driver.jdbc.sqlite.JdbcSqliteDriver
import com.getaltair.altair.data.entity.mobile.Item
import com.getaltair.altair.database.AltairDatabase
import kotlinx.coroutines.test.runTest
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Integration tests for ItemRepositoryImpl.
 *
 * Tests CRUD operations using an in-memory SQLite database.
 * Follows TDD RED-GREEN-REFACTOR pattern.
 */
class ItemRepositoryTest {

    private lateinit var driver: JdbcSqliteDriver
    private lateinit var database: AltairDatabase
    private lateinit var repository: ItemRepositoryImpl

    @BeforeTest
    fun setUp() {
        driver = JdbcSqliteDriver(JdbcSqliteDriver.IN_MEMORY)
        AltairDatabase.Schema.create(driver)
        database = AltairDatabase(driver)
        repository = ItemRepositoryImpl(database)
    }

    @AfterTest
    fun tearDown() {
        driver.close()
    }

    @Test
    fun `create item generates ID and timestamps`() = runTest {
        val item = Item(
            id = "",
            name = "Test Item",
            description = "Item description here",
            createdAt = "",
            updatedAt = "",
        )

        val created = repository.create(item)

        assertNotNull(created.id)
        assertTrue(created.id.isNotBlank())
        assertEquals("Test Item", created.name)
        assertEquals("Item description here", created.description)
        assertNotNull(created.createdAt)
        assertNotNull(created.updatedAt)
        assertNull(created.deletedAt)
        assertEquals(0, created.syncVersion)
    }

    @Test
    fun `create item with existing ID preserves ID`() = runTest {
        val existingId = "01HWITEM123456789ABCDEFGH"
        val item = Item(
            id = existingId,
            name = "Item with ID",
            createdAt = "",
            updatedAt = "",
        )

        val created = repository.create(item)

        assertEquals(existingId, created.id)
    }

    @Test
    fun `findById returns item when exists`() = runTest {
        val item = Item(
            id = "",
            name = "Find Me Item",
            description = "Description",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(item)

        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(created.id, found.id)
        assertEquals("Find Me Item", found.name)
    }

    @Test
    fun `findById returns null when not exists`() = runTest {
        val found = repository.findById("nonexistent-id")

        assertNull(found)
    }

    @Test
    fun `update item changes values and increments syncVersion`() = runTest {
        val item = Item(
            id = "",
            name = "Original Name",
            description = "Original Description",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(item)

        val updated = repository.update(
            created.copy(
                name = "Updated Name",
                description = "Updated Description",
            ),
        )

        assertEquals("Updated Name", updated.name)
        assertEquals("Updated Description", updated.description)
        assertEquals(1, updated.syncVersion)

        // Verify persisted
        val found = repository.findById(created.id)
        assertNotNull(found)
        assertEquals("Updated Name", found.name)
        assertEquals(1, found.syncVersion)
    }

    @Test
    fun `delete performs soft delete`() = runTest {
        val item = Item(
            id = "",
            name = "Delete Me Item",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(item)

        val result = repository.delete(created.id)

        assertTrue(result)

        // Item should still exist but be marked as deleted
        val found = repository.findById(created.id)
        assertNotNull(found)
        assertNotNull(found.deletedAt)
        assertTrue(found.isDeleted)
    }

    @Test
    fun `delete returns false for nonexistent item`() = runTest {
        val result = repository.delete("nonexistent-id")

        assertFalse(result)
    }

    @Test
    fun `delete returns false for already deleted item`() = runTest {
        val item = Item(
            id = "",
            name = "Delete Twice Item",
            createdAt = "",
            updatedAt = "",
        )
        val created = repository.create(item)

        repository.delete(created.id)
        val result = repository.delete(created.id)

        assertFalse(result)
    }

    @Test
    fun `findAll excludes soft-deleted items`() = runTest {
        val item1 = repository.create(
            Item(id = "", name = "Item 1", createdAt = "", updatedAt = ""),
        )
        val item2 = repository.create(
            Item(id = "", name = "Item 2", createdAt = "", updatedAt = ""),
        )
        repository.create(
            Item(id = "", name = "Item 3", createdAt = "", updatedAt = ""),
        )

        // Delete item2
        repository.delete(item2.id)

        val all = repository.findAll()

        assertEquals(2, all.size)
        assertTrue(all.any { it.name == "Item 1" })
        assertTrue(all.any { it.name == "Item 3" })
        assertFalse(all.any { it.name == "Item 2" })
    }

    @Test
    fun `findAll returns empty list when no items`() = runTest {
        val all = repository.findAll()

        assertTrue(all.isEmpty())
    }

    @Test
    fun `item with locationId is stored correctly`() = runTest {
        val locationId = "01HWLOC123456789ABCDEFGHIJ"
        val item = Item(
            id = "",
            name = "Item at Location",
            locationId = locationId,
            createdAt = "",
            updatedAt = "",
        )

        val created = repository.create(item)
        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(locationId, found.locationId)
        assertTrue(found.hasLocation)
    }

    @Test
    fun `item with containerId is stored correctly`() = runTest {
        val containerId = "01HWCONT23456789ABCDEFGHIJ"
        val item = Item(
            id = "",
            name = "Item in Container",
            containerId = containerId,
            createdAt = "",
            updatedAt = "",
        )

        val created = repository.create(item)
        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(containerId, found.containerId)
        assertTrue(found.hasContainer)
    }

    @Test
    fun `findByLocationId returns items at location`() = runTest {
        val locationId = "01HWLOC123456789ABCDEFGHIJ"

        repository.create(
            Item(id = "", name = "Item at Location 1", locationId = locationId, createdAt = "", updatedAt = ""),
        )
        repository.create(
            Item(id = "", name = "Item at Location 2", locationId = locationId, createdAt = "", updatedAt = ""),
        )
        repository.create(
            Item(id = "", name = "Item without Location", locationId = null, createdAt = "", updatedAt = ""),
        )

        val locationItems = repository.findByLocationId(locationId)

        assertEquals(2, locationItems.size)
        assertTrue(locationItems.all { it.locationId == locationId })
    }

    @Test
    fun `findByLocationId excludes soft-deleted items`() = runTest {
        val locationId = "01HWLOC123456789ABCDEFGHIJ"

        val item1 = repository.create(
            Item(id = "", name = "Item 1", locationId = locationId, createdAt = "", updatedAt = ""),
        )
        repository.create(
            Item(id = "", name = "Item 2", locationId = locationId, createdAt = "", updatedAt = ""),
        )

        repository.delete(item1.id)

        val locationItems = repository.findByLocationId(locationId)

        assertEquals(1, locationItems.size)
        assertEquals("Item 2", locationItems.first().name)
    }

    @Test
    fun `findByContainerId returns items in container`() = runTest {
        val containerId = "01HWCONT23456789ABCDEFGHIJ"

        repository.create(
            Item(id = "", name = "Item in Container 1", containerId = containerId, createdAt = "", updatedAt = ""),
        )
        repository.create(
            Item(id = "", name = "Item in Container 2", containerId = containerId, createdAt = "", updatedAt = ""),
        )
        repository.create(
            Item(id = "", name = "Item without Container", containerId = null, createdAt = "", updatedAt = ""),
        )

        val containerItems = repository.findByContainerId(containerId)

        assertEquals(2, containerItems.size)
        assertTrue(containerItems.all { it.containerId == containerId })
    }

    @Test
    fun `findByContainerId excludes soft-deleted items`() = runTest {
        val containerId = "01HWCONT23456789ABCDEFGHIJ"

        val item1 = repository.create(
            Item(id = "", name = "Item 1", containerId = containerId, createdAt = "", updatedAt = ""),
        )
        repository.create(
            Item(id = "", name = "Item 2", containerId = containerId, createdAt = "", updatedAt = ""),
        )

        repository.delete(item1.id)

        val containerItems = repository.findByContainerId(containerId)

        assertEquals(1, containerItems.size)
        assertEquals("Item 2", containerItems.first().name)
    }

    @Test
    fun `item hasLocation and hasContainer return correct values`() = runTest {
        val itemWithBoth = Item(
            id = "1",
            name = "With Both",
            locationId = "loc123",
            containerId = "cont456",
            createdAt = "",
            updatedAt = "",
        )
        val itemWithNeither = Item(
            id = "2",
            name = "With Neither",
            locationId = null,
            containerId = null,
            createdAt = "",
            updatedAt = "",
        )

        assertTrue(itemWithBoth.hasLocation)
        assertTrue(itemWithBoth.hasContainer)
        assertFalse(itemWithNeither.hasLocation)
        assertFalse(itemWithNeither.hasContainer)
    }
}
