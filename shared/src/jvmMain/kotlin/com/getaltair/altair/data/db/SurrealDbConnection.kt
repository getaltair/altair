package com.getaltair.altair.data.db

import com.surrealdb.Surreal
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import java.io.File
import java.util.concurrent.atomic.AtomicBoolean

/**
 * Singleton connection manager for SurrealDB.
 *
 * Provides thread-safe connection lifecycle management with support for
 * embedded SurrealKV storage or in-memory mode.
 *
 * Usage:
 * ```kotlin
 * // Connect with config
 * SurrealDbConnection.connect(SurrealDbConfig.embedded(dataPath))
 *
 * // Use the driver
 * val driver = SurrealDbConnection.getDriver()
 *
 * // Disconnect when done
 * SurrealDbConnection.disconnect()
 * ```
 */
object SurrealDbConnection {
    private var driver: Surreal? = null
    private var currentConfig: SurrealDbConfig? = null
    private val mutex = Mutex()
    private val isConnected = AtomicBoolean(false)

    /**
     * Establishes a connection to SurrealDB using the provided configuration.
     *
     * This method is idempotent - calling it multiple times with the same config
     * will not create multiple connections.
     *
     * @param config The SurrealDB configuration
     * @throws SurrealDbConnectionException if connection fails
     */
    suspend fun connect(config: SurrealDbConfig) {
        mutex.withLock {
            if (isConnected.get() && currentConfig == config) {
                // Already connected with same config
                return
            }

            // Disconnect existing connection if any
            disconnectInternal()

            withContext(Dispatchers.IO) {
                try {
                    // Ensure data directory exists for file-based storage
                    if (config.dataPath != null) {
                        ensureDirectoryExists(config.dataPath)
                    }

                    // Create and connect driver
                    val newDriver = Surreal()
                    newDriver.connect(config.connectionString())
                    newDriver.useNs(config.namespace)
                    newDriver.useDb(config.database)

                    driver = newDriver
                    currentConfig = config
                    isConnected.set(true)

                    println("[SurrealDB] Connected to ${config.connectionString()} (ns=${config.namespace}, db=${config.database})")
                } catch (e: Exception) {
                    throw SurrealDbConnectionException(
                        "Failed to connect to SurrealDB: ${e.message}",
                        e
                    )
                }
            }
        }
    }

    /**
     * Disconnects from SurrealDB and releases resources.
     *
     * Safe to call multiple times or when not connected.
     */
    suspend fun disconnect() {
        mutex.withLock {
            disconnectInternal()
        }
    }

    /**
     * Internal disconnect without acquiring mutex.
     * Must be called while holding the mutex.
     */
    private fun disconnectInternal() {
        driver?.let { d ->
            try {
                d.close()
                println("[SurrealDB] Disconnected")
            } catch (e: Exception) {
                println("[SurrealDB] Warning during disconnect: ${e.message}")
            }
        }
        driver = null
        currentConfig = null
        isConnected.set(false)
    }

    /**
     * Returns the active SurrealDB driver.
     *
     * @throws SurrealDbConnectionException if not connected
     */
    fun getDriver(): Surreal {
        if (!isConnected.get()) {
            throw SurrealDbConnectionException("Not connected to SurrealDB. Call connect() first.")
        }
        return driver ?: throw SurrealDbConnectionException("Driver is null but isConnected is true")
    }

    /**
     * Returns whether a connection is currently established.
     */
    fun isConnected(): Boolean = isConnected.get()

    /**
     * Returns the current configuration, or null if not connected.
     */
    fun getCurrentConfig(): SurrealDbConfig? = currentConfig

    /**
     * Ensures the specified directory exists, creating it if necessary.
     */
    private fun ensureDirectoryExists(path: String) {
        val dir = File(path)
        if (!dir.exists()) {
            val created = dir.mkdirs()
            if (!created && !dir.exists()) {
                throw SurrealDbConnectionException("Failed to create data directory: $path")
            }
            println("[SurrealDB] Created data directory: $path")
        }
    }
}

/**
 * Exception thrown when SurrealDB connection operations fail.
 */
class SurrealDbConnectionException(
    message: String,
    cause: Throwable? = null
) : RuntimeException(message, cause)
