package com.getaltair.altair.data.preferences

import android.content.SharedPreferences
import app.cash.turbine.test
import com.getaltair.altair.data.auth.TokenPreferences
import io.mockk.every
import io.mockk.mockk
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.runTest
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import kotlin.test.assertFalse

/**
 * Unit tests for [TokenPreferences], covering FA-020:
 * clearTokens() nullifies both tokens and emits false on isLoggedInFlow.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class TokenPreferencesTest {
    // Backing storage for the fake SharedPreferences.
    private val storage = mutableMapOf<String, String?>()

    private lateinit var prefs: SharedPreferences
    private lateinit var editor: SharedPreferences.Editor

    @BeforeEach
    fun setUp() {
        editor = mockk(relaxed = true)
        prefs = mockk()

        every { prefs.getString(any(), null) } answers { storage[firstArg()] }
        every { prefs.edit() } returns editor
        every { editor.putString(any(), any()) } answers {
            storage[firstArg()] = secondArg()
            editor
        }
        every { editor.remove(any()) } answers {
            storage.remove(firstArg())
            editor
        }
        every { editor.apply() } returns Unit
    }

    // Build a fresh TokenPreferences using the current state of storage.
    private fun buildPreferences() = TokenPreferences(prefs)

    /**
     * FA-020: After clearTokens(), accessToken must be null.
     */
    @Test
    fun `clearTokens_nullifiesAccessToken - accessToken is null after clearTokens`() {
        storage["access_token"] = "some-access-token"
        storage["refresh_token"] = "some-refresh-token"
        val tokenPreferences = buildPreferences()

        tokenPreferences.clearTokens()

        assertNull(
            tokenPreferences.accessToken,
            "accessToken must be null after clearTokens()",
        )
    }

    /**
     * FA-020: After clearTokens(), refreshToken must be null.
     */
    @Test
    fun `clearTokens_nullifiesRefreshToken - refreshToken is null after clearTokens`() {
        storage["access_token"] = "some-access-token"
        storage["refresh_token"] = "some-refresh-token"
        val tokenPreferences = buildPreferences()

        tokenPreferences.clearTokens()

        assertNull(
            tokenPreferences.refreshToken,
            "refreshToken must be null after clearTokens()",
        )
    }

    /**
     * FA-020: isLoggedInFlow emits false after clearTokens() when previously true.
     */
    @Test
    fun `isLoggedInFlow_emitsFalseAfterClearTokens - flow transitions to false`() =
        runTest {
            // Seed with an access token so initial isLoggedIn = true.
            storage["access_token"] = "some-access-token"
            val tokenPreferences = buildPreferences()

            tokenPreferences.isLoggedInFlow.test {
                // Initial emission: true (token is present).
                awaitItem().let { initial ->
                    assert(initial) { "Expected initial isLoggedInFlow to be true when token is present" }
                }

                tokenPreferences.clearTokens()

                // Next emission: false (token cleared).
                assertFalse(
                    awaitItem(),
                    "isLoggedInFlow must emit false after clearTokens()",
                )

                cancelAndIgnoreRemainingEvents()
            }
        }
}
