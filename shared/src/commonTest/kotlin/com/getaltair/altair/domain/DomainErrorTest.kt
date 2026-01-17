package com.getaltair.altair.domain

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlin.test.assertNull

/**
 * Tests for DomainError sealed interface.
 * Verifies construction, pattern matching, and property access.
 */
class DomainErrorTest {
    @Test
    fun `NetworkError stores message and cause`() {
        val cause = RuntimeException("Connection reset")
        val error =
            DomainError.NetworkError(
                message = "Failed to connect",
                cause = cause,
            )

        assertEquals("Failed to connect", error.message)
        assertEquals(cause, error.cause)
    }

    @Test
    fun `NetworkError cause is optional`() {
        val error = DomainError.NetworkError(message = "Timeout")

        assertEquals("Timeout", error.message)
        assertNull(error.cause)
    }

    @Test
    fun `ValidationError stores field and message`() {
        val error =
            DomainError.ValidationError(
                field = "email",
                message = "Invalid email format",
            )

        assertEquals("email", error.field)
        assertEquals("Invalid email format", error.message)
    }

    @Test
    fun `NotFoundError stores resource and id`() {
        val error =
            DomainError.NotFoundError(
                resource = "Quest",
                id = "quest-123",
            )

        assertEquals("Quest", error.resource)
        assertEquals("quest-123", error.id)
    }

    @Test
    fun `UnauthorizedError has default message`() {
        val error = DomainError.UnauthorizedError()

        assertEquals("Unauthorized access", error.message)
    }

    @Test
    fun `UnauthorizedError accepts custom message`() {
        val error = DomainError.UnauthorizedError(message = "Token expired")

        assertEquals("Token expired", error.message)
    }

    @Test
    fun `UnexpectedError stores message and cause`() {
        val cause = IllegalStateException("Invariant violated")
        val error =
            DomainError.UnexpectedError(
                message = "Something went wrong",
                cause = cause,
            )

        assertEquals("Something went wrong", error.message)
        assertEquals(cause, error.cause)
    }

    @Test
    fun `pattern matching works with when expression`() {
        val errors: List<DomainError> =
            listOf(
                DomainError.NetworkError("Network issue"),
                DomainError.ValidationError("name", "Required"),
                DomainError.NotFoundError("User", "user-1"),
                DomainError.UnauthorizedError(),
                DomainError.UnexpectedError("Oops"),
            )

        val descriptions =
            errors.map { error ->
                when (error) {
                    is DomainError.NetworkError -> "network:${error.message}"
                    is DomainError.ValidationError -> "validation:${error.field}"
                    is DomainError.NotFoundError -> "notfound:${error.resource}:${error.id}"
                    is DomainError.UnauthorizedError -> "unauthorized"
                    is DomainError.UnexpectedError -> "unexpected:${error.message}"
                }
            }

        assertEquals(
            listOf(
                "network:Network issue",
                "validation:name",
                "notfound:User:user-1",
                "unauthorized",
                "unexpected:Oops",
            ),
            descriptions,
        )
    }

    @Test
    fun `all error types are subtypes of DomainError`() {
        val networkError: DomainError = DomainError.NetworkError("test")
        val validationError: DomainError = DomainError.ValidationError("f", "m")
        val notFoundError: DomainError = DomainError.NotFoundError("r", "i")
        val unauthorizedError: DomainError = DomainError.UnauthorizedError()
        val unexpectedError: DomainError = DomainError.UnexpectedError("e")

        assertIs<DomainError>(networkError)
        assertIs<DomainError>(validationError)
        assertIs<DomainError>(notFoundError)
        assertIs<DomainError>(unauthorizedError)
        assertIs<DomainError>(unexpectedError)
    }
}
