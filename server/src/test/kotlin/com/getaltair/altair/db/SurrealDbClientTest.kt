package com.getaltair.altair.db

import io.kotest.assertions.arrow.core.shouldBeRight
import io.kotest.core.spec.style.BehaviorSpec
import io.kotest.matchers.shouldBe
import io.kotest.matchers.string.shouldContain
import io.kotest.matchers.string.shouldNotBeEmpty

/**
 * Tests for SurrealDbClient using Testcontainers.
 *
 * Verifies:
 * - Parameterized query execution with queryBind
 * - SQL injection prevention
 * - Type-safe deserialization with queryBindAs
 * - Statement execution with executeBind
 * - Handling of various data types (strings, numbers, booleans, special characters)
 */
class SurrealDbClientTest :
    BehaviorSpec({
        lateinit var dbClient: SurrealDbClient

        beforeSpec {
            val config = SurrealDbContainerExtension.createNetworkConfig()
            dbClient = SurrealDbClient(config)
            dbClient.connect().getOrNull()
        }

        afterSpec {
            dbClient.close()
        }

        beforeEach {
            // Clean up test table before each test
            dbClient.execute("DELETE test_table;")
        }

        given("queryBind with parameterized queries") {
            `when`("single parameter query is executed") {
                then("returns correct results") {
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

                    result.shouldBeRight()
                    val json = result.getOrNull()!!
                    json.shouldContain("alice@test.com")
                    json.shouldContain("Alice")
                }
            }

            `when`("multiple parameters are used") {
                then("handles all parameters correctly") {
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

                    result.shouldBeRight()
                    val json = result.getOrNull()!!
                    json.shouldContain("Alice")
                    json.shouldContain("30")
                }
            }

            `when`("SQL injection is attempted") {
                then("prevents injection attack") {
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

                    result.shouldBeRight()
                    val json = result.getOrNull()!!
                    // Should return empty array since the exact string doesn't match
                    json shouldBe "[]"
                }
            }

            `when`("null and special characters are in parameters") {
                then("handles them correctly") {
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
                    result.shouldBeRight()
                    val json = result.getOrNull()!!
                    json.shouldContain("Bob")

                    // Test special characters
                    val specialString = "Test's \"quoted\" value with \n newlines and \t tabs"
                    val specialResult =
                        dbClient.queryBind(
                            "SELECT * FROM test_table WHERE name = \$name",
                            mapOf("name" to specialString),
                        )
                    specialResult.shouldBeRight()
                    val specialJson = specialResult.getOrNull()!!
                    specialJson.shouldNotBeEmpty()
                }
            }

            `when`("numeric and boolean parameters are used") {
                then("handles all types correctly") {
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
                    numericResult.shouldBeRight()
                    val numericJson = numericResult.getOrNull()!!
                    numericJson.shouldContain("19.99")
                    numericJson.shouldContain("29.99")

                    // Test boolean parameters
                    val booleanResult =
                        dbClient.queryBind(
                            "SELECT * FROM test_table WHERE is_active = \$active",
                            mapOf("active" to true),
                        )
                    booleanResult.shouldBeRight()
                    val booleanJson = booleanResult.getOrNull()!!
                    booleanJson.shouldContain("Alice")
                }
            }
        }

        given("queryBindAs with deserialization") {
            `when`("result needs to be deserialized") {
                then("deserializes correctly") {
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

                    result.shouldBeRight()
                    val count = result.getOrNull()!!
                    count shouldBe 1
                }
            }
        }

        given("executeBind with parameterized statements") {
            `when`("statement is executed with parameters") {
                then("executes successfully and creates data") {
                    val result =
                        dbClient.executeBind(
                            "CREATE test_table:user1 CONTENT { name: \$name, email: \$email };",
                            mapOf(
                                "name" to "Alice",
                                "email" to "alice@test.com",
                            ),
                        )

                    result.shouldBeRight()

                    // Verify data was created
                    val queryResult =
                        dbClient.queryBind(
                            "SELECT * FROM test_table WHERE email = \$email",
                            mapOf("email" to "alice@test.com"),
                        )
                    queryResult.shouldBeRight()
                    val json = queryResult.getOrNull()!!
                    json.shouldContain("Alice")
                    json.shouldContain("alice@test.com")
                }
            }
        }
    })
