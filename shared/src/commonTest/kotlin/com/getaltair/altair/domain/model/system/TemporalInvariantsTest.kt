package com.getaltair.altair.domain.model.system

import com.getaltair.altair.domain.types.Ulid
import kotlinx.datetime.Instant
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue
import kotlin.time.Clock
import kotlin.time.Duration.Companion.days
import kotlin.time.Duration.Companion.hours
import kotlin.time.Duration.Companion.minutes

/**
 * Tests for temporal invariants on InviteCode and RefreshToken.
 */
@Suppress("TooManyFunctions")
class TemporalInvariantsTest {
    private val now: Instant = Clock.System.now()
    private val userId = Ulid.generate()

    // ===== InviteCode Temporal Invariant Tests =====

    @Test
    fun `InviteCode rejects expiresAt before createdAt`() {
        val createdAt = now
        val expiresAt = now - 1.hours

        assertFailsWith<IllegalArgumentException> {
            InviteCode(
                id = Ulid.generate(),
                code = "TESTCODE123",
                createdBy = userId,
                expiresAt = expiresAt,
                createdAt = createdAt,
            )
        }.also { exception ->
            assertTrue(exception.message?.contains("expiresAt must be after createdAt") == true)
        }
    }

    @Test
    fun `InviteCode rejects expiresAt equal to createdAt`() {
        val sameTime = now

        assertFailsWith<IllegalArgumentException> {
            InviteCode(
                id = Ulid.generate(),
                code = "TESTCODE123",
                createdBy = userId,
                expiresAt = sameTime,
                createdAt = sameTime,
            )
        }.also { exception ->
            assertTrue(exception.message?.contains("expiresAt must be after createdAt") == true)
        }
    }

    @Test
    fun `InviteCode rejects createdAt far in the future`() {
        val farFutureCreatedAt = now + 1.hours
        val expiresAt = farFutureCreatedAt + 1.days

        assertFailsWith<IllegalArgumentException> {
            InviteCode(
                id = Ulid.generate(),
                code = "TESTCODE123",
                createdBy = userId,
                expiresAt = expiresAt,
                createdAt = farFutureCreatedAt,
            )
        }.also { exception ->
            assertTrue(exception.message?.contains("createdAt must not be in the far future") == true)
        }
    }

    @Test
    fun `InviteCode accepts createdAt within clock skew tolerance`() {
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

        assertEquals(nearFutureCreatedAt, inviteCode.createdAt)
    }

    @Test
    fun `InviteCode rejects expiresAt too far from createdAt`() {
        val createdAt = now
        val expiresAt = createdAt + 100.days // Exceeds MAX_EXPIRY_DURATION (90 days)

        assertFailsWith<IllegalArgumentException> {
            InviteCode(
                id = Ulid.generate(),
                code = "TESTCODE123",
                createdBy = userId,
                expiresAt = expiresAt,
                createdAt = createdAt,
            )
        }.also { exception ->
            assertTrue(exception.message?.contains("expiresAt must be within") == true)
        }
    }

    @Test
    fun `InviteCode accepts expiresAt at max expiry duration`() {
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

        assertEquals(expiresAt, inviteCode.expiresAt)
    }

    @Test
    fun `InviteCode accepts valid temporal values`() {
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

        assertEquals(createdAt, inviteCode.createdAt)
        assertEquals(expiresAt, inviteCode.expiresAt)
    }

    // ===== InviteCode Factory Method Tests =====

    @Test
    fun `InviteCode create factory uses default expiry duration`() {
        val fakeClock = FakeClock(now)
        val inviteCode =
            InviteCode.create(
                id = Ulid.generate(),
                code = "TESTCODE123",
                createdBy = userId,
                clock = fakeClock,
            )

        assertEquals(now, inviteCode.createdAt)
        assertEquals(now + InviteCode.DEFAULT_EXPIRY_DURATION, inviteCode.expiresAt)
    }

