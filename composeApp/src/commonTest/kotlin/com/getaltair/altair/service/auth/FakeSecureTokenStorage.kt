package com.getaltair.altair.service.auth

/**
 * In-memory implementation of SecureTokenStorage for testing.
 */
class FakeSecureTokenStorage : SecureTokenStorage {
    private var accessToken: String? = null
    private var refreshToken: String? = null
    private var tokenExpiration: Long? = null
    private var userId: String? = null

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

    override suspend fun saveUserId(userId: String) {
        this.userId = userId
    }

    override suspend fun getUserId(): String? = userId

    override suspend fun clear() {
        accessToken = null
        refreshToken = null
        tokenExpiration = null
        userId = null
    }

    override suspend fun hasStoredCredentials(): Boolean = refreshToken != null
}
