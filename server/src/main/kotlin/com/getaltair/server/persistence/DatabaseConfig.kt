package com.getaltair.server.persistence

/**
 * Configuration for SurrealDB connection.
 *
 * All values are configurable via environment variables with sensible
 * defaults for local development. In production, these should be set
 * via environment variables or a secrets manager.
 *
 * @property host SurrealDB host (default: localhost)
 * @property port SurrealDB port (default: 8000)
 * @property namespace SurrealDB namespace (default: altair)
 * @property database SurrealDB database name (default: main)
 * @property username Database username (default: root for dev)
 * @property password Database password (default: root for dev)
 */
data class DatabaseConfig(
    val host: String = System.getenv("SURREAL_HOST") ?: "localhost",
    val port: Int = System.getenv("SURREAL_PORT")?.toIntOrNull() ?: 8000,
    val namespace: String = System.getenv("SURREAL_NAMESPACE") ?: "altair",
    val database: String = System.getenv("SURREAL_DATABASE") ?: "main",
    val username: String = System.getenv("SURREAL_USERNAME") ?: "root",
    val password: String = System.getenv("SURREAL_PASSWORD") ?: "root"
) {
    /**
     * WebSocket connection URL for SurrealDB.
     */
    val connectionUrl: String get() = "ws://$host:$port"

    /**
     * HTTP connection URL for SurrealDB REST API.
     */
    val httpUrl: String get() = "http://$host:$port"
}
