package com.getaltair.altair.service.auth

import org.junit.Assume.assumeTrue
import kotlin.test.AfterTest
import kotlin.test.BeforeTest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import kotlin.test.assertTrue

/**
 * Integration tests for native credential stores.
 *
 * These tests run only on their respective platforms and require the native
 * credential store services to be available:
 * - macOS: Keychain Services
 * - Windows: Credential Manager
 * - Linux: Secret Service daemon (gnome-keyring, kwallet, etc.)
 *
 * Tests are skipped if the native store is unavailable.
 */
@Suppress("TooManyFunctions") // Integration test with multiple scenarios
class NativeCredentialStoreIntegrationTest {
    private var provider: CredentialStoreProvider? = null

    @BeforeTest
    fun setup() {
        provider = NativeCredentialStoreFactory.create()
    }

    @AfterTest
    fun cleanup() {
        // Clean up any test credentials
        provider?.delete(TEST_KEY)
    }

    @Test
    fun `native store is available on current platform`() {
        assumeTrue("Native credential store not available", provider != null)
        assertTrue(provider!!.isAvailable())
    }

    @Test
    fun `store and retrieve credential`() {
        assumeTrue("Native credential store not available", provider != null)
        val p = provider!!

        val stored = p.store(TEST_KEY, TEST_VALUE)
        assertTrue(stored, "Failed to store credential")

        val retrieved = p.retrieve(TEST_KEY)
        assertEquals(TEST_VALUE, retrieved)
    }

    @Test
    fun `retrieve returns null for non-existent key`() {
        assumeTrue("Native credential store not available", provider != null)
        val p = provider!!

        val retrieved = p.retrieve("non-existent-key-${System.nanoTime()}")
        assertNull(retrieved)
    }

    @Test
    fun `delete removes credential`() {
        assumeTrue("Native credential store not available", provider != null)
        val p = provider!!

        p.store(TEST_KEY, TEST_VALUE)
        val deleted = p.delete(TEST_KEY)
        assertTrue(deleted, "Failed to delete credential")

        val retrieved = p.retrieve(TEST_KEY)
        assertNull(retrieved)
    }

    @Test
    fun `delete returns true for non-existent key`() {
        assumeTrue("Native credential store not available", provider != null)
        val p = provider!!

        val deleted = p.delete("non-existent-key-${System.nanoTime()}")
        assertTrue(deleted)
    }

    @Test
    fun `overwriting credential updates value`() {
        assumeTrue("Native credential store not available", provider != null)
        val p = provider!!

        p.store(TEST_KEY, "original-value")
        p.store(TEST_KEY, "updated-value")

        val retrieved = p.retrieve(TEST_KEY)
        assertEquals("updated-value", retrieved)
    }

    @Test
    fun `special characters in value are preserved`() {
        assumeTrue("Native credential store not available", provider != null)
        val p = provider!!

        val specialValue = "value-with-special-chars!@#\$%^&*()_+-=[]{}|;':\",./<>?"
        p.store(TEST_KEY, specialValue)

        val retrieved = p.retrieve(TEST_KEY)
        assertEquals(specialValue, retrieved)
    }

    @Test
    fun `unicode in value is preserved`() {
        assumeTrue("Native credential store not available", provider != null)
        val p = provider!!

        val unicodeValue = "日本語-émoji-🎉-مرحبا-Привет"
        p.store(TEST_KEY, unicodeValue)

        val retrieved = p.retrieve(TEST_KEY)
        assertEquals(unicodeValue, retrieved)
    }

    @Test
    fun `long value is stored correctly`() {
        assumeTrue("Native credential store not available", provider != null)
        val p = provider!!

        // JWTs can be quite long, test with a realistic size
        val longValue = "a".repeat(2048)
        p.store(TEST_KEY, longValue)

        val retrieved = p.retrieve(TEST_KEY)
        assertEquals(longValue, retrieved)
    }

    @Test
    fun `provider reports correct name for platform`() {
        assumeTrue("Native credential store not available", provider != null)
        val p = provider!!

        val osName = System.getProperty("os.name", "").lowercase()
        when {
            osName.contains("mac") || osName.contains("darwin") ->
                assertEquals("macOS Keychain", p.name)
            osName.contains("win") ->
                assertEquals("Windows Credential Manager", p.name)
            osName.contains("linux") || osName.contains("nix") || osName.contains("nux") ->
                assertEquals("Linux Secret Service (secret-tool)", p.name)
        }
    }

    companion object {
        private const val TEST_KEY = "altair-test-credential-${Long.MAX_VALUE}"
        private const val TEST_VALUE = "test-secret-value-12345"
    }
}
