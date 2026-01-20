package com.getaltair.altair.service.auth

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.domain.types.Ulid
import kotlinx.cinterop.ExperimentalForeignApi
import kotlinx.cinterop.alloc
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.ptr
import kotlinx.cinterop.value
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.IO
import kotlinx.coroutines.withContext
import platform.CoreFoundation.CFDictionaryRef
import platform.Foundation.CFBridgingRelease
import platform.Foundation.CFBridgingRetain
import platform.Foundation.NSData
import platform.Foundation.NSString
import platform.Foundation.NSUTF8StringEncoding
import platform.Foundation.create
import platform.Foundation.dataUsingEncoding
import platform.Security.SecItemAdd
import platform.Security.SecItemCopyMatching
import platform.Security.SecItemDelete
import platform.Security.SecItemUpdate
import platform.Security.errSecItemNotFound
import platform.Security.errSecSuccess
import platform.Security.kSecAttrAccessible
import platform.Security.kSecAttrAccessibleWhenUnlockedThisDeviceOnly
import platform.Security.kSecAttrAccount
import platform.Security.kSecAttrService
import platform.Security.kSecClass
import platform.Security.kSecClassGenericPassword
import platform.Security.kSecMatchLimit
import platform.Security.kSecMatchLimitOne
import platform.Security.kSecReturnData
import platform.Security.kSecValueData

/**
 * iOS implementation of SecureTokenStorage using Keychain Services.
 *
 * This implementation:
 * - Uses iOS Keychain for secure storage
 * - Data protected with kSecAttrAccessibleWhenUnlockedThisDeviceOnly
 * - Not backed up to iCloud (device-only)
 * - Protected by device passcode/biometrics
 * - Survives app reinstall but cleared on device wipe
 */
