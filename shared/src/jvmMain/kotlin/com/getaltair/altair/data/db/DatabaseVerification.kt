package com.getaltair.altair.data.db

import com.getaltair.altair.data.db.repository.TestRepository
import com.getaltair.altair.data.entity.TestEntity
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext

/**
 * Database verification utilities for startup and health checks.
 *
 * Provides methods to verify database connectivity and data persistence
 * across application restarts.
 */
object DatabaseVerification {

    private const val PERSISTENCE_MARKER_NAME = "__altair_persistence_marker__"
    private const val PERSISTENCE_MARKER_VALUE = 42

    /**
     * Verifies basic database connectivity by performing a round-trip CRUD operation.
     *
     * @return VerificationResult indicating success or failure with details
     */
    suspend fun verifyConnectivity(): VerificationResult = withContext(Dispatchers.IO) {
        try {
            val repository = TestRepository()

            // Create a test record
            val testEntity = TestEntity(
                id = "",
                name = "connectivity_test_${System.currentTimeMillis()}",
                value = 999,
                createdAt = "",
                updatedAt = "",
            )

            val created = repository.create(testEntity)
            println("[DatabaseVerification] Created test entity: ${created.id}")

            // Read it back
            val found = repository.findById(created.id)
            if (found == null) {
                return@withContext VerificationResult.Failure(
                    "Failed to read back created entity",
                )
            }

            // Verify data matches
            if (found.name != created.name || found.value != created.value) {
                return@withContext VerificationResult.Failure(
                    "Data mismatch: expected name='${created.name}', value=${created.value}, " +
                        "got name='${found.name}', value=${found.value}",
                )
            }

            // Clean up - hard delete the test record
            repository.hardDelete(created.id)
            println("[DatabaseVerification] Connectivity verified successfully")

            VerificationResult.Success
        } catch (e: Exception) {
            println("[DatabaseVerification] Connectivity verification failed: ${e.message}")
            VerificationResult.Failure("Connectivity check failed: ${e.message}", e)
        }
    }

    /**
     * Checks if a persistence marker exists from a previous application session.
     *
     * @return true if marker exists (data persisted from previous session), false otherwise
     */
    suspend fun checkPersistenceMarker(): Boolean = withContext(Dispatchers.IO) {
        try {
            val repository = TestRepository()
            val marker = repository.findByIdIncludeDeleted(PERSISTENCE_MARKER_NAME)

            if (marker != null && marker.value == PERSISTENCE_MARKER_VALUE) {
                println("[DatabaseVerification] Persistence marker found - data persists across restarts")
                true
            } else {
                println("[DatabaseVerification] No persistence marker found")
                false
            }
        } catch (e: Exception) {
            println("[DatabaseVerification] Error checking persistence marker: ${e.message}")
            false
        }
    }

    /**
     * Creates or updates a persistence marker to verify data persists across restarts.
     *
     * @return true if marker was created/updated successfully
     */
    suspend fun createPersistenceMarker(): Boolean = withContext(Dispatchers.IO) {
        try {
            val repository = TestRepository()

            // Check if marker already exists
            val existing = repository.findByIdIncludeDeleted(PERSISTENCE_MARKER_NAME)

            if (existing != null) {
                // Update existing marker
                repository.update(
                    existing.copy(
                        value = PERSISTENCE_MARKER_VALUE,
                        deletedAt = null, // Ensure it's not soft-deleted
                    ),
                )
                println("[DatabaseVerification] Persistence marker updated")
            } else {
                // Create new marker
                repository.create(
                    TestEntity(
                        id = PERSISTENCE_MARKER_NAME,
                        name = "Altair Persistence Marker",
                        value = PERSISTENCE_MARKER_VALUE,
                        createdAt = "",
                        updatedAt = "",
                    ),
                )
                println("[DatabaseVerification] Persistence marker created")
            }

            true
        } catch (e: Exception) {
            println("[DatabaseVerification] Error creating persistence marker: ${e.message}")
            false
        }
    }

    /**
     * Performs full startup verification including connectivity and persistence check.
     *
     * @return StartupVerificationResult with all verification details
     */
    suspend fun performStartupVerification(): StartupVerificationResult = withContext(Dispatchers.IO) {
        println("[DatabaseVerification] Starting database verification...")

        // Step 1: Verify connectivity
        val connectivityResult = verifyConnectivity()
        if (connectivityResult is VerificationResult.Failure) {
            return@withContext StartupVerificationResult(
                connectivityVerified = false,
                persistenceVerified = false,
                isFirstLaunch = true,
                errorMessage = connectivityResult.message,
            )
        }

        // Step 2: Check persistence marker
        val markerExists = checkPersistenceMarker()

        // Step 3: Create/update persistence marker for next launch
        createPersistenceMarker()

        println("[DatabaseVerification] Startup verification complete")

        StartupVerificationResult(
            connectivityVerified = true,
            persistenceVerified = markerExists,
            isFirstLaunch = !markerExists,
            errorMessage = null,
        )
    }
}

/**
 * Result of a single verification operation.
 */
sealed class VerificationResult {
    object Success : VerificationResult()
    data class Failure(val message: String, val cause: Throwable? = null) : VerificationResult()
}

/**
 * Result of full startup verification.
 */
data class StartupVerificationResult(
    val connectivityVerified: Boolean,
    val persistenceVerified: Boolean,
    val isFirstLaunch: Boolean,
    val errorMessage: String?,
) {
    val isSuccess: Boolean
        get() = connectivityVerified

    override fun toString(): String = buildString {
        append("StartupVerification: ")
        if (isSuccess) {
            append("SUCCESS")
            if (isFirstLaunch) {
                append(" (first launch)")
            } else {
                append(" (data persisted from previous session)")
            }
        } else {
            append("FAILED - $errorMessage")
        }
    }
}
