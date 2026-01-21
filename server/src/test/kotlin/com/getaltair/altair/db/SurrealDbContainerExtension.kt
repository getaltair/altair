package com.getaltair.altair.db

import io.kotest.core.listeners.AfterProjectListener
import io.kotest.core.listeners.BeforeProjectListener

/**
 * Kotest extension for managing SurrealDB Testcontainer lifecycle.
 *
 * This extension starts a SurrealDB container before all tests in the project
 * and stops it after all tests complete. The container is shared across all tests
 * for performance.
 *
 * Usage:
 * ```kotlin
 * class MySpec : BehaviorSpec({
 *     extension(SurrealDbContainerExtension)
 *
 *     given("database operations") {
 *         val config = SurrealDbContainerExtension.createNetworkConfig()
 *         val client = SurrealDbClient(config)
 *         // ... tests
 *     }
 * })
 * ```
 *
 * Or register globally in ProjectConfig:
 * ```kotlin
 * class ProjectConfig : AbstractProjectConfig() {
 *     override val extensions = listOf(SurrealDbContainerExtension)
 * }
 * ```
 */
object SurrealDbContainerExtension : BeforeProjectListener, AfterProjectListener {
    private val container = SurrealDbTestContainer()

    /**
     * Checks if the container is currently running.
     *
     * @return true if running, false otherwise
     */
    val isRunning: Boolean
        get() = container.isRunning

    /**
     * Starts the SurrealDB container before any tests run.
     */
    override suspend fun beforeProject() {
        if (!container.isRunning) {
            container.start()
        }
    }

    /**
     * Stops the SurrealDB container after all tests complete.
     */
    override suspend fun afterProject() {
        if (container.isRunning) {
            container.stop()
        }
    }

    /**
     * Creates a DatabaseConfig.Network from the running container.
     *
     * @return NetworkConfig for connecting to the test container
     */
    fun createNetworkConfig(): DatabaseConfig.Network = container.createNetworkConfig()

    /**
     * Gets the WebSocket connection URL for the running container.
     *
     * @return WebSocket URL (e.g., "ws://localhost:12345/rpc")
     */
    fun getConnectionUrl(): String = container.getConnectionUrl()

    /**
     * Gets the mapped port for the SurrealDB container.
     *
     * @return Port number
     */
    fun getPort(): Int = container.getPort()
}
