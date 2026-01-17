package com.getaltair.altair.di

import org.koin.core.annotation.KoinExperimentalAPI
import org.koin.test.verify.verify
import kotlin.test.Test

/**
 * Validates Koin dependency injection configuration.
 * Ensures all definitions are correctly declared and resolvable.
 */
@OptIn(KoinExperimentalAPI::class)
class KoinCheckTest {
    @Test
    fun checkKoinModules() {
        // Verifies all definitions in appModule are correctly declared
        // and can be resolved without missing dependencies
        appModule.verify()
    }
}
