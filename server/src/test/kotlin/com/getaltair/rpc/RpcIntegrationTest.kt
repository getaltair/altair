package com.getaltair.rpc

import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.sync.ChangeOperation
import com.getaltair.altair.dto.sync.ChangeSet
import com.getaltair.altair.dto.sync.EntityChange
import com.getaltair.altair.rpc.AiService
import com.getaltair.altair.rpc.AuthService
import com.getaltair.altair.rpc.CompletionRequest
import com.getaltair.altair.rpc.ContextMessage
import com.getaltair.altair.rpc.MessageRole
import com.getaltair.altair.rpc.SyncService
import com.getaltair.configureRpc
import io.ktor.client.plugins.websocket.WebSockets
import io.ktor.server.testing.testApplication
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.launch
import kotlinx.rpc.krpc.ktor.client.installKrpc
import kotlinx.rpc.krpc.ktor.client.rpc
import kotlinx.rpc.krpc.ktor.client.rpcConfig
import kotlinx.rpc.krpc.serialization.json.json
import kotlinx.rpc.withService
import kotlinx.serialization.json.JsonPrimitive
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNotNull
import kotlin.test.assertTrue

/**
 * Integration tests for RPC services.
 *
 * These tests verify that the RPC infrastructure works end-to-end:
 * client -> WebSocket -> server -> service implementation -> response.
 */
class RpcIntegrationTest {
    @Test
    fun `AuthService login returns valid response`() =
        testApplication {
            application { configureRpc() }

            val client = createRpcClient()
            val authService = client.withService<AuthService>()

            val response = authService.login(AuthRequest("test@example.com", "password"))

            assertNotNull(response.accessToken)
            assertNotNull(response.refreshToken)
            assertEquals("member", response.role)
            assertTrue(response.expiresIn > 0)
        }

    @Test
    fun `AuthService register returns valid response`() =
        testApplication {
            application { configureRpc() }

            val client = createRpcClient()
            val authService = client.withService<AuthService>()

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
    fun `AuthService refresh returns new token`() =
        testApplication {
            application { configureRpc() }

            val client = createRpcClient()
            val authService = client.withService<AuthService>()

            val response = authService.refresh("some-refresh-token")

            assertNotNull(response.accessToken)
            assertTrue(response.expiresIn > 0)
        }

    @Test
    fun `AuthService logout completes without error`() =
        testApplication {
            application { configureRpc() }

            val client = createRpcClient()
            val authService = client.withService<AuthService>()

            // Should complete without throwing
            authService.logout()
        }

    @Test
    fun `SyncService pull returns SyncResponse`() =
        testApplication {
            application { configureRpc() }

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
            application { configureRpc() }

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
            application { configureRpc() }

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
            application { configureRpc() }

            val client = createRpcClient()
            val syncService = client.withService<SyncService>()

            val response = syncService.pull(since = 100L, entityTypes = setOf("Quest", "Note"))

            assertEquals(101L, response.serverVersion)
        }

    @Test
    fun `SyncService streamChanges can be started and cancelled`() =
        testApplication {
            application { configureRpc() }

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
            application { configureRpc() }

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
            application { configureRpc() }

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
            application { configureRpc() }

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
            application { configureRpc() }

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
            application { configureRpc() }

            val client = createRpcClient()
            val aiService = client.withService<AiService>()

            val embeddings = aiService.embed(emptyList())

            assertTrue(embeddings.isEmpty())
        }

    private fun io.ktor.server.testing.ApplicationTestBuilder.createRpcClient() =
        createClient {
            install(WebSockets)
            installKrpc()
        }.rpc {
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
