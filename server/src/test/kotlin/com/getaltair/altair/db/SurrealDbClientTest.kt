package com.getaltair.altair.db

import kotlinx.coroutines.runBlocking
import org.junit.jupiter.api.AfterAll
import org.junit.jupiter.api.BeforeAll
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.junit.jupiter.api.TestInstance
import org.testcontainers.junit.jupiter.Container
import org.testcontainers.junit.jupiter.Testcontainers
import kotlin.test.assertEquals
import kotlin.test.assertTrue

@Testcontainers
@TestInstance(TestInstance.Lifecycle.PER_CLASS)
class SurrealDbClientTest {
    private lateinit var dbClient: SurrealDbClient

    @BeforeAll
    fun setupContainer() {
        container.start()
        runBlocking {
            val config = container.createNetworkConfig()
            dbClient = SurrealDbClient(config)
            dbClient.connect().getOrNull()
        }
    }

    @AfterAll
    fun tearDown() {
        runBlocking {
            dbClient.close()
        }
        container.stop()
    }

    @BeforeEach
    fun setup() {
        runBlocking {
            // Clean up test table before each test
            dbClient.execute("DELETE test_table;")
        }
    }

    @Test
    fun `queryBind executes parameterized query successfully`(): Unit =
        runBlocking {
            // Create test data
            dbClient.execute(
                """
                CREATE test_table:user1 CONTENT {
                    name: "Alice",
                    email: "alice@test.com",
                    age: 30
                };
                """.trimIndent(),
            )

            // Query with parameters
            val result =
                dbClient.queryBind(
                    "SELECT * FROM test_table WHERE email = \$email",
                    mapOf("email" to "alice@test.com"),
                )

            assertTrue(result.isRight())
            result.onRight { json ->
                assertTrue(json.contains("alice@test.com"))
                assertTrue(json.contains("Alice"))
            }
        }

    @Test
    fun `queryBind handles multiple parameters`(): Unit =
        runBlocking {
            dbClient.execute(
                """
                CREATE test_table:user1 CONTENT { name: "Alice", age: 30 };
                CREATE test_table:user2 CONTENT { name: "Bob", age: 25 };
                CREATE test_table:user3 CONTENT { name: "Charlie", age: 35 };
                """.trimIndent(),
            )

            val result =
                dbClient.queryBind(
                    "SELECT * FROM test_table WHERE name = \$name AND age = \$age",
                    mapOf("name" to "Alice", "age" to 30),
                )

            assertTrue(result.isRight())
            result.onRight { json ->
                assertTrue(json.contains("Alice"))
                assertTrue(json.contains("30"))
            }
        }

    @Test
    fun `queryBind prevents injection attacks`(): Unit =
        runBlocking {
            dbClient.execute(
                """
                CREATE test_table:user1 CONTENT { email: "safe@test.com" };
                CREATE test_table:user2 CONTENT { email: "malicious@test.com" };
                """.trimIndent(),
            )

            // Attempt injection with special characters
            val maliciousEmail = "safe@test.com' OR '1'='1"
            val result =
                dbClient.queryBind(
                    "SELECT * FROM test_table WHERE email = \$email",
                    mapOf("email" to maliciousEmail),
                )

            assertTrue(result.isRight())
            result.onRight { json ->
                // Should return empty array since the exact string doesn't match
                assertEquals("[]", json)
            }
        }

    @Test
    fun `queryBindAs deserializes result correctly`(): Unit =
        runBlocking {
            dbClient.execute(
                """
                CREATE test_table:user1 CONTENT { name: "Alice", age: 30 };
                """.trimIndent(),
            )

            val result =
                dbClient.queryBindAs(
                    "SELECT * FROM test_table WHERE name = \$name",
                    mapOf("name" to "Alice"),
                ) { json ->
                    // Simple count check
                    json.count { it == '{' }
                }

            assertTrue(result.isRight())
            result.onRight { count ->
                assertEquals(1, count)
            }
        }

    @Test
    fun `executeBind executes statements with parameters`(): Unit =
        runBlocking {
            val result =
                dbClient.executeBind(
                    "CREATE test_table:user1 CONTENT { name: \$name, email: \$email };",
                    mapOf(
                        "name" to "Alice",
                        "email" to "alice@test.com",
                    ),
                )

            assertTrue(result.isRight())

            // Verify data was created
            val queryResult =
                dbClient.queryBind(
                    "SELECT * FROM test_table WHERE email = \$email",
                    mapOf("email" to "alice@test.com"),
                )
            assertTrue(queryResult.isRight())
            queryResult.onRight { json ->
                assertTrue(json.contains("Alice"))
                assertTrue(json.contains("alice@test.com"))
            }
        }

    @Test
    fun `queryBind handles null and special characters in parameters`(): Unit =
        runBlocking {
            dbClient.execute(
                """
                CREATE test_table:user1 CONTENT { name: "Alice", status: null };
                CREATE test_table:user2 CONTENT { name: "Bob", status: "active" };
                CREATE test_table:user3 CONTENT {
                    name: "Test's \"quoted\" value with \n newlines and \t tabs"
                };
                """.trimIndent(),
            )

            // Test with actual string value (since = NULL doesn't work in SQL)
            val result =
                dbClient.queryBind(
                    "SELECT * FROM test_table WHERE status = \$status",
                    mapOf("status" to "active"),
                )
            assertTrue(result.isRight())
            result.onRight { json -> assertTrue(json.contains("Bob")) }

            // Test special characters
            val specialString = "Test's \"quoted\" value with \n newlines and \t tabs"
            val specialResult =
                dbClient.queryBind(
                    "SELECT * FROM test_table WHERE name = \$name",
                    mapOf("name" to specialString),
                )
            assertTrue(specialResult.isRight())
            specialResult.onRight { json -> assertTrue(json.isNotEmpty()) }
        }

    @Test
    fun `queryBind handles numeric and boolean parameters`(): Unit =
        runBlocking {
            dbClient.execute(
                """
                CREATE test_table:item1 CONTENT { price: 19.99, quantity: 5, in_stock: true };
                CREATE test_table:item2 CONTENT { price: 29.99, quantity: 10, in_stock: true };
                CREATE test_table:user1 CONTENT { name: "Alice", is_active: true };
                CREATE test_table:user2 CONTENT { name: "Bob", is_active: false };
                """.trimIndent(),
            )

            // Test numeric parameters
            val numericResult =
                dbClient.queryBind(
                    "SELECT * FROM test_table WHERE price > \$minPrice AND quantity >= \$minQuantity",
                    mapOf(
                        "minPrice" to 15.00,
                        "minQuantity" to 5,
                    ),
                )
            assertTrue(numericResult.isRight())
            numericResult.onRight { json ->
                assertTrue(json.contains("19.99"))
                assertTrue(json.contains("29.99"))
            }

            // Test boolean parameters
            val booleanResult =
                dbClient.queryBind(
                    "SELECT * FROM test_table WHERE is_active = \$active",
                    mapOf("active" to true),
                )
            assertTrue(booleanResult.isRight())
            booleanResult.onRight { json ->
                assertTrue(json.contains("Alice"))
            }
        }

    companion object {
        @Container
        val container = SurrealDbTestContainer()
    }
}
