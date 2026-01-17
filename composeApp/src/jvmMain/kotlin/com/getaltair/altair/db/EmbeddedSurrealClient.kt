package com.getaltair.altair.db

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.domain.DomainError
import com.surrealdb.Surreal
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.coroutines.withContext
import org.slf4j.LoggerFactory
import java.nio.file.Files

/**
 * Client for embedded SurrealDB on desktop.
 *
 * Uses SurrealKV storage engine for persistent local storage with
 * graph query and vector search capabilities. All operations are
 * mutex-protected for thread safety.
 */
class EmbeddedSurrealClient(
    private val config: DesktopDatabaseConfig,
) {
    private val logger = LoggerFactory.getLogger(EmbeddedSurrealClient::class.java)
    private var surreal: Surreal? = null
    private val mutex = Mutex()

    /**
     * Connects to the embedded SurrealDB database.
     *
     * Creates the data directory if it doesn't exist and initializes
     * the database with the configured namespace and database.
     *
     * @return Either an error if connection fails, or Unit on success
     */
    suspend fun connect(): Either<DomainError, Unit> =
        withContext(Dispatchers.IO) {
            mutex.withLock {
                try {
                    // Ensure data directory exists
                    Files.createDirectories(config.dataDirectory)
                    Files.createDirectories(config.databasePath)

                    val db = Surreal()
                    db.connect(config.connectionUrl)
                    db.useNs(config.namespace)?.useDb(config.database)

                    surreal = db
                    logger.info("Connected to embedded SurrealDB at ${config.databasePath}")
                    Unit.right()
                } catch (e: Exception) {
                    logger.error("Failed to connect to embedded database", e)
                    DomainError.UnexpectedError("Failed to connect to embedded database: ${e.message}", e).left()
                }
            }
        }

    /**
     * Closes the database connection.
     */
    suspend fun close() =
        withContext(Dispatchers.IO) {
            mutex.withLock {
                try {
                    surreal?.close()
                } catch (e: Exception) {
                    logger.error("Error closing embedded SurrealDB connection", e)
                } finally {
                    surreal = null
                    logger.info("Disconnected from embedded SurrealDB")
                }
            }
        }

    /**
     * Executes a SurrealQL query and returns the result as a string.
     *
     * @param query The SurrealQL query to execute
     * @return Either an error if the query fails, or the JSON result string
     */
    suspend fun <T> query(query: String): Either<DomainError, String> =
        withContext(Dispatchers.IO) {
            mutex.withLock {
                try {
                    val db = surreal ?: return@withContext DomainError.UnexpectedError("Not connected").left()
                    val response = db.query(query)
                    response?.toString()?.right() ?: "[]".right()
                } catch (e: Exception) {
                    logger.error("Query failed: $query", e)
                    DomainError.UnexpectedError("Query failed: ${e.message}", e).left()
                }
            }
        }

    /**
     * Executes a SurrealQL statement (CREATE, UPDATE, DELETE).
     *
     * @param statement The SurrealQL statement to execute
     * @return Either an error if execution fails, or Unit on success
     */
    suspend fun execute(statement: String): Either<DomainError, Unit> =
        withContext(Dispatchers.IO) {
            mutex.withLock {
                try {
                    val db = surreal ?: return@withContext DomainError.UnexpectedError("Not connected").left()
                    db.query(statement)
                    Unit.right()
                } catch (e: Exception) {
                    logger.error("Execute failed: $statement", e)
                    DomainError.UnexpectedError("Execute failed: ${e.message}", e).left()
                }
            }
        }

    /**
     * Checks if the client is currently connected.
     */
    suspend fun isConnected(): Boolean =
        mutex.withLock {
            surreal != null
        }
}
