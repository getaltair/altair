package com.getaltair.altair.navigation

import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.Lifecycle
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume
import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import com.getaltair.altair.rpc.PublicAuthService
import com.getaltair.altair.service.auth.AuthManager
import com.getaltair.altair.service.auth.SecureTokenStorage
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertIs
import kotlin.test.assertNotNull

/**
 * Tests for RootComponent navigation behavior and ComponentContextFactory.
 */
class RootComponentTest {
    private fun createTestComponentContext(): DefaultComponentContext {
        val lifecycle = LifecycleRegistry()
        lifecycle.resume()
        return DefaultComponentContext(lifecycle = lifecycle)
    }

    private fun createTestAuthManager(): AuthManager =
        AuthManager(
            tokenStorage = FakeTokenStorage(),
            publicAuthService = FakePublicAuthService(),
            scope = CoroutineScope(Dispatchers.Default + SupervisorJob()),
        )

    // ============================================
    // RootComponent Tests
    // ============================================

    @Test
    fun `initial stack contains Login configuration`() {
        val component =
            RootComponent(
                componentContext = createTestComponentContext(),
                authManager = createTestAuthManager(),
            )

        val stack = component.stack.value
        assertEquals(1, stack.items.size, "Stack should have exactly one item")
        assertIs<Config.Login>(stack.active.configuration, "Active configuration should be Login")
    }

    @Test
    fun `initial child is Login`() {
        val component =
            RootComponent(
                componentContext = createTestComponentContext(),
                authManager = createTestAuthManager(),
            )

        val activeChild = component.stack.value.active.instance
        assertIs<RootComponent.Child.Login>(activeChild, "Active child should be Login")
    }

    // ============================================
    // ComponentContextFactory Tests
    // ============================================

    @Test
    fun `createRootComponentContext returns valid context`() {
        val context = createRootComponentContext()
        assertNotNull(context, "Context should not be null")
    }

    @Test
    fun `createRootComponentContext returns context with resumed lifecycle`() {
        val context = createRootComponentContext()
        val lifecycle = context.getLifecycle()
        assertEquals(
            Lifecycle.State.RESUMED,
            lifecycle.state,
            "Lifecycle should be in RESUMED state",
        )
    }

    @Test
    fun `getLifecycle returns LifecycleRegistry from createRootComponentContext`() {
        val context = createRootComponentContext()
        val lifecycle = context.getLifecycle()
        assertNotNull(lifecycle, "getLifecycle should return non-null LifecycleRegistry")
        assertIs<LifecycleRegistry>(lifecycle, "Should return LifecycleRegistry type")
    }

    @Test
    fun `getLifecycleState returns RESUMED for new context`() {
        val context = createRootComponentContext()
        val state = context.getLifecycleState()
        assertEquals(Lifecycle.State.RESUMED, state)
    }
}

// ============================================
// Test Fakes
// ============================================

private class FakeTokenStorage : SecureTokenStorage {
    override suspend fun saveAccessToken(token: String) = Unit

    override suspend fun getAccessToken(): String? = null

    override suspend fun saveRefreshToken(token: String) = Unit

    override suspend fun getRefreshToken(): String? = null

    override suspend fun saveTokenExpiration(expiresAtMillis: Long) = Unit

    override suspend fun getTokenExpiration(): Long? = null

    override suspend fun saveUserId(userId: String) = Unit

    override suspend fun getUserId(): String? = null

    override suspend fun clear() = Unit

    override suspend fun hasStoredCredentials(): Boolean = false
}

private class FakePublicAuthService : PublicAuthService {
    override suspend fun login(request: AuthRequest): AuthResponse = throw NotImplementedError("Not used in navigation tests")

    override suspend fun refresh(refreshToken: String): TokenRefreshResponse = throw NotImplementedError("Not used in navigation tests")

    override suspend fun register(request: RegisterRequest): AuthResponse = throw NotImplementedError("Not used in navigation tests")
}
