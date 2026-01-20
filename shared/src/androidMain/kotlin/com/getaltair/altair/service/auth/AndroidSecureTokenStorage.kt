package com.getaltair.altair.service.auth

import android.content.Context
import android.content.SharedPreferences
import androidx.security.crypto.EncryptedSharedPreferences
import androidx.security.crypto.MasterKey
import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.domain.types.Ulid
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

/**
 * Android implementation of SecureTokenStorage using EncryptedSharedPreferences.
 *
 * This implementation:
 * - Uses AES256-GCM for encryption (via AndroidX Security)
 * - Stores encryption keys in Android Keystore (hardware-backed when available)
 * - Data is encrypted at rest and protected by device lock
 * - Automatically cleared on app uninstall
 *
 * Requirements:
 * - API level 23+ (Android 6.0+)
 * - Device must have a secure lock screen for full protection
 */
@Suppress("TooManyFunctions") // Implements SecureTokenStorage interface
class AndroidSecureTokenStorage(
    context: Context,
) : SecureTokenStorage {
    private val sharedPreferences: SharedPreferences by lazy {
        createEncryptedPreferences(context)
    }

    private fun createEncryptedPreferences(context: Context): SharedPreferences {
        val masterKey =
            MasterKey
                .Builder(context)
                .setKeyScheme(MasterKey.KeyScheme.AES256_GCM)
                .build()

        return EncryptedSharedPreferences.create(
            context,
            PREFERENCES_FILE_NAME,
            masterKey,
            EncryptedSharedPreferences.PrefKeyEncryptionScheme.AES256_SIV,
            EncryptedSharedPreferences.PrefValueEncryptionScheme.AES256_GCM,
        )
    }

    override suspend fun saveAccessToken(token: String): Either<TokenStorageError, Unit> =
        withContext(Dispatchers.IO) {
            saveString(KEY_ACCESS_TOKEN, token)
        }

    override suspend fun getAccessToken(): String? =
        withContext(Dispatchers.IO) {
            sharedPreferences.getString(KEY_ACCESS_TOKEN, null)
        }

    override suspend fun saveRefreshToken(token: String): Either<TokenStorageError, Unit> =
        withContext(Dispatchers.IO) {
            saveString(KEY_REFRESH_TOKEN, token)
        }

    override suspend fun getRefreshToken(): String? =
        withContext(Dispatchers.IO) {
            sharedPreferences.getString(KEY_REFRESH_TOKEN, null)
        }

    override suspend fun saveTokenExpiration(expiresAtMillis: Long): Either<TokenStorageError, Unit> =
        withContext(Dispatchers.IO) {
            saveLong(KEY_TOKEN_EXPIRATION, expiresAtMillis)
        }

    override suspend fun getTokenExpiration(): Long? =
        withContext(Dispatchers.IO) {
            if (sharedPreferences.contains(KEY_TOKEN_EXPIRATION)) {
                sharedPreferences.getLong(KEY_TOKEN_EXPIRATION, 0L)
            } else {
                null
            }
        }

    override suspend fun saveUserId(userId: Ulid): Either<TokenStorageError, Unit> =
        withContext(Dispatchers.IO) {
            saveString(KEY_USER_ID, userId.value)
        }

    override suspend fun getUserId(): Ulid? =
        withContext(Dispatchers.IO) {
            sharedPreferences.getString(KEY_USER_ID, null)?.let { Ulid(it) }
        }

    @Suppress("TooGenericExceptionCaught") // EncryptedSharedPreferences can throw various exceptions
    override suspend fun clear(): Either<TokenStorageError, Unit> =
        withContext(Dispatchers.IO) {
            try {
                sharedPreferences.edit().clear().apply()
                Unit.right()
            } catch (e: Exception) {
                TokenStorageError.PersistenceFailed("Failed to clear credentials: ${e.message}").left()
            }
        }

    @Suppress("TooGenericExceptionCaught") // EncryptedSharedPreferences can throw various exceptions
    private fun saveString(
        key: String,
        value: String,
    ): Either<TokenStorageError, Unit> =
        try {
            sharedPreferences.edit().putString(key, value).apply()
            Unit.right()
        } catch (e: Exception) {
            TokenStorageError.PersistenceFailed("Failed to save $key: ${e.message}").left()
        }

    @Suppress("TooGenericExceptionCaught") // EncryptedSharedPreferences can throw various exceptions
    private fun saveLong(
        key: String,
        value: Long,
    ): Either<TokenStorageError, Unit> =
        try {
            sharedPreferences.edit().putLong(key, value).apply()
            Unit.right()
        } catch (e: Exception) {
            TokenStorageError.PersistenceFailed("Failed to save $key: ${e.message}").left()
        }

    override suspend fun hasStoredCredentials(): Boolean =
        withContext(Dispatchers.IO) {
            sharedPreferences.contains(KEY_REFRESH_TOKEN)
        }

    companion object {
        private const val PREFERENCES_FILE_NAME = "altair_secure_auth"
        private const val KEY_ACCESS_TOKEN = "access_token"
        private const val KEY_REFRESH_TOKEN = "refresh_token"
        private const val KEY_TOKEN_EXPIRATION = "token_expiration"
        private const val KEY_USER_ID = "user_id"
    }
}
