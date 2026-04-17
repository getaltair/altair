package com.getaltair.altair.data.auth

import android.util.Log
import com.getaltair.altair.data.network.AuthApi
import com.getaltair.altair.data.network.AuthResponse
import com.getaltair.altair.data.network.RefreshRequest
import io.mockk.coEvery
import io.mockk.every
import io.mockk.mockk
import io.mockk.mockkStatic
import io.mockk.unmockkAll
import io.mockk.verify
import kotlinx.coroutines.ExperimentalCoroutinesApi
import okhttp3.Protocol
import okhttp3.Request
import okhttp3.Response
import org.junit.jupiter.api.AfterEach
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Assertions.assertNotNull
import org.junit.jupiter.api.Assertions.assertNull
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test

/**
 * Unit tests for [AuthAuthenticator], covering FA-019:
 * on 401 with a valid refresh token the request is retried with the new token;
 * if a prior 401 response exists the authenticator returns null to prevent loops.
 */
@OptIn(ExperimentalCoroutinesApi::class)
class AuthAuthenticatorTest {
    private lateinit var tokenPreferences: TokenPreferences
    private lateinit var authApi: AuthApi
    private lateinit var authenticator: AuthAuthenticator

    @BeforeEach
    fun setUp() {
        mockkStatic(Log::class)
        tokenPreferences = mockk(relaxed = true)
        authApi = mockk()
        authenticator = AuthAuthenticator(tokenPreferences, authApi)
    }

    @AfterEach
    fun tearDown() {
        unmockkAll()
    }

    /**
     * FA-019: When no prior 401 response exists and a valid refresh token is stored,
     * authenticate() refreshes and returns a new request with the updated Authorization header.
     */
    @Test
    fun `refreshToken_retriesWithNewHeader - returns request with new bearer token`() {
        every { tokenPreferences.refreshToken } returns "valid-refresh-token"
        coEvery {
            authApi.refresh(RefreshRequest("valid-refresh-token"))
        } returns
            AuthResponse(
                accessToken = "new-access-token",
                refreshToken = "new-refresh-token",
            )

        val originalRequest =
            Request
                .Builder()
                .url("https://example.com/api/data")
                .build()
        val response = buildResponse(code = 401, request = originalRequest, priorResponse = null)

        val result = authenticator.authenticate(route = null, response = response)

        assertNotNull(result, "authenticate() must return a non-null request on valid refresh")
        assertEquals(
            "Bearer new-access-token",
            result!!.header("Authorization"),
            "Retried request must carry the new access token in the Authorization header",
        )
    }

    /**
     * FA-019: When the prior response already has a 401 code, authenticate() returns null
     * to prevent an infinite retry loop, and clears stored tokens.
     */
    @Test
    fun `noRetryOnPriorResponse - returns null when priorResponse code is 401`() {
        val originalRequest =
            Request
                .Builder()
                .url("https://example.com/api/data")
                .build()

        val priorResponse = buildResponse(code = 401, request = originalRequest, priorResponse = null)
        val currentResponse = buildResponse(code = 401, request = originalRequest, priorResponse = priorResponse)

        val result = authenticator.authenticate(route = null, response = currentResponse)

        assertNull(result, "authenticate() must return null when priorResponse is 401 to stop retry loop")
        verify { tokenPreferences.clearTokens() }
    }

    /**
     * FA-019: When no refresh token is stored, authenticate() returns null
     * and clears tokens (session is invalid).
     */
    @Test
    fun `noRefreshToken - returns null when refresh token is absent`() {
        every { tokenPreferences.refreshToken } returns null

        val originalRequest =
            Request
                .Builder()
                .url("https://example.com/api/data")
                .build()
        val response = buildResponse(code = 401, request = originalRequest, priorResponse = null)

        val result = authenticator.authenticate(route = null, response = response)

        assertNull(result, "authenticate() must return null when no refresh token is available")
        verify { tokenPreferences.clearTokens() }
    }

    /**
     * FA-019: When the token refresh API call itself throws, authenticate() returns null
     * and clears tokens.
     */
    @Test
    fun `refreshThrows - returns null and clears tokens on API failure`() {
        every { tokenPreferences.refreshToken } returns "stale-token"
        coEvery { authApi.refresh(any()) } throws RuntimeException("network error")

        val originalRequest =
            Request
                .Builder()
                .url("https://example.com/api/data")
                .build()
        val response = buildResponse(code = 401, request = originalRequest, priorResponse = null)

        val result = authenticator.authenticate(route = null, response = response)

        assertNull(result)
        verify { tokenPreferences.clearTokens() }
    }

    // ─── Helpers ──────────────────────────────────────────────────────────────

    private fun buildResponse(
        code: Int,
        request: Request,
        priorResponse: Response?,
    ): Response =
        Response
            .Builder()
            .code(code)
            .request(request)
            .protocol(Protocol.HTTP_1_1)
            .message(if (code == 401) "Unauthorized" else "OK")
            .apply { if (priorResponse != null) priorResponse(priorResponse) }
            .build()
}
