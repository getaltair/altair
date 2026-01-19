package com.getaltair.altair.service.auth.native

import com.getaltair.altair.service.auth.CredentialStoreProvider

/**
 * Linux Secret Service implementation of CredentialStoreProvider.
 *
 * Uses the `secret-tool` CLI to access GNOME Keyring, KDE Wallet, or other
 * Secret Service implementations. This approach is more reliable than direct
 * JNA bindings as it handles the D-Bus communication and schema complexity.
 *
 * Requires:
 * - libsecret-tools package (provides secret-tool)
 * - A Secret Service daemon running (e.g., gnome-keyring-daemon)
 */
class LinuxCredentialStore : CredentialStoreProvider {
    override val name: String = "Linux Secret Service (secret-tool)"

    @Suppress("TooGenericExceptionCaught", "SwallowedException") // Expected when secret-tool not available
    private val secretToolAvailable: Boolean by lazy {
        try {
            val process =
                ProcessBuilder("which", "secret-tool")
                    .redirectErrorStream(true)
                    .start()
            process.waitFor() == 0
        } catch (e: Exception) {
            false
        }
    }

    override fun isAvailable(): Boolean = secretToolAvailable

    @Suppress("TooGenericExceptionCaught", "SwallowedException") // Process errors handled gracefully
    override fun store(
        key: String,
        value: String,
    ): Boolean {
        if (!secretToolAvailable) return false

        return try {
            val process =
                ProcessBuilder(
                    "secret-tool",
                    "store",
                    "--label",
                    buildLabel(key),
                    ATTR_SERVICE,
                    CredentialStoreProvider.SERVICE_NAME,
                    ATTR_KEY,
                    key,
                ).redirectErrorStream(true)
                    .start()

            // Write the password to stdin
            process.outputStream.use { output ->
                output.write(value.toByteArray(Charsets.UTF_8))
            }

            process.waitFor() == 0
        } catch (e: Exception) {
            false
        }
    }

    @Suppress("TooGenericExceptionCaught", "SwallowedException") // Process errors handled gracefully
    override fun retrieve(key: String): String? {
        if (!secretToolAvailable) return null

        return try {
            val process =
                ProcessBuilder(
                    "secret-tool",
                    "lookup",
                    ATTR_SERVICE,
                    CredentialStoreProvider.SERVICE_NAME,
                    ATTR_KEY,
                    key,
                ).redirectErrorStream(true)
                    .start()

            val result = process.inputStream.bufferedReader().readText()
            val exitCode = process.waitFor()

            if (exitCode == 0 && result.isNotEmpty()) {
                result.trimEnd('\n')
            } else {
                null
            }
        } catch (e: Exception) {
            null
        }
    }

    @Suppress("TooGenericExceptionCaught", "SwallowedException") // Process errors handled gracefully
    override fun delete(key: String): Boolean {
        if (!secretToolAvailable) return false

        return try {
            val process =
                ProcessBuilder(
                    "secret-tool",
                    "clear",
                    ATTR_SERVICE,
                    CredentialStoreProvider.SERVICE_NAME,
                    ATTR_KEY,
                    key,
                ).redirectErrorStream(true)
                    .start()

            val exitCode = process.waitFor()
            // secret-tool clear returns 0 on success, 1 if item doesn't exist
            // We consider both cases as success (delete is idempotent)
            exitCode == 0 || exitCode == 1
        } catch (e: Exception) {
            // If we can't delete, consider it a failure
            false
        }
    }

    private fun buildLabel(key: String): String = "Altair: $key"

    companion object {
        private const val ATTR_SERVICE = "service"
        private const val ATTR_KEY = "key"
    }
}
