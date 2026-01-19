package com.getaltair.altair.service.auth

import kotlinx.coroutines.runBlocking
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Tests for IosSecureTokenStorage.
 *
 * These tests must run on an iOS simulator or device because they require:
 * - iOS Keychain Services for secure storage
 *
 * Run with: ./gradlew :shared:iosSimulatorArm64Test
 */
@Suppress("TooManyFunctions")
class IosSecureTokenStorageTest {
    private lateinit var storage: IosSecureTokenStorage

    @BeforeTest
    fun setup() {
        storage = IosSecureTokenStorage(serviceName = "com.getaltair.altair.test")
    }

    @AfterTest
    fun cleanup() {
        runBlocking {
            storage.clear()
        }
    }

    @Test
    fun accessTokenStorageRoundTripsCorrectly() =
        runBlocking {
            val token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test-access-token"

            storage.saveAccessToken(token)
            val retrieved = storage.getAccessToken()

            assertEquals(token, retrieved)
        }

    @Test
    fun refreshTokenStorageRoundTripsCorrectly() =
        runBlocking {
            val token = "refresh-token-abc123-xyz789"

            storage.saveRefreshToken(token)
            val retrieved = storage.getRefreshToken()

            assertEquals(token, retrieved)
        }

    @Test
    fun tokenExpirationStorageRoundTripsCorrectly() =
        runBlocking {
            val expiration = TEST_EXPIRATION_TIMESTAMP

            storage.saveTokenExpiration(expiration)
            val retrieved = storage.getTokenExpiration()

            assertEquals(expiration, retrieved)
        }

    @Test
    fun userIdStorageRoundTripsCorrectly() =
        runBlocking {
            val userId = "01HWUSER00000000000000001"

            storage.saveUserId(userId)
            val retrieved = storage.getUserId()

            assertEquals(userId, retrieved)
        }

    @Test
    fun getAccessTokenReturnsNullWhenNoTokenStored() =
        runBlocking {
            val retrieved = storage.getAccessToken()

            assertNull(retrieved)
        }

    @Test
    fun getRefreshTokenReturnsNullWhenNoTokenStored() =
        runBlocking {
            val retrieved = storage.getRefreshToken()

            assertNull(retrieved)
        }

    @Test
    fun getTokenExpirationReturnsNullWhenNoExpirationStored() =
        runBlocking {
            val retrieved = storage.getTokenExpiration()

            assertNull(retrieved)
        }

    @Test
    fun getUserIdReturnsNullWhenNoUserIdStored() =
        runBlocking {
            val retrieved = storage.getUserId()

            assertNull(retrieved)
        }

    @Test
    fun clearRemovesAllStoredData() =
        runBlocking {
            storage.saveAccessToken("access-token")
            storage.saveRefreshToken("refresh-token")
            storage.saveTokenExpiration(TEST_EXPIRATION_TIMESTAMP)
            storage.saveUserId("user-id")

            storage.clear()

            assertNull(storage.getAccessToken())
            assertNull(storage.getRefreshToken())
            assertNull(storage.getTokenExpiration())
            assertNull(storage.getUserId())
        }

    @Test
    fun hasStoredCredentialsReturnsTrueWhenRefreshTokenExists() =
        runBlocking {
            storage.saveRefreshToken("refresh-token")

            assertTrue(storage.hasStoredCredentials())
        }

    @Test
    fun hasStoredCredentialsReturnsFalseWhenNoRefreshToken() =
        runBlocking {
            storage.saveAccessToken("access-token")

            assertFalse(storage.hasStoredCredentials())
        }

    @Test
    fun hasStoredCredentialsReturnsFalseAfterClear() =
        runBlocking {
            storage.saveRefreshToken("refresh-token")
            storage.clear()

            assertFalse(storage.hasStoredCredentials())
        }

    @Test
    fun overwritingTokenReplacesPreviousValue() =
        runBlocking {
            storage.saveAccessToken("original-token")
            storage.saveAccessToken("updated-token")

            assertEquals("updated-token", storage.getAccessToken())
        }

    @Test
    fun tokensWithSpecialCharactersAreStoredCorrectly() =
        runBlocking {
            val tokenWithSpecialChars = "token-with-special-chars!@#\$%^&*()_+-=[]{}|;':\",./<>?"

            storage.saveAccessToken(tokenWithSpecialChars)
            val retrieved = storage.getAccessToken()

            assertEquals(tokenWithSpecialChars, retrieved)
        }

    @Test
    fun longTokensAreStoredCorrectly() =
        runBlocking {
            val longToken = "a".repeat(LONG_TOKEN_LENGTH)

            storage.saveAccessToken(longToken)
            val retrieved = storage.getAccessToken()

            assertEquals(longToken, retrieved)
        }

    @Test
    fun unicodeContentIsStoredCorrectly() =
        runBlocking {
            val unicodeUserId = "user-日本語-émoji-🎉"

            storage.saveUserId(unicodeUserId)
            val retrieved = storage.getUserId()

            assertEquals(unicodeUserId, retrieved)
        }

    @Test
    fun tokenExpirationBoundaryValuesWorkCorrectly() =
        runBlocking {
            storage.saveTokenExpiration(Long.MAX_VALUE)
            assertEquals(Long.MAX_VALUE, storage.getTokenExpiration())

            storage.saveTokenExpiration(0L)
            assertEquals(0L, storage.getTokenExpiration())
        }

    companion object {
        /** Test timestamp representing January 17, 2024 */
        private const val TEST_EXPIRATION_TIMESTAMP = 1_705_500_900_000L

        /** Long token length for stress testing */
        private const val LONG_TOKEN_LENGTH = 4096
    }
}
