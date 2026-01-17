package com.getaltair.altair.domain

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertIs
import kotlin.test.assertNull
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

/**
 * Tests for DomainError sealed interface.
 * Verifies construction, validation, pattern matching, and user messages.
 */
class DomainErrorTest {
    private val json = Json { prettyPrint = false }

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
    fun `NetworkError rejects blank message`() {
        assertFailsWith<IllegalArgumentException> {
            DomainError.NetworkError(message = "")
        }
        assertFailsWith<IllegalArgumentException> {
            DomainError.NetworkError(message = "   ")
        }
    }

    @Test
    fun `NetworkError toUserMessage returns user-friendly text`() {
        val error = DomainError.NetworkError(message = "Connection refused")
        assertEquals(
            "Unable to connect. Please check your internet connection and try again.",
            error.toUserMessage(),
        )
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
    fun `ValidationError rejects blank field`() {
        assertFailsWith<IllegalArgumentException> {
            DomainError.ValidationError(field = "", message = "Some message")
        }
    }

    @Test
    fun `ValidationError rejects blank message`() {
        assertFailsWith<IllegalArgumentException> {
            DomainError.ValidationError(field = "email", message = "")
        }
    }

    @Test
    fun `ValidationError toUserMessage returns the validation message`() {
        val error = DomainError.ValidationError(field = "email", message = "Please enter a valid email")
        assertEquals("Please enter a valid email", error.toUserMessage())
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
    fun `NotFoundError rejects blank resource`() {
        assertFailsWith<IllegalArgumentException> {
            DomainError.NotFoundError(resource = "", id = "123")
        }
    }

    @Test
    fun `NotFoundError rejects blank id`() {
        assertFailsWith<IllegalArgumentException> {
            DomainError.NotFoundError(resource = "Quest", id = "")
        }
    }

    @Test
    fun `NotFoundError toUserMessage includes resource type`() {
        val error = DomainError.NotFoundError(resource = "Quest", id = "123")
        assertEquals("The requested Quest could not be found.", error.toUserMessage())
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
    fun `UnauthorizedError toUserMessage returns user-friendly text`() {
        val error = DomainError.UnauthorizedError()
        assertEquals(
            "You don't have permission to access this. Please sign in and try again.",
            error.toUserMessage(),
        )
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
    fun `UnexpectedError rejects blank message`() {
        assertFailsWith<IllegalArgumentException> {
            DomainError.UnexpectedError(message = "")
        }
    }

    @Test
    fun `UnexpectedError toUserMessage returns generic user-friendly text`() {
        val error = DomainError.UnexpectedError(message = "Internal database error")
        assertEquals("Something went wrong. Please try again later.", error.toUserMessage())
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

    @Test
    fun `toUserMessage is accessible via DomainError interface`() {
        val error: DomainError = DomainError.NetworkError("test")
        // Verify toUserMessage can be called on the interface type
        val message = error.toUserMessage()
        assertEquals(
            "Unable to connect. Please check your internet connection and try again.",
            message,
        )
    }

    // Serialization tests

    @Test
    fun `NetworkError serializes with type discriminator`() {
        val error: DomainError = DomainError.NetworkError("Connection timeout")
        val serialized = json.encodeToString(error)
        assertEquals("""{"type":"network","message":"Connection timeout"}""", serialized)
    }

    @Test
    fun `ValidationError round-trips through JSON`() {
        val error: DomainError = DomainError.ValidationError("email", "Invalid format")
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `NotFoundError round-trips through JSON`() {
        val error: DomainError = DomainError.NotFoundError("User", "user-123")
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `UnauthorizedError round-trips through JSON`() {
        val error: DomainError = DomainError.UnauthorizedError("Token expired")
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `UnexpectedError round-trips through JSON`() {
        val error: DomainError = DomainError.UnexpectedError("Internal failure")
        val serialized = json.encodeToString(error)
        val deserialized = json.decodeFromString<DomainError>(serialized)
        assertEquals(error, deserialized)
    }

    @Test
    fun `cause field is not serialized`() {
        val error = DomainError.NetworkError("Test", cause = RuntimeException("Cause"))
        val serialized = json.encodeToString<DomainError>(error)
        // Cause should not appear in serialized output
        assertEquals("""{"type":"network","message":"Test"}""", serialized)

        // Deserialized error should have null cause
        val deserialized = json.decodeFromString<DomainError>(serialized) as DomainError.NetworkError
        assertNull(deserialized.cause)
    }
}
