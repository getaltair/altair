package com.getaltair.altair.domain

import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

/**
 * Tests for DomainError sealed interface.
 *
 * Verifies construction, validation, pattern matching, and user messages.
 */
class DomainErrorTest :
    BehaviorSpec({
        val json = Json { prettyPrint = false }

        given("a NetworkError") {
            `when`("constructed with message and cause") {
                then("stores both values") {
                    val cause = RuntimeException("Connection reset")
                    val error =
                        DomainError.NetworkError(
                            message = "Failed to connect",
                            cause = cause,
                        )

                    error.message shouldBe "Failed to connect"
                    error.cause shouldBe cause
                }
            }

            `when`("constructed without cause") {
                then("cause is null") {
                    val error = DomainError.NetworkError(message = "Timeout")

                    error.message shouldBe "Timeout"
                    error.cause.shouldBeNull()
                }
            }

            `when`("constructed with blank message") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        DomainError.NetworkError(message = "")
                    }
                    shouldThrow<IllegalArgumentException> {
                        DomainError.NetworkError(message = "   ")
                    }
                }
            }

            `when`("toUserMessage is called") {
                then("returns user-friendly text") {
                    val error = DomainError.NetworkError(message = "Connection refused")
                    error.toUserMessage() shouldBe
                        "Unable to connect. Please check your internet connection and try again."
                }
            }

            `when`("serialized to JSON") {
                then("includes type discriminator") {
                    val error: DomainError = DomainError.NetworkError("Connection timeout")
                    val serialized = json.encodeToString(error)
                    serialized shouldBe """{"type":"network","message":"Connection timeout"}"""
                }

                then("cause is not serialized") {
                    val error = DomainError.NetworkError("Test", cause = RuntimeException("Cause"))
                    val serialized = json.encodeToString<DomainError>(error)
                    serialized shouldBe """{"type":"network","message":"Test"}"""

                    val deserialized = json.decodeFromString<DomainError>(serialized) as DomainError.NetworkError
                    deserialized.cause.shouldBeNull()
                }
            }
        }

        given("a ValidationError") {
            `when`("constructed with field and message") {
                then("stores both values") {
                    val error =
                        DomainError.ValidationError(
                            field = "email",
                            message = "Invalid email format",
                        )

                    error.field shouldBe "email"
                    error.message shouldBe "Invalid email format"
                }
            }

            `when`("constructed with blank field") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        DomainError.ValidationError(field = "", message = "Some message")
                    }
                }
            }

            `when`("constructed with blank message") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        DomainError.ValidationError(field = "email", message = "")
                    }
                }
            }

            `when`("toUserMessage is called") {
                then("returns the validation message") {
                    val error = DomainError.ValidationError(field = "email", message = "Please enter a valid email")
                    error.toUserMessage() shouldBe "Please enter a valid email"
                }
            }

            `when`("round-tripped through JSON") {
                then("deserializes correctly") {
                    val error: DomainError = DomainError.ValidationError("email", "Invalid format")
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("a NotFoundError") {
            `when`("constructed with resource and id") {
                then("stores both values") {
                    val error =
                        DomainError.NotFoundError(
                            resource = "Quest",
                            id = "quest-123",
                        )

                    error.resource shouldBe "Quest"
                    error.id shouldBe "quest-123"
                }
            }

            `when`("constructed with blank resource") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        DomainError.NotFoundError(resource = "", id = "123")
                    }
                }
            }

            `when`("constructed with blank id") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        DomainError.NotFoundError(resource = "Quest", id = "")
                    }
                }
            }

            `when`("toUserMessage is called") {
                then("includes resource type") {
                    val error = DomainError.NotFoundError(resource = "Quest", id = "123")
                    error.toUserMessage() shouldBe "The requested Quest could not be found."
                }
            }

            `when`("round-tripped through JSON") {
                then("deserializes correctly") {
                    val error: DomainError = DomainError.NotFoundError("User", "user-123")
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("an UnauthorizedError") {
            `when`("constructed without parameters") {
                then("has default message") {
                    val error = DomainError.UnauthorizedError()
                    error.message shouldBe "Unauthorized access"
                }
            }

            `when`("constructed with custom message") {
                then("uses the custom message") {
                    val error = DomainError.UnauthorizedError(message = "Token expired")
                    error.message shouldBe "Token expired"
                }
            }

            `when`("constructed with blank message") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        DomainError.UnauthorizedError(message = "")
                    }
                    shouldThrow<IllegalArgumentException> {
                        DomainError.UnauthorizedError(message = "   ")
                    }
                }
            }

            `when`("toUserMessage is called") {
                then("returns user-friendly text") {
                    val error = DomainError.UnauthorizedError()
                    error.toUserMessage() shouldBe
                        "You don't have permission to access this. Please sign in and try again."
                }
            }

            `when`("round-tripped through JSON") {
                then("deserializes correctly") {
                    val error: DomainError = DomainError.UnauthorizedError("Token expired")
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("an UnexpectedError") {
            `when`("constructed with message and cause") {
                then("stores both values") {
                    val cause = IllegalStateException("Invariant violated")
                    val error =
                        DomainError.UnexpectedError(
                            message = "Something went wrong",
                            cause = cause,
                        )

                    error.message shouldBe "Something went wrong"
                    error.cause shouldBe cause
                }
            }

            `when`("constructed with blank message") {
                then("throws IllegalArgumentException") {
                    shouldThrow<IllegalArgumentException> {
                        DomainError.UnexpectedError(message = "")
                    }
                }
            }

            `when`("toUserMessage is called") {
                then("returns generic user-friendly text") {
                    val error = DomainError.UnexpectedError(message = "Internal database error")
                    error.toUserMessage() shouldBe "Something went wrong. Please try again later."
                }
            }

            `when`("round-tripped through JSON") {
                then("deserializes correctly") {
                    val error: DomainError = DomainError.UnexpectedError("Internal failure")
                    val serialized = json.encodeToString(error)
                    val deserialized = json.decodeFromString<DomainError>(serialized)
                    deserialized shouldBe error
                }
            }
        }

        given("DomainError interface") {
            `when`("pattern matching with when expression") {
                then("all error types are distinguishable") {
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
                                // Module-specific errors (tested in ModuleErrorsTest)
                                is QuestError -> "quest"
                                is NoteError -> "note"
                                is ItemError -> "item"
                                is SyncError -> "sync"
                                is AuthError -> "auth"
                                is UserError -> "user"
                                is EpicError -> "epic"
                            }
                        }

                    descriptions shouldBe
                        listOf(
                            "network:Network issue",
                            "validation:name",
                            "notfound:User:user-1",
                            "unauthorized",
                            "unexpected:Oops",
                        )
                }
            }

            `when`("checking type hierarchy") {
                then("all error types are subtypes of DomainError") {
                    val networkError: DomainError = DomainError.NetworkError("test")
                    val validationError: DomainError = DomainError.ValidationError("f", "m")
                    val notFoundError: DomainError = DomainError.NotFoundError("r", "i")
                    val unauthorizedError: DomainError = DomainError.UnauthorizedError()
                    val unexpectedError: DomainError = DomainError.UnexpectedError("e")

                    networkError.shouldBeInstanceOf<DomainError>()
                    validationError.shouldBeInstanceOf<DomainError>()
                    notFoundError.shouldBeInstanceOf<DomainError>()
                    unauthorizedError.shouldBeInstanceOf<DomainError>()
                    unexpectedError.shouldBeInstanceOf<DomainError>()
                }
            }

            `when`("toUserMessage is called on interface type") {
                then("method is accessible") {
                    val error: DomainError = DomainError.NetworkError("test")
                    val message = error.toUserMessage()
                    message shouldBe "Unable to connect. Please check your internet connection and try again."
                }
            }
        }
    })
