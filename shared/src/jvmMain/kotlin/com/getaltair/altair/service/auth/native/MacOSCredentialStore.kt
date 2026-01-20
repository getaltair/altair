package com.getaltair.altair.service.auth.native

import com.getaltair.altair.service.auth.CredentialStoreProvider
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.ptr.IntByReference
import com.sun.jna.ptr.PointerByReference
import java.nio.charset.StandardCharsets

/**
 * macOS Keychain Services implementation of CredentialStoreProvider.
 *
 * Uses Security.framework via JNA to access the macOS Keychain for storing credentials.
 * Credentials are stored as generic passwords with:
 * - Service name: "com.getaltair.altair"
 * - Account name: the credential key
 */
class MacOSCredentialStore : CredentialStoreProvider {
    override val name: String = "macOS Keychain"

    @Suppress("TooGenericExceptionCaught", "SwallowedException") // Expected when not on macOS
    private val security: SecurityFramework? =
        try {
            Native.load("Security", SecurityFramework::class.java)
        } catch (e: Exception) {
            null
        }

    override fun isAvailable(): Boolean = security != null

    override fun store(
        key: String,
        value: String,
    ): Boolean {
        val sec = security ?: return false

        // First try to delete any existing item
        delete(key)

        val serviceBytes = CredentialStoreProvider.SERVICE_NAME.toByteArray(StandardCharsets.UTF_8)
        val accountBytes = key.toByteArray(StandardCharsets.UTF_8)
        val passwordBytes = value.toByteArray(StandardCharsets.UTF_8)

        val status =
            sec.SecKeychainAddGenericPassword(
                null, // Use default keychain
                serviceBytes.size,
                serviceBytes,
                accountBytes.size,
                accountBytes,
                passwordBytes.size,
                passwordBytes,
                null, // Don't need the item reference
            )

        return status == ERR_SEC_SUCCESS
    }

    @Suppress("ReturnCount") // Multiple early returns for readability
    override fun retrieve(key: String): String? {
        val sec = security ?: return null

        val serviceBytes = CredentialStoreProvider.SERVICE_NAME.toByteArray(StandardCharsets.UTF_8)
        val accountBytes = key.toByteArray(StandardCharsets.UTF_8)
        val passwordLength = IntByReference()
        val passwordData = PointerByReference()

        val status =
            sec.SecKeychainFindGenericPassword(
                null, // Search default keychains
                serviceBytes.size,
                serviceBytes,
                accountBytes.size,
                accountBytes,
                passwordLength,
                passwordData,
                null, // Don't need the item reference
            )

        if (status != ERR_SEC_SUCCESS) {
            return null
        }

        val dataPointer = passwordData.value ?: return null
        return try {
            val bytes = dataPointer.getByteArray(0, passwordLength.value)
            String(bytes, StandardCharsets.UTF_8)
        } finally {
            // Free the password data
            sec.SecKeychainItemFreeContent(null, dataPointer)
        }
    }

    @Suppress("ReturnCount") // Multiple early returns for readability
    override fun delete(key: String): Boolean {
        val sec = security ?: return false

        val serviceBytes = CredentialStoreProvider.SERVICE_NAME.toByteArray(StandardCharsets.UTF_8)
        val accountBytes = key.toByteArray(StandardCharsets.UTF_8)
        val itemRef = PointerByReference()

        val status =
            sec.SecKeychainFindGenericPassword(
                null, // Search default keychains
                serviceBytes.size,
                serviceBytes,
                accountBytes.size,
                accountBytes,
                null, // Don't need password length
                null, // Don't need password data
                itemRef,
            )

        if (status == ERR_SEC_ITEM_NOT_FOUND) {
            return true // Item doesn't exist, consider this success
        }

        if (status != ERR_SEC_SUCCESS) {
            return false
        }

        val item = itemRef.value ?: return true
        val deleteStatus = sec.SecKeychainItemDelete(item)
        sec.CFRelease(item)

        return deleteStatus == ERR_SEC_SUCCESS
    }

    /**
     * JNA interface for macOS Security.framework.
     */
    @Suppress("FunctionName", "FunctionParameterNaming", "LongParameterList")
    private interface SecurityFramework : Library {
        /**
         * Add a generic password to the keychain.
         */
        fun SecKeychainAddGenericPassword(
            keychain: Pointer?,
            serviceNameLength: Int,
            serviceName: ByteArray,
            accountNameLength: Int,
            accountName: ByteArray,
            passwordLength: Int,
            passwordData: ByteArray,
            itemRef: PointerByReference?,
        ): Int

        /**
         * Find a generic password in the keychain.
         */
        fun SecKeychainFindGenericPassword(
            keychainOrArray: Pointer?,
            serviceNameLength: Int,
            serviceName: ByteArray,
            accountNameLength: Int,
            accountName: ByteArray,
            passwordLength: IntByReference?,
            passwordData: PointerByReference?,
            itemRef: PointerByReference?,
        ): Int

        /**
         * Delete a keychain item.
         */
        fun SecKeychainItemDelete(itemRef: Pointer): Int

        /**
         * Free keychain item content.
         */
        fun SecKeychainItemFreeContent(
            attrList: Pointer?,
            data: Pointer,
        ): Int

        /**
         * Release a Core Foundation object.
         */
        fun CFRelease(cf: Pointer)
    }

    companion object {
        private const val ERR_SEC_SUCCESS = 0
        private const val ERR_SEC_ITEM_NOT_FOUND = -25_300
    }
}
