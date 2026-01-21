package com.getaltair.rpc

import com.getaltair.altair.domain.types.Ulid
import com.getaltair.altair.domain.types.enums.UserRole
import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.AuthResponse
import com.getaltair.altair.dto.auth.ChangePasswordRequest
import com.getaltair.altair.dto.auth.InviteCodeResponse
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.auth.SuccessResponse
import com.getaltair.altair.dto.auth.TokenRefreshResponse
import com.getaltair.altair.dto.sync.ChangeOperation
import com.getaltair.altair.dto.sync.ChangeSet
import com.getaltair.altair.dto.sync.EntityChange
import com.getaltair.altair.rpc.AiService
import com.getaltair.altair.rpc.AuthService
import com.getaltair.altair.rpc.CompletionRequest
import com.getaltair.altair.rpc.ContextMessage
import com.getaltair.altair.rpc.MessageRole
import com.getaltair.altair.rpc.PublicAuthService
import com.getaltair.altair.rpc.SyncService
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.booleans.shouldBeTrue
import io.kotest.matchers.collections.shouldBeEmpty
import io.kotest.matchers.collections.shouldContain
import io.kotest.matchers.collections.shouldHaveSize
import io.kotest.matchers.comparables.shouldBeGreaterThan
import io.kotest.matchers.nulls.shouldNotBeNull
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain
import io.ktor.client.plugins.websocket.WebSockets
import io.ktor.server.application.Application
import io.ktor.server.application.install
import io.ktor.server.routing.routing
import io.ktor.server.testing.testApplication
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.launch
import kotlinx.rpc.krpc.ktor.client.installKrpc
import kotlinx.rpc.krpc.ktor.client.rpcConfig
import kotlinx.rpc.krpc.ktor.server.Krpc
import kotlinx.rpc.krpc.ktor.server.rpc
import kotlinx.rpc.krpc.serialization.json.json
import kotlinx.rpc.withService
import kotlinx.serialization.json.JsonPrimitive
import io.ktor.server.websocket.WebSockets as ServerWebSockets
import kotlinx.rpc.krpc.ktor.client.rpc as clientRpc

/**
 * Stub PublicAuthService for testing RPC infrastructure.
 * Returns predictable test data without requiring real auth dependencies.
 */
private class StubPublicAuthService : PublicAuthService {
    override suspend fun login(request: AuthRequest) =
        AuthResponse(
            accessToken = "test-access-token",
            refreshToken = "test-refresh-token",
            expiresIn = 3600,
            userId = Ulid("01HWT3ST000000000000000001"),
            displayName = "Test User",
            role = UserRole.MEMBER,
        )

    override suspend fun refresh(refreshToken: String) =
        TokenRefreshResponse(
            accessToken = "test-refreshed-token",
            refreshToken = "test-new-refresh-token",
            expiresIn = 3600,
        )

    override suspend fun register(request: RegisterRequest) =
        AuthResponse(
            accessToken = "test-access-token",
            refreshToken = "test-refresh-token",
            expiresIn = 3600,
            userId = Ulid("01HWT3ST000000000000000002"),
            displayName = request.displayName,
            role = UserRole.MEMBER,
        )
}

/**
 * Stub AuthService for testing authenticated RPC operations.
 * Returns predictable test data without requiring real auth dependencies.
 */
private class StubAuthService : AuthService {
    override suspend fun logout() =
        SuccessResponse(
            success = true,
            message = "Logged out",
        )

    override suspend fun generateInviteCode() =
        InviteCodeResponse(
            code = "TESTCODE",
            expiresAt = "2099-12-31T23:59:59Z",
        )

    override suspend fun changePassword(request: ChangePasswordRequest) =
        SuccessResponse(
            success = true,
            message = "Password changed",
        )

    override suspend fun revokeAllSessions() =
        SuccessResponse(
            success = true,
            message = "All sessions revoked",
        )
}