    @Test
    fun `InviteCode create factory accepts custom expiry duration`() {
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

        assertEquals(now, inviteCode.createdAt)
        assertEquals(now + customExpiry, inviteCode.expiresAt)
    }

    // ===== RefreshToken Temporal Invariant Tests =====

    @Test
    fun `RefreshToken rejects expiresAt before createdAt`() {
        val createdAt = now
        val expiresAt = now - 1.hours

        assertFailsWith<IllegalArgumentException> {
            RefreshToken(
                id = Ulid.generate(),
                userId = userId,
                tokenHash = "abc123hash",
                deviceName = null,
                expiresAt = expiresAt,
                createdAt = createdAt,
            )
        }.also { exception ->
            assertTrue(exception.message?.contains("expiresAt must be after createdAt") == true)
        }
    }

    @Test
    fun `RefreshToken rejects expiresAt equal to createdAt`() {
        val sameTime = now

        assertFailsWith<IllegalArgumentException> {
            RefreshToken(
                id = Ulid.generate(),
                userId = userId,
                tokenHash = "abc123hash",
                deviceName = null,
                expiresAt = sameTime,
                createdAt = sameTime,
            )
        }.also { exception ->
            assertTrue(exception.message?.contains("expiresAt must be after createdAt") == true)
        }
    }

    @Test
    fun `RefreshToken rejects createdAt far in the future`() {
        val farFutureCreatedAt = now + 1.hours
        val expiresAt = farFutureCreatedAt + 1.days

        assertFailsWith<IllegalArgumentException> {
            RefreshToken(
                id = Ulid.generate(),
                userId = userId,
                tokenHash = "abc123hash",
                deviceName = null,
                expiresAt = expiresAt,
                createdAt = farFutureCreatedAt,
            )
        }.also { exception ->
            assertTrue(exception.message?.contains("createdAt must not be in the far future") == true)
        }
    }

    @Test
    fun `RefreshToken accepts createdAt within clock skew tolerance`() {
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

        assertEquals(nearFutureCreatedAt, token.createdAt)
    }

    @Test
    fun `RefreshToken rejects expiresAt too far from createdAt`() {
        val createdAt = now
        val expiresAt = createdAt + 100.days // Exceeds MAX_EXPIRY_DURATION (90 days)

        assertFailsWith<IllegalArgumentException> {
            RefreshToken(
                id = Ulid.generate(),
                userId = userId,
                tokenHash = "abc123hash",
                deviceName = null,
                expiresAt = expiresAt,
                createdAt = createdAt,
            )
        }.also { exception ->
            assertTrue(exception.message?.contains("expiresAt must be within") == true)
        }
    }

    @Test
    fun `RefreshToken accepts expiresAt at max expiry duration`() {
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

        assertEquals(expiresAt, token.expiresAt)
    }

    @Test
    fun `RefreshToken accepts valid temporal values`() {
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

        assertEquals(createdAt, token.createdAt)
        assertEquals(expiresAt, token.expiresAt)
    }

    // ===== RefreshToken Factory Method Tests =====

    @Test
    fun `RefreshToken create factory uses default expiry duration`() {
        val fakeClock = FakeClock(now)
        val token =
            RefreshToken.create(
                id = Ulid.generate(),
                userId = userId,
                tokenHash = "abc123hash",
                clock = fakeClock,
            )

        assertEquals(now, token.createdAt)
        assertEquals(now + RefreshToken.DEFAULT_EXPIRY_DURATION, token.expiresAt)
    }

    @Test
    fun `RefreshToken create factory accepts custom expiry duration`() {
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

        assertEquals(now, token.createdAt)
        assertEquals(now + customExpiry, token.expiresAt)
        assertEquals("Test Device", token.deviceName)
    }

    // ===== Helper Classes =====

    /**
     * Fake clock for testing that returns a fixed instant.
     */
    private class FakeClock(
        private val fixedInstant: Instant,
    ) : Clock {
        override fun now(): Instant = fixedInstant
    }
}
