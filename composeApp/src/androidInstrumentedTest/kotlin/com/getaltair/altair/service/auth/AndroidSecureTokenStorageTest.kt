package com.getaltair.altair.service.auth

import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import com.getaltair.altair.domain.types.Ulid
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.nulls.shouldBeNull
import io.kotest.matchers.shouldBe
import kotlinx.coroutines.runBlocking
import org.junit.After
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

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
    fun accessTokenStorageRoundTripsCorrectly(): Unit =
        runBlocking {
            val token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test-access-token"

            storage.saveAccessToken(token)
            val retrieved = storage.getAccessToken()

            retrieved shouldBe token
        }

    @Test
    fun refreshTokenStorageRoundTripsCorrectly(): Unit =
        runBlocking {
            val token = "refresh-token-abc123-xyz789"

            storage.saveRefreshToken(token)
            val retrieved = storage.getRefreshToken()

            retrieved shouldBe token
        }

    @Test
    fun tokenExpirationStorageRoundTripsCorrectly(): Unit =
        runBlocking {
            val expiration = TEST_EXPIRATION_TIMESTAMP

            storage.saveTokenExpiration(expiration)
            val retrieved = storage.getTokenExpiration()

            retrieved shouldBe expiration
        }

    @Test
    fun userIdStorageRoundTripsCorrectly(): Unit =
        runBlocking {
            val userId = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")

            storage.saveUserId(userId)
            val retrieved = storage.getUserId()

            retrieved shouldBe userId
        }

    @Test
    fun getAccessTokenReturnsNullWhenNoTokenStored(): Unit =
        runBlocking {
            val retrieved = storage.getAccessToken()

            retrieved.shouldBeNull()
        }

    @Test
    fun getRefreshTokenReturnsNullWhenNoTokenStored(): Unit =
        runBlocking {
            val retrieved = storage.getRefreshToken()

            retrieved.shouldBeNull()
        }

    @Test
    fun clearRemovesAllStoredData(): Unit =
        runBlocking {
            storage.saveAccessToken("access-token")
            storage.saveRefreshToken("refresh-token")
            storage.saveTokenExpiration(TEST_EXPIRATION_TIMESTAMP)
            storage.saveUserId(Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV"))

            storage.clear()

            storage.getAccessToken().shouldBeNull()
            storage.getRefreshToken().shouldBeNull()
            storage.getTokenExpiration().shouldBeNull()
            storage.getUserId().shouldBeNull()
        }

    @Test
    fun hasStoredCredentialsReturnsTrueWhenRefreshTokenExists(): Unit =
        runBlocking {
            storage.saveRefreshToken("refresh-token")

            storage.hasStoredCredentials().shouldBeTrue()
        }

    @Test
    fun hasStoredCredentialsReturnsFalseWhenNoRefreshToken(): Unit =
        runBlocking {
            storage.saveAccessToken("access-token")

            storage.hasStoredCredentials().shouldBeFalse()
        }

    @Test
    fun hasStoredCredentialsReturnsFalseAfterClear(): Unit =
        runBlocking {
            storage.saveRefreshToken("refresh-token")
            storage.clear()

            storage.hasStoredCredentials().shouldBeFalse()
        }

    @Test
    fun overwritingTokenReplacesPreviousValue(): Unit =
        runBlocking {
            storage.saveAccessToken("original-token")
            storage.saveAccessToken("updated-token")

            storage.getAccessToken() shouldBe "updated-token"
        }

    @Test
    fun tokensWithSpecialCharactersAreStoredCorrectly(): Unit =
        runBlocking {
            val tokenWithSpecialChars = "token-with-special-chars!@#\$%^&*()_+-=[]{}|;':\",./<>?"

            storage.saveAccessToken(tokenWithSpecialChars)
            val retrieved = storage.getAccessToken()

            retrieved shouldBe tokenWithSpecialChars
        }

    @Test
    fun longTokensAreStoredCorrectly(): Unit =
        runBlocking {
            val longToken = "a".repeat(LONG_TOKEN_LENGTH)

            storage.saveAccessToken(longToken)
            val retrieved = storage.getAccessToken()

            retrieved shouldBe longToken
        }

    @Test
    fun differentUserIdsAreStoredCorrectly(): Unit =
        runBlocking {
            val userId1 = Ulid("01ARZ3NDEKTSV4RRFFQ69G5FAV")
            val userId2 = Ulid("01BRCD4EFGHTSV5SSGHR8H6GBX")

            storage.saveUserId(userId1)
            storage.getUserId() shouldBe userId1

            storage.saveUserId(userId2)
            storage.getUserId() shouldBe userId2
        }

    companion object {
        /** Test timestamp representing January 17, 2024 */
        private const val TEST_EXPIRATION_TIMESTAMP = 1_705_500_900_000L

        /** Long token length for stress testing */
        private const val LONG_TOKEN_LENGTH = 4096
    }
}
