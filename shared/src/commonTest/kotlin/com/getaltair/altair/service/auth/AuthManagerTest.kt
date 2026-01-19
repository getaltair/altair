package com.getaltair.altair.service.auth

import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.test.StandardTestDispatcher
import kotlinx.coroutines.test.TestScope
import kotlinx.coroutines.test.runTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertIs
import kotlin.test.assertNull
import kotlin.test.assertTrue
import kotlin.time.Clock

/**
 * Tests for AuthManager.
 *
 * Verifies:
 * - 7.2.1: Full registration flow
 * - 7.2.2: Full login flow
 * - 7.2.3: Token refresh after expiration
 * - 7.2.4: Logout clears stored credentials
 */
@OptIn(ExperimentalCoroutinesApi::class)
@Suppress("TooManyFunctions")
class AuthManagerTest {
    private lateinit var tokenStorage: FakeSecureTokenStorage
    private lateinit var authService: FakePublicAuthService
    private lateinit var testScope: TestScope
    private lateinit var authManager: AuthManager

    @BeforeTest
    fun setup() {
        tokenStorage = FakeSecureTokenStorage()
        authService = FakePublicAuthService()
        testScope = TestScope(StandardTestDispatcher())
        authManager = AuthManager(tokenStorage, authService, testScope)
    }

    // ===== 7.2.2: Full Login Flow =====

    @Test
    fun loginWithValidCredentialsSucceeds() =
        runTest {
            val response = createAuthResponse()
            authService.loginResponse = response

            val result = authManager.login("test@example.com", "password123")

            assertTrue(result.isRight())
            assertEquals("user-123", result.getOrNull())
            assertIs<AuthState.Authenticated>(authManager.authState.value)
        }

    @Test
    fun loginStoresTokensCorrectly() =
        runTest {
            val response = createAuthResponse()
            authService.loginResponse = response

            authManager.login("test@example.com", "password123")

            assertEquals("access-token-123", tokenStorage.getAccessToken())
            assertEquals("refresh-token-123", tokenStorage.getRefreshToken())
            assertEquals("user-123", tokenStorage.getUserId())
            assertTrue(tokenStorage.hasStoredCredentials())
        }

    @Test
    fun loginWithInvalidCredentialsReturnsError() =
        runTest {
            authService.loginError = IllegalArgumentException("Invalid credentials")

            val result = authManager.login("test@example.com", "wrong-password")

            assertTrue(result.isLeft())
            assertIs<AuthError.InvalidCredentials>(result.leftOrNull())
            assertEquals(AuthState.Unauthenticated, authManager.authState.value)
        }

    @Test
    fun loginWithNetworkErrorReturnsNetworkFailure() =
        runTest {
            authService.loginError = Exception("Connection refused")

            val result = authManager.login("test@example.com", "password")

            assertTrue(result.isLeft())
            // "Connection refused" contains "connect", so maps to NetworkFailure
            assertIs<AuthError.NetworkFailure>(result.leftOrNull())
        }

    // ===== 7.2.1: Full Registration Flow =====

    @Test
    fun registerWithValidDataSucceeds() =
        runTest {
            val response = createAuthResponse()
            authService.registerResponse = response

            val result =
                authManager.register(
                    email = "new@example.com",
                    password = "password123",
                    displayName = "New User",
                    inviteCode = "INVITE123",
                )

            assertTrue(result.isRight())
            assertEquals("user-123", result.getOrNull())
            assertIs<AuthState.Authenticated>(authManager.authState.value)
        }

    @Test
    fun registerStoresTokensCorrectly() =
        runTest {
            val response = createAuthResponse()
            authService.registerResponse = response

            authManager.register(
                email = "new@example.com",
                password = "password123",
                displayName = "New User",
            )

            assertEquals("access-token-123", tokenStorage.getAccessToken())
            assertEquals("refresh-token-123", tokenStorage.getRefreshToken())
            assertEquals("user-123", tokenStorage.getUserId())
            assertTrue(tokenStorage.hasStoredCredentials())
        }

    @Test
    fun registerWithInvalidInviteCodeReturnsError() =
        runTest {
            authService.registerError = IllegalArgumentException("Invalid invite code")

            val result =
                authManager.register(
                    email = "new@example.com",
                    password = "password123",
                    displayName = "New User",
                    inviteCode = "INVALID",
                )

            assertTrue(result.isLeft())
            assertIs<AuthError.InvalidInviteCode>(result.leftOrNull())
        }

