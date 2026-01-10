package com.getaltair.altair.data.db.repository

import com.getaltair.altair.data.db.SurrealDbConnection
import com.getaltair.altair.data.entity.TestEntity
import com.getaltair.altair.data.repository.Repository
import com.github.f4b6a3.ulid.UlidCreator
import com.surrealdb.Response
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import java.time.Instant
import java.time.format.DateTimeFormatter

/**
 * Repository implementation for TestEntity using SurrealDB.
 *
 * Provides CRUD operations with:
 * - ULID generation for new entities
 * - ISO 8601 timestamp handling
 * - Soft delete support
 * - Sync version tracking
 */
class TestRepository : Repository<TestEntity, String> {

    private val tableName = TestEntity.TABLE_NAME

    override suspend fun create(entity: TestEntity): TestEntity = withContext(Dispatchers.IO) {
        val driver = SurrealDbConnection.getDriver()
        val now = currentTimestamp()
        val id = if (entity.id.isBlank()) generateUlid() else entity.id

        val newEntity = entity.copy(
            id = id,
            createdAt = now,
            updatedAt = now,
            syncVersion = 0,
        )

        // Use raw query for reliable cross-version compatibility
        val query = buildString {
            append("CREATE $tableName:")
            append(id)
            append(" SET ")
            append("name = '${escapeString(newEntity.name)}', ")
            append("value = ${newEntity.value}, ")
            append("created_at = '${newEntity.createdAt}', ")
            append("updated_at = '${newEntity.updatedAt}', ")
            if (newEntity.deletedAt != null) {
                append("deleted_at = '${newEntity.deletedAt}', ")
            } else {
                append("deleted_at = NONE, ")
            }
            append("sync_version = ${newEntity.syncVersion}")
        }

        driver.query(query)

        newEntity
    }

    override suspend fun findById(id: String): TestEntity? = withContext(Dispatchers.IO) {
        val entity = findByIdInternal(id)
        // Return null if soft-deleted
        if (entity?.deletedAt != null) null else entity
    }

    override suspend fun update(entity: TestEntity): TestEntity = withContext(Dispatchers.IO) {
        val driver = SurrealDbConnection.getDriver()
        val now = currentTimestamp()

        val updatedEntity = entity.copy(
            updatedAt = now,
            syncVersion = entity.syncVersion + 1,
        )

        val query = buildString {
            append("UPDATE $tableName:")
            append(entity.id)
            append(" SET ")
            append("name = '${escapeString(updatedEntity.name)}', ")
            append("value = ${updatedEntity.value}, ")
            append("updated_at = '${updatedEntity.updatedAt}', ")
            if (updatedEntity.deletedAt != null) {
                append("deleted_at = '${updatedEntity.deletedAt}', ")
            }
            append("sync_version = ${updatedEntity.syncVersion}")
        }

        driver.query(query)

        updatedEntity
    }

    override suspend fun delete(id: String): Boolean = withContext(Dispatchers.IO) {
        val driver = SurrealDbConnection.getDriver()

        // First check if entity exists and is not already deleted
        val existing = findByIdInternal(id)
        if (existing == null || existing.deletedAt != null) {
            return@withContext false
        }

        val now = currentTimestamp()

        val query = buildString {
            append("UPDATE $tableName:")
            append(id)
            append(" SET ")
            append("deleted_at = '$now', ")
            append("updated_at = '$now'")
        }

        driver.query(query)
        true
    }

    override suspend fun findAll(): List<TestEntity> = withContext(Dispatchers.IO) {
        val driver = SurrealDbConnection.getDriver()

        try {
            val query = "SELECT * FROM $tableName WHERE deleted_at IS NONE"
            val response = driver.query(query)

            parseResponseToEntities(response)
        } catch (e: Exception) {
            println("[TestRepository] Error in findAll: ${e.message}")
            emptyList()
        }
    }

    /**
     * Finds an entity by ID including soft-deleted entities.
     * Useful for testing and recovery scenarios.
     */
    suspend fun findByIdIncludeDeleted(id: String): TestEntity? = withContext(Dispatchers.IO) {
        findByIdInternal(id)
    }

    /**
     * Internal find that doesn't filter soft-deleted entities.
     */
    private fun findByIdInternal(id: String): TestEntity? {
        val driver = SurrealDbConnection.getDriver()

        return try {
            val query = "SELECT * FROM $tableName:$id"
            val response = driver.query(query)

            val entities = parseResponseToEntities(response)
            entities.firstOrNull()
        } catch (e: Exception) {
            println("[TestRepository] Error in findByIdInternal: ${e.message}")
            null
        }
    }

