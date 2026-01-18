package com.getaltair

import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.server.testing.*
import kotlin.test.Test
import kotlin.test.assertEquals

class ApplicationTest {
    @Test
    fun testRoot() =
        testApplication {
            application {
                // Skip database and security setup for basic routing test
                // Security requires JwtConfig from Koin which needs environment variables
                configureSerialization()
                configureMonitoring()
                configureHTTP()
                configureRouting()
            }
            client.get("/").apply {
                assertEquals(HttpStatusCode.OK, status)
            }
        }
}
