package com.getaltair.altair.domain.repository

interface AuthRepository {
    suspend fun login(
        email: String,
        password: String,
    )

    suspend fun register(
        email: String,
        password: String,
        displayName: String,
    )

    suspend fun logout()
}
