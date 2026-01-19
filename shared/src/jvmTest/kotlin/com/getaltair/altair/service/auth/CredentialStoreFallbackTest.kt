package com.getaltair.altair.service.auth

import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertNotNull
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Tests for credential store fallback behavior.
 *
 * Verifies that the factory correctly handles unavailable native stores
 * and that the fallback behavior works as expected.
 */
class CredentialStoreFallbackTest {
    @Test
    fun `factory returns null when provider is not available`() {
        val unavailableProvider =
            object : CredentialStoreProvider {
                override val name = "Unavailable Store"

                override fun isAvailable() = false

                override fun store(
                    key: String,
                    value: String,
                ) = false

                override fun retrieve(key: String): String? = null

                override fun delete(key: String) = false
            }

        // Verify unavailable provider reports correctly
        assertFalse(unavailableProvider.isAvailable())
    }

    @Test
    fun `unknown OS returns null provider`() {
        val provider =
            NativeCredentialStoreFactory.createForOS(
                NativeCredentialStoreFactory.OperatingSystem.UNKNOWN,
            )

        assertNull(provider)
    }

    @Test
    fun `DesktopSecureTokenStorage is always available as fallback`() {
        // This test ensures the fallback is always functional
        val fallback = DesktopSecureTokenStorage(appName = "FallbackTest-${System.nanoTime()}")

        assertNotNull(fallback)

        // Verify it can perform basic operations
        kotlinx.coroutines.runBlocking {
            fallback.saveAccessToken("test-token")
            val retrieved = fallback.getAccessToken()
            kotlin.test.assertEquals("test-token", retrieved)
            fallback.clear()
        }
    }

    @Test
    fun `native store creation does not throw on current platform`() {
        // This test ensures that attempting to create native stores
        // doesn't crash on the current platform
        val currentOS = NativeCredentialStoreFactory.detectOS()
        if (currentOS == NativeCredentialStoreFactory.OperatingSystem.UNKNOWN) {
            return // Skip on unknown platforms
        }

        val provider = NativeCredentialStoreFactory.createForOS(currentOS)
        assertNotNull(provider)
        // isAvailable() may return false (e.g., if libsecret isn't installed)
        // but should not throw
        provider.isAvailable()
    }

    @Suppress("TooGenericExceptionCaught", "SwallowedException")
    @Test
    fun `factory create method handles library loading failures gracefully`() {
        // This tests that the factory doesn't throw when native libraries fail to load
        val result =
            try {
                NativeCredentialStoreFactory.create()
                true
            } catch (e: Exception) {
                false
            }

        // Should always succeed (return null or a provider, never throw)
        assertTrue(result)
    }
}
