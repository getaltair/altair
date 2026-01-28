package com.getaltair.altair.shared.core.error

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

/**
 * Tests for Arrow Either usage with domain errors.
 */
class DomainErrorTest {

    @Test
    fun eitherRightContainsValue() {
        val result: Either<QuestError, String> = "success".right()

        assertTrue(result.isRight())
        assertEquals("success", result.getOrNull())
    }

    @Test
    fun eitherLeftContainsError() {
        val result: Either<QuestError, String> = QuestError.WipLimitExceeded.left()

        assertTrue(result.isLeft())
        result.onLeft { error ->
            assertEquals("Cannot start quest: WIP limit of 1 already reached", error.message)
        }
    }

    @Test
    fun toEitherConvertsNullableCorrectly() {
        val found: String? = "found"
        val notFound: String? = null

        val foundResult = found.toEither { QuestError.NotFound("123") }
        val notFoundResult = notFound.toEither { QuestError.NotFound("456") }

        assertTrue(foundResult.isRight())
        assertTrue(notFoundResult.isLeft())
    }

    @Test
    fun questErrorNotFoundHasCorrectMessage() {
        val error = QuestError.NotFound("quest-123")
        assertEquals("Quest not found: quest-123", error.message)
    }
}
