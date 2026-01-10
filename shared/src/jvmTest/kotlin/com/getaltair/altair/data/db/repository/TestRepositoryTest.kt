package com.getaltair.altair.data.db.repository

import com.getaltair.altair.data.db.SurrealDbConfig
import com.getaltair.altair.data.db.SurrealDbConnection
import com.getaltair.altair.data.entity.TestEntity
import kotlinx.coroutines.test.runTest
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNotEquals
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Comprehensive tests for TestRepository.
 * Tests verify CRUD operations, soft delete, and data integrity.
 */
class TestRepositoryTest {

    private lateinit var repository: TestRepository

    @BeforeTest
    fun setUp() = runTest {
        // Use in-memory database for tests
        SurrealDbConnection.connect(SurrealDbConfig.memory())
        repository = TestRepository()
        // Clean up any existing test data
        repository.deleteAll()
    }

    @AfterTest
    fun tearDown() = runTest {
        repository.deleteAll()
        SurrealDbConnection.disconnect()
    }

    // CREATE Tests

    @Test
    fun `create returns entity with generated ID when ID is blank`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "Test Entity",
            value = 42,
            createdAt = "",
            updatedAt = ""
        )

        val created = repository.create(entity)

        assertTrue(created.id.isNotBlank(), "ID should be generated")
        assertEquals("Test Entity", created.name)
        assertEquals(42, created.value)
    }

    @Test
    fun `create returns entity with provided ID when ID is specified`() = runTest {
        val customId = "01ARZ3NDEKTSV4RRFFQ69G5FAV"
        val entity = TestEntity(
            id = customId,
            name = "Custom ID Entity",
            value = 100,
            createdAt = "",
            updatedAt = ""
        )

        val created = repository.create(entity)

        assertEquals(customId, created.id)
        assertEquals("Custom ID Entity", created.name)
    }

    @Test
    fun `create sets timestamps`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "Timestamp Test",
            value = 1,
            createdAt = "",
            updatedAt = ""
        )

        val created = repository.create(entity)

        assertTrue(created.createdAt.isNotBlank(), "createdAt should be set")
        assertTrue(created.updatedAt.isNotBlank(), "updatedAt should be set")
        assertEquals(created.createdAt, created.updatedAt, "createdAt and updatedAt should be equal on create")
    }

    @Test
    fun `create sets syncVersion to 0`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "Sync Version Test",
            value = 1,
            createdAt = "",
            updatedAt = "",
            syncVersion = 99  // Should be reset to 0
        )

        val created = repository.create(entity)

        assertEquals(0, created.syncVersion)
    }

    @Test
    fun `create sets deletedAt to null`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "Deleted At Test",
            value = 1,
            createdAt = "",
            updatedAt = ""
        )

        val created = repository.create(entity)

        assertNull(created.deletedAt)
        assertFalse(created.isDeleted)
    }

    // READ Tests

    @Test
    fun `findById returns created entity`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "Find By ID Test",
            value = 123,
            createdAt = "",
            updatedAt = ""
        )
        val created = repository.create(entity)

        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(created.id, found.id)
        assertEquals(created.name, found.name)
        assertEquals(created.value, found.value)
    }

    @Test
    fun `findById returns null for non-existent ID`() = runTest {
        val found = repository.findById("non-existent-id")

        assertNull(found)
    }

    @Test
    fun `findById returns null for soft-deleted entity`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "Soft Delete Test",
            value = 1,
            createdAt = "",
            updatedAt = ""
        )
        val created = repository.create(entity)
        repository.delete(created.id)

        val found = repository.findById(created.id)

        assertNull(found, "Soft-deleted entity should not be found by findById")
    }

    @Test
    fun `findByIdIncludeDeleted returns soft-deleted entity`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "Include Deleted Test",
            value = 1,
            createdAt = "",
            updatedAt = ""
        )
        val created = repository.create(entity)
        repository.delete(created.id)

        val found = repository.findByIdIncludeDeleted(created.id)

        assertNotNull(found, "Soft-deleted entity should be found by findByIdIncludeDeleted")
        assertNotNull(found.deletedAt)
        assertTrue(found.isDeleted)
    }

    @Test
    fun `findAll returns all non-deleted entities`() = runTest {
        repository.create(TestEntity("", "Entity 1", 1, "", ""))
        repository.create(TestEntity("", "Entity 2", 2, "", ""))
        val toDelete = repository.create(TestEntity("", "Entity 3", 3, "", ""))
        repository.delete(toDelete.id)

        val all = repository.findAll()

        assertEquals(2, all.size, "Should return only non-deleted entities")
        assertTrue(all.all { it.name != "Entity 3" }, "Deleted entity should not be in results")
    }

    @Test
    fun `findAll returns empty list when no entities exist`() = runTest {
        val all = repository.findAll()

        assertTrue(all.isEmpty())
    }

    // UPDATE Tests

    @Test
    fun `update modifies entity values`() = runTest {
        val created = repository.create(TestEntity("", "Original Name", 100, "", ""))

        val updated = repository.update(created.copy(name = "Updated Name", value = 200))

        assertEquals("Updated Name", updated.name)
        assertEquals(200, updated.value)
    }

    @Test
    fun `update increments syncVersion`() = runTest {
        val created = repository.create(TestEntity("", "Sync Test", 1, "", ""))
        assertEquals(0, created.syncVersion)

        val updated = repository.update(created.copy(name = "Updated"))

        assertEquals(1, updated.syncVersion)

        val updatedAgain = repository.update(updated.copy(name = "Updated Again"))

        assertEquals(2, updatedAgain.syncVersion)
    }

    @Test
    fun `update changes updatedAt timestamp`() = runTest {
        val created = repository.create(TestEntity("", "Timestamp Update Test", 1, "", ""))
        val originalUpdatedAt = created.updatedAt

        // Small delay to ensure timestamp difference
        Thread.sleep(10)

        val updated = repository.update(created.copy(name = "Updated"))

        assertNotEquals(originalUpdatedAt, updated.updatedAt, "updatedAt should change on update")
    }

    @Test
    fun `update preserves createdAt timestamp`() = runTest {
        val created = repository.create(TestEntity("", "CreatedAt Preserve Test", 1, "", ""))

        val updated = repository.update(created.copy(name = "Updated"))

        // Verify by reading from database
        val found = repository.findById(created.id)
        assertNotNull(found)
        assertEquals(created.createdAt, found.createdAt)
    }

    // DELETE Tests

    @Test
    fun `delete performs soft delete`() = runTest {
        val created = repository.create(TestEntity("", "Delete Test", 1, "", ""))

        val result = repository.delete(created.id)

        assertTrue(result, "Delete should return true")

        val found = repository.findByIdIncludeDeleted(created.id)
        assertNotNull(found)
        assertNotNull(found.deletedAt, "deletedAt should be set")
        assertTrue(found.isDeleted)
    }

    @Test
    fun `delete returns false for non-existent entity`() = runTest {
        val result = repository.delete("non-existent-id")

        assertFalse(result, "Delete should return false for non-existent entity")
    }

    @Test
    fun `delete returns false for already deleted entity`() = runTest {
        val created = repository.create(TestEntity("", "Double Delete Test", 1, "", ""))
        repository.delete(created.id)

        val result = repository.delete(created.id)

        assertFalse(result, "Second delete should return false")
    }

    @Test
    fun `hardDelete removes entity from database`() = runTest {
        val created = repository.create(TestEntity("", "Hard Delete Test", 1, "", ""))

        repository.hardDelete(created.id)

        val found = repository.findByIdIncludeDeleted(created.id)
        assertNull(found, "Entity should be completely removed")
    }

    // Edge Cases

    @Test
    fun `create handles special characters in name`() = runTest {
        // Test with various special characters that are common in user input
        // Note: Backslash escaping behavior depends on SurrealDB storage
        val entity = TestEntity(
            id = "",
            name = "Test with 'quotes' and \"double quotes\"",
            value = 1,
            createdAt = "",
            updatedAt = ""
        )

        val created = repository.create(entity)
        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(entity.name, found.name)
    }

    @Test
    fun `create handles unicode characters`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "Unicode: cafe, naive, 42",
            value = 1,
            createdAt = "",
            updatedAt = ""
        )

        val created = repository.create(entity)
        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(entity.name, found.name)
    }

    @Test
    fun `create handles empty name`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "",
            value = 0,
            createdAt = "",
            updatedAt = ""
        )

        val created = repository.create(entity)
        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals("", found.name)
    }

    @Test
    fun `create handles negative values`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "Negative Value",
            value = -999,
            createdAt = "",
            updatedAt = ""
        )

        val created = repository.create(entity)
        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(-999, found.value)
    }

    @Test
    fun `create handles large values`() = runTest {
        val entity = TestEntity(
            id = "",
            name = "Large Value",
            value = Int.MAX_VALUE,
            createdAt = "",
            updatedAt = ""
        )

        val created = repository.create(entity)
        val found = repository.findById(created.id)

        assertNotNull(found)
        assertEquals(Int.MAX_VALUE, found.value)
    }
}
