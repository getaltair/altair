package com.getaltair.altair.service.auth

import kotlinx.coroutines.runBlocking
import java.util.prefs.Preferences
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
 * Tests for DesktopSecureTokenStorage.
 *
 * Verifies:
 * - Token storage and retrieval
 * - Encryption/decryption round-trip
 * - Clear functionality
 * - hasStoredCredentials behavior
 */
class DesktopSecureTokenStorageTest {
    private lateinit var storage: DesktopSecureTokenStorage
    private val testAppName = "AltairTest-${System.nanoTime()}"

    @BeforeTest
    fun setup() {
        storage = DesktopSecureTokenStorage(appName = testAppName)
    }

    @AfterTest
    fun cleanup() {
        runBlocking {
            storage.clear()
        }
    }

    @Test
    fun `access token storage round-trips correctly`() = runBlocking {
        val token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test-access-token"

        storage.saveAccessToken(token)
        val retrieved = storage.getAccessToken()

        assertEquals(token, retrieved)
    }

    @Test
    fun `refresh token storage round-trips correctly`() = runBlocking {
        val token = "refresh-token-abc123-xyz789"

        storage.saveRefreshToken(token)
        val retrieved = storage.getRefreshToken()

        assertEquals(token, retrieved)
    }

    @Test
    fun `token expiration storage round-trips correctly`() = runBlocking {
        val expiration = 1705500900000L

        storage.saveTokenExpiration(expiration)
        val retrieved = storage.getTokenExpiration()

        assertEquals(expiration, retrieved)
    }

    @Test
    fun `user id storage round-trips correctly`() = runBlocking {
        val userId = "01HWUSER00000000000000001"

        storage.saveUserId(userId)
        val retrieved = storage.getUserId()

        assertEquals(userId, retrieved)
    }

    @Test
    fun `getAccessToken returns null when no token stored`() = runBlocking {
        val retrieved = storage.getAccessToken()

        assertNull(retrieved)
    }

    @Test
    fun `getRefreshToken returns null when no token stored`() = runBlocking {
        val retrieved = storage.getRefreshToken()

        assertNull(retrieved)
    }

    @Test
    fun `getTokenExpiration returns null when no expiration stored`() = runBlocking {
        val retrieved = storage.getTokenExpiration()

        assertNull(retrieved)
    }

    @Test
    fun `getUserId returns null when no user id stored`() = runBlocking {
        val retrieved = storage.getUserId()

        assertNull(retrieved)
    }

    @Test
    fun `clear removes all stored data`() = runBlocking {
        storage.saveAccessToken("access-token")
        storage.saveRefreshToken("refresh-token")
        storage.saveTokenExpiration(1705500900000L)
        storage.saveUserId("user-id")

        storage.clear()

        assertNull(storage.getAccessToken())
        assertNull(storage.getRefreshToken())
        assertNull(storage.getTokenExpiration())
        assertNull(storage.getUserId())
    }

    @Test
    fun `hasStoredCredentials returns true when refresh token exists`() = runBlocking {
        storage.saveRefreshToken("refresh-token")

        assertTrue(storage.hasStoredCredentials())
    }

    @Test
    fun `hasStoredCredentials returns false when no refresh token`() = runBlocking {
        storage.saveAccessToken("access-token")

        assertFalse(storage.hasStoredCredentials())
    }

    @Test
    fun `hasStoredCredentials returns false after clear`() = runBlocking {
        storage.saveRefreshToken("refresh-token")
        storage.clear()

        assertFalse(storage.hasStoredCredentials())
    }

    @Test
    fun `overwriting token replaces previous value`() = runBlocking {
        storage.saveAccessToken("original-token")
        storage.saveAccessToken("updated-token")

        assertEquals("updated-token", storage.getAccessToken())
    }

    @Test
    fun `tokens with special characters are stored correctly`() = runBlocking {
        val tokenWithSpecialChars = "token-with-special-chars!@#\$%^&*()_+-=[]{}|;':\",./<>?"

        storage.saveAccessToken(tokenWithSpecialChars)
        val retrieved = storage.getAccessToken()

        assertEquals(tokenWithSpecialChars, retrieved)
    }

    @Test
    fun `long tokens are stored correctly`() = runBlocking {
        val longToken = "a".repeat(4096)

        storage.saveAccessToken(longToken)
        val retrieved = storage.getAccessToken()

        assertEquals(longToken, retrieved)
    }

    @Test
    fun `unicode content is stored correctly`() = runBlocking {
        val unicodeUserId = "user-日本語-émoji-🎉"

        storage.saveUserId(unicodeUserId)
        val retrieved = storage.getUserId()

        assertEquals(unicodeUserId, retrieved)
    }

    @Test
    fun `encrypted data is not stored as plaintext`() = runBlocking {
        val token = "sensitive-access-token-12345"

        storage.saveAccessToken(token)

        val prefs = Preferences.userNodeForPackage(DesktopSecureTokenStorage::class.java)
        val storedValue = prefs.get("access_token", null)

        assertNotNull(storedValue)
        assertNotEquals(token, storedValue)
        assertTrue(storedValue.matches(Regex("^[A-Za-z0-9+/=]+$")))
    }

    @Test
    fun `empty string token is stored and retrieved correctly`() = runBlocking {
        val emptyToken = ""

        storage.saveAccessToken(emptyToken)
        val retrieved = storage.getAccessToken()

        assertEquals(emptyToken, retrieved)
    }

    @Test
    fun `token expiration boundary values work correctly`() = runBlocking {
        storage.saveTokenExpiration(Long.MAX_VALUE)
        assertEquals(Long.MAX_VALUE, storage.getTokenExpiration())

        storage.saveTokenExpiration(0L)
        assertEquals(0L, storage.getTokenExpiration())
    }
}