    @Test
    fun registerWithExistingEmailReturnsError() =
        runTest {
            authService.registerError = IllegalArgumentException("Email already registered")

            val result =
                authManager.register(
                    email = "existing@example.com",
                    password = "password123",
                    displayName = "User",
                )

            assertTrue(result.isLeft())
            assertIs<AuthError.EmailAlreadyExists>(result.leftOrNull())
        }

    // ===== 7.2.3: Token Refresh After Expiration =====

    @Test
    fun refreshTokenReturnsNewTokens() =
        runTest {
            // Setup initial tokens
            tokenStorage.saveAccessToken("old-access-token")
            tokenStorage.saveRefreshToken("old-refresh-token")
            tokenStorage.saveTokenExpiration(Clock.System.now().toEpochMilliseconds() - 1000)

            authService.refreshResponse =
                TokenRefreshResponse(
                    accessToken = "new-access-token",
                    refreshToken = "new-refresh-token",
                    expiresIn = 900,
                )

            val result = authManager.refreshToken()

            assertTrue(result.isRight())
            assertEquals("new-access-token", tokenStorage.getAccessToken())
            assertEquals("new-refresh-token", tokenStorage.getRefreshToken())
        }

    @Test
    fun refreshTokenWithExpiredRefreshTokenLogsOut() =
        runTest {
            // Setup initial tokens
            tokenStorage.saveAccessToken("old-access-token")
            tokenStorage.saveRefreshToken("old-refresh-token")
            tokenStorage.saveUserId("user-123")

            authService.refreshError = IllegalArgumentException("Refresh token expired")

            val result = authManager.refreshToken()

            assertTrue(result.isLeft())
            assertIs<AuthError.SessionExpired>(result.leftOrNull())
            // Should logout on failed refresh
            assertEquals(AuthState.Unauthenticated, authManager.authState.value)
        }

    @Test
    fun refreshTokenWithNoStoredRefreshTokenReturnsError() =
        runTest {
            // No tokens stored
            val result = authManager.refreshToken()

            assertTrue(result.isLeft())
            assertIs<AuthError.SessionExpired>(result.leftOrNull())
        }

    @Test
    fun getValidAccessTokenRefreshesWhenExpired() =
        runTest {
            // Setup expired tokens
            tokenStorage.saveAccessToken("old-access-token")
            tokenStorage.saveRefreshToken("old-refresh-token")
            tokenStorage.saveTokenExpiration(Clock.System.now().toEpochMilliseconds() - 1000)

            authService.refreshResponse =
                TokenRefreshResponse(
                    accessToken = "new-access-token",
                    refreshToken = "new-refresh-token",
                    expiresIn = 900,
                )

            val token = authManager.getValidAccessToken()

            assertEquals("new-access-token", token)
            assertEquals(1, authService.refreshCallCount)
        }

    @Test
    fun getValidAccessTokenDoesNotRefreshWhenNotExpired() =
        runTest {
            // Setup non-expired tokens (expires in 10 minutes)
            tokenStorage.saveAccessToken("valid-access-token")
            tokenStorage.saveRefreshToken("valid-refresh-token")
            tokenStorage.saveTokenExpiration(Clock.System.now().toEpochMilliseconds() + 600_000L)

            val token = authManager.getValidAccessToken()

            assertEquals("valid-access-token", token)
            assertEquals(0, authService.refreshCallCount)
        }

    // ===== 7.2.4: Logout Clears Stored Credentials =====

    @Test
    fun logoutClearsAllStoredCredentials() =
        runTest {
            // Setup stored credentials
            tokenStorage.saveAccessToken("access-token")
            tokenStorage.saveRefreshToken("refresh-token")
            tokenStorage.saveTokenExpiration(Clock.System.now().toEpochMilliseconds() + 3_600_000L)
            tokenStorage.saveUserId("user-123")

            assertTrue(tokenStorage.hasStoredCredentials())

            authManager.logout()

            // Verify all credentials are cleared
            assertNull(tokenStorage.getAccessToken())
            assertNull(tokenStorage.getRefreshToken())
            assertNull(tokenStorage.getTokenExpiration())
            assertNull(tokenStorage.getUserId())
            assertFalse(tokenStorage.hasStoredCredentials())
        }

