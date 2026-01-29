package com.getaltair.altair

import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.server.response.*
import io.ktor.server.routing.*
import io.ktor.server.testing.*
import kotlin.test.Test
import kotlin.test.assertEquals

class ApplicationTest {

    /**
     * Simple health check test.
     * Tests the basic routing without full Koin DI setup.
     * Full integration tests with database will be added in Phase 5.
     */
    @Test
    fun testRoot() = testApplication {
        routing {
            get("/") {
                call.respondText("Altair API Server")
            }
            get("/health") {
                call.respondText("OK")
            }
        }
        client.get("/").apply {
            assertEquals(HttpStatusCode.OK, status)
        }
        client.get("/health").apply {
            assertEquals(HttpStatusCode.OK, status)
        }
    }
}
