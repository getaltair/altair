package com.getaltair.altair.di

import org.koin.core.annotation.KoinExperimentalAPI
import org.koin.core.context.stopKoin
import org.koin.test.verify.verify
import kotlin.test.AfterTest
import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

/**
 * Validates Koin dependency injection configuration.
 * Ensures all definitions are correctly declared and resolvable.
 */
@OptIn(KoinExperimentalAPI::class)
class KoinCheckTest {
    @AfterTest
    fun tearDown() {
        // Clean up Koin after each test to ensure isolation
        stopKoin()
    }

    @Test
    fun `checkKoinModules verifies all dependencies resolve`() {
        // Verifies all definitions in appModule are correctly declared
        // and can be resolved without missing dependencies
        appModule.verify()
    }

    @Test
    fun `initKoin returns true on first initialization`() {
        val result = initKoin()
        assertTrue(result, "initKoin should return true on first initialization")
    }

    @Test
    fun `initKoin returns false when already initialized`() {
        // First initialization
        val first = initKoin()
        assertTrue(first, "First initKoin should succeed")

        // Second initialization should be idempotent
        val second = initKoin()
        assertFalse(second, "Second initKoin should return false (already initialized)")
    }

    @Test
    fun `initKoin applies custom configuration`() {
        var configApplied = false

        initKoin {
            configApplied = true
        }

        assertTrue(configApplied, "Custom configuration should be applied")
    }

    @Test
    fun `initKoin can be restarted after stopKoin`() {
        // Initialize
        val first = initKoin()
        assertTrue(first, "First initKoin should succeed")

        // Stop
        stopKoin()

        // Re-initialize
        val second = initKoin()
        assertTrue(second, "initKoin should succeed after stopKoin")
    }
}
