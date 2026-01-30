package com.getaltair.altair.persistence

import arrow.core.Either
import arrow.core.left
import arrow.core.right
import com.getaltair.altair.shared.domain.error.AltairError
import com.surrealdb.Surreal
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.io.File
import kotlin.reflect.KClass

/**
 * Desktop SurrealDB client using embedded file-based storage.
 * No external service required - runs entirely locally.
 */
class DesktopSurrealDbClient(private val config: DesktopDatabaseConfig) {

    private var db: Surreal? = null

    suspend fun connect(): Either<AltairError, Unit> = withContext(Dispatchers.IO) {
        try {
            // Ensure storage directory exists
            File(config.storagePath).mkdirs()

            val surreal = Surreal()
            surreal.connect(config.connectionUrl)
            surreal.useNs(config.namespace).useDb(config.database)
            db = surreal

            // Run migrations
            runMigrationsFromResources()

            Unit.right()
        } catch (e: Exception) {
            AltairError.StorageError.DatabaseError(
                message = "Failed to connect to embedded SurrealDB: ${e.message}"
            ).left()
        }
    }

    private fun runMigrationsFromResources() {
        val migrationFiles = listOf("V001__desktop_schema.surql")
        val surreal = requireConnection()

        migrationFiles.forEach { filename ->
            val sql = javaClass.classLoader
                .getResourceAsStream("db/migrations/$filename")
                ?.bufferedReader()
                ?.readText()

            if (sql != null) {
                surreal.query(sql)
            }
        }
    }

    suspend fun disconnect() {
        withContext(Dispatchers.IO) {
            db?.close()
            db = null
        }
    }

    fun isConnected(): Boolean = db != null

    private fun requireConnection(): Surreal {
        return db ?: throw IllegalStateException("Not connected to database. Call connect() first.")
    }

    private fun bindParams(sql: String, params: Map<String, Any?>): String {
        var boundSql = sql
        params.forEach { (key, value) ->
            val valueStr = when (value) {
                null -> "NONE"
                is String -> "'${value.replace("'", "\\'")}'"
                is Number -> value.toString()
                is Boolean -> value.toString()
                is List<*> -> value.joinToString(", ", "[", "]") { item ->
                    when (item) {
                        is String -> "'${item.replace("'", "\\'")}'"
                        else -> item.toString()
                    }
                }
                else -> "'${value.toString().replace("'", "\\'")}'"
            }
            boundSql = boundSql.replace("\$$key", valueStr)
        }
        return boundSql
    }

    suspend fun <T : Any> query(
        sql: String,
        params: Map<String, Any?> = emptyMap(),
        type: KClass<T>
    ): Either<AltairError, List<T>> = withContext(Dispatchers.IO) {
        try {
            val surreal = requireConnection()
            val boundSql = if (params.isEmpty()) sql else bindParams(sql, params)
            surreal.query(boundSql)
            // TODO: Properly deserialize based on SDK version
            emptyList<T>().right()
        } catch (e: Exception) {
            AltairError.StorageError.DatabaseError(
                message = "Query failed: ${e.message}"
            ).left()
        }
    }

    suspend fun <T : Any> queryOne(
        sql: String,
        params: Map<String, Any?> = emptyMap(),
        type: KClass<T>
    ): Either<AltairError, T?> = query(sql, params, type).map { it.firstOrNull() }

    suspend fun <T : Any> create(table: String, data: T): Either<AltairError, T> =
        withContext(Dispatchers.IO) {
            try {
                val surreal = requireConnection()
                surreal.create(table, data)
                data.right()
            } catch (e: Exception) {
                AltairError.StorageError.DatabaseError(
                    message = "Create failed: ${e.message}"
                ).left()
            }
        }

    suspend fun <T : Any> update(table: String, id: String, data: T): Either<AltairError, T> =
        withContext(Dispatchers.IO) {
            try {
                val surreal = requireConnection()
                val sql = "UPDATE $table:$id CONTENT \$data RETURN AFTER"
                surreal.query(bindParams(sql, mapOf("data" to data)))
                data.right()
            } catch (e: Exception) {
                AltairError.StorageError.DatabaseError(
                    message = "Update failed: ${e.message}"
                ).left()
            }
        }

    suspend fun delete(table: String, id: String): Either<AltairError, Unit> =
        withContext(Dispatchers.IO) {
            try {
                val surreal = requireConnection()
                surreal.delete("$table:$id")
                Unit.right()
            } catch (e: Exception) {
                AltairError.StorageError.DatabaseError(
                    message = "Delete failed: ${e.message}"
                ).left()
            }
        }

    suspend fun runMigration(sql: String): Either<AltairError, Unit> =
        withContext(Dispatchers.IO) {
            try {
                val surreal = requireConnection()
                surreal.query(sql)
                Unit.right()
            } catch (e: Exception) {
                AltairError.StorageError.DatabaseError(
                    message = "Migration failed: ${e.message}"
                ).left()
            }
        }
}
