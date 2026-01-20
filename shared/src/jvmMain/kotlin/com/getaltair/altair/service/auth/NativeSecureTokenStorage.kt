package com.getaltair.altair.service.auth

import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

/**
 * SecureTokenStorage implementation that delegates to a native credential store provider.
 *
 * This class wraps a [CredentialStoreProvider] and implements the [SecureTokenStorage]
 * interface, providing native OS credential storage for authentication tokens.
 *
 * @param provider The native credential store provider to use
 */
class NativeSecureTokenStorage(
    private val provider: CredentialStoreProvider,
) : SecureTokenStorage {
    override suspend fun saveAccessToken(token: String) =
        withContext(Dispatchers.IO) {
            provider.store(KEY_ACCESS_TOKEN, token)
            Unit
        }

    override suspend fun getAccessToken(): String? =
        withContext(Dispatchers.IO) {
            provider.retrieve(KEY_ACCESS_TOKEN)
        }

    override suspend fun saveRefreshToken(token: String) =
        withContext(Dispatchers.IO) {
            provider.store(KEY_REFRESH_TOKEN, token)
            Unit
        }

    override suspend fun getRefreshToken(): String? =
        withContext(Dispatchers.IO) {
            provider.retrieve(KEY_REFRESH_TOKEN)
        }

    override suspend fun saveTokenExpiration(expiresAtMillis: Long) =
        withContext(Dispatchers.IO) {
            provider.store(KEY_TOKEN_EXPIRATION, expiresAtMillis.toString())
            Unit
        }

    override suspend fun getTokenExpiration(): Long? =
        withContext(Dispatchers.IO) {
            provider.retrieve(KEY_TOKEN_EXPIRATION)?.toLongOrNull()
        }

    override suspend fun saveUserId(userId: Ulid) =
        withContext(Dispatchers.IO) {
            provider.store(KEY_USER_ID, userId.value)
            Unit
        }

    override suspend fun getUserId(): Ulid? =
        withContext(Dispatchers.IO) {
            provider.retrieve(KEY_USER_ID)?.let { Ulid(it) }
        }

    override suspend fun clear() {
        withContext(Dispatchers.IO) {
            provider.delete(KEY_ACCESS_TOKEN)
            provider.delete(KEY_REFRESH_TOKEN)
            provider.delete(KEY_TOKEN_EXPIRATION)
            provider.delete(KEY_USER_ID)
        }
    }

    override suspend fun hasStoredCredentials(): Boolean =
        withContext(Dispatchers.IO) {
            provider.retrieve(KEY_REFRESH_TOKEN) != null
        }

    companion object {
        private const val KEY_ACCESS_TOKEN = "access_token"
        private const val KEY_REFRESH_TOKEN = "refresh_token"
        private const val KEY_TOKEN_EXPIRATION = "token_expiration"
        private const val KEY_USER_ID = "user_id"
    }
}
