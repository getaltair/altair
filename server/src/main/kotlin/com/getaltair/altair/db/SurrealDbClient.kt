package com.getaltair.altair.db

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.ensure
import com.getaltair.altair.domain.DomainError
import com.surrealdb.Surreal
import com.surrealdb.signin.Root
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import kotlinx.serialization.json.Json
import org.slf4j.LoggerFactory

/**
 * Wrapper around SurrealDB Java client with connection management.
 *
 * Provides coroutine-friendly access to SurrealDB with automatic
 * connection pooling for network mode and direct access for embedded mode.
 */
class SurrealDbClient(
    private val config: DatabaseConfig,
) : AutoCloseable {
    private val logger = LoggerFactory.getLogger(SurrealDbClient::class.java)
    private val mutex = Mutex()
    private var surreal: Surreal? = null
    private var isConnected = false

    val json =
        Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
            isLenient = true
        }

    /**
     * Connects to the database.
     *
     * For network config, establishes WebSocket connection.
     * For embedded config, initializes local database.
     */
    suspend fun connect(): Either<DomainError, Unit> =
        either {
            mutex.withLock {
                if (isConnected) return@either

                withContext(Dispatchers.IO) {
                    try {
                        surreal = Surreal()
                        when (config) {
                            is DatabaseConfig.Network -> connectNetwork(config)
                            is DatabaseConfig.Embedded -> connectEmbedded(config)
                        }
                        isConnected = true
                        logger.info("Connected to SurrealDB")
                    } catch (e: Exception) {
                        logger.error("Failed to connect to SurrealDB", e)
                        raise(DomainError.NetworkError("Failed to connect to database: ${e.message}", e))
                    }
                }
            }
        }

    private fun connectNetwork(config: DatabaseConfig.Network) {
        surreal?.connect(config.connectionUrl)
        surreal?.signin(Root(config.username, config.password))
        surreal?.useNs(config.namespace)?.useDb(config.database)
    }

    private fun connectEmbedded(config: DatabaseConfig.Embedded) {
        surreal?.connect("surrealkv://${config.dataPath}")
        surreal?.useNs(config.namespace)?.useDb(config.database)
    }

    /**
     * Executes a SurrealQL query with parameters.
     *
     * @param query The SurrealQL query string
     * @param params Optional query parameters
     * @return Either an error or the raw result string
     */
    suspend fun <T> query(
        query: String,
        params: Map<String, Any?> = emptyMap(),
    ): Either<DomainError, String> =
        either {
            ensure(isConnected) {
                DomainError.UnexpectedError("Database not connected")
            }

            withContext(Dispatchers.IO) {
                try {
                    val db = surreal ?: raise(DomainError.UnexpectedError("Database client is null"))
                    val response = db.query(query)
                    // The Response class returns results - convert to JSON string
                    response?.toString() ?: "[]"
                } catch (e: Exception) {
                    logger.error("Query failed: $query", e)
                    raise(DomainError.UnexpectedError("Query failed: ${e.message}", e))
                }
            }
        }

    /**
     * Executes a SurrealQL query and deserializes the result.
     *
     * @param query The SurrealQL query string
     * @param params Optional query parameters
     * @param deserializer Function to deserialize the JSON result
     * @return Either an error or the deserialized result
     */
    suspend fun <T> queryAs(
        query: String,
        params: Map<String, Any?> = emptyMap(),
        deserializer: (String) -> T,
    ): Either<DomainError, T> =
        either {
            val result = query<Any>(query, params).bind()
            try {
                deserializer(result)
            } catch (e: Exception) {
                logger.error("Failed to deserialize query result", e)
                raise(DomainError.UnexpectedError("Failed to deserialize result: ${e.message}", e))
            }
        }

    /**
     * Executes a raw SurrealQL statement (for DDL operations like CREATE TABLE).
     */
    suspend fun execute(statement: String): Either<DomainError, Unit> =
        either {
            ensure(isConnected) {
                DomainError.UnexpectedError("Database not connected")
            }

            withContext(Dispatchers.IO) {
                try {
                    val db = surreal ?: raise(DomainError.UnexpectedError("Database client is null"))
                    db.query(statement)
                } catch (e: Exception) {
                    logger.error("Execute failed: $statement", e)
                    raise(DomainError.UnexpectedError("Execute failed: ${e.message}", e))
                }
            }
        }

    /**
     * Checks if the database connection is healthy.
     */
    suspend fun healthCheck(): Either<DomainError, Boolean> =
        either {
            query<Any>("SELECT * FROM ONLY none LIMIT 1").map { true }.bind()
        }

    override fun close() {
        try {
            surreal?.close()
            surreal = null
            isConnected = false
            logger.info("Disconnected from SurrealDB")
        } catch (e: Exception) {
            logger.warn("Error closing SurrealDB connection", e)
        }
    }
}