    @Test
    fun logoutSetsStateToUnauthenticated() =
        runTest {
            // Setup initial authenticated state via login
            val response = createAuthResponse()
            authService.loginResponse = response
            authManager.login("test@example.com", "password123")

            assertIs<AuthState.Authenticated>(authManager.authState.value)

            authManager.logout()

            assertEquals(AuthState.Unauthenticated, authManager.authState.value)
        }

    @Test
    fun logoutIsIdempotent() =
        runTest {
            // Logout when already logged out
            authManager.logout()
            authManager.logout()

            assertEquals(AuthState.Unauthenticated, authManager.authState.value)
            assertFalse(tokenStorage.hasStoredCredentials())
        }

    // ===== Initialize Tests =====

    @Test
    fun initializeWithNoCredentialsSetsUnauthenticated() =
        runTest {
            authManager.initialize()

            assertEquals(AuthState.Unauthenticated, authManager.authState.value)
        }

    @Test
    fun initializeWithValidTokensSetsAuthenticated() =
        runTest {
            // Setup stored credentials with valid (non-expired) token
            tokenStorage.saveAccessToken("valid-access-token")
            tokenStorage.saveRefreshToken("valid-refresh-token")
            tokenStorage.saveUserId("user-123")
            // Token expires in 10 minutes (not expired)
            tokenStorage.saveTokenExpiration(Clock.System.now().toEpochMilliseconds() + 600_000L)

            authManager.initialize()

            assertIs<AuthState.Authenticated>(authManager.authState.value)
            assertEquals("user-123", (authManager.authState.value as AuthState.Authenticated).userId)
        }

    @Test
    fun initializeWithMissingAccessTokenSetsUnauthenticated() =
        runTest {
            // Only refresh token stored, no access token
            tokenStorage.saveRefreshToken("refresh-token")
            tokenStorage.saveUserId("user-123")

            authManager.initialize()

            assertEquals(AuthState.Unauthenticated, authManager.authState.value)
        }

    @Test
    fun initializeWithMissingUserIdSetsUnauthenticated() =
        runTest {
            // Access token stored but no user ID
            tokenStorage.saveAccessToken("access-token")
            tokenStorage.saveRefreshToken("refresh-token")

            authManager.initialize()

            assertEquals(AuthState.Unauthenticated, authManager.authState.value)
        }

    @Test
    fun initializeWithExpiredTokenTriesToRefresh() =
        runTest {
            // Setup stored credentials with expired token
            tokenStorage.saveAccessToken("expired-access-token")
            tokenStorage.saveRefreshToken("valid-refresh-token")
            tokenStorage.saveUserId("user-123")
            // Token expired 1 second ago
            tokenStorage.saveTokenExpiration(Clock.System.now().toEpochMilliseconds() - 1000L)

            // Setup successful refresh response
            authService.refreshResponse =
                TokenRefreshResponse(
                    accessToken = "new-access-token",
                    refreshToken = "new-refresh-token",
                    expiresIn = 900,
                )

            authManager.initialize()

            // Should have attempted refresh
            assertEquals(1, authService.refreshCallCount)
            // Should be authenticated after successful refresh
            assertIs<AuthState.Authenticated>(authManager.authState.value)
        }

    @Test
    fun initializeWithExpiredTokenAndFailedRefreshSetsUnauthenticated() =
        runTest {
            // Setup stored credentials with expired token
            tokenStorage.saveAccessToken("expired-access-token")
            tokenStorage.saveRefreshToken("expired-refresh-token")
            tokenStorage.saveUserId("user-123")
            // Token expired 1 second ago
            tokenStorage.saveTokenExpiration(Clock.System.now().toEpochMilliseconds() - 1000L)

            // Setup failed refresh
            authService.refreshError = IllegalArgumentException("Refresh token expired")

            authManager.initialize()

            // Should have attempted refresh
            assertEquals(1, authService.refreshCallCount)
            // Should be unauthenticated after failed refresh
            assertEquals(AuthState.Unauthenticated, authManager.authState.value)
        }

    @Test
    fun initializeWithNoExpirationInfoSetsAuthenticated() =
        runTest {
            // Setup stored credentials but no expiration info
            tokenStorage.saveAccessToken("access-token")
            tokenStorage.saveRefreshToken("refresh-token")
            tokenStorage.saveUserId("user-123")
            // No expiration saved

            authManager.initialize()

            // Should assume token is valid when no expiration info
            assertIs<AuthState.Authenticated>(authManager.authState.value)
        }

