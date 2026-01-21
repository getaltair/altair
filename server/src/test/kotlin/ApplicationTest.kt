package com.getaltair

import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.shouldBe
import io.ktor.client.request.*
import io.ktor.http.*
import io.ktor.server.testing.*

/**
 * Tests for application startup and basic routing.
 *
 * Verifies:
 * - Server starts successfully with minimal configuration
 * - Root endpoint is accessible
 */
class ApplicationTest :
    BehaviorSpec({
        given("server startup") {
            `when`("configuring basic application") {
                then("root endpoint returns OK") {
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
                            status shouldBe HttpStatusCode.OK
                        }
                    }
                }
            }
        }
    })
