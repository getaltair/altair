package com.getaltair.altair.db

import org.testcontainers.containers.GenericContainer
import org.testcontainers.containers.wait.strategy.Wait
import org.testcontainers.utility.DockerImageName

/**
 * Testcontainers helper for SurrealDB integration tests.
 *
 * Starts a SurrealDB container and provides connection details.
 */
class SurrealDbTestContainer :
    GenericContainer<SurrealDbTestContainer>(
        DockerImageName.parse("surrealdb/surrealdb:v2.0.0"),
    ) {
    init {
        withExposedPorts(SURREALDB_PORT)
        withCommand("start", "--log", "info", "--user", "root", "--pass", "root", "memory")
        waitingFor(Wait.forLogMessage(".*Started web server.*\\n", 1))
    }

    fun getConnectionUrl(): String = "ws://${super.getHost()}:${getMappedPort(SURREALDB_PORT)}/rpc"

    fun getPort(): Int = getMappedPort(SURREALDB_PORT)

    fun createNetworkConfig(): DatabaseConfig.Network =
        DatabaseConfig.Network(
            host = super.getHost(),
            port = getMappedPort(SURREALDB_PORT),
            useTls = false,
            namespace = "test",
            database = "test",
            username = "root",
            password = "root",
        )

    companion object {
        private const val SURREALDB_PORT = 8000
    }
}
