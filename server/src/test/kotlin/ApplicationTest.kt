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
                // Skip database setup for basic routing test
                // Use moduleWithoutDatabase() for tests that don't need DB
                configureSerialization()
                configureSecurity()
                configureMonitoring()
                configureHTTP()
                configureRouting()
            }
            client.get("/").apply {
                assertEquals(HttpStatusCode.OK, status)
            }
        }
}
