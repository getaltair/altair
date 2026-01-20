package com.getaltair.altair.service.auth

import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.test.runTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Tests for NativeSecureTokenStorage.
 *
 * Uses a fake in-memory credential store provider to test the storage
 * wrapper without requiring actual native credential stores.
 */
@Suppress("TooManyFunctions")
class NativeSecureTokenStorageTest {
    private val fakeProvider = FakeCredentialStoreProvider()
    private val storage = NativeSecureTokenStorage(fakeProvider)

    @Test
    fun `access token storage round-trips correctly`() =
        runTest {
            val token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test-access-token"

            storage.saveAccessToken(token)
            val retrieved = storage.getAccessToken()

            assertEquals(token, retrieved)
        }

    @Test
    fun `refresh token storage round-trips correctly`() =
        runTest {
            val token = "refresh-token-abc123-xyz789"

            storage.saveRefreshToken(token)
            val retrieved = storage.getRefreshToken()

            assertEquals(token, retrieved)
        }

    @Test
    fun `token expiration storage round-trips correctly`() =
        runTest {
            val expiration = 1_705_500_900_000L

            storage.saveTokenExpiration(expiration)
            val retrieved = storage.getTokenExpiration()

            assertEquals(expiration, retrieved)
        }

    @Test
    fun `user id storage round-trips correctly`() =
        runTest {
            val userId = Ulid("01HW0ABCD00000000000000001")

            storage.saveUserId(userId)
            val retrieved = storage.getUserId()

            assertEquals(userId, retrieved)
        }

    @Test
    fun `getAccessToken returns null when no token stored`() =
        runTest {
            val retrieved = storage.getAccessToken()

            assertNull(retrieved)
        }

    @Test
    fun `getRefreshToken returns null when no token stored`() =
        runTest {
            val retrieved = storage.getRefreshToken()

            assertNull(retrieved)
        }

    @Test
    fun `getTokenExpiration returns null when no expiration stored`() =
        runTest {
            val retrieved = storage.getTokenExpiration()

            assertNull(retrieved)
        }

    @Test
    fun `getUserId returns null when no user id stored`() =
        runTest {
            val retrieved = storage.getUserId()

            assertNull(retrieved)
        }

    @Test
    fun `clear removes all stored data`() =
        runTest {
            storage.saveAccessToken("access-token")
            storage.saveRefreshToken("refresh-token")
            storage.saveTokenExpiration(1_705_500_900_000L)
            storage.saveUserId(Ulid("01HW0ABCD00000000000000001"))

            storage.clear()

            assertNull(storage.getAccessToken())
            assertNull(storage.getRefreshToken())
            assertNull(storage.getTokenExpiration())
            assertNull(storage.getUserId())
        }

    @Test
    fun `hasStoredCredentials returns true when refresh token exists`() =
        runTest {
            storage.saveRefreshToken("refresh-token")

            assertTrue(storage.hasStoredCredentials())
        }

    @Test
    fun `hasStoredCredentials returns false when no refresh token`() =
        runTest {
            storage.saveAccessToken("access-token")

            assertFalse(storage.hasStoredCredentials())
        }

    @Test
    fun `hasStoredCredentials returns false after clear`() =
        runTest {
            storage.saveRefreshToken("refresh-token")
            storage.clear()

            assertFalse(storage.hasStoredCredentials())
        }

    @Test
    fun `overwriting token replaces previous value`() =
        runTest {
            storage.saveAccessToken("original-token")
            storage.saveAccessToken("updated-token")

            assertEquals("updated-token", storage.getAccessToken())
        }

    @Test
    fun `tokens with special characters are stored correctly`() =
        runTest {
            val tokenWithSpecialChars = "token-with-special-chars!@#\$%^&*()_+-=[]{}|;':\",./<>?"

            storage.saveAccessToken(tokenWithSpecialChars)
            val retrieved = storage.getAccessToken()

            assertEquals(tokenWithSpecialChars, retrieved)
        }

    @Test
    fun `different ulid values are stored correctly`() =
        runTest {
            val differentUserId = Ulid("01HW0ABCD99999999999999999")

            storage.saveUserId(differentUserId)
            val retrieved = storage.getUserId()

            assertEquals(differentUserId, retrieved)
        }

    @Test
    fun `token expiration boundary values work correctly`() =
        runTest {
            storage.saveTokenExpiration(Long.MAX_VALUE)
            assertEquals(Long.MAX_VALUE, storage.getTokenExpiration())

            storage.saveTokenExpiration(0L)
            assertEquals(0L, storage.getTokenExpiration())
        }

    @Test
    fun `delegates to provider store method`() =
        runTest {
            storage.saveAccessToken("token")

            assertTrue(fakeProvider.storeCalled)
            assertEquals("access_token", fakeProvider.lastStoredKey)
            assertEquals("token", fakeProvider.lastStoredValue)
        }

    @Test
    fun `delegates to provider retrieve method`() =
        runTest {
            fakeProvider.store("access_token", "my-token")

            storage.getAccessToken()

            assertTrue(fakeProvider.retrieveCalled)
            assertEquals("access_token", fakeProvider.lastRetrievedKey)
        }

    @Test
    fun `delegates to provider delete method on clear`() =
        runTest {
            storage.clear()

            assertTrue(fakeProvider.deletedKeys.contains("access_token"))
            assertTrue(fakeProvider.deletedKeys.contains("refresh_token"))
            assertTrue(fakeProvider.deletedKeys.contains("token_expiration"))
            assertTrue(fakeProvider.deletedKeys.contains("user_id"))
        }

    /**
     * Fake in-memory credential store provider for testing.
     */
    private class FakeCredentialStoreProvider : CredentialStoreProvider {
        override val name: String = "Fake Provider"

        private val credentials = mutableMapOf<String, String>()

        var storeCalled = false
        var retrieveCalled = false
        var lastStoredKey: String? = null
        var lastStoredValue: String? = null
        var lastRetrievedKey: String? = null
        val deletedKeys = mutableListOf<String>()

        override fun isAvailable(): Boolean = true

        override fun store(
            key: String,
            value: String,
        ): Boolean {
            storeCalled = true
            lastStoredKey = key
            lastStoredValue = value
            credentials[key] = value
            return true
        }

        override fun retrieve(key: String): String? {
            retrieveCalled = true
            lastRetrievedKey = key
            return credentials[key]
        }

        override fun delete(key: String): Boolean {
            deletedKeys.add(key)
            credentials.remove(key)
            return true
        }
    }
}
