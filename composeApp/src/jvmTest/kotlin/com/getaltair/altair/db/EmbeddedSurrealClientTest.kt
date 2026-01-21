package com.getaltair.altair.db

import com.getaltair.altair.domain.DomainError
import io.kotest.assertions.arrow.core.shouldBeLeft
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.booleans.shouldBeFalse
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain
import io.kotest.matchers.types.shouldBeInstanceOf
import java.nio.file.Files

/**
 * Tests for EmbeddedSurrealClient.
 *
 * These tests verify the basic lifecycle and error handling of the embedded database client.
 */
class EmbeddedSurrealClientTest :
    BehaviorSpec({
        lateinit var tempDir: java.nio.file.Path
        lateinit var config: DesktopDatabaseConfig
        lateinit var client: EmbeddedSurrealClient

        beforeEach {
            tempDir = Files.createTempDirectory("embedded-surreal-test")
            config =
                DesktopDatabaseConfig(
                    dataDirectory = tempDir,
                    namespace = "test",
                    database = "test_db",
                )
            client = EmbeddedSurrealClient(config)
        }

        afterEach {
            client.close()
            // Clean up temp directory
            tempDir.toFile().deleteRecursively()
        }

        given("client lifecycle") {
            `when`("client is created") {
                then("starts in disconnected state") {
                    client.isConnected().shouldBeFalse()
                }
            }

            `when`("close is called on disconnected client") {
                then("does not throw") {
                    // Should not throw even when called on disconnected client
                    client.close()
                    client.isConnected().shouldBeFalse()
                }
            }
        }

        given("error handling") {
            `when`("query is called when not connected") {
                then("returns error") {
                    val result = client.query<Any>("SELECT * FROM test")

                    result.shouldBeLeft()
                    val error = result.leftOrNull()
                    error.shouldBeInstanceOf<DomainError.UnexpectedError>()
                    error?.message shouldContain "Not connected"
                }
            }

            `when`("execute is called when not connected") {
                then("returns error") {
                    val result = client.execute("CREATE test SET name = 'value'")

                    result.shouldBeLeft()
                    val error = result.leftOrNull()
                    error.shouldBeInstanceOf<DomainError.UnexpectedError>()
                    error?.message shouldContain "Not connected"
                }
            }
        }

        given("configuration") {
            `when`("config is created") {
                then("creates proper connection URL") {
                    // databasePath is dataDirectory/db
                    val expectedPath = tempDir.resolve("db").toAbsolutePath()
                    val expectedUrl = "surrealkv://$expectedPath"
                    config.connectionUrl shouldBe expectedUrl
                }

                then("creates proper paths") {
                    config.dataDirectory shouldBe tempDir
                    config.databasePath shouldBe tempDir.resolve("db")
                }
            }
        }
    })