    /**
     * Hard deletes an entity from the database.
     * Use with caution - this cannot be undone.
     */
    suspend fun hardDelete(id: String): Boolean = withContext(Dispatchers.IO) {
        val driver = SurrealDbConnection.getDriver()

        try {
            val query = "DELETE $tableName:$id"
            driver.query(query)
            true
        } catch (e: Exception) {
            println("[TestRepository] Error in hardDelete: ${e.message}")
            false
        }
    }

    /**
     * Deletes all test entities from the database.
     * Useful for test cleanup.
     */
    suspend fun deleteAll() = withContext(Dispatchers.IO) {
        val driver = SurrealDbConnection.getDriver()
        try {
            driver.query("DELETE $tableName")
        } catch (e: Exception) {
            println("[TestRepository] Error in deleteAll: ${e.message}")
        }
    }

    private fun generateUlid(): String = UlidCreator.getUlid().toString()

    private fun currentTimestamp(): String = DateTimeFormatter.ISO_INSTANT.format(Instant.now())

    /**
     * Escapes a string for safe inclusion in SurrealQL queries.
     * In SurrealQL single-quoted strings, only backslash and single quote need escaping.
     */
    private fun escapeString(value: String): String {
        return value
            .replace("\\", "\\\\")
            .replace("'", "\\'")
        // Note: Double quotes don't need escaping inside single-quoted strings in SurrealQL
    }

    /**
     * Parses a SurrealDB Response to a list of TestEntity.
     * The Response.take(index) returns the result at the given statement index.
     */
    @Suppress("UNCHECKED_CAST")
    private fun parseResponseToEntities(response: Response): List<TestEntity> {
        val entities = mutableListOf<TestEntity>()

        try {
            // Use reflection to understand the Response API and parse results
            // Response.take(0) returns the result of the first statement
            val result = response.take(0)

            // The result type depends on the SurrealDB SDK version
            // Try to iterate if it's iterable, otherwise convert via toString
            when (result) {
                is Iterable<*> -> {
                    for (item in result) {
                        val entity = convertToEntity(item)
                        if (entity != null) {
                            entities.add(entity)
                        }
                    }
                }

                is Iterator<*> -> {
                    while (result.hasNext()) {
                        val entity = convertToEntity(result.next())
                        if (entity != null) {
                            entities.add(entity)
                        }
                    }
                }

                is Array<*> -> {
                    for (item in result) {
                        val entity = convertToEntity(item)
                        if (entity != null) {
                            entities.add(entity)
                        }
                    }
                }

                else -> {
                    // SurrealDB Value type - check toString representation
                    val str = result.toString().trim()
                    if (str.startsWith("[") && str.endsWith("]")) {
                        // It's an array like [{ ... }, { ... }]
                        val arrayEntities = parseArrayString(str)
                        entities.addAll(arrayEntities)
                    } else if (str.startsWith("{") && str.endsWith("}")) {
                        // Single object
                        val entity = parseJsonLikeString(str)
                        if (entity != null) {
                            entities.add(entity)
                        }
                    } else {
                        // Try direct conversion as last resort
                        val entity = convertToEntity(result)
                        if (entity != null) {
                            entities.add(entity)
                        }
                    }
                }
            }
        } catch (e: Exception) {
            println("[TestRepository] Error parsing response: ${e.message}")
            e.printStackTrace()
        }

        return entities
    }

    /**
     * Converts a result item to TestEntity.
     */
    @Suppress("UNCHECKED_CAST")
    private fun convertToEntity(item: Any?): TestEntity? {
        if (item == null) return null

        return try {
            // The item should be a Map-like structure from SurrealDB
            when (item) {
                is Map<*, *> -> {
                    val map = item as Map<String, Any?>
                    mapToEntity(map)
                }

                else -> {
                    // Try to access it as a Value object via reflection or toString parsing
                    val str = item.toString()
                    // SurrealDB Value can be an array like [{ ... }] or a single object { ... }
                    val trimmed = str.trim()
                    when {
                        trimmed.startsWith("[") && trimmed.endsWith("]") -> {
                            // It's an array - parse items
                            null // This will be handled at the parseResponseToEntities level
                        }

                        trimmed.startsWith("{") && trimmed.endsWith("}") -> {
                            parseJsonLikeString(trimmed)
                        }

                        else -> null
                    }
                }
            }
        } catch (e: Exception) {
            println("[TestRepository] Error converting item: ${e.message}")
            null
        }
    }

    /**
     * Maps a Map to TestEntity.
     */
    private fun mapToEntity(map: Map<String, Any?>): TestEntity {
        // Extract ID from SurrealDB Thing format (e.g., "test_entity:01ABC...")
        val rawId = map["id"]?.toString() ?: ""
        val id = if (rawId.contains(":")) {
            rawId.substringAfter(":")
        } else {
            rawId
        }

        return TestEntity(
            id = id,
            name = map["name"]?.toString() ?: "",
            value = (map["value"] as? Number)?.toInt() ?: map["value"]?.toString()?.toIntOrNull() ?: 0,
            createdAt = map["created_at"]?.toString() ?: "",
            updatedAt = map["updated_at"]?.toString() ?: "",
            deletedAt = map["deleted_at"]?.toString()?.takeIf { it != "null" && it != "NONE" },
            syncVersion =
            (map["sync_version"] as? Number)?.toInt() ?: map["sync_version"]?.toString()?.toIntOrNull() ?: 0,
        )
    }

