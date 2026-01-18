package com.getaltair.altair.service.auth

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.rpc.PublicAuthService
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlin.time.Clock
import kotlin.time.ExperimentalTime

/**
 * Manages authentication state and token lifecycle.
 *
 * Responsibilities:
 * - Login and registration flows
 * - Token storage and retrieval
 * - Automatic token refresh before expiration
 * - Logout and credential cleanup
 * - Auth state observation via StateFlow
 *
 * This class is the primary entry point for client-side authentication.
 */
@OptIn(ExperimentalTime::class)
@Suppress("TooManyFunctions") // Auth management requires multiple operations
class AuthManager(
    private val tokenStorage: SecureTokenStorage,
    private val publicAuthService: PublicAuthService,
    private val scope: CoroutineScope,
) {
    private val _authState = MutableStateFlow<AuthState>(AuthState.Loading)
    val authState: StateFlow<AuthState> = _authState.asStateFlow()

    private val refreshMutex = Mutex()
    private var refreshJob: Job? = null

    /**
     * Initialize the auth manager by checking for stored credentials.
     *
     * Call this on app startup to restore session state.
     */
    suspend fun initialize() {
        if (tokenStorage.hasStoredCredentials()) {
            val accessToken = tokenStorage.getAccessToken()
            val userId = tokenStorage.getUserId()
            val expiration = tokenStorage.getTokenExpiration()

            if (accessToken != null && userId != null) {
                // Check if token is expired
                handleTokenExpiration(expiration, userId)
            } else {
                _authState.value = AuthState.Unauthenticated
            }
        } else {
            _authState.value = AuthState.Unauthenticated
        }
    }

    private suspend fun handleTokenExpiration(
        expiration: Long?,
        userId: String,
    ) {
        if (expiration != null && isTokenExpired(expiration)) {
            // Try to refresh
            refreshToken().fold(
                ifLeft = {
                    // Refresh failed, user needs to re-login
                    _authState.value = AuthState.Unauthenticated
                },
                ifRight = {
                    _authState.value = AuthState.Authenticated(userId)
                    startTokenRefreshTimer()
                },
            )
        } else {
            _authState.value = AuthState.Authenticated(userId)
            startTokenRefreshTimer()
        }
    }

    /**
     * Login with email and password.
     *
     * @param email User's email address
     * @param password User's password
     * @return Either an error or the authenticated user ID
     */
    suspend fun login(
        email: String,
        password: String,
    ): Either<AuthError, String> {
        _authState.value = AuthState.Loading

        @Suppress("TooGenericExceptionCaught", "SwallowedException") // RPC calls can throw various exceptions
        return try {
            val response = publicAuthService.login(AuthRequest(email, password))
            storeTokens(response)
            _authState.value = AuthState.Authenticated(response.userId)
            startTokenRefreshTimer()
            response.userId.right()
        } catch (e: Exception) {
            _authState.value = AuthState.Unauthenticated
            AuthError.InvalidCredentials.left()
        }
    }

    /**
     * Register a new user account.
     *
     * @param email User's email address
     * @param password User's password
     * @param displayName User's display name
     * @param inviteCode Optional invite code (required for non-first users)
     * @return Either an error or the new user ID
     */
    suspend fun register(
        email: String,
        password: String,
        displayName: String,
        inviteCode: String? = null,
    ): Either<AuthError, String> {
        _authState.value = AuthState.Loading

        @Suppress("TooGenericExceptionCaught") // RPC calls can throw various exceptions
        return try {
            val response =
                publicAuthService.register(
                    RegisterRequest(
                        email = email,
                        password = password,
                        displayName = displayName,
                        inviteCode = inviteCode,
                    ),
                )
            storeTokens(response)
            _authState.value = AuthState.Authenticated(response.userId)
            startTokenRefreshTimer()
            response.userId.right()
        } catch (e: Exception) {
            _authState.value = AuthState.Unauthenticated
            mapRegistrationError(e).left()
        }
    }

    /**
     * Logout and clear stored credentials.
     */
    suspend fun logout() {
        refreshJob?.cancel()
        refreshJob = null
        tokenStorage.clear()
        _authState.value = AuthState.Unauthenticated
    }

    /**
     * Get the current access token, refreshing if needed.
     *
     * @return The access token, or null if not authenticated
     */
    suspend fun getValidAccessToken(): String? {
        val expiration = tokenStorage.getTokenExpiration()

        // Check if token is expired or about to expire
        if (expiration != null && shouldRefresh(expiration)) {
            refreshToken()
        }

        return tokenStorage.getAccessToken()
    }

    /**
     * Manually refresh the access token.
     *
     * @return Either an error or Unit on success
     */
    suspend fun refreshToken(): Either<AuthError, Unit> =
        refreshMutex.withLock {
            val refreshToken =
                tokenStorage.getRefreshToken()
                    ?: return AuthError.TokenExpired(0L).left()

            @Suppress("TooGenericExceptionCaught", "SwallowedException") // RPC can throw various exceptions
            return try {
                val response = publicAuthService.refresh(refreshToken)

                // Store new access token (refresh token is not rotated by server for refresh calls)
                val expiresAtMillis =
                    Clock.System.now().toEpochMilliseconds() +
                        response.expiresIn * MILLIS_PER_SECOND
                tokenStorage.saveAccessToken(response.accessToken)
                tokenStorage.saveTokenExpiration(expiresAtMillis)

                Unit.right()
            } catch (e: Exception) {
                // Refresh failed, user needs to re-login
                logout()
                AuthError.TokenExpired(0L).left()
            }
        }

    private suspend fun storeTokens(response: AuthResponse) {
        val expiresAtMillis =
            Clock.System.now().toEpochMilliseconds() +
                response.expiresIn * MILLIS_PER_SECOND

        tokenStorage.saveAccessToken(response.accessToken)
        tokenStorage.saveRefreshToken(response.refreshToken)
        tokenStorage.saveTokenExpiration(expiresAtMillis)
        tokenStorage.saveUserId(response.userId)
    }

    private fun startTokenRefreshTimer() {
        refreshJob?.cancel()
        refreshJob =
            scope.launch {
                while (true) {
                    val expiration = tokenStorage.getTokenExpiration()
                    if (expiration != null) {
                        val delayMs = calculateRefreshDelay(expiration)
                        if (delayMs > 0) {
                            delay(delayMs)
                        }
                        refreshToken()
                    } else {
                        // No expiration info, refresh in 5 minutes
                        delay(REFRESH_BUFFER_MS)
                        refreshToken()
                    }
                }
            }
    }

    private fun isTokenExpired(expiresAtMillis: Long): Boolean {
        val nowMillis = Clock.System.now().toEpochMilliseconds()
        return nowMillis >= expiresAtMillis
    }

    private fun shouldRefresh(expiresAtMillis: Long): Boolean =
        // Refresh if token expires in less than 5 minutes
        Clock.System.now().toEpochMilliseconds() >= expiresAtMillis - REFRESH_BUFFER_MS

    private fun calculateRefreshDelay(expiresAtMillis: Long): Long {
        // Refresh 5 minutes before expiration
        val targetRefreshTime = expiresAtMillis - REFRESH_BUFFER_MS
        return maxOf(0L, targetRefreshTime - Clock.System.now().toEpochMilliseconds())
    }

    private fun mapRegistrationError(e: Exception): AuthError {
        val message = e.message ?: ""
        return when {
            message.contains("already registered", ignoreCase = true) -> AuthError.EmailAlreadyExists
            message.contains("invite code", ignoreCase = true) -> AuthError.InvalidInviteCode
            message.contains("password", ignoreCase = true) -> AuthError.WeakPassword
            else -> AuthError.RegistrationFailed(message)
        }
    }

    companion object {
        private const val MILLIS_PER_SECOND = 1000L
        private const val SECONDS_PER_MINUTE = 60L
        private const val REFRESH_BUFFER_MINUTES = 5L
        private const val REFRESH_BUFFER_MS = REFRESH_BUFFER_MINUTES * SECONDS_PER_MINUTE * MILLIS_PER_SECOND
    }
}

/**
 * Represents the current authentication state.
 */
sealed class AuthState {
    /**
     * Authentication state is being determined.
     */
    data object Loading : AuthState()

    /**
     * User is not authenticated.
     */
    data object Unauthenticated : AuthState()

    /**
     * User is authenticated.
     *
     * @param userId The authenticated user's ID
     */
    data class Authenticated(
        val userId: String,
    ) : AuthState()
}
