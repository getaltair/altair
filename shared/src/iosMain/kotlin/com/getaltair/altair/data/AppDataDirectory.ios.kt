package com.getaltair.altair.data

/**
 * iOS stub implementation of getAppDataDirectory.
 *
 * iOS SurrealDB integration is out of scope for SPEC-DB-001 (Desktop SurrealDB Integration).
 * This stub exists to satisfy the Kotlin Multiplatform expect/actual requirement and
 * allow the project to build successfully.
 *
 * Future implementation notes for iOS:
 * - Would use NSSearchPathForDirectoriesInDomains for Application Support directory
 * - Path pattern: ~/Library/Application Support/altair/db/
 *
 * @throws UnsupportedOperationException always, as iOS is not supported in SPEC-DB-001
 * @return Nothing - always throws
 */
actual fun getAppDataDirectory(): String {
    throw UnsupportedOperationException(
        "iOS SurrealDB not implemented - see SPEC-DB-001. " +
            "This SPEC covers Desktop (JVM) only."
    )
}
