package com.getaltair.altair.api

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.dto.auth.*
import io.ktor.client.*
import io.ktor.client.call.*
import io.ktor.client.request.*
import io.ktor.client.statement.*
import io.ktor.http.*

/**
 * Client API for authentication operations.
 * Handles login, registration, token management, and user information.
 *
 * Uses Arrow's Either for functional error handling:
 * - Left: ApiError with detailed failure information
 * - Right: Successful response data
 */
class AuthApi(private val httpClient: HttpClient) {

    /**
     * Authenticate user with username and password.
     * Returns JWT tokens on success.
     *
     * @param request Login credentials (username and password)
     * @return Either ApiError or TokenResponse with access and refresh tokens
     */
    suspend fun login(request: LoginRequest): Either<ApiError, TokenResponse> =
        post("/api/auth/login", request)

    /**
     * Register new user with invite code.
     * Returns JWT tokens on success.
     *
     * @param request Registration details (username, email, password, invite code)
     * @return Either ApiError or TokenResponse with access and refresh tokens
     */
    suspend fun register(request: RegisterRequest): Either<ApiError, TokenResponse> =
        post("/api/auth/register", request)

    /**
     * Refresh access token using refresh token.
     *
     * @param request Refresh token request
     * @return Either ApiError or TokenResponse with new access token
     */
    suspend fun refresh(request: RefreshTokenRequest): Either<ApiError, TokenResponse> =
        post("/api/auth/refresh", request)

    /**
     * Invalidate current session and refresh token.
     * Requires valid JWT authentication.
     *
     * @return Either ApiError or Unit on successful logout
     */
    suspend fun logout(): Either<ApiError, Unit> =
        postUnit<Unit>("/api/auth/logout")

    /**
     * Change current user's password.
     * Requires valid JWT authentication and current password.
     *
     * @param request Password change request with current and new password
     * @return Either ApiError or Unit on successful password change
     */
    suspend fun changePassword(request: ChangePasswordRequest): Either<ApiError, Unit> =
        postUnit("/api/auth/change-password", request)

    /**
     * Get current authenticated user's information.
     * Requires valid JWT authentication.
     *
     * @return Either ApiError or UserResponse with user details
     */
    suspend fun me(): Either<ApiError, UserResponse> =
        get("/api/auth/me")

    // Helper methods for HTTP operations

    /**
     * Perform GET request and parse response.
     */
    private suspend inline fun <reified T> get(path: String): Either<ApiError, T> = try {
        val response = httpClient.get(path)
        handleResponse(response)
    } catch (e: Exception) {
        ApiError.NetworkError(e.message ?: "Network error").left()
    }

    /**
     * Perform POST request with body and parse response.
     */
    private suspend inline fun <reified R, reified T> post(
        path: String,
        body: R
    ): Either<ApiError, T> = try {
        val response = httpClient.post(path) {
            contentType(ContentType.Application.Json)
            setBody(body)
        }
        handleResponse(response)
    } catch (e: Exception) {
        ApiError.NetworkError(e.message ?: "Network error").left()
    }

    /**
     * Perform POST request and return Unit on success.
     */
    private suspend inline fun <reified R> postUnit(
        path: String,
        body: R? = null
    ): Either<ApiError, Unit> = try {
        val response = if (body != null) {
            httpClient.post(path) {
                contentType(ContentType.Application.Json)
                setBody(body)
            }
        } else {
            httpClient.post(path)
        }
        if (response.status.isSuccess()) {
            Unit.right()
        } else {
            parseError(response).left()
        }
    } catch (e: Exception) {
        ApiError.NetworkError(e.message ?: "Network error").left()
    }

    /**
     * Handle HTTP response, parsing success or error.
     */
    private suspend inline fun <reified T> handleResponse(
        response: HttpResponse
    ): Either<ApiError, T> =
        if (response.status.isSuccess()) {
            response.body<T>().right()
        } else {
            parseError(response).left()
        }

    /**
     * Parse HTTP error response into typed ApiError.
     */
    private suspend fun parseError(response: HttpResponse): ApiError = try {
        val error = response.body<ErrorResponse>()
        when (response.status) {
            HttpStatusCode.Unauthorized -> ApiError.Unauthorized(error.message)
            HttpStatusCode.Forbidden -> ApiError.Forbidden(error.message)
            HttpStatusCode.NotFound -> ApiError.NotFound(error.message)
            HttpStatusCode.Conflict -> ApiError.Conflict(error.message)
            HttpStatusCode.BadRequest -> ApiError.ValidationError(error.message, error.details)
            else -> ApiError.ServerError(response.status.value, error.message)
        }
    } catch (e: Exception) {
        ApiError.ServerError(response.status.value, response.status.description)
    }
}
