package com.getaltair.altair.service.auth

import arrow.core.Either
import arrow.core.left
import arrow.core.raise.either
import arrow.core.right
import com.getaltair.altair.domain.AuthError
import com.getaltair.altair.domain.types.Ulid
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
        userId: Ulid,
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
    ): Either<AuthError, Ulid> {
        _authState.value = AuthState.Loading

        @Suppress("TooGenericExceptionCaught") // RPC calls can throw various exceptions
        return try {
            val response = publicAuthService.login(AuthRequest(email, password))
            storeTokens(response).fold(
                ifLeft = {
                    _authState.value = AuthState.Unauthenticated
                    return it.left()
                },
                ifRight = {
                    _authState.value = AuthState.Authenticated(response.userId)
                    startTokenRefreshTimer()
                    response.userId.right()
                },
            )
        } catch (
            @Suppress("SwallowedException") e: IllegalArgumentException,
        ) {
            // Server validation errors (invalid credentials, account not active)
            // Exception message intentionally not logged to avoid leaking credential status
            _authState.value = AuthState.Unauthenticated
            AuthError.InvalidCredentials.left()
        } catch (e: Exception) {
            _authState.value = AuthState.Unauthenticated
            mapExceptionToAuthError(e).left()
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
    ): Either<AuthError, Ulid> {
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
            storeTokens(response).fold(
                ifLeft = {
                    _authState.value = AuthState.Unauthenticated
                    return it.left()
                },
                ifRight = {
                    _authState.value = AuthState.Authenticated(response.userId)
                    startTokenRefreshTimer()
                    response.userId.right()
                },
            )
        } catch (e: IllegalArgumentException) {
            // Server validation errors
            _authState.value = AuthState.Unauthenticated
            mapRegistrationError(e).left()
        } catch (e: Exception) {
            _authState.value = AuthState.Unauthenticated
            mapExceptionToAuthError(e).left()
        }
    }

    /**
     * Logout and clear stored credentials.
     */
    suspend fun logout() {
        refreshJob?.cancel()
        refreshJob = null
        // Clear tokens - ignore errors as we're logging out anyway
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
                    ?: return AuthError.SessionExpired.left()

            @Suppress("TooGenericExceptionCaught") // RPC can throw various exceptions
            return try {
                val response = publicAuthService.refresh(refreshToken)

                // Store new tokens (server uses token rotation - new refresh token each time)
                either {
                    val expiresAtMillis =
                        Clock.System.now().toEpochMilliseconds() +
                            response.expiresIn * MILLIS_PER_SECOND

                    tokenStorage
                        .saveAccessToken(response.accessToken)
                        .mapLeft { AuthError.ServerError("Failed to store access token: ${it::class.simpleName}") }
                        .bind()

                    tokenStorage
                        .saveRefreshToken(response.refreshToken)
                        .mapLeft { AuthError.ServerError("Failed to store refresh token: ${it::class.simpleName}") }
                        .bind()

                    tokenStorage
                        .saveTokenExpiration(expiresAtMillis)
                        .mapLeft { AuthError.ServerError("Failed to store token expiration: ${it::class.simpleName}") }
                        .bind()
                }.onLeft {
                    logout()
                }
            } catch (
                @Suppress("SwallowedException") e: Exception,
            ) {
                // Refresh failed (token revoked, expired, or network error), user needs to re-login
                // Exception details intentionally not propagated - all refresh failures result in logout
                logout()
                AuthError.SessionExpired.left()
            }
        }

    private suspend fun storeTokens(response: AuthResponse): Either<AuthError, Unit> =
        either {
            val expiresAtMillis =
                Clock.System.now().toEpochMilliseconds() +
                    response.expiresIn * MILLIS_PER_SECOND

            // Store all tokens, failing fast on first error
            tokenStorage
                .saveAccessToken(response.accessToken)
                .mapLeft { AuthError.ServerError("Failed to store access token: ${it::class.simpleName}") }
                .bind()

            tokenStorage
                .saveRefreshToken(response.refreshToken)
                .mapLeft { AuthError.ServerError("Failed to store refresh token: ${it::class.simpleName}") }
                .bind()

            tokenStorage
                .saveTokenExpiration(expiresAtMillis)
                .mapLeft { AuthError.ServerError("Failed to store token expiration: ${it::class.simpleName}") }
                .bind()

            tokenStorage
                .saveUserId(response.userId)
                .mapLeft { AuthError.ServerError("Failed to store user ID: ${it::class.simpleName}") }
                .bind()
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
            else -> AuthError.RegistrationFailed(message.ifBlank { "Unknown registration error" })
        }
    }

    private fun mapExceptionToAuthError(e: Exception): AuthError {
        val message = e.message ?: ""
        return when {
            // Network-related errors
            e::class.simpleName?.contains("IOException") == true ||
                e::class.simpleName?.contains("ConnectException") == true ||
                e::class.simpleName?.contains("SocketException") == true ||
                e::class.simpleName?.contains("TimeoutException") == true ||
                message.contains("connect", ignoreCase = true) ||
                message.contains("timeout", ignoreCase = true) ||
                message.contains("network", ignoreCase = true) ->
                AuthError.NetworkFailure(message.ifBlank { "Connection failed" })

            // Server errors
            message.contains("500", ignoreCase = true) ||
                message.contains("503", ignoreCase = true) ||
                message.contains("server error", ignoreCase = true) ->
                AuthError.ServerError(message.ifBlank { "Server error" })

            // Default to server error for unknown exceptions
            else -> AuthError.ServerError(message.ifBlank { "Unknown error occurred" })
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
        val userId: Ulid,
    ) : AuthState()
}