/**
 * Configure RPC for tests with stub services.
 * Uses a single unauthenticated endpoint for simplicity in tests.
 */
private fun Application.configureTestRpc() {
    install(ServerWebSockets)
    install(Krpc)

    routing {
        rpc("/rpc") {
            rpcConfig {
                serialization {
                    json()
                }
            }

            registerService<SyncService> { SyncServiceImpl() }
            registerService<PublicAuthService> { StubPublicAuthService() }
            registerService<AuthService> { StubAuthService() }
            registerService<AiService> { AiServiceImpl() }
        }
    }
}

/**
 * Integration tests for RPC services.
 *
 * These tests verify that the RPC infrastructure works end-to-end:
 * client -> WebSocket -> server -> service implementation -> response.
 *
 * Verifies:
 * - PublicAuthService (login, register, token refresh)
 * - AuthService (logout, invite codes)
 * - SyncService (pull, push, streaming)
 * - AiService (embeddings, transcription, completions)
 */
class RpcIntegrationTest :
    BehaviorSpec({
        given("PublicAuthService RPC") {
            `when`("logging in") {
                then("returns valid response") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val authService = client.withService<PublicAuthService>()

                        val response = authService.login(AuthRequest("test@example.com", "password"))

                        response.accessToken.shouldNotBeNull()
                        response.refreshToken.shouldNotBeNull()
                        response.role shouldBe UserRole.MEMBER
                        response.expiresIn shouldBe 3600
                    }
                }
            }

            `when`("registering new user") {
                then("returns valid response") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val authService = client.withService<PublicAuthService>()

                        val response =
                            authService.register(
                                RegisterRequest(
                                    email = "new@example.com",
                                    password = "password123",
                                    displayName = "New User",
                                ),
                            )

                        response.accessToken.shouldNotBeNull()
                        response.displayName shouldBe "New User"
                    }
                }
            }

            `when`("refreshing token") {
                then("returns new tokens with rotation") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val authService = client.withService<PublicAuthService>()

                        val response = authService.refresh("some-refresh-token")

                        response.accessToken.shouldNotBeNull()
                        response.refreshToken.shouldNotBeNull()
                        response.expiresIn shouldBe 3600
                    }
                }
            }
        }

        given("AuthService RPC") {
            `when`("logging out") {
                then("completes successfully") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val authService = client.withService<AuthService>()

                        val response = authService.logout()

                        response.success.shouldBeTrue()
                    }
                }
            }

            `when`("generating invite code") {
                then("returns code") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val authService = client.withService<AuthService>()

                        val response = authService.generateInviteCode()

                        response.code.shouldNotBeNull()
                        response.expiresAt.shouldNotBeNull()
                    }
                }
            }
        }

        given("SyncService RPC") {
            `when`("pulling changes with zero since") {
                then("returns SyncResponse") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val syncService = client.withService<SyncService>()

                        val response = syncService.pull(since = 0L, entityTypes = emptySet())

                        response.serverVersion shouldBe 1L
                        response.changes.shouldBeEmpty()
                        response.conflicts.shouldBeEmpty()
                    }
                }
            }

            `when`("pulling with non-zero since") {
                then("returns incremented version") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val syncService = client.withService<SyncService>()

                        val response = syncService.pull(since = 100L, entityTypes = setOf("Quest", "Note"))

                        response.serverVersion shouldBe 101L
                    }
                }
            }

            `when`("pushing empty changes") {
                then("acknowledges changes") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val syncService = client.withService<SyncService>()

                        val changeSet = ChangeSet(changes = emptyList(), clientTimestamp = System.currentTimeMillis())
                        val result = syncService.push(changeSet)

                        result.success.shouldBeTrue()
                        result.serverVersion shouldBeGreaterThan 0L
                    }
                }
            }

            `when`("pushing entity changes") {
                then("acknowledges entity IDs from changes") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val syncService = client.withService<SyncService>()

                        val changes =
                            ChangeSet(
                                changes =
                                    listOf(
                                        EntityChange(
                                            entityType = "Quest",
                                            entityId = "01HWTEST000000000000000001",
                                            operation = ChangeOperation.CREATE,
                                            version = 1L,
                                            data = JsonPrimitive("test"),
                                            timestamp = System.currentTimeMillis(),
                                        ),
                                        EntityChange(
                                            entityType = "Note",
                                            entityId = "01HWTEST000000000000000002",
                                            operation = ChangeOperation.UPDATE,
                                            version = 2L,
                                            data = null,
                                            timestamp = System.currentTimeMillis(),
                                        ),
                                    ),
                                clientTimestamp = System.currentTimeMillis(),
                            )

                        val result = syncService.push(changes)

                        result.success.shouldBeTrue()
                        result.acknowledged shouldHaveSize 2
                        result.acknowledged shouldContain "01HWTEST000000000000000001"
                        result.acknowledged shouldContain "01HWTEST000000000000000002"
                    }
                }
            }

            `when`("streaming changes") {
                then("can be started and cancelled") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val syncService = client.withService<SyncService>()

                        val scope = CoroutineScope(Dispatchers.Default)
                        val job =
                            scope.launch {
                                syncService.streamChanges(setOf("Quest")).collect { /* no-op */ }
                            }

                        // Let it establish connection
                        delay(100)
                        job.isActive.shouldBeTrue()

                        // Verify cancellation works
                        job.cancel()
                        job.join()
                        job.isCancelled.shouldBeTrue()
                    }
                }
            }
        }

        given("AiService RPC") {
            `when`("embedding texts") {
                then("returns embeddings") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val aiService = client.withService<AiService>()

                        val embeddings = aiService.embed(listOf("Hello", "World"))

                        embeddings shouldHaveSize 2
                        embeddings[0].isNotEmpty().shouldBeTrue()
                        embeddings[1].isNotEmpty().shouldBeTrue()
                    }
                }

                then("handles empty list") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val aiService = client.withService<AiService>()

                        val embeddings = aiService.embed(emptyList())

                        embeddings.shouldBeEmpty()
                    }
                }
            }

            `when`("transcribing audio") {
                then("returns placeholder text") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val aiService = client.withService<AiService>()

                        val result = aiService.transcribe(byteArrayOf(1, 2, 3), "wav")

                        result shouldContain "Transcription placeholder"
                        result shouldContain "3 bytes"
                        result shouldContain "wav"
                    }
                }
            }

            `when`("completing with minimal request") {
                then("streams tokens") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val aiService = client.withService<AiService>()

                        val request = CompletionRequest(prompt = "Test prompt")
                        val tokens = aiService.complete(request).toList()

                        tokens.isNotEmpty().shouldBeTrue()
                        val fullResponse = tokens.joinToString("")
                        fullResponse shouldContain "stub response"
                    }
                }
            }

            `when`("completing with all optional parameters") {
                then("handles request") {
                    testApplication {
                        application { configureTestRpc() }

                        val client = createRpcClient()
                        val aiService = client.withService<AiService>()

                        val request =
                            CompletionRequest(
                                prompt = "Test",
                                systemPrompt = "You are helpful",
                                maxTokens = 512,
                                temperature = 0.5f,
                                context = listOf(ContextMessage(role = MessageRole.USER, content = "Hello")),
                            )

                        val tokens = aiService.complete(request).toList()
                        tokens.isNotEmpty().shouldBeTrue()
                    }
                }
            }
        }
    }) {
    companion object {
        private fun io.ktor.server.testing.ApplicationTestBuilder.createRpcClient() =
            createClient {
                install(WebSockets)
                installKrpc()
            }.clientRpc {
                url {
                    host = "localhost"
                    port = 80
                    protocol = io.ktor.http.URLProtocol.WS
                    pathSegments = listOf("rpc")
                }
                rpcConfig {
                    serialization {
                        json()
                    }
                }
            }
    }
}
