package com.getaltair.rpc

import com.getaltair.altair.dto.auth.AuthRequest
import com.getaltair.altair.dto.auth.RegisterRequest
import com.getaltair.altair.dto.sync.ChangeSet
import com.getaltair.altair.rpc.AiService
import com.getaltair.altair.rpc.AuthService
import com.getaltair.altair.rpc.CompletionRequest
import com.getaltair.altair.rpc.SyncService
import com.getaltair.configureRpc
import io.ktor.client.plugins.websocket.WebSockets
import io.ktor.server.testing.testApplication
import kotlinx.coroutines.flow.toList
import kotlinx.rpc.krpc.ktor.client.installKrpc
import kotlinx.rpc.krpc.ktor.client.rpc
import kotlinx.rpc.krpc.ktor.client.rpcConfig
import kotlinx.rpc.krpc.serialization.json.json
import kotlinx.rpc.withService
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