@OptIn(ExperimentalForeignApi::class)
@Suppress("TooManyFunctions") // Implements SecureTokenStorage interface with keychain helpers
class IosSecureTokenStorage(
    private val serviceName: String = "com.getaltair.altair",
) : SecureTokenStorage {
    override suspend fun saveAccessToken(token: String): Either<TokenStorageError, Unit> =
        withContext(Dispatchers.IO) {
            saveToKeychain(KEY_ACCESS_TOKEN, token)
        }

    override suspend fun getAccessToken(): String? =
        withContext(Dispatchers.IO) {
            getFromKeychain(KEY_ACCESS_TOKEN)
        }

    override suspend fun saveRefreshToken(token: String): Either<TokenStorageError, Unit> =
        withContext(Dispatchers.IO) {
            saveToKeychain(KEY_REFRESH_TOKEN, token)
        }

    override suspend fun getRefreshToken(): String? =
        withContext(Dispatchers.IO) {
            getFromKeychain(KEY_REFRESH_TOKEN)
        }

    override suspend fun saveTokenExpiration(expiresAtMillis: Long): Either<TokenStorageError, Unit> =
        withContext(Dispatchers.IO) {
            saveToKeychain(KEY_TOKEN_EXPIRATION, expiresAtMillis.toString())
        }

    override suspend fun getTokenExpiration(): Long? =
        withContext(Dispatchers.IO) {
            getFromKeychain(KEY_TOKEN_EXPIRATION)?.toLongOrNull()
        }

    override suspend fun saveUserId(userId: Ulid): Either<TokenStorageError, Unit> =
        withContext(Dispatchers.IO) {
            saveToKeychain(KEY_USER_ID, userId.value)
        }

    override suspend fun getUserId(): Ulid? =
        withContext(Dispatchers.IO) {
            getFromKeychain(KEY_USER_ID)?.let { Ulid(it) }
        }

    override suspend fun clear(): Either<TokenStorageError, Unit> =
        withContext(Dispatchers.IO) {
            var hasError = false
            deleteFromKeychain(KEY_ACCESS_TOKEN).onLeft { hasError = true }
            deleteFromKeychain(KEY_REFRESH_TOKEN).onLeft { hasError = true }
            deleteFromKeychain(KEY_TOKEN_EXPIRATION).onLeft { hasError = true }
            deleteFromKeychain(KEY_USER_ID).onLeft { hasError = true }

            if (hasError) {
                TokenStorageError.PersistenceFailed("Failed to clear some keychain items").left()
            } else {
                Unit.right()
            }
        }

    override suspend fun hasStoredCredentials(): Boolean =
        withContext(Dispatchers.IO) {
            getFromKeychain(KEY_REFRESH_TOKEN) != null
        }

    @Suppress("ReturnCount") // Multiple error conditions require multiple returns
    private fun saveToKeychain(
        key: String,
        value: String,
    ): Either<TokenStorageError, Unit> {
        val data =
            (value as NSString).dataUsingEncoding(NSUTF8StringEncoding)
                ?: return TokenStorageError.ValidationFailed("Failed to encode value for $key").left()

        // First try to update existing item
        val updateQuery = createQuery(key)
        val updateAttributes =
            mapOf<Any?, Any?>(
                kSecValueData to data,
            )

        val updateStatus =
            SecItemUpdate(
                CFBridgingRetain(updateQuery) as CFDictionaryRef,
                CFBridgingRetain(updateAttributes) as CFDictionaryRef,
            )

        if (updateStatus == errSecItemNotFound) {
            // Item doesn't exist, add new
            val addQuery =
                createQuery(key).toMutableMap().apply {
                    put(kSecValueData, data)
                    put(kSecAttrAccessible, kSecAttrAccessibleWhenUnlockedThisDeviceOnly)
                }

            val addStatus = SecItemAdd(CFBridgingRetain(addQuery) as CFDictionaryRef, null)
            return if (addStatus == errSecSuccess) {
                Unit.right()
            } else {
                TokenStorageError
                    .PersistenceFailed("Failed to add $key to keychain (status: $addStatus)")
                    .left()
            }
        } else if (updateStatus == errSecSuccess) {
            return Unit.right()
        } else {
            return TokenStorageError
                .PersistenceFailed("Failed to update $key in keychain (status: $updateStatus)")
                .left()
        }
    }

    private fun getFromKeychain(key: String): String? {
        val query =
            createQuery(key).toMutableMap().apply {
                put(kSecReturnData, true)
                put(kSecMatchLimit, kSecMatchLimitOne)
            }

        return memScoped {
            val result = alloc<kotlinx.cinterop.COpaquePointerVar>()
            val status =
                SecItemCopyMatching(
                    CFBridgingRetain(query) as CFDictionaryRef,
                    result.ptr,
                )

            if (status != errSecSuccess) return@memScoped null

            val cfData = result.value ?: return@memScoped null
            val data = CFBridgingRelease(cfData) as? NSData ?: return@memScoped null
            NSString.create(data, NSUTF8StringEncoding) as? String
        }
    }

    private fun deleteFromKeychain(key: String): Either<TokenStorageError, Unit> {
        val query = createQuery(key)
        val status = SecItemDelete(CFBridgingRetain(query) as CFDictionaryRef)
        return if (status == errSecSuccess || status == errSecItemNotFound) {
            // Success or item already deleted
            Unit.right()
        } else {
            TokenStorageError.PersistenceFailed("Failed to delete $key from keychain (status: $status)").left()
        }
    }

    private fun createQuery(key: String): Map<Any?, Any?> =
        mapOf(
            kSecClass to kSecClassGenericPassword,
            kSecAttrService to serviceName,
            kSecAttrAccount to key,
        )

    companion object {
        private const val KEY_ACCESS_TOKEN = "access_token"
        private const val KEY_REFRESH_TOKEN = "refresh_token"
        private const val KEY_TOKEN_EXPIRATION = "token_expiration"
        private const val KEY_USER_ID = "user_id"
    }
}
