package com.getaltair.altair.di

import org.koin.core.context.stopKoin
import org.koin.dsl.koinApplication
import org.koin.test.KoinTest
import org.koin.test.check.checkModules
import kotlin.test.AfterTest
import kotlin.test.Test

/**
 * Verifies Koin module configuration is valid.
 * Catches DI wiring issues at test time rather than runtime.
 */
class KoinModuleTest : KoinTest {

    @AfterTest
    fun tearDown() {
        stopKoin()
    }

    @Test
    fun verifyKoinConfiguration() {
        // Koin 4.x pattern: use koinApplication block with checkModules inside
        koinApplication {
            modules(allModules)
            checkModules()
        }
    }
}
