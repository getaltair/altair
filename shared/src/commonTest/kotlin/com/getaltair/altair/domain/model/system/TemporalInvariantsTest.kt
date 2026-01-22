package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.types.Ulid
import io.kotest.assertions.throwables.shouldThrow
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain
import kotlin.time.Clock
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.hours
import kotlin.time.Duration.Companion.minutes
import kotlin.time.Instant

/**
 * Tests for temporal invariants on InviteCode and RefreshToken.
 *
 * Validates that temporal ordering constraints are enforced correctly.
 */
class TemporalInvariantsTest :
    BehaviorSpec({
        val now: Instant = Clock.System.now()
        val userId = Ulid.generate()

        given("an InviteCode") {
            `when`("expiresAt is before createdAt") {
                then("construction fails with IllegalArgumentException") {
                    val createdAt = now
                    val expiresAt = now - 1.hours

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            InviteCode(
                                id = Ulid.generate(),
                                code = "TESTCODE123",
                                createdBy = userId,
                                expiresAt = expiresAt,
                                createdAt = createdAt,
                            )
                        }
                    exception.message shouldContain "expiresAt must be after createdAt"
                }
            }

            `when`("expiresAt equals createdAt") {
                then("construction fails with IllegalArgumentException") {
                    val sameTime = now

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            InviteCode(
                                id = Ulid.generate(),
                                code = "TESTCODE123",
                                createdBy = userId,
                                expiresAt = sameTime,
                                createdAt = sameTime,
                            )
                        }
                    exception.message shouldContain "expiresAt must be after createdAt"
                }
            }

            `when`("createdAt is far in the future") {
                then("construction fails with IllegalArgumentException") {
                    val farFutureCreatedAt = now + 1.hours
                    val expiresAt = farFutureCreatedAt + 1.days

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            InviteCode(
                                id = Ulid.generate(),
                                code = "TESTCODE123",
                                createdBy = userId,
                                expiresAt = expiresAt,
                                createdAt = farFutureCreatedAt,
                            )
                        }
                    exception.message shouldContain "createdAt must not be in the far future"
                }
            }

            `when`("createdAt is within clock skew tolerance") {
                then("construction succeeds") {
                    val nearFutureCreatedAt = now + 2.minutes
                    val expiresAt = nearFutureCreatedAt + 1.days

                    val inviteCode =
                        InviteCode(
                            id = Ulid.generate(),
                            code = "TESTCODE123",
                            createdBy = userId,
                            expiresAt = expiresAt,
                            createdAt = nearFutureCreatedAt,
                        )

                    inviteCode.createdAt shouldBe nearFutureCreatedAt
                }
            }

            `when`("expiresAt is too far from createdAt") {
                then("construction fails with IllegalArgumentException") {
                    val createdAt = now
                    val expiresAt = createdAt + 100.days // Exceeds MAX_EXPIRY_DURATION (90 days)

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            InviteCode(
                                id = Ulid.generate(),
                                code = "TESTCODE123",
                                createdBy = userId,
                                expiresAt = expiresAt,
                                createdAt = createdAt,
                            )
                        }
                    exception.message shouldContain "expiresAt must be within"
                }
            }

            `when`("expiresAt is at max expiry duration") {
                then("construction succeeds") {
                    val createdAt = now
                    val expiresAt = createdAt + InviteCode.MAX_EXPIRY_DURATION

                    val inviteCode =
                        InviteCode(
                            id = Ulid.generate(),
                            code = "TESTCODE123",
                            createdBy = userId,
                            expiresAt = expiresAt,
                            createdAt = createdAt,
                        )

                    inviteCode.expiresAt shouldBe expiresAt
                }
            }

            `when`("valid temporal values are provided") {
                then("construction succeeds with correct timestamps") {
                    val createdAt = now
                    val expiresAt = createdAt + 7.days

                    val inviteCode =
                        InviteCode(
                            id = Ulid.generate(),
                            code = "TESTCODE123",
                            createdBy = userId,
                            expiresAt = expiresAt,
                            createdAt = createdAt,
                        )

                    inviteCode.createdAt shouldBe createdAt
                    inviteCode.expiresAt shouldBe expiresAt
                }
            }
        }

        given("InviteCode factory method") {
            `when`("creating with default expiry duration") {
                then("uses DEFAULT_EXPIRY_DURATION") {
                    val fakeClock = FakeClock(now)
                    val inviteCode =
                        InviteCode.create(
                            id = Ulid.generate(),
                            code = "TESTCODE123",
                            createdBy = userId,
                            clock = fakeClock,
                        )

                    inviteCode.createdAt shouldBe now
                    inviteCode.expiresAt shouldBe now + InviteCode.DEFAULT_EXPIRY_DURATION
                }
            }

            `when`("creating with custom expiry duration") {
                then("uses the custom duration") {
                    val fakeClock = FakeClock(now)
                    val customExpiry = 14.days
                    val inviteCode =
                        InviteCode.create(
                            id = Ulid.generate(),
                            code = "TESTCODE123",
                            createdBy = userId,
                            expiresIn = customExpiry,
                            clock = fakeClock,
                        )

                    inviteCode.createdAt shouldBe now
                    inviteCode.expiresAt shouldBe now + customExpiry
                }
            }
        }

        given("a RefreshToken") {
            `when`("expiresAt is before createdAt") {
                then("construction fails with IllegalArgumentException") {
                    val createdAt = now
                    val expiresAt = now - 1.hours

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            RefreshToken(
                                id = Ulid.generate(),
                                userId = userId,
                                tokenHash = "abc123hash",
                                deviceName = null,
                                expiresAt = expiresAt,
                                createdAt = createdAt,
                            )
                        }
                    exception.message shouldContain "expiresAt must be after createdAt"
                }
            }

            `when`("expiresAt equals createdAt") {
                then("construction fails with IllegalArgumentException") {
                    val sameTime = now

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            RefreshToken(
                                id = Ulid.generate(),
                                userId = userId,
                                tokenHash = "abc123hash",
                                deviceName = null,
                                expiresAt = sameTime,
                                createdAt = sameTime,
                            )
                        }
                    exception.message shouldContain "expiresAt must be after createdAt"
                }
            }

            `when`("createdAt is far in the future") {
                then("construction fails with IllegalArgumentException") {
                    val farFutureCreatedAt = now + 1.hours
                    val expiresAt = farFutureCreatedAt + 1.days

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            RefreshToken(
                                id = Ulid.generate(),
                                userId = userId,
                                tokenHash = "abc123hash",
                                deviceName = null,
                                expiresAt = expiresAt,
                                createdAt = farFutureCreatedAt,
                            )
                        }
                    exception.message shouldContain "createdAt must not be in the far future"
                }
            }

            `when`("createdAt is within clock skew tolerance") {
                then("construction succeeds") {
                    val nearFutureCreatedAt = now + 2.minutes
                    val expiresAt = nearFutureCreatedAt + 1.days

                    val token =
                        RefreshToken(
                            id = Ulid.generate(),
                            userId = userId,
                            tokenHash = "abc123hash",
                            deviceName = null,
                            expiresAt = expiresAt,
                            createdAt = nearFutureCreatedAt,
                        )

                    token.createdAt shouldBe nearFutureCreatedAt
                }
            }

            `when`("expiresAt is too far from createdAt") {
                then("construction fails with IllegalArgumentException") {
                    val createdAt = now
                    val expiresAt = createdAt + 100.days // Exceeds MAX_EXPIRY_DURATION (90 days)

                    val exception =
                        shouldThrow<IllegalArgumentException> {
                            RefreshToken(
                                id = Ulid.generate(),
                                userId = userId,
                                tokenHash = "abc123hash",
                                deviceName = null,
                                expiresAt = expiresAt,
                                createdAt = createdAt,
                            )
                        }
                    exception.message shouldContain "expiresAt must be within"
                }
            }

            `when`("expiresAt is at max expiry duration") {
                then("construction succeeds") {
                    val createdAt = now
                    val expiresAt = createdAt + RefreshToken.MAX_EXPIRY_DURATION

                    val token =
                        RefreshToken(
                            id = Ulid.generate(),
                            userId = userId,
                            tokenHash = "abc123hash",
                            deviceName = null,
                            expiresAt = expiresAt,
                            createdAt = createdAt,
                        )

                    token.expiresAt shouldBe expiresAt
                }
            }

            `when`("valid temporal values are provided") {
                then("construction succeeds with correct timestamps") {
                    val createdAt = now
                    val expiresAt = createdAt + 30.days

                    val token =
                        RefreshToken(
                            id = Ulid.generate(),
                            userId = userId,
                            tokenHash = "abc123hash",
                            deviceName = "Test Device",
                            expiresAt = expiresAt,
                            createdAt = createdAt,
                        )

                    token.createdAt shouldBe createdAt
                    token.expiresAt shouldBe expiresAt
                }
            }
        }

        given("RefreshToken factory method") {
            `when`("creating with default expiry duration") {
                then("uses DEFAULT_EXPIRY_DURATION") {
                    val fakeClock = FakeClock(now)
                    val token =
                        RefreshToken.create(
                            id = Ulid.generate(),
                            userId = userId,
                            tokenHash = "abc123hash",
                            clock = fakeClock,
                        )

                    token.createdAt shouldBe now
                    token.expiresAt shouldBe now + RefreshToken.DEFAULT_EXPIRY_DURATION
                }
            }

            `when`("creating with custom expiry duration") {
                then("uses the custom duration and device name") {
                    val fakeClock = FakeClock(now)
                    val customExpiry = 14.days
                    val token =
                        RefreshToken.create(
                            id = Ulid.generate(),
                            userId = userId,
                            tokenHash = "abc123hash",
                            deviceName = "Test Device",
                            expiresIn = customExpiry,
                            clock = fakeClock,
                        )

                    token.createdAt shouldBe now
                    token.expiresAt shouldBe now + customExpiry
                    token.deviceName shouldBe "Test Device"
                }
            }
        }
    }) {
    /**
     * Fake clock for testing that returns a fixed instant.
     */
    private class FakeClock(
        private val fixedInstant: Instant,
    ) : Clock {
        override fun now(): Instant = fixedInstant
    }
}
