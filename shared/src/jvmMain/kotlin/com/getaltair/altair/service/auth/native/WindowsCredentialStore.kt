package com.getaltair.altair.service.auth.native

import com.getaltair.altair.service.auth.CredentialStoreProvider
import com.sun.jna.LastErrorException
import com.sun.jna.Memory
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.Structure
import com.sun.jna.platform.win32.WinDef.DWORD
import com.sun.jna.ptr.PointerByReference
import com.sun.jna.win32.StdCallLibrary
import com.sun.jna.win32.W32APIOptions
import java.nio.charset.StandardCharsets

/**
 * Windows Credential Manager implementation of CredentialStoreProvider.
 *
 * Uses advapi32.dll via JNA to access Windows Credential Manager.
 * Credentials are stored as generic credentials with:
 * - Target name: "com.getaltair.altair/{key}"
 */
class WindowsCredentialStore : CredentialStoreProvider {
    override val name: String = "Windows Credential Manager"

    @Suppress("TooGenericExceptionCaught", "SwallowedException") // Expected when not on Windows
    private val advapi32: Advapi32? =
        try {
            Native.load("advapi32", Advapi32::class.java, W32APIOptions.DEFAULT_OPTIONS)
        } catch (e: Exception) {
            null
        }

    override fun isAvailable(): Boolean = advapi32 != null

    override fun store(
        key: String,
        value: String,
    ): Boolean {
        val api = advapi32 ?: return false

        val targetName = buildTargetName(key)
        val credentialBlob = value.toByteArray(StandardCharsets.UTF_8)

        val credential = CREDENTIAL()
        credential.type = CRED_TYPE_GENERIC
        credential.targetName = targetName
        credential.credentialBlobSize = DWORD(credentialBlob.size.toLong())

        // Allocate native memory for the credential blob
        val blobMemory = Memory(credentialBlob.size.toLong())
        blobMemory.write(0, credentialBlob, 0, credentialBlob.size)
        credential.credentialBlob = blobMemory

        credential.persist = CRED_PERSIST_LOCAL_MACHINE
        credential.userName = CredentialStoreProvider.SERVICE_NAME

        @Suppress("SwallowedException") // Credential store failures are expected and handled
        return try {
            api.CredWriteW(credential, 0)
            true
        } catch (e: LastErrorException) {
            false
        }
    }

    @Suppress("ReturnCount") // Multiple early returns for readability
    override fun retrieve(key: String): String? {
        val api = advapi32 ?: return null

        val targetName = buildTargetName(key)
        val credentialRef = PointerByReference()

        @Suppress("SwallowedException") // Credential store failures are expected and handled
        return try {
            val success = api.CredReadW(targetName, CRED_TYPE_GENERIC, 0, credentialRef)
            if (!success) {
                return null
            }

            val credPointer = credentialRef.value ?: return null
            try {
                val credential = CREDENTIAL(credPointer)
                credential.read()

                if (credential.credentialBlobSize.toInt() > 0 && credential.credentialBlob != null) {
                    val bytes =
                        credential.credentialBlob!!.getByteArray(
                            0,
                            credential.credentialBlobSize.toInt(),
                        )
                    String(bytes, StandardCharsets.UTF_8)
                } else {
                    null
                }
            } finally {
                api.CredFree(credPointer)
            }
        } catch (e: LastErrorException) {
            null
        }
    }

    override fun delete(key: String): Boolean {
        val api = advapi32 ?: return false

        val targetName = buildTargetName(key)

        return try {
            api.CredDeleteW(targetName, CRED_TYPE_GENERIC, 0)
            true
        } catch (
            @Suppress("TooGenericExceptionCaught") e: LastErrorException,
        ) {
            // Error 1168 (ERROR_NOT_FOUND) means the credential doesn't exist
            e.errorCode == ERROR_NOT_FOUND
        }
    }

    private fun buildTargetName(key: String): String = "${CredentialStoreProvider.SERVICE_NAME}/$key"

    /**
     * JNA interface for Windows Advapi32.dll Credential Management APIs.
     */
    @Suppress("FunctionName", "FunctionParameterNaming")
    private interface Advapi32 : StdCallLibrary {
        /**
         * Write a credential to the credential store.
         */
        @Throws(LastErrorException::class)
        fun CredWriteW(
            credential: CREDENTIAL,
            flags: Int,
        ): Boolean

        /**
         * Read a credential from the credential store.
         */
        @Throws(LastErrorException::class)
        fun CredReadW(
            targetName: String,
            type: Int,
            flags: Int,
            credential: PointerByReference,
        ): Boolean

        /**
         * Delete a credential from the credential store.
         */
        @Throws(LastErrorException::class)
        fun CredDeleteW(
            targetName: String,
            type: Int,
            flags: Int,
        ): Boolean

        /**
         * Free a credential returned by CredRead.
         */
        fun CredFree(credential: Pointer)
    }

    /**
     * Windows CREDENTIAL structure.
     */
    @Structure.FieldOrder(
        "flags",
        "type",
        "targetName",
        "comment",
        "lastWritten",
        "credentialBlobSize",
        "credentialBlob",
        "persist",
        "attributeCount",
        "attributes",
        "targetAlias",
        "userName",
    )
    @Suppress("VariableNaming", "MagicNumber")
    class CREDENTIAL : Structure {
        @JvmField
        var flags: Int = 0

        @JvmField
        var type: Int = 0

        @JvmField
        var targetName: String? = null

        @JvmField
        var comment: String? = null

        @JvmField
        var lastWritten: FILETIME = FILETIME()

        @JvmField
        var credentialBlobSize: DWORD = DWORD(0)

        @JvmField
        var credentialBlob: Pointer? = null

        @JvmField
        var persist: Int = 0

        @JvmField
        var attributeCount: Int = 0

        @JvmField
        var attributes: Pointer? = null

        @JvmField
        var targetAlias: String? = null

        @JvmField
        var userName: String? = null

        constructor() : super()

        constructor(p: Pointer) : super(p)

        override fun getFieldOrder(): List<String> =
            listOf(
                "flags",
                "type",
                "targetName",
                "comment",
                "lastWritten",
                "credentialBlobSize",
                "credentialBlob",
                "persist",
                "attributeCount",
                "attributes",
                "targetAlias",
                "userName",
            )
    }

    /**
     * Windows FILETIME structure.
     */
    @Structure.FieldOrder("dwLowDateTime", "dwHighDateTime")
    @Suppress("VariableNaming")
    class FILETIME : Structure() {
        @JvmField
        var dwLowDateTime: Int = 0

        @JvmField
        var dwHighDateTime: Int = 0
    }

    companion object {
        private const val CRED_TYPE_GENERIC = 1
        private const val CRED_PERSIST_LOCAL_MACHINE = 2
        private const val ERROR_NOT_FOUND = 1168
    }
}
