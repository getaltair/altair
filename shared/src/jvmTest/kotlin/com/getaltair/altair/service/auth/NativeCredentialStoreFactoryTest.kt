package com.getaltair.altair.service.auth

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/**
 * Tests for NativeCredentialStoreFactory.
 *
 * Tests platform detection and provider creation logic.
 */
class NativeCredentialStoreFactoryTest {
    @Test
    fun `detectOS returns correct OS for current platform`() {
        val os = NativeCredentialStoreFactory.detectOS()

        // Should return something other than UNKNOWN on supported platforms
        val osName = System.getProperty("os.name", "").lowercase()
        when {
            osName.contains("mac") || osName.contains("darwin") ->
                assertEquals(NativeCredentialStoreFactory.OperatingSystem.MACOS, os)
            osName.contains("win") ->
                assertEquals(NativeCredentialStoreFactory.OperatingSystem.WINDOWS, os)
            osName.contains("linux") || osName.contains("nix") || osName.contains("nux") ->
                assertEquals(NativeCredentialStoreFactory.OperatingSystem.LINUX, os)
        }
    }

    @Test
    fun `createForOS returns correct provider type for current OS`() {
        // Only test the current platform to avoid JNA library loading errors
        val currentOS = NativeCredentialStoreFactory.detectOS()
        if (currentOS == NativeCredentialStoreFactory.OperatingSystem.UNKNOWN) {
            return // Skip on unknown platforms
        }

        val provider = NativeCredentialStoreFactory.createForOS(currentOS)
        assertNotNull(provider)

        when (currentOS) {
            NativeCredentialStoreFactory.OperatingSystem.MACOS ->
                assertEquals("macOS Keychain", provider.name)
            NativeCredentialStoreFactory.OperatingSystem.WINDOWS ->
                assertEquals("Windows Credential Manager", provider.name)
            NativeCredentialStoreFactory.OperatingSystem.LINUX ->
                assertEquals("Linux Secret Service (secret-tool)", provider.name)
            NativeCredentialStoreFactory.OperatingSystem.UNKNOWN ->
                { /* Skip */ }
        }
    }

    @Test
    fun `createForOS returns null for unknown OS`() {
        val provider =
            NativeCredentialStoreFactory.createForOS(
                NativeCredentialStoreFactory.OperatingSystem.UNKNOWN,
            )
        assertEquals(null, provider)
    }

    @Test
    fun `create returns provider or null based on availability`() {
        val result = NativeCredentialStoreFactory.create()

        // The result depends on whether native libraries are available
        // We just verify it doesn't throw
        if (result != null) {
            assertTrue(result.isAvailable())
        }
    }
}
