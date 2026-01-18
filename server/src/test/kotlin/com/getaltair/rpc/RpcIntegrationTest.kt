package com.getaltair.rpc

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
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue
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
            userId = "01HWTEST000000000000000001",
            displayName = "Test User",
            role = "member",
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
            userId = "01HWTEST000000000000000002",
            displayName = request.displayName,
            role = "member",
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
 */
class RpcIntegrationTest {
    @Test
    fun `PublicAuthService login returns valid response`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val authService = client.withService<PublicAuthService>()

            val response = authService.login(AuthRequest("test@example.com", "password"))

            assertNotNull(response.accessToken)
            assertNotNull(response.refreshToken)
            assertEquals("member", response.role)
            assertTrue(response.expiresIn > 0)
        }

    @Test
    fun `PublicAuthService register returns valid response`() =
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

            assertNotNull(response.accessToken)
            assertEquals("New User", response.displayName)
        }

    @Test
    fun `PublicAuthService refresh returns new tokens with rotation`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val authService = client.withService<PublicAuthService>()

            val response = authService.refresh("some-refresh-token")

            assertNotNull(response.accessToken)
            assertNotNull(response.refreshToken)
            assertTrue(response.expiresIn > 0)
        }

    @Test
    fun `AuthService logout completes successfully`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val authService = client.withService<AuthService>()

            val response = authService.logout()

            assertTrue(response.success)
        }

    @Test
    fun `AuthService generateInviteCode returns code`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val authService = client.withService<AuthService>()

            val response = authService.generateInviteCode()

            assertNotNull(response.code)
            assertNotNull(response.expiresAt)
        }

    @Test
    fun `SyncService pull returns SyncResponse`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val syncService = client.withService<SyncService>()

            val response = syncService.pull(since = 0L, entityTypes = emptySet())

            assertTrue(response.serverVersion > 0)
            assertTrue(response.changes.isEmpty())
            assertTrue(response.conflicts.isEmpty())
        }

    @Test
    fun `SyncService push acknowledges changes`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val syncService = client.withService<SyncService>()

            val changeSet = ChangeSet(changes = emptyList(), clientTimestamp = System.currentTimeMillis())
            val result = syncService.push(changeSet)

            assertTrue(result.success)
            assertTrue(result.serverVersion > 0)
        }

    @Test
    fun `SyncService push acknowledges entity IDs from changes`() =
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

            assertTrue(result.success)
            assertEquals(2, result.acknowledged.size)
            assertTrue(result.acknowledged.contains("01HWTEST000000000000000001"))
            assertTrue(result.acknowledged.contains("01HWTEST000000000000000002"))
        }

    @Test
    fun `SyncService pull with non-zero since returns incremented version`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val syncService = client.withService<SyncService>()

            val response = syncService.pull(since = 100L, entityTypes = setOf("Quest", "Note"))

            assertEquals(101L, response.serverVersion)
        }

    @Test
    fun `SyncService streamChanges can be started and cancelled`() =
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
            assertTrue(job.isActive)

            // Verify cancellation works
            job.cancel()
            job.join()
            assertTrue(job.isCancelled)
        }

    @Test
    fun `AiService embed returns embeddings`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val aiService = client.withService<AiService>()

            val embeddings = aiService.embed(listOf("Hello", "World"))

            assertEquals(2, embeddings.size)
            assertTrue(embeddings[0].isNotEmpty())
            assertTrue(embeddings[1].isNotEmpty())
        }

    @Test
    fun `AiService transcribe returns placeholder text`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val aiService = client.withService<AiService>()

            val result = aiService.transcribe(byteArrayOf(1, 2, 3), "wav")

            assertTrue(result.contains("Transcription placeholder"))
            assertTrue(result.contains("3 bytes"))
            assertTrue(result.contains("wav"))
        }

    @Test
    fun `AiService complete streams tokens`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val aiService = client.withService<AiService>()

            val request = CompletionRequest(prompt = "Test prompt")
            val tokens = aiService.complete(request).toList()

            assertTrue(tokens.isNotEmpty())
            val fullResponse = tokens.joinToString("")
            assertTrue(fullResponse.contains("stub response"))
        }

    @Test
    fun `AiService complete handles request with all optional parameters`() =
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
            assertTrue(tokens.isNotEmpty())
        }

    @Test
    fun `AiService embed handles empty list`() =
        testApplication {
            application { configureTestRpc() }

            val client = createRpcClient()
            val aiService = client.withService<AiService>()

            val embeddings = aiService.embed(emptyList())

            assertTrue(embeddings.isEmpty())
        }

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
