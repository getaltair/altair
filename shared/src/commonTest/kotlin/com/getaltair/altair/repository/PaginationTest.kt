package com.getaltair.altair.repository

import arrow.core.Either
import com.getaltair.altair.domain.DomainError
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertIs
import kotlin.test.assertTrue

/**
 * Tests for pagination types: [PageRequest] and [PageResult].
 */
class PaginationTest {
    // PageRequest validation tests

    @Test
    fun `PageRequest with valid parameters returns Right`() {
        val result = PageRequest(limit = 50, offset = 0)
        assertIs<Either.Right<PageRequest>>(result)
        assertEquals(50, result.value.limit)
        assertEquals(0, result.value.offset)
    }

    @Test
    fun `PageRequest with default parameters returns Right`() {
        val result = PageRequest()
        assertIs<Either.Right<PageRequest>>(result)
        assertEquals(PageRequest.DEFAULT_PAGE_SIZE, result.value.limit)
        assertEquals(0, result.value.offset)
    }

    @Test
    fun `PageRequest with minimum valid limit returns Right`() {
        val result = PageRequest(limit = 1)
        assertIs<Either.Right<PageRequest>>(result)
        assertEquals(1, result.value.limit)
    }

    @Test
    fun `PageRequest with maximum valid limit returns Right`() {
        val result = PageRequest(limit = PageRequest.MAX_PAGE_SIZE)
        assertIs<Either.Right<PageRequest>>(result)
        assertEquals(PageRequest.MAX_PAGE_SIZE, result.value.limit)
    }

    @Test
    fun `PageRequest with valid offset returns Right`() {
        val result = PageRequest(offset = 100)
        assertIs<Either.Right<PageRequest>>(result)
        assertEquals(100, result.value.offset)
    }

    @Test
    fun `PageRequest with zero limit returns Left with validation error`() {
        val result = PageRequest(limit = 0)
        assertIs<Either.Left<DomainError.ValidationError>>(result)
        assertEquals("limit", result.value.field)
        assertTrue(result.value.message.contains("1"))
        assertTrue(result.value.message.contains("${PageRequest.MAX_PAGE_SIZE}"))
    }

    @Test
    fun `PageRequest with negative limit returns Left with validation error`() {
        val result = PageRequest(limit = -1)
        assertIs<Either.Left<DomainError.ValidationError>>(result)
        assertEquals("limit", result.value.field)
    }

    @Test
    fun `PageRequest with limit exceeding maximum returns Left with validation error`() {
        val result = PageRequest(limit = PageRequest.MAX_PAGE_SIZE + 1)
        assertIs<Either.Left<DomainError.ValidationError>>(result)
        assertEquals("limit", result.value.field)
    }

    @Test
    fun `PageRequest with negative offset returns Left with validation error`() {
        val result = PageRequest(limit = 50, offset = -1)
        assertIs<Either.Left<DomainError.ValidationError>>(result)
        assertEquals("offset", result.value.field)
        assertTrue(result.value.message.contains("non-negative"))
    }

    @Test
    fun `PageRequest validates limit before offset`() {
        // Both limit and offset are invalid, but limit should be validated first
        val result = PageRequest(limit = 0, offset = -1)
        assertIs<Either.Left<DomainError.ValidationError>>(result)
        assertEquals("limit", result.value.field)
    }

    // PageRequest.unsafeCreate tests

    @Test
    fun `unsafeCreate with valid parameters succeeds`() {
        val request = PageRequest.unsafeCreate(limit = 25, offset = 50)
        assertEquals(25, request.limit)
        assertEquals(50, request.offset)
    }

    @Test
    fun `unsafeCreate with default parameters succeeds`() {
        val request = PageRequest.unsafeCreate()
        assertEquals(PageRequest.DEFAULT_PAGE_SIZE, request.limit)
        assertEquals(0, request.offset)
    }

    @Test
    fun `unsafeCreate with zero limit throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            PageRequest.unsafeCreate(limit = 0)
        }
    }

    @Test
    fun `unsafeCreate with negative limit throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            PageRequest.unsafeCreate(limit = -1)
        }
    }

    @Test
    fun `unsafeCreate with limit exceeding maximum throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            PageRequest.unsafeCreate(limit = PageRequest.MAX_PAGE_SIZE + 1)
        }
    }

    @Test
    fun `unsafeCreate with negative offset throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            PageRequest.unsafeCreate(offset = -1)
        }
    }

    // PageRequest constants tests

    @Test
    fun `DEFAULT_PAGE_SIZE is 50`() {
        assertEquals(50, PageRequest.DEFAULT_PAGE_SIZE)
    }

    @Test
    fun `MAX_PAGE_SIZE is 100`() {
        assertEquals(100, PageRequest.MAX_PAGE_SIZE)
    }

    // PageResult tests

    @Test
    fun `PageResult stores items and metadata correctly`() {
        val items = listOf("a", "b", "c")
        val result = PageResult(items = items, totalCount = 10, hasMore = true)

        assertEquals(items, result.items)
        assertEquals(10, result.totalCount)
        assertTrue(result.hasMore)
    }

    @Test
    fun `PageResult with empty items list`() {
        val result = PageResult(items = emptyList<String>(), totalCount = 0, hasMore = false)

        assertTrue(result.items.isEmpty())
        assertEquals(0, result.totalCount)
    }

    @Test
    fun `PageResult hasMore is false when no more items`() {
        val result = PageResult(items = listOf(1, 2, 3), totalCount = 3, hasMore = false)

        assertEquals(false, result.hasMore)
    }

    @Test
    fun `PageResult data class equality works`() {
        val result1 = PageResult(items = listOf(1, 2), totalCount = 5, hasMore = true)
        val result2 = PageResult(items = listOf(1, 2), totalCount = 5, hasMore = true)

        assertEquals(result1, result2)
    }

    @Test
    fun `PageResult copy works correctly`() {
        val original = PageResult(items = listOf("a"), totalCount = 100, hasMore = true)
        val copied = original.copy(hasMore = false)

        assertEquals(listOf("a"), copied.items)
        assertEquals(100, copied.totalCount)
        assertEquals(false, copied.hasMore)
    }

    // PageResult validation tests

    @Test
    fun `PageResult with negative totalCount throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            PageResult(items = listOf(1, 2, 3), totalCount = -1, hasMore = false)
        }
    }

    @Test
    fun `PageResult with totalCount less than items size throws IllegalArgumentException`() {
        assertFailsWith<IllegalArgumentException> {
            PageResult(items = listOf(1, 2, 3), totalCount = 2, hasMore = false)
        }
    }

    @Test
    fun `PageResult with totalCount equal to items size succeeds`() {
        val result = PageResult(items = listOf(1, 2, 3), totalCount = 3, hasMore = false)
        assertEquals(3, result.totalCount)
    }

    @Test
    fun `PageResult with totalCount greater than items size succeeds`() {
        val result = PageResult(items = listOf(1, 2, 3), totalCount = 100, hasMore = true)
        assertEquals(100, result.totalCount)
    }

    @Test
    fun `PageResult with zero totalCount and empty items succeeds`() {
        val result = PageResult(items = emptyList<Int>(), totalCount = 0, hasMore = false)
        assertEquals(0, result.totalCount)
        assertTrue(result.items.isEmpty())
    }
}
