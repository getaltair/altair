package com.getaltair.altair.db

import arrow.core.Either
import arrow.core.raise.either
import arrow.core.raise.ensure
import com.getaltair.altair.domain.DomainError
import com.surrealdb.Response
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
 * Provides coroutine-friendly access to SurrealDB with mutex-based
 * thread safety for concurrent access. Supports both network and
 * embedded connection modes.
 */
@Suppress("TooManyFunctions") // Contains necessary Response-to-JSON conversion helpers
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
     * Executes a SurrealQL query.
     *
     * Note: Query parameters should be embedded directly in the query string
     * with proper escaping. The repositories handle escaping for user-provided values.
     *
     * @param query The SurrealQL query string with parameters embedded
     * @return Either an error or the raw result string (JSON array)
     */
    suspend fun <T> query(query: String): Either<DomainError, String> =
        either {
            ensure(isConnected) {
                DomainError.UnexpectedError("Database not connected")
            }

            withContext(Dispatchers.IO) {
                try {
                    val db = surreal ?: raise(DomainError.UnexpectedError("Database client is null"))
                    val response = db.query(query)
                    convertResponseToJson(response)
                } catch (e: Exception) {
                    logger.error("Query failed: $query", e)
                    raise(DomainError.UnexpectedError("Query failed: ${e.message}", e))
                }
            }
        }

    /**
     * Converts a SurrealDB Response to a JSON string array.
     *
     * Uses the SDK's Value API to properly extract and serialize results.
     */
    private fun convertResponseToJson(response: Response?): String {
        if (response == null || response.size() == 0) {
            return "[]"
        }

        val result = response.take(0)
        if (result.isNone() || result.isNull()) {
            return "[]"
        }

        return if (result.isArray()) {
            val array = result.getArray()
            val items =
                buildList {
                    val iter = array.synchronizedIterator()
                    while (iter.hasNext()) {
                        add(convertValueToJson(iter.next()))
                    }
                }
            "[${items.joinToString(",")}]"
        } else {
            // Single object result, wrap in array
            "[${convertValueToJson(result)}]"
        }
    }

    /**
     * Converts a single Value to its JSON representation.
     */
    @Suppress("CyclomaticComplexMethod") // Necessary to handle all Value types
    private fun convertValueToJson(value: com.surrealdb.Value): String =
        when {
            value.isNone() || value.isNull() -> "null"
            value.isBoolean() -> value.getBoolean().toString()
            value.isLong() -> value.getLong().toString()
            value.isDouble() -> value.getDouble().toString()
            value.isDuration() -> "\"${value.getDuration()}\""
            value.isString() -> "\"${escapeJsonString(value.getString())}\""
            value.isThing() -> "\"${value.getThing()}\""
            value.isUuid() -> "\"${value.getUuid()}\""
            value.isBytes() -> "\"${java.util.Base64.getEncoder().encodeToString(value.getBytes())}\""
            value.isArray() -> {
                val array = value.getArray()
                val items =
                    buildList {
                        val iter = array.synchronizedIterator()
                        while (iter.hasNext()) {
                            add(convertValueToJson(iter.next()))
                        }
                    }
                "[${items.joinToString(",")}]"
            }
            value.isObject() -> {
                val obj = value.getObject()
                val entries =
                    buildList {
                        val iter = obj.synchronizedIterator()
                        while (iter.hasNext()) {
                            val entry = iter.next()
                            add("\"${entry.key}\":${convertValueToJson(entry.value)}")
                        }
                    }
                "{${entries.joinToString(",")}}"
            }
            else -> {
                // Handle unknown types (including datetime which returns as d'...' format)
                val str = value.toString()
                val result = extractDatetimeIfPresent(str) ?: escapeJsonString(str)
                logger.warn("Unhandled Value type, converting to string: {}", value)
                "\"$result\""
            }
        }

    /**
     * Extracts ISO datetime string from SurrealDB datetime format (d'...' or d"...").
     * Returns null if the string is not a datetime format.
     */
    private fun extractDatetimeIfPresent(str: String): String? =
        when {
            str.startsWith("d'") && str.endsWith("'") -> str.substring(2, str.length - 1)
            str.startsWith("d\"") && str.endsWith("\"") -> str.substring(2, str.length - 1)
            else -> null
        }

    /**
     * Escapes special characters in a JSON string.
     */
    private fun escapeJsonString(s: String): String =
        s
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\t", "\\t")

    /**
     * Executes a SurrealQL query and deserializes the result.
     *
     * @param query The SurrealQL query string with parameters embedded
     * @param deserializer Function to deserialize the JSON result
     * @return Either an error or the deserialized result
     */
    suspend fun <T> queryAs(
        query: String,
        deserializer: (String) -> T,
    ): Either<DomainError, T> =
        either {
            val result = query<Any>(query).bind()
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
        } catch (e: Exception) {
            logger.warn("Error closing SurrealDB connection", e)
        } finally {
            surreal = null
            isConnected = false
            logger.info("Disconnected from SurrealDB")
        }
    }
}
