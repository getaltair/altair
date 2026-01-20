package com.getaltair.altair.service.auth

import com.getaltair.altair.domain.types.Ulid

/**
 * In-memory implementation of SecureTokenStorage for testing.
 */
class FakeSecureTokenStorage : SecureTokenStorage {
    private var accessToken: String? = null
    private var refreshToken: String? = null
    private var tokenExpiration: Long? = null
    private var userId: Ulid? = null

    override suspend fun saveAccessToken(token: String) {
        accessToken = token
    }

    override suspend fun getAccessToken(): String? = accessToken

    override suspend fun saveRefreshToken(token: String) {
        refreshToken = token
    }

    override suspend fun getRefreshToken(): String? = refreshToken

    override suspend fun saveTokenExpiration(expiresAtMillis: Long) {
        tokenExpiration = expiresAtMillis
    }

    override suspend fun getTokenExpiration(): Long? = tokenExpiration

    override suspend fun saveUserId(userId: Ulid) {
        this.userId = userId
    }

    override suspend fun getUserId(): Ulid? = userId

    override suspend fun clear() {
        accessToken = null
        refreshToken = null
        tokenExpiration = null
        userId = null
    }

    override suspend fun hasStoredCredentials(): Boolean = refreshToken != null
}
