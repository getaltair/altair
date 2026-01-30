package com.getaltair.altair.di

import com.getaltair.altair.api.TokenPair
import com.getaltair.altair.api.TokenProvider
import org.koin.core.context.stopKoin
import org.koin.dsl.koinApplication
import org.koin.dsl.module
import org.koin.test.KoinTest
import org.koin.test.check.checkModules
import kotlin.test.AfterTest
import kotlin.test.Test

/**
 * Test implementation of TokenProvider for Koin verification.
 */
class TestTokenProvider : TokenProvider {
    override val serverUrl: String = "http://localhost:8080"

    override suspend fun getTokens(): TokenPair? = null
    override suspend fun refresh(): TokenPair? = null
    override suspend fun storeTokens(tokens: TokenPair) {}
    override suspend fun clearTokens() {}
}

/**
 * Test module providing TokenProvider for API module.
 * Replaces platform-specific TokenProvider implementations for testing.
 */
private val testModule = module {
    single<TokenProvider> { TestTokenProvider() }
}

/**
 * Modules to test - excludes platformModule which requires platform context.
 * The testModule provides TokenProvider instead.
 */
private val testableModules = listOf(testModule, appModule, apiModule)

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
        // Use testableModules (excludes platformModule which requires Android Context)
        // testModule provides TokenProvider needed by apiModule
        koinApplication {
            modules(testableModules)
            checkModules()
        }
    }
}
