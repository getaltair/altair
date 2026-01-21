package com.getaltair.altair.navigation

import arrow.core.right
import com.arkivanov.decompose.DefaultComponentContext
import com.arkivanov.essenty.lifecycle.Lifecycle
import com.arkivanov.essenty.lifecycle.LifecycleRegistry
import com.arkivanov.essenty.lifecycle.resume
import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import com.getaltair.altair.rpc.PublicAuthService
import com.getaltair.altair.service.auth.AuthManager
import com.getaltair.altair.service.auth.SecureTokenStorage
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.nulls.shouldNotBeNull
import io.kotest.matchers.shouldBe
import io.kotest.matchers.types.shouldBeInstanceOf
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.SupervisorJob

/**
 * Tests for RootComponent navigation behavior and ComponentContextFactory.
 *
 * Verifies:
 * - RootComponent initial stack and child configuration
 * - ComponentContextFactory functions (createRootComponentContext, getLifecycle, etc.)
 */
class RootComponentTest :
    BehaviorSpec({
        fun createTestComponentContext(): DefaultComponentContext {
            val lifecycle = LifecycleRegistry()
            lifecycle.resume()
            return DefaultComponentContext(lifecycle = lifecycle)
        }

        fun createTestAuthManager(): AuthManager =
            AuthManager(
                tokenStorage = FakeTokenStorage(),
                publicAuthService = FakePublicAuthService(),
                scope = CoroutineScope(Dispatchers.Default + SupervisorJob()),
            )

        given("RootComponent initialization") {
            `when`("created with default settings") {
                then("initial stack contains Login configuration") {
                    val component =
                        RootComponent(
                            componentContext = createTestComponentContext(),
                            authManager = createTestAuthManager(),
                        )

                    val stack = component.stack.value
                    stack.items shouldHaveSize 1
                    stack.active.configuration.shouldBeInstanceOf<Config.Login>()
                }

                then("initial child is Login") {
                    val component =
                        RootComponent(
                            componentContext = createTestComponentContext(),
                            authManager = createTestAuthManager(),
                        )

                    val activeChild = component.stack.value.active.instance
                    activeChild.shouldBeInstanceOf<RootComponent.Child.Login>()
                }
            }
        }

        given("ComponentContextFactory") {
            `when`("creating root component context") {
                then("returns valid context") {
                    val context = createRootComponentContext()
                    context.shouldNotBeNull()
                }

                then("returns context with resumed lifecycle") {
                    val context = createRootComponentContext()
                    val lifecycle = context.getLifecycle()
                    lifecycle.state shouldBe Lifecycle.State.RESUMED
                }

                then("returns LifecycleRegistry from getLifecycle") {
                    val context = createRootComponentContext()
                    val lifecycle = context.getLifecycle()

                    lifecycle.shouldNotBeNull()
                    lifecycle.shouldBeInstanceOf<LifecycleRegistry>()
                }

                then("getLifecycleState returns RESUMED for new context") {
                    val context = createRootComponentContext()
                    val state = context.getLifecycleState()
                    state shouldBe Lifecycle.State.RESUMED
                }
            }
        }
    })

// ============================================
// Test Fakes
// ============================================

private class FakeTokenStorage : SecureTokenStorage {
    override suspend fun saveAccessToken(token: String) = Unit.right()

    override suspend fun getAccessToken(): String? = null

    override suspend fun saveRefreshToken(token: String) = Unit.right()

    override suspend fun getRefreshToken(): String? = null

    override suspend fun saveTokenExpiration(expiresAtMillis: Long) = Unit.right()

    override suspend fun getTokenExpiration(): Long? = null

    override suspend fun saveUserId(userId: Ulid) = Unit.right()

    override suspend fun getUserId(): Ulid? = null

    override suspend fun clear() = Unit.right()

    override suspend fun hasStoredCredentials(): Boolean = false
}

private class FakePublicAuthService : PublicAuthService {
    override suspend fun login(request: AuthRequest): AuthResponse = error("Stub")

    override suspend fun refresh(refreshToken: String): TokenRefreshResponse = error("Stub")

    override suspend fun register(request: RegisterRequest): AuthResponse = error("Stub")
}
