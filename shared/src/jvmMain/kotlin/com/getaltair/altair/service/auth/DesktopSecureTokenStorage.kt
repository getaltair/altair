package com.getaltair.altair.service.auth

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.File
import java.security.SecureRandom
import java.util.Base64
import java.util.prefs.Preferences
import javax.crypto.Cipher
import javax.crypto.SecretKey
import javax.crypto.SecretKeyFactory
import javax.crypto.spec.GCMParameterSpec
import javax.crypto.spec.PBEKeySpec
import javax.crypto.spec.SecretKeySpec

/**
 * Desktop implementation of SecureTokenStorage using Java Preferences
 * with AES-GCM encryption.
 *
 * This implementation:
 * - Uses AES-256-GCM for encryption
 * - Derives encryption key from a password using PBKDF2
 * - Stores encrypted data in Java Preferences (user-specific)
 * - Generates a unique salt per installation
 *
 * Note: For maximum security in production, consider using the system's
 * native credential store (e.g., Windows Credential Manager, macOS Keychain,
 * or Linux Secret Service via libsecret).
 */
@Suppress("TooManyFunctions") // Implements SecureTokenStorage interface with encryption helpers
class DesktopSecureTokenStorage(
    private val appName: String = "Altair",
) : SecureTokenStorage {
    private val preferences = Preferences.userNodeForPackage(DesktopSecureTokenStorage::class.java)
    private val secureRandom = SecureRandom()

    // Lazy initialization of encryption key
    private val encryptionKey: SecretKey by lazy {
        deriveKey()
    }

    override suspend fun saveAccessToken(token: String) =
        withContext(Dispatchers.IO) {
            saveEncrypted(KEY_ACCESS_TOKEN, token)
        }

    override suspend fun getAccessToken(): String? =
        withContext(Dispatchers.IO) {
            getDecrypted(KEY_ACCESS_TOKEN)
        }

    override suspend fun saveRefreshToken(token: String) =
        withContext(Dispatchers.IO) {
            saveEncrypted(KEY_REFRESH_TOKEN, token)
        }

    override suspend fun getRefreshToken(): String? =
        withContext(Dispatchers.IO) {
            getDecrypted(KEY_REFRESH_TOKEN)
        }

    override suspend fun saveTokenExpiration(expiresAtMillis: Long) =
        withContext(Dispatchers.IO) {
            saveEncrypted(KEY_TOKEN_EXPIRATION, expiresAtMillis.toString())
        }

    override suspend fun getTokenExpiration(): Long? =
        withContext(Dispatchers.IO) {
            getDecrypted(KEY_TOKEN_EXPIRATION)?.toLongOrNull()
        }

    override suspend fun saveUserId(userId: String) =
        withContext(Dispatchers.IO) {
            saveEncrypted(KEY_USER_ID, userId)
        }

    override suspend fun getUserId(): String? =
        withContext(Dispatchers.IO) {
            getDecrypted(KEY_USER_ID)
        }

    override suspend fun clear() =
        withContext(Dispatchers.IO) {
            preferences.remove(KEY_ACCESS_TOKEN)
            preferences.remove(KEY_REFRESH_TOKEN)
            preferences.remove(KEY_TOKEN_EXPIRATION)
            preferences.remove(KEY_USER_ID)
            preferences.flush()
        }

    override suspend fun hasStoredCredentials(): Boolean =
        withContext(Dispatchers.IO) {
            getDecrypted(KEY_REFRESH_TOKEN) != null
        }

    @Suppress("TooGenericExceptionCaught") // Crypto operations can throw various exceptions
    private fun saveEncrypted(
        key: String,
        value: String,
    ) {
        try {
            val encrypted = encrypt(value)
            preferences.put(key, encrypted)
            preferences.flush()
        } catch (e: Exception) {
            // Log error but don't crash
            System.err.println("Failed to save encrypted value for $key: ${e.message}")
        }
    }

    @Suppress("TooGenericExceptionCaught") // Crypto operations can throw various exceptions
    private fun getDecrypted(key: String): String? =
        try {
            val encrypted = preferences.get(key, null) ?: return null
            decrypt(encrypted)
        } catch (e: Exception) {
            // If decryption fails, the data may be corrupted
            System.err.println("Failed to decrypt value for $key: ${e.message}")
            null
        }

    private fun encrypt(plaintext: String): String {
        val cipher = Cipher.getInstance(CIPHER_ALGORITHM)
        val iv = ByteArray(GCM_IV_LENGTH)
        secureRandom.nextBytes(iv)

        val spec = GCMParameterSpec(GCM_TAG_LENGTH * BITS_PER_BYTE, iv)
        cipher.init(Cipher.ENCRYPT_MODE, encryptionKey, spec)

        val ciphertext = cipher.doFinal(plaintext.toByteArray(Charsets.UTF_8))

        // Combine IV + ciphertext
        val combined = ByteArray(iv.size + ciphertext.size)
        System.arraycopy(iv, 0, combined, 0, iv.size)
        System.arraycopy(ciphertext, 0, combined, iv.size, ciphertext.size)

        return Base64.getEncoder().encodeToString(combined)
    }

    private fun decrypt(encrypted: String): String {
        val combined = Base64.getDecoder().decode(encrypted)

        val iv = combined.copyOfRange(0, GCM_IV_LENGTH)
        val ciphertext = combined.copyOfRange(GCM_IV_LENGTH, combined.size)

        val cipher = Cipher.getInstance(CIPHER_ALGORITHM)
        val spec = GCMParameterSpec(GCM_TAG_LENGTH * BITS_PER_BYTE, iv)
        cipher.init(Cipher.DECRYPT_MODE, encryptionKey, spec)

        val plaintext = cipher.doFinal(ciphertext)
        return String(plaintext, Charsets.UTF_8)
    }

    private fun deriveKey(): SecretKey {
        // Get or create installation-specific salt
        val salt = getOrCreateSalt()

        // Use a deterministic password based on app name and machine-specific data
        val password = "$appName-${System.getProperty("user.name")}-${getMachineId()}"

        val factory = SecretKeyFactory.getInstance(KEY_DERIVATION_ALGORITHM)
        val spec = PBEKeySpec(password.toCharArray(), salt, PBKDF2_ITERATIONS, KEY_LENGTH)
        val tmp = factory.generateSecret(spec)

        return SecretKeySpec(tmp.encoded, "AES")
    }

    private fun getOrCreateSalt(): ByteArray {
        val storedSalt = preferences.get(KEY_SALT, null)
        if (storedSalt != null) {
            return Base64.getDecoder().decode(storedSalt)
        }

        // Generate new salt for this installation
        val salt = ByteArray(SALT_LENGTH)
        secureRandom.nextBytes(salt)
        preferences.put(KEY_SALT, Base64.getEncoder().encodeToString(salt))
        preferences.flush()
        return salt
    }

    @Suppress("TooGenericExceptionCaught", "SwallowedException") // File/env access can fail
    private fun getMachineId(): String {
        // Try to get a machine-specific identifier
        return try {
            // On Linux, use machine-id
            val machineIdFile = File("/etc/machine-id")
            if (machineIdFile.exists()) {
                return machineIdFile.readText().trim()
            }

            // On Windows, use computer name
            System.getenv("COMPUTERNAME")
                ?: System.getenv("HOSTNAME")
                ?: "unknown-machine"
        } catch (e: Exception) {
            "fallback-${System.getProperty("os.name")}"
        }
    }

    companion object {
        private const val KEY_ACCESS_TOKEN = "access_token"
        private const val KEY_REFRESH_TOKEN = "refresh_token"
        private const val KEY_TOKEN_EXPIRATION = "token_expiration"
        private const val KEY_USER_ID = "user_id"
        private const val KEY_SALT = "encryption_salt"

        private const val CIPHER_ALGORITHM = "AES/GCM/NoPadding"
        private const val KEY_DERIVATION_ALGORITHM = "PBKDF2WithHmacSHA256"
        private const val KEY_LENGTH = 256
        private const val GCM_IV_LENGTH = 12
        private const val GCM_TAG_LENGTH = 16
        private const val SALT_LENGTH = 16
        private const val PBKDF2_ITERATIONS = 100_000
        private const val BITS_PER_BYTE = 8
    }
}
