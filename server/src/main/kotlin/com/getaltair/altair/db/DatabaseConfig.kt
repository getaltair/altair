package com.getaltair.altair.db

/**
 * Configuration for SurrealDB database connection.
 *
 * Supports both network (server) and embedded (desktop) modes via sealed interface.
 */
sealed interface DatabaseConfig {
    /**
     * Network connection to a SurrealDB server instance.
     *
     * @property host The hostname or IP address
     * @property port The port number (default: 8000)
     * @property useTls Whether to use TLS/SSL (default: false for local development)
     * @property namespace The SurrealDB namespace
     * @property database The SurrealDB database name
     * @property username The username for authentication
     * @property password The password for authentication
     * @property poolSize The connection pool size (default: 10)
     */
    data class Network(
        val host: String,
        val port: Int = DEFAULT_PORT,
        val useTls: Boolean = false,
        val namespace: String,
        val database: String,
        val username: String,
        val password: String,
        val poolSize: Int = DEFAULT_POOL_SIZE,
    ) : DatabaseConfig {
        init {
            require(host.isNotBlank()) { "Host must not be blank" }
            require(port in 1..65535) { "Port must be between 1 and 65535" }
            require(namespace.isNotBlank()) { "Namespace must not be blank" }
            require(database.isNotBlank()) { "Database must not be blank" }
            require(username.isNotBlank()) { "Username must not be blank" }
            require(poolSize >= 1) { "Pool size must be at least 1" }
        }

        val connectionUrl: String
            get() {
                val protocol = if (useTls) "wss" else "ws"
                return "$protocol://$host:$port/rpc"
            }
    }

    /**
     * Embedded SurrealDB instance (desktop mode).
     *
     * Uses SurrealKV storage engine for in-process database.
     *
     * @property dataPath The filesystem path for database files
     * @property namespace The SurrealDB namespace
     * @property database The SurrealDB database name
     */
    data class Embedded(
        val dataPath: String,
        val namespace: String,
        val database: String,
    ) : DatabaseConfig {
        init {
            require(dataPath.isNotBlank()) { "Data path must not be blank" }
            require(namespace.isNotBlank()) { "Namespace must not be blank" }
            require(database.isNotBlank()) { "Database must not be blank" }
        }
    }

    companion object {
        const val DEFAULT_PORT = 8000
        const val DEFAULT_POOL_SIZE = 10
        const val DEFAULT_NAMESPACE = "altair"
        const val DEFAULT_DATABASE = "main"

        /**
         * Creates a Network configuration from environment variables.
         *
         * Environment variables:
         * - SURREALDB_HOST (required)
         * - SURREALDB_PORT (optional, default: 8000)
         * - SURREALDB_USE_TLS (optional, default: false)
         * - SURREALDB_NAMESPACE (optional, default: altair)
         * - SURREALDB_DATABASE (optional, default: main)
         * - SURREALDB_USERNAME (required)
         * - SURREALDB_PASSWORD (required)
         * - SURREALDB_POOL_SIZE (optional, default: 10)
         */
        fun fromEnvironment(): Network {
            val host =
                System.getenv("SURREALDB_HOST")
                    ?: error("SURREALDB_HOST environment variable is required")
            val port = System.getenv("SURREALDB_PORT")?.toIntOrNull() ?: DEFAULT_PORT
            val useTls = System.getenv("SURREALDB_USE_TLS")?.toBooleanStrictOrNull() ?: false
            val namespace = System.getenv("SURREALDB_NAMESPACE") ?: DEFAULT_NAMESPACE
            val database = System.getenv("SURREALDB_DATABASE") ?: DEFAULT_DATABASE
            val username =
                System.getenv("SURREALDB_USERNAME")
                    ?: error("SURREALDB_USERNAME environment variable is required")
            val password =
                System.getenv("SURREALDB_PASSWORD")
                    ?: error("SURREALDB_PASSWORD environment variable is required")
            val poolSize = System.getenv("SURREALDB_POOL_SIZE")?.toIntOrNull() ?: DEFAULT_POOL_SIZE

            return Network(
                host = host,
                port = port,
                useTls = useTls,
                namespace = namespace,
                database = database,
                username = username,
                password = password,
                poolSize = poolSize,
            )
        }
    }
}
