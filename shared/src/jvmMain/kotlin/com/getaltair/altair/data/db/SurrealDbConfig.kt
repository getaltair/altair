package com.getaltair.altair.data.db

/**
 * Configuration for SurrealDB connection.
 *
 * @property namespace The SurrealDB namespace to use (default: "altair")
 * @property database The SurrealDB database name to use (default: "main")
 * @property dataPath The file path for SurrealKV storage. If null, uses in-memory mode.
 */
data class SurrealDbConfig(
    val namespace: String = DEFAULT_NAMESPACE,
    val database: String = DEFAULT_DATABASE,
    val dataPath: String? = null,
) {
    companion object {
        const val DEFAULT_NAMESPACE = "altair"
        const val DEFAULT_DATABASE = "main"

        /**
         * Creates a configuration for embedded file-based storage.
         *
         * @param dataPath The directory path for SurrealKV storage
         * @return SurrealDbConfig configured for file-based storage
         */
        fun embedded(dataPath: String): SurrealDbConfig = SurrealDbConfig(dataPath = dataPath)

        /**
         * Creates a configuration for in-memory storage.
         * Data will not persist across application restarts.
         *
         * @return SurrealDbConfig configured for in-memory storage
         */
        fun memory(): SurrealDbConfig = SurrealDbConfig(dataPath = null)
    }

    /**
     * Returns the connection string for SurrealDB.
     * - For file-based storage: "surrealkv://<dataPath>"
     * - For in-memory storage: "memory"
     */
    fun connectionString(): String = if (dataPath != null) {
        "surrealkv://$dataPath"
    } else {
        "memory"
    }

    /**
     * Whether this configuration uses persistent storage.
     */
    val isPersistent: Boolean
        get() = dataPath != null
}