    /**
     * Parses a JSON-like string to TestEntity.
     * This is a fallback for when the SDK returns string representation.
     */
    private fun parseJsonLikeString(str: String): TestEntity? = try {
        // Simple parsing for JSON-like format
        val content = str.trim('{', '}', ' ')
        val pairs = mutableMapOf<String, String>()

        // Split by comma, but be careful of nested structures and quoted strings
        var current = StringBuilder()
        var depth = 0
        var inQuote = false
        var quoteChar = ' '

        for (char in content) {
            when {
                !inQuote && (char == '\'' || char == '"') -> {
                    inQuote = true
                    quoteChar = char
                    current.append(char)
                }

                inQuote && char == quoteChar -> {
                    inQuote = false
                    current.append(char)
                }

                !inQuote && (char == '{' || char == '[') -> {
                    depth++
                    current.append(char)
                }

                !inQuote && (char == '}' || char == ']') -> {
                    depth--
                    current.append(char)
                }

                !inQuote && char == ',' && depth == 0 -> {
                    parseKeyValue(current.toString())?.let { (k, v) -> pairs[k] = v }
                    current = StringBuilder()
                }

                else -> current.append(char)
            }
        }
        // Don't forget the last pair
        parseKeyValue(current.toString())?.let { (k, v) -> pairs[k] = v }

        val rawId = pairs["id"] ?: ""
        val id = if (rawId.contains(":")) rawId.substringAfter(":") else rawId

        TestEntity(
            id = trimQuotes(id),
            name = trimQuotes(pairs["name"] ?: ""),
            value = trimQuotes(pairs["value"] ?: "0").toIntOrNull() ?: 0,
            createdAt = trimQuotes(pairs["created_at"] ?: ""),
            updatedAt = trimQuotes(pairs["updated_at"] ?: ""),
            deletedAt = trimQuotes(pairs["deleted_at"] ?: "").takeIf {
                it.isNotEmpty() && it != "null" &&
                    it != "NONE"
            },
            syncVersion = trimQuotes(pairs["sync_version"] ?: "0").toIntOrNull() ?: 0,
        )
    } catch (e: Exception) {
        println("[TestRepository] Error parsing JSON-like string: ${e.message}")
        null
    }

    /**
     * Removes surrounding single or double quotes from a string and unescapes.
     */
    private fun trimQuotes(str: String): String {
        val trimmed = str.trim()
        val unquoted = when {
            trimmed.startsWith("'") && trimmed.endsWith("'") -> trimmed.drop(1).dropLast(1)
            trimmed.startsWith("\"") && trimmed.endsWith("\"") -> trimmed.drop(1).dropLast(1)
            else -> trimmed
        }
        // Unescape common escape sequences
        return unquoted
            .replace("\\'", "'")
            .replace("\\\"", "\"")
            .replace("\\\\", "\\")
    }

    private fun parseKeyValue(str: String): Pair<String, String>? {
        val colonIndex = str.indexOf(':')
        if (colonIndex == -1) return null

        val key = str.substring(0, colonIndex).trim().trim('"')
        val value = str.substring(colonIndex + 1).trim()
        return key to value
    }

    /**
     * Parses an array string like [{ ... }, { ... }] into a list of TestEntity.
     */
    private fun parseArrayString(str: String): List<TestEntity> {
        val entities = mutableListOf<TestEntity>()

        try {
            // Remove outer brackets
            val content = str.trim().removePrefix("[").removeSuffix("]").trim()
            if (content.isEmpty()) return entities

            // Split by }, { pattern to get individual objects
            // Be careful with nested braces
            val objects = mutableListOf<String>()
            var current = StringBuilder()
            var depth = 0

            for (char in content) {
                when {
                    char == '{' -> {
                        if (depth == 0) {
                            current = StringBuilder()
                        }
                        depth++
                        current.append(char)
                    }

                    char == '}' -> {
                        depth--
                        current.append(char)
                        if (depth == 0) {
                            objects.add(current.toString())
                        }
                    }

                    depth > 0 -> {
                        current.append(char)
                    }
                    // Ignore characters between objects (like commas and spaces)
                }
            }

            // Parse each object
            for (objStr in objects) {
                val entity = parseJsonLikeString(objStr)
                if (entity != null) {
                    entities.add(entity)
                }
            }
        } catch (e: Exception) {
            println("[TestRepository] Error parsing array string: ${e.message}")
        }

        return entities
    }
}
