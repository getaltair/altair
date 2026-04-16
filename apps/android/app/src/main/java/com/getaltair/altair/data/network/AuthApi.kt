package com.getaltair.altair.data.network

import kotlinx.serialization.Serializable
import retrofit2.http.Body
import retrofit2.http.GET
import retrofit2.http.POST

@Serializable
data class LoginRequest(
    val email: String,
    val password: String,
)

@Serializable
data class RegisterRequest(
    val email: String,
    val password: String,
    val displayName: String,
)

@Serializable
data class AuthResponse(
    val accessToken: String,
    val refreshToken: String,
)

@Serializable
data class RefreshRequest(
    val refreshToken: String,
)

@Serializable
data class PowerSyncTokenResponse(
    val token: String,
)

interface AuthApi {
    @POST("api/auth/login")
    suspend fun login(
        @Body request: LoginRequest,
    ): AuthResponse

    @POST("api/auth/register")
    suspend fun register(
        @Body request: RegisterRequest,
    ): AuthResponse

    @POST("api/auth/refresh")
    suspend fun refresh(
        @Body request: RefreshRequest,
    ): AuthResponse

    @GET("api/auth/powersync-token")
    suspend fun getPowerSyncToken(): PowerSyncTokenResponse
}
