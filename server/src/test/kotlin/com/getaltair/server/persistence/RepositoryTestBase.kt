package com.getaltair.server.persistence

import com.getaltair.altair.shared.domain.common.Ulid
import com.getaltair.server.auth.AuthContext
import org.junit.Assume.assumeTrue
import org.junit.Before

/**
 * Base class for repository integration tests.
 *
 * These tests require a running SurrealDB instance. They are automatically
 * skipped when the database is not available.
 *
 * ## Running Tests
 *
 * ```bash
 * # Start SurrealDB
 * docker compose up -d surrealdb
 *
 * # Set test environment variable
 * export SURREALDB_TEST_URL="ws://localhost:8000"
 *
 * # Run tests
 * ./gradlew :server:test
 * ```
 *
 * ## CI Setup
 *
 * Add SurrealDB as a service container in your CI configuration:
 *
 * ```yaml
 * services:
 *   surrealdb:
 *     image: surrealdb/surrealdb:v2.1
 *     command: start --log info --user root --pass root memory
 *     ports:
 *       - 8000:8000
 * ```
 */
abstract class RepositoryTestBase {

    protected lateinit var db: SurrealDbClient
    protected lateinit var authContextUserA: AuthContext
    protected lateinit var authContextUserB: AuthContext
    protected val userAId = Ulid.generate()
    protected val userBId = Ulid.generate()

    private var surrealDbAvailable = false

    @Before
    fun setUp() {
        // Skip tests if SurrealDB is not running
        surrealDbAvailable = System.getenv("SURREALDB_TEST_URL") != null
        assumeTrue("SurrealDB not available - set SURREALDB_TEST_URL to run integration tests", surrealDbAvailable)

        // Create test auth contexts
        authContextUserA = object : AuthContext {
            override val currentUserId: Ulid = userAId
            override val isAuthenticated: Boolean = true
        }
        authContextUserB = object : AuthContext {
            override val currentUserId: Ulid = userBId
            override val isAuthenticated: Boolean = true
        }

        // Database setup would happen here when SurrealDB is available
    }
}