    // ===== Loading State Transition Tests =====

    @Test
    fun loginSetsLoadingStateDuringOperation() =
        runTest {
            val response = createAuthResponse()
            authService.loginResponse = response

            // Initial state is Loading (default)
            assertIs<AuthState.Loading>(authManager.authState.value)

            authManager.login("test@example.com", "password123")

            // After login, state should be Authenticated
            assertIs<AuthState.Authenticated>(authManager.authState.value)
        }

    @Test
    fun registerSetsLoadingStateDuringOperation() =
        runTest {
            val response = createAuthResponse()
            authService.registerResponse = response

            // Initial state is Loading (default)
            assertIs<AuthState.Loading>(authManager.authState.value)

            authManager.register(
                email = "new@example.com",
                password = "password123",
                displayName = "New User",
            )

            // After register, state should be Authenticated
            assertIs<AuthState.Authenticated>(authManager.authState.value)
        }

    // ===== Error Mapping Tests for mapRegistrationError =====

    @Test
    fun registerWithWeakPasswordReturnsWeakPasswordError() =
        runTest {
            authService.registerError = IllegalArgumentException("Password is too weak")

            val result =
                authManager.register(
                    email = "new@example.com",
                    password = "123",
                    displayName = "User",
                )

            assertTrue(result.isLeft())
            assertIs<AuthError.WeakPassword>(result.leftOrNull())
        }

    @Test
    fun registerWithUnknownErrorReturnsRegistrationFailed() =
        runTest {
            authService.registerError = IllegalArgumentException("Some unknown server issue")

            val result =
                authManager.register(
                    email = "new@example.com",
                    password = "password123",
                    displayName = "User",
                )

            assertTrue(result.isLeft())
            val error = result.leftOrNull()
            assertIs<AuthError.RegistrationFailed>(error)
            assertEquals("Some unknown server issue", error.reason)
        }

    @Test
    fun registerWithBlankErrorMessageReturnsDefaultMessage() =
        runTest {
            authService.registerError = IllegalArgumentException("")

            val result =
                authManager.register(
                    email = "new@example.com",
                    password = "password123",
                    displayName = "User",
                )

            assertTrue(result.isLeft())
            val error = result.leftOrNull()
            assertIs<AuthError.RegistrationFailed>(error)
            assertEquals("Unknown registration error", error.reason)
        }

    // ===== Error Mapping Tests for mapExceptionToAuthError =====

    @Test
    fun loginWithTimeoutErrorReturnsNetworkFailure() =
        runTest {
            authService.loginError = Exception("Request timeout")

            val result = authManager.login("test@example.com", "password")

            assertTrue(result.isLeft())
            assertIs<AuthError.NetworkFailure>(result.leftOrNull())
        }

    @Test
    fun loginWithNetworkKeywordReturnsNetworkFailure() =
        runTest {
            authService.loginError = Exception("Network unreachable")

            val result = authManager.login("test@example.com", "password")

            assertTrue(result.isLeft())
            assertIs<AuthError.NetworkFailure>(result.leftOrNull())
        }

    @Test
    fun loginWith500ErrorReturnsServerError() =
        runTest {
            authService.loginError = Exception("HTTP 500 Internal Server Error")

            val result = authManager.login("test@example.com", "password")

            assertTrue(result.isLeft())
            assertIs<AuthError.ServerError>(result.leftOrNull())
        }

    @Test
    fun loginWith503ErrorReturnsServerError() =
        runTest {
            authService.loginError = Exception("HTTP 503 Service Unavailable")

            val result = authManager.login("test@example.com", "password")

            assertTrue(result.isLeft())
            assertIs<AuthError.ServerError>(result.leftOrNull())
        }

    @Test
    fun loginWithServerErrorKeywordReturnsServerError() =
        runTest {
            authService.loginError = Exception("Internal server error occurred")

            val result = authManager.login("test@example.com", "password")

            assertTrue(result.isLeft())
            assertIs<AuthError.ServerError>(result.leftOrNull())
        }

    @Test
    fun loginWithUnknownExceptionReturnsServerError() =
        runTest {
            authService.loginError = Exception("Something unexpected happened")

            val result = authManager.login("test@example.com", "password")

            assertTrue(result.isLeft())
            val error = result.leftOrNull()
            assertIs<AuthError.ServerError>(error)
            assertEquals("Something unexpected happened", error.message)
        }

