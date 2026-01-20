package com.getaltair.altair.service.auth

import com.getaltair.altair.service.auth.native.LinuxCredentialStore
import com.getaltair.altair.service.auth.native.MacOSCredentialStore
import com.getaltair.altair.service.auth.native.WindowsCredentialStore

/**
 * Factory for creating platform-appropriate credential store providers.
 *
 * Detects the current OS and attempts to create the appropriate native
 * credential store. If the native store is unavailable (library not found,
 * no daemon running, etc.), returns null to indicate fallback is needed.
 */
object NativeCredentialStoreFactory {
    /**
     * The detected operating system.
     */
    enum class OperatingSystem {
        MACOS,
        WINDOWS,
        LINUX,
        UNKNOWN,
    }

    /**
     * Detect the current operating system.
     */
    fun detectOS(): OperatingSystem {
        val osName = System.getProperty("os.name", "").lowercase()
        return when {
            osName.contains("mac") || osName.contains("darwin") -> OperatingSystem.MACOS
            osName.contains("win") -> OperatingSystem.WINDOWS
            osName.contains("linux") || osName.contains("nix") || osName.contains("nux") ->
                OperatingSystem.LINUX
            else -> OperatingSystem.UNKNOWN
        }
    }

    /**
     * Create the appropriate credential store provider for the current platform.
     *
     * @return A [CredentialStoreProvider] if native credentials are available, null otherwise.
     */
    fun create(): CredentialStoreProvider? {
        val provider = createForOS(detectOS())
        return if (provider?.isAvailable() == true) provider else null
    }

    /**
     * Create a credential store provider for a specific OS.
     *
     * Useful for testing or when you need to explicitly target a platform.
     *
     * @param os The target operating system
     * @return A [CredentialStoreProvider] instance (may not be available)
     */
    fun createForOS(os: OperatingSystem): CredentialStoreProvider? =
        when (os) {
            OperatingSystem.MACOS -> MacOSCredentialStore()
            OperatingSystem.WINDOWS -> WindowsCredentialStore()
            OperatingSystem.LINUX -> LinuxCredentialStore()
            OperatingSystem.UNKNOWN -> null
        }
}
