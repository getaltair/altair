package com.getaltair.altair.di

import io.kotest.assertions.throwables.shouldNotThrow
import io.kotest.core.spec.style.FunSpec
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.booleans.shouldBeTrue
import org.koin.core.annotation.KoinExperimentalAPI
import org.koin.core.context.stopKoin
import org.koin.test.verify.verify

/**
 * Validates Koin dependency injection configuration.
 * Ensures all definitions are correctly declared and resolvable.
 */
@OptIn(KoinExperimentalAPI::class)
class KoinCheckTest :
    FunSpec({
        afterEach {
            // Clean up Koin after each test to ensure isolation
            stopKoin()
        }

        test("checkKoinModules verifies all dependencies resolve") {
            // Verifies all definitions in appModule are correctly declared
            // and can be resolved without missing dependencies
            shouldNotThrow<Exception> {
                appModule.verify()
            }
        }

        test("initKoin returns true on first initialization") {
            val result = initKoin()
            result.shouldBeTrue()
        }

        test("initKoin returns false when already initialized") {
            // First initialization
            val first = initKoin()
            first.shouldBeTrue()

            // Second initialization should be idempotent
            val second = initKoin()
            second.shouldBeFalse()
        }

        test("initKoin applies custom configuration") {
            var configApplied = false

            initKoin {
                configApplied = true
            }

            configApplied.shouldBeTrue()
        }

        test("initKoin can be restarted after stopKoin") {
            // Initialize
            val first = initKoin()
            first.shouldBeTrue()

            // Stop
            stopKoin()

            // Re-initialize
            val second = initKoin()
            second.shouldBeTrue()
        }
    })