    @Test
    fun loginWithBlankExceptionMessageReturnsDefaultMessage() =
        runTest {
            authService.loginError = Exception("")

            val result = authManager.login("test@example.com", "password")

            assertTrue(result.isLeft())
            val error = result.leftOrNull()
            assertIs<AuthError.ServerError>(error)
            assertEquals("Unknown error occurred", error.message)
        }

    // ===== Token Refresh Timing Tests =====

    @Test
    fun getValidAccessTokenRefreshesWhenTokenExpiresWithinFiveMinutes() =
        runTest {
            // Setup token that expires in 4 minutes (within the 5-minute buffer)
            tokenStorage.saveAccessToken("old-access-token")
            tokenStorage.saveRefreshToken("valid-refresh-token")
            tokenStorage.saveTokenExpiration(Clock.System.now().toEpochMilliseconds() + 240_000L) // 4 minutes

            authService.refreshResponse =
                TokenRefreshResponse(
                    accessToken = "new-access-token",
                    refreshToken = "new-refresh-token",
                    expiresIn = 900,
                )

            val token = authManager.getValidAccessToken()

            // Should have refreshed since token expires within 5-minute buffer
            assertEquals(1, authService.refreshCallCount)
            assertEquals("new-access-token", token)
        }

    @Test
    fun getValidAccessTokenDoesNotRefreshWhenTokenExpiresAfterFiveMinutes() =
        runTest {
            // Setup token that expires in 6 minutes (outside the 5-minute buffer)
            tokenStorage.saveAccessToken("valid-access-token")
            tokenStorage.saveRefreshToken("valid-refresh-token")
            tokenStorage.saveTokenExpiration(Clock.System.now().toEpochMilliseconds() + 360_000L) // 6 minutes

            val token = authManager.getValidAccessToken()

            // Should NOT refresh since token expires after the 5-minute buffer
            assertEquals(0, authService.refreshCallCount)
            assertEquals("valid-access-token", token)
        }

    @Test
    fun getValidAccessTokenWithNoExpirationReturnsCurrentToken() =
        runTest {
            // Setup token with no expiration info
            tokenStorage.saveAccessToken("access-token")
            tokenStorage.saveRefreshToken("refresh-token")
            // No expiration saved

            val token = authManager.getValidAccessToken()

            // Should NOT refresh when no expiration info (cannot determine if refresh needed)
            assertEquals(0, authService.refreshCallCount)
            assertEquals("access-token", token)
        }

    // ===== Token Storage Verification Tests =====

    @Test
    fun loginPassesCorrectCredentialsToService() =
        runTest {
            val response = createAuthResponse()
            authService.loginResponse = response

            authManager.login("user@example.com", "mypassword")

            assertEquals("user@example.com", authService.lastLoginRequest?.email)
            assertEquals("mypassword", authService.lastLoginRequest?.password)
        }

    @Test
    fun registerPassesCorrectDataToService() =
        runTest {
            val response = createAuthResponse()
            authService.registerResponse = response

            authManager.register(
                email = "new@example.com",
                password = "password123",
                displayName = "Test User",
                inviteCode = "INVITE-ABC",
            )

            assertEquals("new@example.com", authService.lastRegisterRequest?.email)
            assertEquals("password123", authService.lastRegisterRequest?.password)
            assertEquals("Test User", authService.lastRegisterRequest?.displayName)
            assertEquals("INVITE-ABC", authService.lastRegisterRequest?.inviteCode)
        }

    @Test
    fun refreshPassesCorrectTokenToService() =
        runTest {
            tokenStorage.saveRefreshToken("my-refresh-token")

            authService.refreshResponse =
                TokenRefreshResponse(
                    accessToken = "new-access",
                    refreshToken = "new-refresh",
                    expiresIn = 900,
                )

            authManager.refreshToken()

            assertEquals("my-refresh-token", authService.lastRefreshToken)
        }

    // ===== Helper Methods =====

    private fun createAuthResponse(
        userId: String = "user-123",
        accessToken: String = "access-token-123",
        refreshToken: String = "refresh-token-123",
        expiresIn: Long = 900,
    ) = AuthResponse(
        accessToken = accessToken,
        refreshToken = refreshToken,
        expiresIn = expiresIn,
        userId = userId,
        displayName = "Test User",
        role = "member",
    )
}
