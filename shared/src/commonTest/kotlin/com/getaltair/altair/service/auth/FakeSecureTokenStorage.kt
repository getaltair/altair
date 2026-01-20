package com.getaltair.altair.service.auth

import arrow.core.Either
import arrow.core.right
import com.getaltair.altair.domain.types.Ulid

/**
 * In-memory implementation of SecureTokenStorage for testing.
 */
class FakeSecureTokenStorage : SecureTokenStorage {
    private var accessToken: String? = null
    private var refreshToken: String? = null
    private var tokenExpiration: Long? = null
    private var userId: Ulid? = null

    override suspend fun saveAccessToken(token: String): Either<TokenStorageError, Unit> {
        accessToken = token
        return Unit.right()
    }

    override suspend fun getAccessToken(): String? = accessToken

    override suspend fun saveRefreshToken(token: String): Either<TokenStorageError, Unit> {
        refreshToken = token
        return Unit.right()
    }

    override suspend fun getRefreshToken(): String? = refreshToken

    override suspend fun saveTokenExpiration(expiresAtMillis: Long): Either<TokenStorageError, Unit> {
        tokenExpiration = expiresAtMillis
        return Unit.right()
    }

    override suspend fun getTokenExpiration(): Long? = tokenExpiration

    override suspend fun saveUserId(userId: Ulid): Either<TokenStorageError, Unit> {
        this.userId = userId
        return Unit.right()
    }

    override suspend fun getUserId(): Ulid? = userId

    override suspend fun clear(): Either<TokenStorageError, Unit> {
        accessToken = null
        refreshToken = null
        tokenExpiration = null
        userId = null
        return Unit.right()
    }

    override suspend fun hasStoredCredentials(): Boolean = refreshToken != null
}
