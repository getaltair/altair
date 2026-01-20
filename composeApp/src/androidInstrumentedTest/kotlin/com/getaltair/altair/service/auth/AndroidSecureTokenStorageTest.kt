package com.getaltair.altair.service.auth

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.runBlocking
import org.junit.After
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Instrumented tests for AndroidSecureTokenStorage.
 *
 * These tests must run on an Android device or emulator because they require:
 * - Android Keystore for encryption key generation
 * - EncryptedSharedPreferences which uses the Keystore
 *
 * Run with: ./gradlew :composeApp:connectedAndroidTest
 */
@RunWith(AndroidJUnit4::class)
@Suppress("TooManyFunctions")
class AndroidSecureTokenStorageTest {
    private lateinit var storage: AndroidSecureTokenStorage

    @Before
    fun setup() {
        val context = InstrumentationRegistry.getInstrumentation().targetContext
        storage = AndroidSecureTokenStorage(context)
    }

    @After
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
            val userId = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")

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
    fun clearRemovesAllStoredData() =
        runBlocking {
            storage.saveAccessToken("access-token")
            storage.saveRefreshToken("refresh-token")
            storage.saveTokenExpiration(TEST_EXPIRATION_TIMESTAMP)
            storage.saveUserId(Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV"))

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
    fun differentUserIdsAreStoredCorrectly() =
        runBlocking {
            val userId1 = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")
            val userId2 = Ulid("01BRCD4EFGHTSV5SSGHR8H6GBX")

            storage.saveUserId(userId1)
            assertEquals(userId1, storage.getUserId())

            storage.saveUserId(userId2)
            assertEquals(userId2, storage.getUserId())
        }

    companion object {
        /** Test timestamp representing January 17, 2024 */
        private const val TEST_EXPIRATION_TIMESTAMP = 1_705_500_900_000L

        /** Long token length for stress testing */
        private const val LONG_TOKEN_LENGTH = 4096
    }
}
